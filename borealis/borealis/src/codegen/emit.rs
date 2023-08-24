//! GenC code generation from BOOM structures

use crate::boom::Parameter;

use {
    crate::boom::{
        bits_to_int, Expression, Literal, NamedType, NamedValue, Operation, Statement, Type, Value,
    },
    common::intern::InternedString,
    itertools::Itertools,
    std::{
        cell::RefCell,
        fmt::{self, Write},
        rc::Rc,
    },
};

/// Used to emit BOOM to GenC strings
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
                if let Some(expression) = expression {
                    expression.emit(w)?;
                    write!(w, " = ")?;
                }

                write!(
                    w,
                    "{name}({});",
                    arguments.iter().map(Emit::emit_string).join(", ")
                )
            }

            Statement::Exit(str) => write!(w, "// exit {str:?}"),

            Statement::Comment(str) => write!(w, "// {str}"),

            Statement::If { .. } | Statement::Jump { .. } | Statement::Goto(_) => {
                panic!("control flow statements should have been removed by this point")
            }

            Statement::Label(label) => write!(w, "// label {label:?}"),

            Statement::End(_) => todo!(),
            Statement::Undefined => todo!(),
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

        match &*self.borrow() {
            Unit => write!(w, "void"),

            // probably bad to emit unknown length bitvector as uint64!
            LargeBits(_) | LargeInt => write!(w, "uint64"),

            FixedInt(len) | FixedBits(len, _) => write!(
                w,
                "{}",
                match *len {
                    0 => panic!("unexpected 0 length bitvector"),
                    1..=8 => "uint8",
                    9..=16 => "uint16",
                    17..=32 => "uint32",
                    33..=64 => "uint64",
                    65..=128 => "uint128",
                    _ => panic!("bitvector length exceeds 128 bits, not representable in GenC"),
                }
            ),

            FixedSignedBits(len) => write!(
                w,
                "{}",
                match *len {
                    0 => panic!("unexpected 0 length bitvector"),
                    1..=8 => "sint8",
                    9..=16 => "sint16",
                    17..=32 => "sint32",
                    33..=64 => "sint64",
                    65..=128 => "sint128",
                    _ => panic!("bitvector length exceeds 128 bits, not representable in GenC"),
                }
            ),

            Enum { .. } => write!(w, "uint32"), // <-- tom responsible for this

            Real | Float => write!(w, "double"),

            Reference(typ) => {
                typ.emit(w)?;
                write!(w, "&")
            }

            Bool => panic!("bools should not exist in the AST after passes"),

            // need to figure out what the rest mean
            _ => write!(w, "unknown"),
        }
    }
}

impl Emit for Parameter {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        self.typ.emit(w)?;
        if self.is_ref {
            write!(w, "&")?;
        }
        write!(w, " {}", self.name)
    }
}

impl Emit for Rc<RefCell<Value>> {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        fn write_uid<W: Write>(
            w: &mut W,
            id: InternedString,
            typs: &Vec<Rc<RefCell<Type>>>,
        ) -> fmt::Result {
            write!(w, "{id}")?;

            if !typs.is_empty() {
                write!(w, "<")?;

                let mut typs = typs.iter();
                if let Some(typ) = typs.next() {
                    typ.emit(w)?;
                }
                for typ in typs {
                    write!(w, ", ")?;
                    typ.emit(w)?;
                }

                write!(w, ">")?;
            }

            Ok(())
        }

        match &*self.borrow() {
            Value::Identifier(ident) => write!(w, "{ident}"),
            Value::Literal(literal) => literal.emit(w),
            Value::Operation(op) => op.emit(w),
            Value::Struct { name, fields } => {
                write!(w, "struct {name} {{")?;

                for NamedValue { name, value } in fields {
                    write!(w, "{name}: ")?;
                    value.emit(w)?;
                    write!(w, ",")?;
                }

                write!(w, "}}")
            }
            Value::Field { value, field_name } => {
                value.emit(w)?;
                write!(w, ".{field_name}")
            }
            Value::CtorKind {
                value,
                identifier,
                types,
            } => {
                value.emit(w)?;
                write!(w, " is ")?;
                write_uid(w, *identifier, types)
            }
            Value::CtorUnwrap {
                value,
                identifier,
                types,
            } => {
                value.emit(w)?;
                write!(w, " as ")?;
                write_uid(w, *identifier, types)
            }
        }
    }
}

impl Emit for Rc<RefCell<Literal>> {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        match &*self.borrow() {
            Literal::Int(bi) => write!(w, "{bi}"),
            Literal::Bits(bits) => write!(w, "{}", bits_to_int(bits)),
            Literal::Bit(bit) => write!(w, "{}", bit.value()),
            Literal::Bool(bool) => write!(w, "{bool}"),
            Literal::String(s) => write!(w, "{s:?}"),
            Literal::Unit => write!(w, "()"),
            Literal::Reference(s) => write!(w, "{s}&"),
        }
    }
}

impl Emit for Operation {
    fn emit<W: Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Operation::Not(value) => {
                write!(w, "!")?;
                value.emit(w)
            }
            Operation::Complement(value) => {
                write!(w, "~")?;
                value.emit(w)
            }
            Operation::Equal(lhs, rhs) => emit_op2(w, lhs, rhs, "=="),
            Operation::NotEqual(lhs, rhs) => emit_op2(w, lhs, rhs, "!="),
            Operation::LessThan(lhs, rhs) => emit_op2(w, lhs, rhs, "<"),
            Operation::GreaterThan(lhs, rhs) => emit_op2(w, lhs, rhs, ">"),
            Operation::LessThanOrEqual(lhs, rhs) => emit_op2(w, lhs, rhs, "<="),
            Operation::GreaterThanOrEqual(lhs, rhs) => emit_op2(w, lhs, rhs, ">="),
            Operation::Subtract(lhs, rhs) => emit_op2(w, lhs, rhs, "-"),
            Operation::Add(lhs, rhs) => emit_op2(w, lhs, rhs, "+"),
            Operation::Or(lhs, rhs) => emit_op2(w, lhs, rhs, "|"),
            Operation::Xor(lhs, rhs) => emit_op2(w, lhs, rhs, "^"),
            Operation::And(lhs, rhs) => emit_op2(w, lhs, rhs, "&"),
            Operation::LeftShift(lhs, rhs) => emit_op2(w, lhs, rhs, "<<"),
            Operation::RightShift(lhs, rhs) => emit_op2(w, lhs, rhs, ">>"),

            Operation::RotateLeft(lhs, rhs) => emit_op2(w, lhs, rhs, "<<<"),
            Operation::RotateRight(lhs, rhs) => emit_op2(w, lhs, rhs, ">>>"),

            Operation::Cast(value, typ) => {
                write!(w, "(")?;
                typ.emit(w)?;
                write!(w, ")(")?;
                value.emit(w)?;
                write!(w, ")")
            }
        }
    }
}

fn emit_op2<W: Write>(
    w: &mut W,
    lhs: &Rc<RefCell<Value>>,
    rhs: &Rc<RefCell<Value>>,
    op: &str,
) -> fmt::Result {
    write!(w, "(")?;
    lhs.emit(w)?;
    write!(w, " {op} ")?;
    rhs.emit(w)?;
    write!(w, ")")
}
