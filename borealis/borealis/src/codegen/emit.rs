//! GenC code generation from BOOM structures

use {
    crate::boom::{
        pretty_print::BoomPrettyPrinter, visitor::Visitor, Expression, NamedType, Statement, Type,
        Value,
    },
    itertools::Itertools,
    std::{
        cell::RefCell,
        fmt::{self, Write},
        rc::Rc,
    },
};

/// Used to render items to GenC strings
pub trait Emit {
    /// Renders `self` as GenC
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result;

    /// Renders `self` as GenC to a `String`
    fn emit_string(&self) -> String {
        let mut buf = String::new();

        self.emit(&mut buf).unwrap();

        buf
    }
}

impl Emit for NamedType {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        self.typ.emit(w)?;
        write!(w, " {}", self.name)?;
        Ok(())
    }
}

impl Emit for Rc<RefCell<Type>> {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        use Type::*;

        let str = match &*self.borrow() {
            Unit => "void",
            Fbits(len, _) | Fint(len) => match *len {
                0 => panic!("unexpected 0 length bitvector"),
                1..=8 => "uint8",
                9..=16 => "uint16",
                17..=32 => "uint32",
                33..=64 => "uint64",
                65..=128 => "uint128",
                _ => panic!("bitvector length exceeds 128 bits, not representable in GenC"),
            },
            // need to figure out what these mean
            Lbits(_) | Lint => "uint128",
            Bool => "bool", // panic!("bools should not exist in the AST after passes"),
            Union { .. } => "union",
            t => panic!("{t:?}"),
        };

        write!(w, "{str}")
    }
}

impl Emit for Value {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        let mut condition_buf = vec![];
        BoomPrettyPrinter::new(&mut condition_buf).visit_value(self);
        write!(w, "{}", String::from_utf8(condition_buf).unwrap())
    }
}

impl Emit for Rc<RefCell<Statement>> {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        match &*self.borrow() {
            Statement::TypeDeclaration { name, typ } => {
                typ.emit(w)?;
                write!(w, " {name};",)
            }
            Statement::Copy { expression, value } => {
                expression.emit(w)?;
                write!(w, " = ")?;
                value.emit(w)?;
                write!(w, ";")
            }
            Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                expression.emit(w)?;

                write!(
                    w,
                    " = {name}({});",
                    arguments.iter().map(Emit::emit_string).join(", ")
                )
            }

            Statement::End(_) => todo!(),
            Statement::Undefined => todo!(),
            Statement::Exit(_) => todo!(),

            Statement::Comment(str) => write!(w, "// {str}"),
            Statement::Clear { identifier } => write!(w, "// clear {identifier}"),

            Statement::Try { .. } => todo!(),

            Statement::If { .. }
            | Statement::Jump { .. }
            | Statement::Label(_)
            | Statement::Goto(_) => {
                panic!("control flow statements should have been removed by this point")
            }
        }
    }
}

impl Emit for Expression {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(w, "{ident}"),
            Expression::Field { expression, field } => {
                expression.emit(w)?;
                write!(w, ".{field}")
            }
            Expression::Address(_) => todo!(),
        }
    }
}
