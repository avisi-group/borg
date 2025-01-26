//! Borealis Object Of Machine, Internal intermediate representation used to
//! convert JIB AST to GenC AST

#![allow(missing_docs)]

extern crate std;

use {
    crate::{
        boom::control_flow::ControlFlowBlock, intern::InternedString, shared::Shared, HashMap,
    },
    alloc::{boxed::Box, vec::Vec},
    core::{
        fmt::{self, Debug},
        ops::Add,
    },
};

pub mod control_flow;

/// BOOM AST
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Ast {
    /// Register types by identifier
    pub registers: HashMap<InternedString, Shared<Type>>,
    /// Function definitions by identifier
    pub functions: HashMap<InternedString, FunctionDefinition>,
    pub constants: HashMap<InternedString, i32>,
    /// name -> fields (implicit indices)
    pub enums: HashMap<InternedString, Vec<InternedString>>,
    pub structs: HashMap<InternedString, Vec<NamedType>>,
    pub unions: HashMap<InternedString, Vec<NamedType>>,
    pub pragmas: HashMap<InternedString, InternedString>,
}

/// Top-level definition of a BOOM item
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Definition {
    /// Struct definition
    Struct {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    Union {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    Pragma {
        key: InternedString,
        value: InternedString,
    },
}

/// Function signature and body
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FunctionDefinition {
    /// Function type signature
    pub signature: FunctionSignature,
    /// Entry block into the control flow graph
    pub entry_block: ControlFlowBlock,
}

impl FunctionDefinition {
    /// Gets the type from the type declaration (if it exists) of a local
    /// variable within a function
    pub fn get_ident_type(&self, ident: InternedString) -> Option<Type> {
        // search every statement for ident, should only have a single type declaration,
        // return that type otherwise none
        self.entry_block
            .iter()
            .flat_map(|block| block.statements())
            .filter_map(|statement| {
                if let Statement::VariableDeclaration { name, typ } = &*statement.get() {
                    Some((*name, typ.clone()))
                } else {
                    None
                }
            })
            .chain(
                self.signature
                    .parameters
                    .get()
                    .iter()
                    .map(|Parameter { name, typ, .. }| (*name, typ.clone())),
            )
            .find(|(name, ..)| *name == ident)
            .map(|(.., typ)| typ.get().clone())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Parameter {
    pub name: InternedString,
    pub typ: Shared<Type>,
}

/// Function parameter and return types
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FunctionSignature {
    pub name: InternedString,
    pub parameters: Shared<Vec<Parameter>>,
    pub return_type: Option<Shared<Type>>,
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NamedType {
    pub name: InternedString,
    pub typ: Shared<Type>,
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NamedValue {
    pub name: InternedString,
    pub value: Shared<Value>,
}

/// Type
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Type {
    // removed before emitting
    Unit,
    String,

    // maybe useful to be distinct?
    Bool,
    Bit,

    Real,
    Float,

    Integer {
        size: Size,
    },
    Bits {
        size: Size,
    },

    Constant(i64),

    Union {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    Struct {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    Tuple(Vec<Shared<Self>>),

    Vector {
        element_type: Shared<Self>,
    },

    FixedVector {
        length: isize,
        element_type: Shared<Self>,
    },

    Reference(Shared<Self>),
}

impl Type {
    // Gets the size of a type
    pub fn get_size(&self) -> Size {
        match self {
            Type::Integer { size } | Type::Bits { size } => size.clone(),
            _ => Size::Unknown,
        }
    }
}

/// Size of a BOOM type in bits
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Size {
    /// Size is known statically at borealis compile time
    Static(usize),
    /// Size is unknown (emitted as uint64)
    Unknown,
}

impl TryFrom<&Size> for Shared<Value> {
    type Error = ();

    fn try_from(value: &Size) -> Result<Self, Self::Error> {
        match value {
            Size::Static(size) => Ok(Literal::Int((*size).try_into().unwrap()).into()),
            Size::Unknown => Err(()),
        }
    }
}

impl TryFrom<Size> for Shared<Value> {
    type Error = ();

    fn try_from(value: Size) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Size::Static(l), Size::Static(r)) => Size::Static(l + r),

            _ => panic!("cannot add unknown"),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Statement {
    VariableDeclaration {
        name: InternedString,
        typ: Shared<Type>,
    },
    Copy {
        expression: Expression,
        value: Shared<Value>,
    },
    FunctionCall {
        expression: Option<Expression>, // expressions to write return value(s) to
        name: InternedString,
        arguments: Vec<Shared<Value>>,
    },
    Label(InternedString),
    Goto(InternedString),
    Jump {
        condition: Shared<Value>,
        target: InternedString,
    },
    End(InternedString),
    Undefined,
    If {
        condition: Shared<Value>,
        if_body: Vec<Shared<Statement>>,
        else_body: Vec<Shared<Statement>>,
    },
    Exit(InternedString),
    Comment(InternedString),
    /// Fatal error, printing the supplied values
    Panic(Shared<Value>),
}

impl From<Statement> for Shared<Statement> {
    fn from(value: Statement) -> Self {
        Shared::new(value)
    }
}

/// Expression
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Expression {
    Identifier(InternedString),
    Field {
        expression: Box<Self>,
        field: InternedString,
    },
    Address(Box<Self>),
    Tuple(Vec<Self>),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Value {
    Identifier(InternedString),
    Literal(Shared<Literal>),
    Operation(Operation),
    Struct {
        name: InternedString,
        fields: Vec<NamedValue>,
    },
    Field {
        value: Shared<Self>,
        field_name: InternedString,
    },
    CtorKind {
        value: Shared<Self>,
        identifier: InternedString,
        types: Vec<Shared<Type>>,
    },
    CtorUnwrap {
        value: Shared<Self>,
        identifier: InternedString,
        types: Vec<Shared<Type>>,
    },
    Tuple(Vec<Shared<Self>>),
    VectorAccess {
        value: Shared<Self>,
        index: Shared<Self>,
    },
    VectorMutate {
        vector: Shared<Self>,
        element: Shared<Self>,
        index: Shared<Self>,
    },
}

impl Value {
    /// Attempts to evaluate the value of a value as a boolean, returning None
    /// on failure
    pub fn evaluate_bool(&self, ctx: &ControlFlowBlock) -> Option<bool> {
        match &self {
            Self::Identifier(identifier) => {
                let defs = ctx
                    .statements()
                    .iter()
                    .filter_map(|statement| {
                        if let Statement::Copy {
                            expression: Expression::Identifier(target_identifier),
                            value,
                        } = &*statement.get()
                        {
                            if identifier == target_identifier {
                                Some(value.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // probably a function parameter, or assignment of result of function
                if defs.is_empty() {
                    return None;
                }

                let value = defs.last().unwrap().get();
                value.evaluate_bool(ctx)
            }
            Self::Literal(literal) => match &*literal.get() {
                Literal::Bool(value) => Some(*value),
                _ => None,
            },

            // Self::Operation(op) => op.evaluate_bool(),
            _ => None,
        }
    }

    /// Gets the identifier of the inner variable, if it exists
    pub fn get_ident(&self) -> Option<InternedString> {
        match self {
            Value::Identifier(ident) => Some(*ident),
            Value::Operation(Operation::Not(value)) => value.get().get_ident(),
            _ => None,
        }
    }
}

impl From<Literal> for Shared<Value> {
    fn from(value: Literal) -> Self {
        Shared::new(Value::Literal(Shared::new(value)))
    }
}

impl From<Operation> for Shared<Value> {
    fn from(value: Operation) -> Self {
        Shared::new(Value::Operation(value))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Literal {
    Int(i128),
    // Little-endian order
    Bits(Vec<Bit>),
    Bit(Bit),
    Bool(bool),
    String(InternedString),
    Vector(Vec<Shared<Literal>>),
    Unit,
    Reference(InternedString),
    Undefined,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Operation {
    Not(Shared<Value>),
    Complement(Shared<Value>),

    Equal(Shared<Value>, Shared<Value>),
    NotEqual(Shared<Value>, Shared<Value>),

    LessThan(Shared<Value>, Shared<Value>),
    LessThanOrEqual(Shared<Value>, Shared<Value>),
    GreaterThan(Shared<Value>, Shared<Value>),
    GreaterThanOrEqual(Shared<Value>, Shared<Value>),

    Subtract(Shared<Value>, Shared<Value>),
    Add(Shared<Value>, Shared<Value>),
    Or(Shared<Value>, Shared<Value>),
    Multiply(Shared<Value>, Shared<Value>),
    And(Shared<Value>, Shared<Value>),
    Xor(Shared<Value>, Shared<Value>),
    Divide(Shared<Value>, Shared<Value>),

    Cast(Shared<Value>, Shared<Type>),

    LeftShift(Shared<Value>, Shared<Value>),
    RightShift(Shared<Value>, Shared<Value>),
    RotateRight(Shared<Value>, Shared<Value>),
    RotateLeft(Shared<Value>, Shared<Value>),
}

/// Bit
#[derive(PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Bit {
    /// Fixed zero
    Zero,
    /// Fixed one
    One,
    /// Unknown bit
    Unknown,
}

impl Bit {
    pub fn is_unknown(&self) -> bool {
        match self {
            Self::Zero | Self::One => false,
            Self::Unknown => true,
        }
    }

    pub fn is_fixed(&self) -> bool {
        !self.is_unknown()
    }

    /// Gets the value of the bit, panicking if unknown
    pub fn value(&self) -> u64 {
        match self {
            Bit::Zero => 0,
            Bit::One => 1,
            Bit::Unknown => panic!("unknown bit has no value"),
        }
    }
}

impl From<u64> for Bit {
    fn from(value: u64) -> Self {
        match value {
            0 => Bit::Zero,
            1 => Bit::One,
            _ => panic!("value must be 0 or 1 to be interpreted as a bit"),
        }
    }
}

impl Debug for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Unknown => write!(f, "x"),
        }
    }
}
