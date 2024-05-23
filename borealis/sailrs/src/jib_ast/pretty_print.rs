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
        io::Write,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print JIB AST (sequence of definitions)
pub fn print_ast<'a, W: Write, I: IntoIterator<Item = &'a Definition>>(writer: &mut W, iter: I) {
    let mut visitor = JibPrettyPrinter {
        writer,
        indent: Rc::new(AtomicUsize::from(0)),
        abstract_functions: HashSet::default(),
    };

    iter.into_iter().for_each(|i| visitor.visit_definition(i));
}

/// Pretty-print JIB AST
struct JibPrettyPrinter<W> {
    writer: W,
    indent: Rc<AtomicUsize>,
    abstract_functions: HashSet<InternedString>,
}

impl<W: Write> JibPrettyPrinter<W> {
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
        writeln!(self.writer,).unwrap();
    }

    fn indent(&self) -> IndentHandle {
        self.indent.fetch_add(1, Ordering::SeqCst);
        IndentHandle {
            indent: self.indent.clone(),
        }
    }

    fn print_uid(&mut self, id: &Identifier, typs: &[Type]) {
        write!(self.writer, "{}", id.as_interned()).unwrap();

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

impl<W: Write> Visitor for JibPrettyPrinter<W> {
    fn visit_definition(&mut self, node: &Definition) {
        match node {
            Definition::Register(id, typ, init) => {
                self.prindent(format!("register {} : ", id.as_interned()));
                self.visit_type(typ);
                write!(self.writer, " = {{").unwrap();
                {
                    let _h = self.indent();
                    init.iter().for_each(|i| self.visit_instruction(i));
                }
                self.prindentln("}");
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
                        writeln!(self.writer, ",").unwrap();
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
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}");
            }
            Definition::Let(_, bindings, instructions) => {
                self.prindent("let (");

                let mut bindings = bindings.iter();
                if let Some((ident, _)) = bindings.next() {
                    write!(self.writer, "{}", ident.as_interned()).unwrap();
                }
                for (ident, _) in bindings {
                    write!(self.writer, ", ").unwrap();
                    write!(self.writer, "{}", ident.as_interned()).unwrap();
                }

                writeln!(self.writer, ") {{").unwrap();

                {
                    let _h = self.indent();
                    instructions.iter().for_each(|i| self.visit_instruction(i));
                }

                writeln!(self.writer, "}}").unwrap();
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
                    write!(self.writer, ", ").unwrap();
                    self.visit_type(typ);
                }

                write!(self.writer, ") -> ").unwrap();
                self.visit_type(typ);

                writeln!(self.writer,).unwrap();
            }
            Definition::Fundef(name, _, args, body) => {
                self.prindent(format!("fn {}(", name.as_interned()));

                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    write!(self.writer, "{}", arg.as_interned()).unwrap();
                }
                for arg in args {
                    write!(self.writer, ", {}", arg.as_interned()).unwrap();
                }

                writeln!(self.writer, ") {{").unwrap();

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

        writeln!(self.writer,).unwrap();
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
                write!(self.writer, ": ").unwrap();
                self.visit_type(typ);
                writeln!(self.writer,).unwrap();
            }
            InstructionAux::Copy(exp, val) => {
                self.prindent("");
                self.visit_expression(exp);
                write!(self.writer, " = ").unwrap();
                self.visit_value(val);
                writeln!(self.writer,).unwrap();
            }
            InstructionAux::Clear(_, name) => {
                self.prindent("clear(");
                self.visit_name(name);
                writeln!(self.writer, ")").unwrap();
            }
            InstructionAux::Funcall(exp, _, (name, _), args) => {
                self.prindent("");
                self.visit_expression(exp);
                write!(self.writer, " = {}(", name.as_interned()).unwrap();

                // print correct number of commas
                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    self.visit_value(arg);
                }
                for arg in args {
                    write!(self.writer, ", ").unwrap();
                    self.visit_value(arg);
                }

                writeln!(self.writer, ")").unwrap();
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
                writeln!(self.writer, ") {{").unwrap();

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
            InstructionAux::Init(typ, name, value) => {
                self.prindent("init ");
                self.visit_type(typ);
                write!(self.writer, " ").unwrap();
                self.visit_name(name);
                write!(self.writer, " ").unwrap();
                self.visit_value(value);
                writeln!(self.writer,).unwrap();
            }
            InstructionAux::Jump(value, s) => {
                self.prindent(format!("jump {} ", s));
                self.visit_value(value);
                writeln!(self.writer,).unwrap();
            }
            InstructionAux::Undefined(_) => self.prindentln("undefined"),
            InstructionAux::Exit(s) => self.prindentln(format!("exit({s})")),
            InstructionAux::End(name) => {
                self.prindent("end(");
                self.visit_name(name);
                writeln!(self.writer, ")").unwrap();
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
            Value::Lit(val, _) => write!(self.writer, "{val:?}").unwrap(),
            Value::Call(op, vals) => {
                write!(self.writer, "{op:?}(").unwrap();
                let mut vals = vals.iter();
                if let Some(val) = vals.next() {
                    self.visit_value(val);
                }
                for val in vals {
                    write!(self.writer, ", ").unwrap();
                    self.visit_value(val);
                }
                write!(self.writer, ")").unwrap()
            }
            Value::Tuple(_, _) => todo!(),
            Value::Struct(fields, Type::Struct(ident, _)) => {
                self.prindentln(format!("struct {} {{", ident.as_interned()));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|(ident, value)| {
                        self.prindent(format!("{}: ", ident.as_interned()));
                        self.visit_value(value);
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}")
            }
            Value::Struct(_, _) => panic!("encountered struct with non-struct type"),
            Value::CtorKind(f, (ctor, unifiers), _) => {
                self.visit_value(f);
                write!(self.writer, " is ").unwrap();
                self.print_uid(ctor, unifiers.as_ref());
            }
            Value::CtorUnwrap(f, (ctor, unifiers), _) => {
                self.visit_value(f);
                write!(self.writer, " as ").unwrap();
                self.print_uid(ctor, unifiers.as_ref());
            }
            Value::TupleMember(_, _, _) => todo!(),
            Value::Field(value, ident) => {
                self.visit_value(value);
                write!(self.writer, ".").unwrap();
                write!(self.writer, "{}", ident.as_interned()).unwrap();
            }
        }
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Id(name, _) => self.visit_name(name),
            Expression::Rmw(_, _, _) => todo!(),
            Expression::Field(expression, ident) => {
                self.visit_expression(expression);
                write!(self.writer, ".").unwrap();
                write!(self.writer, "{}", ident.as_interned()).unwrap();
            }
            Expression::Addr(inner) => {
                write!(self.writer, "*").unwrap();
                self.visit_expression(inner);
            }
            Expression::Tuple(_, _) => todo!(),
            Expression::Void => todo!(),
        }
    }

    fn visit_type(&mut self, node: &Type) {
        match node {
            Type::Lint => write!(self.writer, "%i").unwrap(),
            Type::Fint(n) => write!(self.writer, "%i{n}").unwrap(),
            Type::Constant(bi) => write!(self.writer, "{}", bi.0).unwrap(),
            Type::Lbits => write!(self.writer, "%bv").unwrap(),
            Type::Sbits(n) => write!(self.writer, "%sbv{n}").unwrap(),
            Type::Fbits(n) => write!(self.writer, "%bv{n}").unwrap(),
            Type::Unit => write!(self.writer, "%unit").unwrap(),
            Type::Bool => write!(self.writer, "%bool").unwrap(),
            Type::Bit => write!(self.writer, "%bit").unwrap(),
            Type::String => write!(self.writer, "%string").unwrap(),
            Type::Real => write!(self.writer, "%real").unwrap(),
            Type::Float(n) => write!(self.writer, "%f{n}").unwrap(),
            Type::RoundingMode => write!(self.writer, "%rounding_mode").unwrap(),
            Type::Tup(typs) => {
                write!(self.writer, "(").unwrap();
                let mut typs = typs.iter();
                if let Some(typ) = typs.next() {
                    self.visit_type(typ);
                }
                for typ in typs {
                    write!(self.writer, ", ").unwrap();
                    self.visit_type(typ);
                }
                write!(self.writer, ")").unwrap();
            }

            Type::Enum(ident, _) => write!(self.writer, "enum {}", ident.as_interned()).unwrap(),
            Type::Struct(ident, _) => {
                write!(self.writer, "struct {}", ident.as_interned()).unwrap()
            }
            Type::Variant(ident, _) => {
                write!(self.writer, "union {}", ident.as_interned()).unwrap()
            }

            Type::Vector(typ) => {
                write!(self.writer, "%vec<").unwrap();
                self.visit_type(typ);
                write!(self.writer, ">").unwrap();
            }
            Type::Fvector(n, typ) => {
                write!(self.writer, "%fvec<{n}, ").unwrap();
                self.visit_type(typ);
                write!(self.writer, ">").unwrap();
            }
            Type::List(inner) => {
                write!(self.writer, "list<").unwrap();
                self.visit_type(inner);
                write!(self.writer, ">").unwrap();
            }
            Type::Ref(inner) => {
                write!(self.writer, "&").unwrap();
                self.visit_type(inner);
            }
            Type::Poly(kid) => write!(self.writer, "{:?}", kid.inner).unwrap(),
        }
    }

    fn visit_name(&mut self, node: &Name) {
        match node {
            Name::Global(ident, _) | Name::Name(ident, _) => {
                write!(self.writer, "{}", ident.as_interned()).unwrap()
            }
            Name::HaveException(_) | Name::CurrentException(_) => {
                write!(self.writer, "exception").unwrap()
            }
            Name::ThrowLocation(_) => write!(self.writer, "throw").unwrap(),
            Name::Return(_) => write!(self.writer, "return").unwrap(),
        }
    }
}
