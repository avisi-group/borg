//! Borealis Object Of Machine (BOOM)
//!
//! Internal intermediate representation used to convert JIB AST to GenC AST.

//! TODO USE A STACK TO STORE PARENTS

#![allow(missing_docs)]

use {
    crate::boom::convert::BoomEmitter, common::intern::InternedString, num_bigint::BigInt,
    sail::jib_ast, std::collections::HashSet,
};

pub mod convert;

/// Root AST node
#[derive(Debug, Clone)]
pub struct Ast {
    /// Sequence of definitions
    pub definitions: Vec<Definition>,
    pub labels: HashSet<InternedString>,
}

impl Ast {
    pub fn from_jib<'a, I: IntoIterator<Item = &'a jib_ast::Definition>>(iter: I) -> Self {
        let mut emitter = BoomEmitter::new();
        emitter.process(iter);
        emitter.finish()
    }
}

/// Top-level definition of a BOOM item
#[derive(Debug, Clone)]
pub enum Definition {
    /// Register definition
    Register {
        /// Name of the register
        name: InternedString,
        /// Type of the register
        typ: Type,
    },

    /// Function definition
    Function {
        name: InternedString,
        parameters: Vec<NamedType>,
        return_type: Type,
        body: Vec<Statement>,
    },

    /// Enum definition
    Enum {
        name: InternedString,
        variants: Vec<InternedString>,
    },

    /// Union definition
    Union {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    /// Struct definition
    Struct {
        name: InternedString,
        fields: Vec<NamedType>,
    },

    Pragma {
        key: InternedString,
        value: InternedString,
    },

    Let {
        bindings: Vec<NamedType>,
        body: Vec<Statement>,
    },
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone)]
pub struct NamedType {
    pub name: InternedString,
    pub typ: Type,
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone)]
pub struct NamedValue {
    pub name: InternedString,
    pub value: Value,
}

/// Type
#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Bool,
    String,
    Real,
    Float,
    Constant(BigInt),
    Lint,
    Fint(isize),
    Fbits(isize, bool),
    Lbits(bool),
    Bit,
    Enum {
        name: InternedString,
        variants: Vec<InternedString>,
    },
    Union {
        name: InternedString,
        fields: Vec<NamedType>,
    },
    Struct {
        name: InternedString,
        fields: Vec<NamedType>,
    },
    List {
        element_type: Box<Self>,
    },
    Vector {
        element_type: Box<Self>,
    },
    FVector {
        length: isize,
        element_type: Box<Self>,
    },
    Reference(Box<Self>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    TypeDeclaration {
        name: InternedString,
        typ: Type,
    },
    Block {
        body: Vec<Statement>,
    },
    Try {
        body: Vec<Statement>,
    },
    Copy {
        expression: Expression,
        value: Value,
    },
    Clear {
        identifier: InternedString,
    },
    FunctionCall {
        expression: Expression,
        name: InternedString,
        arguments: Vec<Value>,
    },
    Label(InternedString),
    Goto(InternedString),
    Jump {
        condition: Value,
        target: InternedString,
    },
    End(InternedString),
    Undefined,
    If {
        condition: Value,
        if_body: Vec<Statement>,
        else_body: Vec<Statement>,
    },
    Exit(InternedString),
    Comment(InternedString),
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(InternedString),
    Field {
        expression: Box<Self>,
        field: InternedString,
    },
    Address(Box<Self>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(InternedString),
    Literal(Literal),
    Operation(Operation),
    Struct {
        name: InternedString,
        fields: Vec<NamedValue>,
    },
    Field {
        value: Box<Self>,
        field_name: InternedString,
    },
    CtorKind {
        value: Box<Self>,
        identifier: InternedString,
        types: Vec<Type>,
    },
    CtorUnwrap {
        value: Box<Self>,
        identifier: InternedString,
        types: Vec<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(BigInt),
    Bits(Vec<Bit>),
    Bit(Bit),
    Bool(bool),
    String(InternedString),
    Unit,
    Reference(InternedString),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Not(Box<Value>),
    Equal(Box<Value>, Box<Value>),
    LessThan(Box<Value>, Box<Value>),
    GreaterThan(Box<Value>, Box<Value>),
    Subtract(Box<Value>, Box<Value>),
    Add(Box<Value>, Box<Value>),
}

#[derive(Debug, Clone, Copy)]
pub enum Bit {
    _0,
    _1,
    Unknown,
}
