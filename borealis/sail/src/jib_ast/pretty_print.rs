//! JIB AST pretty printing

use {
    crate::{
        jib_ast::{
            visitor::Visitor, Definition, Expression, Instruction, InstructionAux, Name, Type,
            TypeDefinition, Value,
        },
        sail_ast::Identifier,
    },
    common::{intern::InternedString, HashSet},
    std::{
        collections::LinkedList,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print JIB AST (sequence of definitions)
pub fn print_ast<'a, I: IntoIterator<Item = &'a Definition>>(iter: I) {
    let mut visitor = JibPrettyPrinter {
        indent: Rc::new(AtomicUsize::from(0)),
        abstract_functions: HashSet::default(),
    };

    iter.into_iter().for_each(|i| visitor.visit_definition(i));
}

/// Pretty-print JIB AST
struct JibPrettyPrinter {
    indent: Rc<AtomicUsize>,
    abstract_functions: HashSet<InternedString>,
}

impl JibPrettyPrinter {
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

    fn print_uid(&mut self, id: &Identifier, typs: &LinkedList<Type>) {
        print!("{}", id.as_interned());

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
            Definition::Register(id, typ, _) => {
                self.prindent(format!("register {} : ", id.as_interned()));
                self.visit_type(typ);
            }
            Definition::Type(TypeDefinition::Enum(id, ids)) => {
                self.prindentln(format!("enum {} {{", id.as_interned()));

                {
                    let _h = self.indent();
                    ids.iter()
                        .for_each(|id| self.prindentln(format!("{},", id.as_interned())));
                }

                self.prindentln("}");
            }
            Definition::Type(TypeDefinition::Struct(id, fields)) => {
                self.prindentln(format!("struct {} {{", id.as_interned()));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|(name, typ)| {
                        self.prindent(format!("{name}: "));
                        self.visit_type(typ);
                        println!(",");
                    });
                }

                self.prindentln("}");
            }
            Definition::Type(TypeDefinition::Variant(id, fields)) => {
                self.prindentln(format!("union {} {{", id.as_interned()));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|(name, typ)| {
                        self.prindent(format!("{name}: "));
                        self.visit_type(typ);
                        println!(",");
                    });
                }

                self.prindentln("}");
            }
            Definition::Let(_, bindings, instructions) => {
                self.prindent("let (");

                let mut bindings = bindings.iter();
                if let Some((ident, _)) = bindings.next() {
                    print!("{}", ident.as_interned());
                }
                for (ident, _) in bindings {
                    print!(", ");
                    print!("{}", ident.as_interned());
                }

                println!(") {{");

                {
                    let _h = self.indent();
                    instructions.iter().for_each(|i| self.visit_instruction(i));
                }

                println!("}}");
            }
            Definition::Val(id, ext, typs, typ) => {
                let keyword =
                    if let Some(true) = ext.map(|ext| self.abstract_functions.contains(&ext)) {
                        "abstract"
                    } else {
                        "val"
                    };

                self.prindent(format!("{keyword} {} : (", id.as_interned()));

                let mut typs = typs.iter();
                if let Some(typ) = typs.next() {
                    self.visit_type(typ);
                }
                for typ in typs {
                    print!(", ");
                    self.visit_type(typ);
                }

                print!(") -> ");
                self.visit_type(typ);

                println!();
            }
            Definition::Fundef(name, _, args, body) => {
                self.prindent(format!("fn {}(", name.as_interned()));

                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    print!("{}", arg.as_interned());
                }
                for arg in args {
                    print!(", {}", arg.as_interned());
                }

                println!(") {{");

                {
                    let _h = self.indent();
                    body.iter().for_each(|i| self.visit_instruction(i));
                }

                self.prindentln("}");
            }
            Definition::Startup(_, _) => todo!(),
            Definition::Finish(_, _) => todo!(),
            Definition::Pragma(key, value) => {
                if *key == "abstract".into() {
                    self.abstract_functions.insert(*value);
                } else {
                    self.prindent(format!("#{key} {value}"));
                }
            }
        }

        println!();
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
                print!(" = {}(", name.as_interned());

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
                let mut vals = vals.iter();
                if let Some(val) = vals.next() {
                    self.visit_value(val);
                }
                for val in vals {
                    print!(", ");
                    self.visit_value(val);
                }
                print!(")")
            }
            Value::Tuple(_, _) => todo!(),
            Value::Struct(fields, Type::Struct(ident, _)) => {
                self.prindentln(format!("struct {} {{", ident.as_interned()));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|((ident, _), value)| {
                        self.prindent(format!("{}: ", ident.as_interned()));
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
                print!("{}", ident.as_interned());
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
                print!("{}", ident.as_interned());
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
            Type::Lint => print!("%i"),
            Type::Fint(n) => print!("%i{n}"),
            Type::Constant(bi) => print!("{}", bi.0),
            Type::Lbits => print!("%bv"),
            Type::Sbits(n) => print!("%sbv{n}"),
            Type::Fbits(n) => print!("%bv{n}"),
            Type::Unit => print!("%unit"),
            Type::Bool => print!("%bool"),
            Type::Bit => print!("%bit"),
            Type::String => print!("%string"),
            Type::Real => print!("%real"),
            Type::Float(n) => print!("%f{n}"),
            Type::RoundingMode => print!("%rounding_mode"),
            Type::Tup(typs) => {
                print!("(");
                let mut typs = typs.iter();
                if let Some(typ) = typs.next() {
                    self.visit_type(typ);
                }
                for typ in typs {
                    print!(", ");
                    self.visit_type(typ);
                }
                print!(")");
            }

            Type::Enum(ident, _) => print!("enum {}", ident.as_interned()),
            Type::Struct(ident, _) => print!("struct {}", ident.as_interned()),
            Type::Variant(ident, _) => print!("union {}", ident.as_interned()),

            Type::Vector(_, typ) => {
                print!("%vec<");
                self.visit_type(typ);
                print!(">");
            }
            Type::Fvector(n, _, typ) => {
                print!("%fvec<{n}, ");
                self.visit_type(typ);
                print!(">");
            }
            Type::List(inner) => {
                print!("list<");
                self.visit_type(inner);
                print!(">");
            }
            Type::Ref(inner) => {
                print!("&");
                self.visit_type(inner);
            }
            Type::Poly(kid) => print!("{:?}", kid.inner),
        }
    }

    fn visit_name(&mut self, node: &Name) {
        match node {
            Name::Global(ident, _) | Name::Name(ident, _) => {
                print!("{}", ident.as_interned())
            }
            Name::HaveException(_) | Name::CurrentException(_) => print!("exception"),
            Name::ThrowLocation(_) => print!("throw"),
            Name::Return(_) => print!("return"),
        }
    }
}
