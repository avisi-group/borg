//! JIB AST pretty printing

use {
    sail::jib_ast::{visitor::Visitor, Expression, Instruction, InstructionAux, Name, Type, Value},
    std::{
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print a sequence of instructions
pub fn print_instructions<'a, I: IntoIterator<Item = &'a Instruction>>(instructions: I) {
    let mut visitor = JibPrettyPrinter {
        indent: Rc::new(AtomicUsize::from(0)),
    };

    instructions
        .into_iter()
        .for_each(|i| visitor.visit_instruction(&i));
}

/// Pretty-print JIB AST
struct JibPrettyPrinter {
    indent: Rc<AtomicUsize>,
}

impl JibPrettyPrinter {
    pub fn print<'a, T: AsRef<str>>(&self, s: T) {
        print!(
            "{}{}",
            PADDING.repeat(self.indent.load(Ordering::SeqCst)),
            s.as_ref()
        );
    }

    pub fn println<'a, T: AsRef<str>>(&self, s: T) {
        self.print(s);
        println!();
    }

    pub fn indent(&self) -> IndentHandle {
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

impl Visitor for JibPrettyPrinter {
    fn visit_instruction(&mut self, node: &Instruction) {
        match &node.inner {
            InstructionAux::Block(instructions) => {
                self.println("block {");

                {
                    let _h = self.indent();
                    instructions.iter().for_each(|i| self.visit_instruction(i));
                }

                self.println("}");
            }
            InstructionAux::Decl(typ, name) => {
                self.print("");
                self.visit_name(name);
                print!(": ");
                self.visit_type(typ);
                println!();
            }
            InstructionAux::Copy(exp, val) => {
                self.print("");
                self.visit_expression(exp);
                print!(" = ");
                self.visit_value(val);
                println!();
            }
            InstructionAux::Clear(_, name) => {
                self.print("clear(");
                self.visit_name(name);
                println!(")");
            }
            InstructionAux::Funcall(exp, _, (name, _), args) => {
                self.print("");
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
                self.println(format!("goto \"{label}\""));
            }
            InstructionAux::Label(label) => {
                self.println(format!("label \"{label}\""));
            }
            InstructionAux::If(condition, if_body, else_body, _) => {
                self.print("if (");
                self.visit_value(condition);
                self.println(") {");

                {
                    let _h = self.indent();
                    if_body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.println("} else {");

                {
                    let _h = self.indent();
                    else_body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.println("}");
            }
            InstructionAux::Init(_, _, _) => todo!(),
            InstructionAux::Jump(_, _) => todo!(),
            InstructionAux::Undefined(_) => self.println("undefined"),
            InstructionAux::Exit(s) => self.println(format!("exit({s})")),
            InstructionAux::End(name) => {
                self.print("end(");
                self.visit_name(name);
                println!(")");
            }
            InstructionAux::TryBlock(_) => todo!(),
            InstructionAux::Throw(_) => todo!(),
            InstructionAux::Comment(s) => self.println(format!("// {s}")),
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
            Value::Struct(_, _) => todo!(),
            Value::CtorKind(_, _, _, _) => todo!(),
            Value::CtorUnwrap(_, _, _) => todo!(),
            Value::TupleMember(_, _, _) => todo!(),
            Value::Field(_, _) => todo!(),
        }
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Id(name, _) => self.visit_name(name),
            Expression::Rmw(_, _, _) => todo!(),
            Expression::Field(_, _) => todo!(),
            Expression::Addr(_) => todo!(),
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
