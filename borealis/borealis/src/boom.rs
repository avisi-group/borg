//! Borealis Object Of Machine (BOOM)
//!
//! Internal intermediate representation used to convert JIB AST to GenC AST.

//! TODO USE A STACK TO STORE PARENTS

#![allow(missing_docs)]

use {common::intern::InternedString, num_bigint::BigInt};

/// Root AST node
#[derive(Debug)]
pub struct Ast {
    /// Sequence of definitions
    pub definitions: Vec<Definition>,
}

/// Top-level definition of a BOOM item
#[derive(Debug)]
pub enum Definition {
    /// Definition of a register
    Register {
        /// Name of the register
        name: InternedString,
        /// Type of the register
        typ: Type,
    },
    Function {
        name: InternedString,
        parameters: Vec<NamedType>,
        body: Vec<Expression>,
    },
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
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug)]
pub struct NamedType {
    pub name: InternedString,
    pub typ: Type,
}

/// Type
#[derive(Debug)]
pub enum Type {
    Unit,
    Bool,
    String,
    Float,
    Constant(BigInt),
}

/// Expression
#[derive(Debug)]
pub enum Expression {
    Identifier(InternedString),
    Field {
        name: InternedString,
        field: InternedString,
    },
    Address(InternedString),
}
