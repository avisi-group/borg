//! BOOM AST pretty printing

use {
    crate::{
        boom::{
            bits_to_int,
            control_flow::{util::find_common_block, ControlFlowBlock, Terminator},
            visitor::Visitor,
            Ast, Definition, Expression, FunctionDefinition, FunctionSignature, Literal, NamedType,
            NamedValue, Operation, Parameter, Size, Statement, Type, Value,
        },
        Indent,
    },
    common::intern::InternedString,
    std::{
        cell::RefCell,
        io::Write,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print BOOM AST
pub fn print_ast<W: Write>(w: &mut W, ast: Rc<RefCell<Ast>>) {
    let Ast {
        definitions,
        registers,
        functions,
    } = &*ast.borrow();

    let mut visitor = PrettyPrinter::new(w);

    definitions
        .iter()
        .for_each(|def| visitor.visit_definition(def));

    registers.iter().for_each(|(name, typ)| {
        write!(visitor.writer, "register {name}: ").unwrap();
        visitor.visit_type(typ.clone());
        writeln!(visitor.writer).unwrap();
    });

    functions
        .iter()
        .for_each(|(_, fundef)| visitor.visit_function_definition(fundef));
}

/// Pretty-print BOOM statement
pub fn print_statement<W: Write>(w: &mut W, statement: Rc<RefCell<Statement>>) {
    let mut visitor = PrettyPrinter::new(w);
    visitor.visit_statement(statement);
}

/// Pretty-print BOOM AST
pub struct PrettyPrinter<'writer, W> {
    indent: Rc<AtomicUsize>,
    writer: &'writer mut W,
}

impl<'writer, W: Write> PrettyPrinter<'writer, W> {
    /// Creates a new `BoomPrettyPrinter` with the supplied writer
    pub fn new(writer: &'writer mut W) -> Self {
        Self {
            indent: Rc::new(AtomicUsize::new(0)),
            writer,
        }
    }
}

impl<'writer, W: Write> PrettyPrinter<'writer, W> {
    fn prindent<T: AsRef<str>>(&mut self, s: T) {
        write!(
            self.writer,
            "{}{}",
            PADDING.repeat(self.indent.load(Ordering::SeqCst)),
            s.as_ref()
        )
        .unwrap();
    }

    fn prindentln<T: AsRef<str>>(&mut self, s: T) {
        self.prindent(s);
        writeln!(self.writer).unwrap();
    }

    fn indent(&self) -> IndentHandle {
        self.indent.fetch_add(1, Ordering::SeqCst);
        IndentHandle {
            indent: self.indent.clone(),
        }
    }
}

struct IndentHandle {
    indent: Rc<AtomicUsize>,
}

impl Drop for IndentHandle {
    fn drop(&mut self) {
        self.indent.fetch_sub(1, Ordering::SeqCst);
    }
}

impl<'writer, W: Write> Visitor for PrettyPrinter<'writer, W> {
    fn visit_definition(&mut self, node: &Definition) {
        match node {
            Definition::Enum { name, variants } => {
                self.prindentln(format!("enum {name} {{"));

                {
                    let _h = self.indent();
                    variants
                        .iter()
                        .for_each(|id| self.prindentln(format!("{id},")));
                }

                self.prindentln("}");
            }
            Definition::Union { name, fields } => {
                self.prindentln(format!("union {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedType { name, typ }| {
                        self.prindent(format!("{name}: "));
                        self.visit_type(typ.clone());
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}");
            }
            Definition::Struct { name, fields } => {
                self.prindentln(format!("struct {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedType { name, typ }| {
                        self.prindent(format!("{name}: "));
                        self.visit_type(typ.clone());
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}");
            }
            Definition::Pragma { key, value } => {
                self.prindent(format!("#{key} {value}"));
            }
            Definition::Let { bindings, body } => {
                self.prindent("let (");

                let mut bindings = bindings.iter();
                if let Some(NamedType { name, .. }) = bindings.next() {
                    write!(self.writer, "{name}").unwrap();
                }
                for NamedType { name, .. } in bindings {
                    write!(self.writer, ", ").unwrap();
                    write!(self.writer, "{name}").unwrap();
                }

                writeln!(self.writer, ") {{").unwrap();

                {
                    let _h = self.indent();
                    body.iter()
                        .for_each(|statement| self.visit_statement(statement.clone()));
                }

                writeln!(self.writer, "}}").unwrap();
            }
        }
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.visit_function_signature(&node.signature);

        node.entry_block.iter().for_each(|b| {
            writeln!(self.writer, "    {}:", b).unwrap();
            {
                b.statements().iter().for_each(|stmt| {
                    write!(self.writer, "        ").unwrap();
                    self.visit_statement(stmt.clone());
                    writeln!(self.writer).unwrap();
                });

                match b.terminator() {
                    Terminator::Return(None) => writeln!(self.writer, "        return;").unwrap(),
                    Terminator::Return(Some(value)) => {
                        write!(self.writer, "        return ").unwrap();
                        self.visit_value(Rc::new(RefCell::new(value)));
                        writeln!(self.writer, ";").unwrap();
                    }
                    Terminator::Conditional {
                        condition,
                        target,
                        fallthrough,
                    } => {
                        write!(self.writer, "        if (").unwrap();
                        self.visit_value(Rc::new(RefCell::new(condition)));
                        writeln!(self.writer, ") {{").unwrap();
                        writeln!(self.writer, "            goto {target};").unwrap();
                        writeln!(self.writer, "        }} else {{").unwrap();
                        writeln!(self.writer, "            goto {fallthrough};").unwrap();
                        writeln!(self.writer, "        }}").unwrap();
                    }
                    Terminator::Unconditional { target } => {
                        writeln!(self.writer, "        goto {target};").unwrap();
                    }
                }
            }
            writeln!(self.writer).unwrap();
        });

        writeln!(self.writer, "}}").unwrap();
    }

    fn visit_function_signature(
        &mut self,
        FunctionSignature {
            name,
            parameters,
            return_type,
        }: &FunctionSignature,
    ) {
        self.prindent(format!("fn {}(", name));

        let parameters = parameters.borrow();
        let mut parameters = parameters.iter();
        if let Some(param) = parameters.next() {
            self.visit_parameter(param);
        }
        for param in parameters {
            write!(self.writer, ", ").unwrap();
            self.visit_parameter(param);
        }

        write!(self.writer, ") -> ").unwrap();
        self.visit_type(return_type.clone());

        writeln!(self.writer, " {{").unwrap();
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        match &*node.borrow() {
            Statement::TypeDeclaration { name, typ } => {
                self.visit_type(typ.clone());
                write!(self.writer, " {name};").unwrap();
            }

            Statement::Copy { expression, value } => {
                self.visit_expression(expression);
                write!(self.writer, " = ").unwrap();
                self.visit_value(value.clone());
                write!(self.writer, ";").unwrap();
            }

            Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                if let Some(expression) = expression {
                    self.visit_expression(expression);
                    write!(self.writer, " = ").unwrap();
                }

                write!(self.writer, "{name}(").unwrap();

                let mut args = arguments.iter();
                if let Some(arg) = args.next() {
                    self.visit_value(arg.clone());
                }
                args.for_each(|arg| {
                    write!(self.writer, ", ").unwrap();
                    self.visit_value(arg.clone());
                });

                write!(self.writer, ");").unwrap();
            }

            Statement::Comment(str) => write!(self.writer, "// {str}").unwrap(),
            Statement::Label(str) => write!(self.writer, "// label({str})").unwrap(),
            Statement::Exit(str) => write!(self.writer, "// exit({str})").unwrap(),

            Statement::If { .. } | Statement::Jump { .. } | Statement::Goto(_) => {
                panic!(
                    "control flow statements should have been removed by this point: {:?}",
                    node
                )
            }

            Statement::End(_) => todo!(),
            Statement::Undefined => todo!(),
        }
    }

    fn visit_parameter(&mut self, node: &Parameter) {
        self.visit_type(node.typ.clone());

        if node.is_ref {
            write!(self.writer, "&").unwrap();
        }
        write!(self.writer, " {}", node.name).unwrap();
    }

    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        match &*node.borrow() {
            Type::Unit => write!(self.writer, "()"),
            Type::String => write!(self.writer, "str"),
            Type::Bool => write!(self.writer, "bool"),
            Type::Bit => write!(self.writer, "bit"),
            Type::Real => write!(self.writer, "real"),
            Type::Float => write!(self.writer, "float"),

            Type::Int { signed, size } => {
                match signed {
                    true => write!(self.writer, "i"),
                    false => write!(self.writer, "u"),
                }
                .unwrap();

                match size {
                    Size::Static(size) => write!(self.writer, "{size}").unwrap(),
                    Size::Runtime(value) => {
                        write!(self.writer, "rt(").unwrap();
                        self.visit_value(value.clone());
                        write!(self.writer, ")").unwrap();
                    }
                    Size::Unknown => write!(self.writer, "???").unwrap(),
                };

                Ok(())
            }
            Type::Constant(i) => write!(self.writer, "constant({i})"),
            Type::Enum { name, .. } | Type::Union { name, .. } | Type::Struct { name, .. } => {
                write!(self.writer, "{name}")
            }
            Type::List { element_type } => {
                write!(self.writer, "list<").unwrap();
                self.visit_type(element_type.clone());
                write!(self.writer, ">").unwrap();
                Ok(())
            }
            Type::Vector { element_type } => {
                write!(self.writer, "vec<").unwrap();
                self.visit_type(element_type.clone());
                write!(self.writer, ">").unwrap();
                Ok(())
            }
            Type::FixedVector {
                length,
                element_type,
            } => {
                write!(self.writer, "[").unwrap();
                self.visit_type(element_type.clone());
                write!(self.writer, "; {length}]").unwrap();
                Ok(())
            }
            Type::Reference(typ) => {
                write!(self.writer, "&").unwrap();
                self.visit_type(typ.clone());
                Ok(())
            }
        }
        .unwrap()
    }

    fn visit_value(&mut self, node: Rc<RefCell<Value>>) {
        fn write_uid<W: Write>(
            printer: &mut PrettyPrinter<'_, W>,
            id: InternedString,
            typs: &Vec<Rc<RefCell<Type>>>,
        ) {
            write!(printer.writer, "{id}").unwrap();

            if !typs.is_empty() {
                write!(printer.writer, "<").unwrap();

                let mut typs = typs.iter();
                if let Some(typ) = typs.next() {
                    printer.visit_type(typ.clone());
                }
                for typ in typs {
                    write!(printer.writer, ", ").unwrap();
                    printer.visit_type(typ.clone());
                }

                write!(printer.writer, ">").unwrap();
            }
        }

        match &*node.borrow() {
            Value::Identifier(ident) => write!(self.writer, "{ident}").unwrap(),
            Value::Literal(literal) => self.visit_literal(literal.clone()),
            Value::Operation(op) => self.visit_operation(op),
            Value::Struct { name, fields } => {
                write!(self.writer, "struct {name} {{").unwrap();

                for NamedValue { name, value } in fields {
                    write!(self.writer, "{name}: ").unwrap();
                    self.visit_value(value.clone());
                    write!(self.writer, ",").unwrap();
                }

                write!(self.writer, "}}").unwrap();
            }
            Value::Field { value, field_name } => {
                self.visit_value(value.clone());
                write!(self.writer, ".{field_name}").unwrap();
            }
            Value::CtorKind {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value.clone());
                write!(self.writer, " is ").unwrap();
                write_uid(self, *identifier, types);
            }
            Value::CtorUnwrap {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value.clone());
                write!(self.writer, " as ").unwrap();
                write_uid(self, *identifier, types);
            }
        }
    }

    fn visit_control_flow_block(&mut self, _: &ControlFlowBlock) {
        unreachable!()
    }

    fn visit_named_type(&mut self, NamedType { name, typ }: &NamedType) {
        write!(self.writer, "{name}: ").unwrap();
        self.visit_type(typ.clone());
        write!(self.writer, ", ").unwrap();
    }

    fn visit_named_value(&mut self, NamedValue { name, value }: &NamedValue) {
        write!(self.writer, "{name}: ").unwrap();
        self.visit_value(value.clone());
        write!(self.writer, ", ").unwrap();
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Identifier(ident) => write!(self.writer, "{ident}").unwrap(),
            Expression::Field { expression, field } => {
                self.visit_expression(&expression);
                write!(self.writer, ".{field}").unwrap();
            }
            Expression::Address(expression) => {
                write!(self.writer, "&").unwrap();
                self.visit_expression(&expression);
            }
        }
    }

    fn visit_literal(&mut self, node: Rc<RefCell<Literal>>) {
        match &*node.borrow() {
            Literal::Int(bi) => write!(self.writer, "{bi}"),
            Literal::Bits(bits) => write!(self.writer, "{}", bits_to_int(bits)),
            Literal::Bit(bit) => write!(self.writer, "{}", bit.value()),
            Literal::Bool(bool) => write!(self.writer, "{bool}"),
            Literal::String(s) => write!(self.writer, "{s:?}"),
            Literal::Unit => write!(self.writer, "()"),
            Literal::Reference(s) => write!(self.writer, "&{s}"),
        }
        .unwrap()
    }

    fn visit_operation(&mut self, node: &Operation) {
        fn emit_op2<W: Write>(
            printer: &mut PrettyPrinter<'_, W>,
            lhs: &Rc<RefCell<Value>>,
            rhs: &Rc<RefCell<Value>>,
            op: &str,
        ) {
            write!(printer.writer, "(").unwrap();
            printer.visit_value(lhs.clone());
            write!(printer.writer, " {op} ").unwrap();
            printer.visit_value(rhs.clone());
            write!(printer.writer, ")").unwrap();
        }

        match node {
            Operation::Not(value) => {
                write!(self.writer, "!").unwrap();
                self.visit_value(value.clone());
            }
            Operation::Complement(value) => {
                write!(self.writer, "~").unwrap();
                self.visit_value(value.clone());
            }
            Operation::Equal(lhs, rhs) => emit_op2(self, lhs, rhs, "=="),
            Operation::NotEqual(lhs, rhs) => emit_op2(self, lhs, rhs, "!="),
            Operation::LessThan(lhs, rhs) => emit_op2(self, lhs, rhs, "<"),
            Operation::GreaterThan(lhs, rhs) => emit_op2(self, lhs, rhs, ">"),
            Operation::LessThanOrEqual(lhs, rhs) => emit_op2(self, lhs, rhs, "<="),
            Operation::GreaterThanOrEqual(lhs, rhs) => emit_op2(self, lhs, rhs, ">="),
            Operation::Subtract(lhs, rhs) => emit_op2(self, lhs, rhs, "-"),
            Operation::Add(lhs, rhs) => emit_op2(self, lhs, rhs, "+"),
            Operation::Multiply(lhs, rhs) => emit_op2(self, lhs, rhs, "*"),
            Operation::Or(lhs, rhs) => emit_op2(self, lhs, rhs, "|"),
            Operation::Xor(lhs, rhs) => emit_op2(self, lhs, rhs, "^"),
            Operation::And(lhs, rhs) => emit_op2(self, lhs, rhs, "&"),
            Operation::Divide(lhs, rhs) => emit_op2(self, lhs, rhs, "/"),
            Operation::LeftShift(lhs, rhs) => emit_op2(self, lhs, rhs, "<<"),
            Operation::RightShift(lhs, rhs) => emit_op2(self, lhs, rhs, ">>"),

            Operation::RotateLeft(lhs, rhs) => emit_op2(self, lhs, rhs, "<<<"),
            Operation::RotateRight(lhs, rhs) => emit_op2(self, lhs, rhs, ">>>"),

            Operation::Cast(value, typ) => {
                self.visit_value(value.clone());
                write!(self.writer, " as ").unwrap();
                self.visit_type(typ.clone());
            }
        }
    }
}
