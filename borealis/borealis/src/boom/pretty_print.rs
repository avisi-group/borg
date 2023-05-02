//! BOOM AST pretty printing

use {
    crate::boom::{
        visitor::Visitor, Ast, Definition, Expression, FunctionDefinition, FunctionSignature,
        Literal, NamedType, NamedValue, Operation, Statement, Type, Value,
    },
    common::intern::InternedString,
    std::{
        cell::RefCell,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print JIB AST (sequence of definitions)
pub fn print_ast(
    Ast {
        definitions,
        registers,
        functions,
    }: &Ast,
) {
    let mut visitor = BoomPrettyPrinter::default();

    definitions
        .iter()
        .for_each(|def| visitor.visit_definition(def));

    registers.iter().for_each(|(name, typ)| {
        print!("register {name}: ");
        visitor.visit_type(typ);
        println!();
    });

    functions
        .iter()
        .for_each(|(_, fundef)| visitor.visit_function_definition(fundef));
}

pub fn print_statement(statement: Rc<RefCell<Statement>>) {
    let mut visitor = BoomPrettyPrinter::default();
    visitor.visit_statement(statement);
}

/// Pretty-print JIB AST
#[derive(Default)]
struct BoomPrettyPrinter {
    indent: Rc<AtomicUsize>,
}

impl BoomPrettyPrinter {
    fn prindent<T: AsRef<str>>(&self, s: T) {
        print!(
            "{}{}",
            PADDING.repeat(self.indent.load(Ordering::SeqCst)),
            s.as_ref()
        );
    }

    fn prindentln<T: AsRef<str>>(&self, s: T) {
        self.prindent(s);
        println!();
    }

    fn indent(&self) -> IndentHandle {
        self.indent.fetch_add(1, Ordering::SeqCst);
        IndentHandle {
            indent: self.indent.clone(),
        }
    }

    fn print_uid(&mut self, id: InternedString, typs: &Vec<Type>) {
        print!("{id}");

        if !typs.is_empty() {
            print!("<");

            let mut typs = typs.iter();
            if let Some(typ) = typs.next() {
                self.visit_type(typ);
            }
            for typ in typs {
                print!(", ");
                self.visit_type(typ);
            }

            print!(">");
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

impl Visitor for BoomPrettyPrinter {
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
                        println!(",");
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
                        println!(",");
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
                    print!("{name}");
                }
                for NamedType { name, .. } in bindings {
                    print!(", ");
                    print!("{name}");
                }

                println!(") {{");

                {
                    let _h = self.indent();
                    body.iter()
                        .for_each(|statement| self.visit_statement(statement.clone()));
                }

                println!("}}");
            }
        }
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.visit_function_signature(&node.signature);
        println!(" {{");

        {
            let _h = self.indent();
            node.body
                .iter()
                .for_each(|statement| self.visit_statement(statement.clone()));
        }

        self.prindentln("}");
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
            print!(", ");
            self.visit_named_type(param);
        }

        print!(") -> ");
        self.visit_type(return_type);
    }

    fn visit_named_type(&mut self, NamedType { name, typ }: &NamedType) {
        print!("{name}: ");
        self.visit_type(typ);
    }

    fn visit_named_value(&mut self, NamedValue { name, value }: &NamedValue) {
        print!("{name}: ");
        self.visit_value(value);
    }

    fn visit_type(&mut self, node: &Type) {
        match node {
            Type::Unit => print!("()"),
            Type::Bool => print!("bool"),
            Type::String => print!("String"),
            Type::Real => print!("real"),
            Type::Float => print!("float"),
            Type::Constant(bi) => print!("constant<{bi}>"),
            Type::Lint => print!("int"),
            Type::Fint(size) => print!("int<{size}>"),
            Type::Fbits(size, _) => print!("bitvector<{size}>"),
            Type::Lbits(_) => print!("bitvector"),
            Type::Bit => print!("bit"),
            Type::Enum { name, .. } => print!("enum {name}"),
            Type::Union { name, .. } => print!("union {name}"),
            Type::Struct { name, .. } => print!("struct {name}"),
            Type::List { element_type } => {
                print!("list<");
                self.visit_type(element_type);
                print!(">");
            }
            Type::Vector { element_type } => {
                print!("vec<");
                self.visit_type(element_type);
                print!(">");
            }
            Type::FVector {
                length,
                element_type,
            } => {
                print!("fvec<{length}, ");
                self.visit_type(element_type);
                print!(">");
            }
            Type::Reference(inner) => {
                print!("&");
                self.visit_type(inner);
            }
        }
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        match &*node.borrow() {
            Statement::TypeDeclaration { name, typ } => {
                self.prindent(format!("{name}: "));
                self.visit_type(typ);
                println!();
            }
            Statement::Block { body } => {
                self.prindentln("block {");
                {
                    let _h = self.indent();
                    body.iter().for_each(|s| self.visit_statement(s.clone()));
                }
                self.prindentln("}");
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
                print!(" = ");
                self.visit_value(value);
                println!();
            }
            Statement::Clear { identifier } => self.prindentln(format!("clear({identifier})")),
            Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                self.prindent("");
                self.visit_expression(expression);
                print!(" = {name}(");

                // print correct number of commas
                let mut args = arguments.iter();
                if let Some(arg) = args.next() {
                    self.visit_value(arg);
                }
                for arg in args {
                    print!(", ");
                    self.visit_value(arg);
                }

                println!(")");
            }
            Statement::Label(label) => self.prindentln(format!("label \"{label}\"")),
            Statement::Goto(label) => self.prindentln(format!("goto \"{label}\"")),
            Statement::Jump { condition, target } => {
                self.prindent(format!("jump {} ", target));
                self.visit_value(condition);
                println!();
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
                println!(") {{");

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
            Expression::Identifier(ident) => print!("{ident}"),
            Expression::Field { expression, field } => {
                self.visit_expression(expression);
                print!(".");
                print!("{field}");
            }
            Expression::Address(exp) => {
                print!("*");
                self.visit_expression(exp);
            }
        }
    }

    fn visit_value(&mut self, node: &Value) {
        match node {
            Value::Identifier(ident) => print!("{ident}"),
            Value::Literal(literal) => self.visit_literal(literal),
            Value::Operation(op) => self.visit_operation(op),
            Value::Struct { name, fields } => {
                self.prindentln(format!("struct {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedValue { name, value }| {
                        self.prindent(format!("{name}: "));
                        self.visit_value(value);
                        println!(",");
                    });
                }

                self.prindentln("}")
            }
            Value::Field { value, field_name } => {
                self.visit_value(value);
                print!(".{field_name}");
            }
            Value::CtorKind {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value);
                print!(" is ");
                self.print_uid(*identifier, types);
            }
            Value::CtorUnwrap {
                value,
                identifier,
                types,
            } => {
                self.visit_value(value);
                print!(" as ");
                self.print_uid(*identifier, types);
            }
        }
    }

    fn visit_literal(&mut self, node: &Literal) {
        match node {
            Literal::Int(bi) => print!("{bi}"),
            Literal::Bits(bits) => print!("{bits:?}"),
            Literal::Bit(bit) => print!("{bit:?}"),
            Literal::Bool(bool) => print!("{bool}"),
            Literal::String(s) => print!("{s:?}"),
            Literal::Unit => print!("()"),
            Literal::Reference(s) => print!("&{s}"),
        }
    }

    fn visit_operation(&mut self, node: &Operation) {
        match node {
            Operation::Not(value) => {
                print!("!");
                self.visit_value(value);
            }
            Operation::Equal(lhs, rhs) => {
                self.visit_value(lhs);
                print!(" == ");
                self.visit_value(rhs);
            }
            Operation::LessThan(lhs, rhs) => {
                self.visit_value(lhs);
                print!(" < ");
                self.visit_value(rhs);
            }
            Operation::GreaterThan(lhs, rhs) => {
                self.visit_value(lhs);
                print!(" > ");
                self.visit_value(rhs);
            }
            Operation::Subtract(lhs, rhs) => {
                self.visit_value(lhs);
                print!(" - ");
                self.visit_value(rhs);
            }
            Operation::Add(lhs, rhs) => {
                self.visit_value(lhs);
                print!(" + ");
                self.visit_value(rhs);
            }
        }
    }
}
