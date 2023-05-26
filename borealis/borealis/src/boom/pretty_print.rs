//! BOOM AST pretty printing

use {
    crate::boom::{
        visitor::Visitor, Ast, Definition, Expression, FunctionDefinition, FunctionSignature,
        Literal, NamedType, NamedValue, Operation, Statement, Type, Value,
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

/// Pretty-print JIB AST (sequence of definitions)
pub fn print_ast<W: Write>(
    w: &mut W,
    Ast {
        definitions,
        registers,
        functions,
    }: &Ast,
) {
    let mut visitor = BoomPrettyPrinter::new(w);

    definitions
        .iter()
        .for_each(|def| visitor.visit_definition(def));

    registers.iter().for_each(|(name, typ)| {
        write!(visitor.writer, "register {name}: ").unwrap();
        visitor.visit_type(typ);
        writeln!(visitor.writer).unwrap();
    });

    functions
        .iter()
        .for_each(|(_, fundef)| visitor.visit_function_definition(fundef));
}

/// Pretty-print JIB statement
pub fn print_statement<W: Write>(w: &mut W, statement: Rc<RefCell<Statement>>) {
    let mut visitor = BoomPrettyPrinter::new(w);
    visitor.visit_statement(statement);
}

/// Pretty-print JIB AST
pub struct BoomPrettyPrinter<'writer, W> {
    indent: Rc<AtomicUsize>,
    writer: &'writer mut W,
}

impl<'writer, W: Write> BoomPrettyPrinter<'writer, W> {
    /// Creates a new `BoomPrettyPrinter` with the supplied writer
    pub fn new(writer: &'writer mut W) -> Self {
        Self {
            indent: Rc::new(AtomicUsize::new(0)),
            writer,
        }
    }
}

impl<'writer, W: Write> BoomPrettyPrinter<'writer, W> {
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

    fn print_uid(&mut self, id: InternedString, typs: &Vec<Type>) {
        write!(self.writer, "{id}").unwrap();

        if !typs.is_empty() {
            write!(self.writer, "<").unwrap();

            let mut typs = typs.iter();
            if let Some(typ) = typs.next() {
                self.visit_type(typ);
            }
            for typ in typs {
                write!(self.writer, ", ").unwrap();
                self.visit_type(typ);
            }

            write!(self.writer, ">").unwrap();
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

impl<'writer, W: Write> Visitor for BoomPrettyPrinter<'writer, W> {
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
                        self.visit_type(typ);
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
                        self.visit_type(typ);
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

        let mut parameters = parameters.iter();
        if let Some(param) = parameters.next() {
            self.visit_named_type(param);
        }
        for param in parameters {
            write!(self.writer, ", ").unwrap();
            self.visit_named_type(param);
        }

        write!(self.writer, ") -> ").unwrap();
        self.visit_type(return_type);
    }

    fn visit_named_type(&mut self, NamedType { name, typ }: &NamedType) {
        write!(self.writer, "{name}: ").unwrap();
        self.visit_type(typ);
    }

    fn visit_named_value(&mut self, NamedValue { name, value }: &NamedValue) {
        write!(self.writer, "{name}: ").unwrap();
        self.visit_value(value);
    }

    fn visit_type(&mut self, node: &Type) {
        match node {
            Type::Unit => write!(self.writer, "()"),
            Type::Bool => write!(self.writer, "bool"),
            Type::String => write!(self.writer, "String"),
            Type::Real => write!(self.writer, "real"),
            Type::Float => write!(self.writer, "float"),
            Type::Constant(bi) => write!(self.writer, "constant<{bi}>"),
            Type::Lint => write!(self.writer, "int"),
            Type::Fint(size) => write!(self.writer, "int<{size}>"),
            Type::Fbits(size, _) => write!(self.writer, "bitvector<{size}>"),
            Type::Lbits(_) => write!(self.writer, "bitvector"),
            Type::Bit => write!(self.writer, "bit"),
            Type::Enum { name, .. } => write!(self.writer, "enum {name}"),
            Type::Union { name, .. } => write!(self.writer, "union {name}"),
            Type::Struct { name, .. } => write!(self.writer, "struct {name}"),
            Type::List { element_type } => {
                write!(self.writer, "list<").unwrap();
                self.visit_type(element_type);
                write!(self.writer, ">")
            }
            Type::Vector { element_type } => {
                write!(self.writer, "vec<").unwrap();
                self.visit_type(element_type);
                write!(self.writer, ">")
            }
            Type::FVector {
                length,
                element_type,
            } => {
                write!(self.writer, "fvec<{length}, ").unwrap();
                self.visit_type(element_type);
                write!(self.writer, ">")
            }
            Type::Reference(inner) => {
                write!(self.writer, "&").unwrap();
                self.visit_type(inner);
                Ok(())
            }
        }
        .unwrap();
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        match &*node.borrow() {
            Statement::TypeDeclaration { name, typ } => {
                self.prindent(format!("{name}: "));
                self.visit_type(typ);
                writeln!(self.writer).unwrap();
            }

            Statement::Try { body } => {
                self.prindentln("try {");
                {
                    let _h = self.indent();
                    body.iter().for_each(|s| self.visit_statement(s.clone()));
                }
                self.prindentln("}");
            }
            Statement::Copy { expression, value } => {
                self.prindent("");
                self.visit_expression(expression);
                write!(self.writer, " = ").unwrap();
                self.visit_value(value);
                writeln!(self.writer).unwrap();
            }
            Statement::Clear { identifier } => self.prindentln(format!("clear({identifier})")),
            Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                self.prindent("");
                self.visit_expression(expression);
                write!(self.writer, " = {name}(").unwrap();

                // print correct number of commas
                let mut args = arguments.iter();
                if let Some(arg) = args.next() {
                    self.visit_value(arg);
                }
                for arg in args {
                    write!(self.writer, ", ").unwrap();
                    self.visit_value(arg);
                }

                writeln!(self.writer, ")").unwrap();
            }
            Statement::Label(label) => self.prindentln(format!("label \"{label}\"")),
            Statement::Goto(label) => self.prindentln(format!("goto \"{label}\"")),
            Statement::Jump { condition, target } => {
                self.prindent(format!("jump {} ", target));
                self.visit_value(condition);
                writeln!(self.writer).unwrap();
            }
            Statement::End(label) => self.prindentln(format!("end \"{label}\"")),
            Statement::Undefined => self.prindentln("undefined"),
            Statement::If {
                condition,
                if_body,
                else_body,
            } => {
                self.prindent("if (");
                self.visit_value(condition);
                writeln!(self.writer, ") {{").unwrap();

                {
                    let _h = self.indent();
                    if_body.iter().for_each(|s| self.visit_statement(s.clone()));
                }

                self.prindentln("} else {");

                {
                    let _h = self.indent();
                    else_body
                        .iter()
                        .for_each(|s| self.visit_statement(s.clone()));
                }

                self.prindentln("}");
            }
            Statement::Exit(s) => self.prindentln(format!("exit({s})")),
            Statement::Comment(s) => self.prindentln(format!("// {s}")),
        }
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Identifier(ident) => {
                write!(self.writer, "{ident}").unwrap();
            }
            Expression::Field { expression, field } => {
                self.visit_expression(expression);
                write!(self.writer, ".").unwrap();
                write!(self.writer, "{field}").unwrap();
            }
            Expression::Address(exp) => {
                write!(self.writer, "*").unwrap();
                self.visit_expression(exp);
            }
        }
    }

    fn visit_value(&mut self, node: &Value) {
        match node {
            Value::Identifier(ident) => write!(self.writer, "{ident}").unwrap(),
            Value::Literal(literal) => self.visit_literal(literal),
            Value::Operation(op) => self.visit_operation(op),
            Value::Struct { name, fields } => {
                self.prindentln(format!("struct {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedValue { name, value }| {
                        self.prindent(format!("{name}: "));
                        self.visit_value(value);
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}")
            }
            Value::Field { value, field_name } => {
                self.visit_value(value);
                write!(self.writer, ".{field_name}").unwrap();
            }
            Value::CtorKind {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value);
                write!(self.writer, " is ").unwrap();
                self.print_uid(*identifier, types);
            }
            Value::CtorUnwrap {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value);
                write!(self.writer, " as ").unwrap();
                self.print_uid(*identifier, types);
            }
        }
    }

    fn visit_literal(&mut self, node: &Literal) {
        match node {
            Literal::Int(bi) => write!(self.writer, "{bi}"),
            Literal::Bits(bits) => write!(self.writer, "{bits:?}"),
            Literal::Bit(bit) => write!(self.writer, "{bit:?}"),
            Literal::Bool(bool) => write!(self.writer, "{bool}"),
            Literal::String(s) => write!(self.writer, "{s:?}"),
            Literal::Unit => write!(self.writer, "()"),
            Literal::Reference(s) => write!(self.writer, "&{s}"),
        }
        .unwrap();
    }

    fn visit_operation(&mut self, node: &Operation) {
        match node {
            Operation::Not(value) => {
                write!(self.writer, "!").unwrap();
                self.visit_value(value);
            }
            Operation::Equal(lhs, rhs) => {
                self.visit_value(lhs);
                write!(self.writer, " == ").unwrap();
                self.visit_value(rhs);
            }
            Operation::LessThan(lhs, rhs) => {
                self.visit_value(lhs);
                write!(self.writer, " < ").unwrap();
                self.visit_value(rhs);
            }
            Operation::GreaterThan(lhs, rhs) => {
                self.visit_value(lhs);
                write!(self.writer, " > ").unwrap();
                self.visit_value(rhs);
            }
            Operation::Subtract(lhs, rhs) => {
                self.visit_value(lhs);
                write!(self.writer, " - ").unwrap();
                self.visit_value(rhs);
            }
            Operation::Add(lhs, rhs) => {
                self.visit_value(lhs);
                write!(self.writer, " + ").unwrap();
                self.visit_value(rhs);
            }
        }
    }
}
