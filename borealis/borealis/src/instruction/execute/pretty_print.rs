//! JIB AST pretty printing

use {
    sail::{
        jib_ast::{
            visitor::Visitor, Definition, Expression, Instruction, InstructionAux, Name, Type,
            Value,
        },
        sail_ast::Identifier,
    },
    std::{
        collections::LinkedList,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print a sequence of definition
pub fn print_definitions<'a, I: IntoIterator<Item = &'a Definition>>(iter: I) {
    let mut visitor = JibPrettyPrinter {
        indent: Rc::new(AtomicUsize::from(0)),
    };

    iter.into_iter().for_each(|i| visitor.visit_definition(&i));
}

/// Pretty-print a sequence of instructions
pub fn print_instructions<'a, I: IntoIterator<Item = &'a Instruction>>(iter: I) {
    let mut visitor = JibPrettyPrinter {
        indent: Rc::new(AtomicUsize::from(0)),
    };

    iter.into_iter().for_each(|i| visitor.visit_instruction(&i));
}

/// Pretty-print JIB AST
struct JibPrettyPrinter {
    indent: Rc<AtomicUsize>,
}

impl JibPrettyPrinter {
    fn prindent<'a, T: AsRef<str>>(&self, s: T) {
        print!(
            "{}{}",
            PADDING.repeat(self.indent.load(Ordering::SeqCst)),
            s.as_ref()
        );
    }

    fn prindentln<'a, T: AsRef<str>>(&self, s: T) {
        self.prindent(s);
        println!();
    }

    fn indent(&self) -> IndentHandle {
        self.indent.fetch_add(1, Ordering::SeqCst);
        IndentHandle {
            indent: self.indent.clone(),
        }
    }

    fn print_uid(&mut self, id: &Identifier, typs: &LinkedList<Type>) {
        print!("{}", id.get_string());

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

impl Visitor for JibPrettyPrinter {
    fn visit_definition(&mut self, node: &Definition) {
        match node {
            Definition::RegDec(id, typ, body) => {
                self.prindent(format!("register {} : ", id.get_string()));
                self.visit_type(typ);
                dbg!(body);
                panic!();
            }
            Definition::Type(_) => todo!(),
            Definition::Let(_, _, _) => todo!(),
            Definition::Spec(_, _, _, _) => todo!(),
            Definition::Fundef(name, _, args, body) => {
                self.prindent(format!("fn {}(", name.get_string()));

                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    print!("{}", arg.get_string());
                }
                for arg in args {
                    print!(", {}", arg.get_string());
                }

                println!(") {{");

                {
                    let _h = self.indent();
                    body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("}\n");
            }
            Definition::Startup(_, _) => todo!(),
            Definition::Finish(_, _) => todo!(),
            Definition::Pragma(_, _) => todo!(),
        }
    }

    fn visit_instruction(&mut self, node: &Instruction) {
        match &node.inner {
            InstructionAux::Block(instructions) => {
                self.prindentln("block {");

                {
                    let _h = self.indent();
                    instructions.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("}");
            }
            InstructionAux::Decl(typ, name) => {
                self.prindent("");
                self.visit_name(name);
                print!(": ");
                self.visit_type(typ);
                println!();
            }
            InstructionAux::Copy(exp, val) => {
                self.prindent("");
                self.visit_expression(exp);
                print!(" = ");
                self.visit_value(val);
                println!();
            }
            InstructionAux::Clear(_, name) => {
                self.prindent("clear(");
                self.visit_name(name);
                println!(")");
            }
            InstructionAux::Funcall(exp, _, (name, _), args) => {
                self.prindent("");
                self.visit_expression(exp);
                print!(" = {}(", name.get_string());

                // print correct number of commas
                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    self.visit_value(arg);
                }
                for arg in args {
                    print!(", ");
                    self.visit_value(arg);
                }

                println!(")");
            }
            InstructionAux::Goto(label) => {
                self.prindentln(format!("goto \"{label}\""));
            }
            InstructionAux::Label(label) => {
                self.prindentln(format!("label \"{label}\""));
            }
            InstructionAux::If(condition, if_body, else_body, _) => {
                self.prindent("if (");
                self.visit_value(condition);
                println!(") {{");

                {
                    let _h = self.indent();
                    if_body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("} else {");

                {
                    let _h = self.indent();
                    else_body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("}");
            }
            InstructionAux::Init(_, _, _) => todo!(),
            InstructionAux::Jump(value, s) => {
                self.prindent(format!("jump {} ", s));
                self.visit_value(value);
                println!();
            }
            InstructionAux::Undefined(_) => self.prindentln("undefined"),
            InstructionAux::Exit(s) => self.prindentln(format!("exit({s})")),
            InstructionAux::End(name) => {
                self.prindent("end(");
                self.visit_name(name);
                println!(")");
            }
            InstructionAux::TryBlock(body) => {
                self.prindentln("try {");

                {
                    let _h = self.indent();
                    body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("}");
            }
            InstructionAux::Throw(_) => todo!(),
            InstructionAux::Comment(s) => self.prindentln(format!("// {s}")),
            InstructionAux::Raw(_) => todo!(),
            InstructionAux::Return(_) => todo!(),
            InstructionAux::Reset(_, _) => todo!(),
            InstructionAux::Reinit(_, _, _) => todo!(),
        }
    }

    fn visit_value(&mut self, node: &Value) {
        match node {
            Value::Id(name, _) => self.visit_name(name),
            Value::Lit(val, _) => print!("{val:?}"),
            Value::Call(op, vals) => {
                print!("{op:?}(");
                for val in vals {
                    self.visit_value(val);
                }
                print!(")")
            }
            Value::Tuple(_, _) => todo!(),
            Value::Struct(fields, Type::Struct(ident, _)) => {
                self.prindentln(format!("struct {} {{", ident.get_string()));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|((ident, _), value)| {
                        self.prindent(format!("{}: ", ident.get_string()));
                        self.visit_value(value);
                        println!(",");
                    });
                }

                self.prindentln("}")
            }
            Value::Struct(_, _) => panic!("encountered struct with non-struct type"),
            Value::CtorKind(f, ctor, unifiers, _) => {
                self.visit_value(f);
                print!(" is ");
                self.print_uid(ctor, unifiers);
            }
            Value::CtorUnwrap(f, (ctor, unifiers), _) => {
                self.visit_value(f);
                print!(" as ");
                self.print_uid(ctor, unifiers);
            }
            Value::TupleMember(_, _, _) => todo!(),
            Value::Field(value, (ident, _)) => {
                self.visit_value(value);
                print!(".");
                print!("{}", ident.get_string());
            }
        }
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Id(name, _) => self.visit_name(name),
            Expression::Rmw(_, _, _) => todo!(),
            Expression::Field(expression, (ident, _)) => {
                self.visit_expression(expression);
                print!(".");
                print!("{}", ident.get_string());
            }
            Expression::Addr(inner) => {
                print!("*");
                self.visit_expression(inner);
            }
            Expression::Tuple(_, _) => todo!(),
            Expression::Void => todo!(),
        }
    }

    fn visit_type(&mut self, node: &Type) {
        match node {
            Type::Variant(ident, _) => print!("{}", ident.get_string()),
            _ => print!("{:?}", node),
        }
    }

    fn visit_name(&mut self, node: &Name) {
        match node {
            Name::Global(ident, _) | Name::Name(ident, _) => {
                print!("{}", ident.get_string())
            }
            Name::HaveException(_) | Name::CurrentException(_) => print!("exception"),
            Name::ThrowLocation(_) => print!("throw"),
            Name::Return(_) => print!("return"),
        }
    }
}
