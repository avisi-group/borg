//! Borealis Object Of Machine (BOOM)
//!
//! Internal intermediate representation used to convert JIB AST to GenC AST.

//! TODO USE A STACK TO STORE PARENTS

#![allow(missing_docs)]

use {
    crate::boom::{
        convert::BoomEmitter,
        visitor::{Visitor, Walkable},
    },
    common::intern::InternedString,
    num_bigint::BigInt,
    sail::jib_ast,
    std::collections::HashMap,
};

pub mod convert;
pub mod visitor;

/// Root AST node
#[derive(Debug, Clone)]
pub struct Ast {
    /// Sequence of definitions
    pub definitions: Vec<Definition>,
    /// Register types by identifier
    pub registers: HashMap<InternedString, Type>,
    /// Function definitions by identifier
    pub functions: HashMap<InternedString, FunctionDefinition>,
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

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Enum { .. } | Self::Pragma { .. } => (),

            Self::Union { fields, .. } | Self::Struct { fields, .. } => {
                fields
                    .iter()
                    .for_each(|named_type| visitor.visit_named_type(named_type));
            }

            Self::Let { bindings, body } => {
                bindings
                    .iter()
                    .for_each(|named_type| visitor.visit_named_type(named_type));

                body.iter()
                    .for_each(|statement| visitor.visit_statement(statement));
            }
        }
    }
}

/// Function signature and body
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub signature: FunctionSignature,
    pub body: Vec<Statement>,
}

impl Walkable for FunctionDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_signature(&self.signature);
        self.body
            .iter()
            .for_each(|statement| visitor.visit_statement(statement));
    }
}

/// Function parameter and return types
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub parameters: Vec<NamedType>,
    pub return_type: Type,
}

impl Walkable for FunctionSignature {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.parameters
            .iter()
            .for_each(|parameter| visitor.visit_named_type(parameter));
        visitor.visit_type(&self.return_type);
    }
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone)]
pub struct NamedType {
    pub name: InternedString,
    pub typ: Type,
}

impl Walkable for NamedType {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type(&self.typ);
    }
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone)]
pub struct NamedValue {
    pub name: InternedString,
    pub value: Value,
}

impl Walkable for NamedValue {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_value(&self.value);
    }
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

impl Walkable for Type {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Unit
            | Self::Bool
            | Self::String
            | Self::Real
            | Self::Float
            | Self::Constant(_)
            | Self::Lint
            | Self::Fint(_)
            | Self::Fbits(_, _)
            | Self::Lbits(_)
            | Self::Bit
            | Self::Enum { .. } => (),

            Self::Union { fields, .. } | Self::Struct { fields, .. } => fields
                .iter()
                .for_each(|field| visitor.visit_named_type(field)),

            Self::List { element_type }
            | Self::Vector { element_type }
            | Self::FVector { element_type, .. }
            | Self::Reference(element_type) => visitor.visit_type(element_type),
        }
    }
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

impl Walkable for Statement {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::TypeDeclaration { typ, .. } => visitor.visit_type(typ),
            Self::Block { body } | Self::Try { body } => body
                .iter()
                .for_each(|statement| visitor.visit_statement(statement)),
            Self::Copy { expression, value } => {
                visitor.visit_expression(expression);
                visitor.visit_value(value);
            }
            Self::Clear { .. } => (),
            Self::FunctionCall {
                expression,
                arguments,
                ..
            } => {
                visitor.visit_expression(expression);
                arguments
                    .iter()
                    .for_each(|argument| visitor.visit_value(argument));
            }
            Self::Label(_) => (),
            Self::Goto(_) => (),
            Self::Jump { condition, .. } => visitor.visit_value(condition),
            Self::End(_) => (),
            Self::Undefined => (),
            Self::If {
                condition,
                if_body,
                else_body,
            } => {
                visitor.visit_value(condition);
                if_body
                    .iter()
                    .for_each(|statement| visitor.visit_statement(statement));
                else_body
                    .iter()
                    .for_each(|statement| visitor.visit_statement(statement));
            }
            Self::Exit(_) => (),
            Self::Comment(_) => (),
        }
    }
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

impl Walkable for Expression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Identifier(_) => (),
            Self::Field { expression, .. } | Self::Address(expression) => {
                visitor.visit_expression(expression)
            }
        }
    }
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

impl Walkable for Value {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Value::Identifier(_) => (),
            Value::Literal(literal) => visitor.visit_literal(literal),
            Value::Operation(operation) => visitor.visit_operation(operation),
            Value::Struct { fields, .. } => fields
                .iter()
                .for_each(|field| visitor.visit_named_value(field)),
            Value::Field { value, .. } => visitor.visit_value(value),
            Value::CtorKind { value, types, .. } | Value::CtorUnwrap { value, types, .. } => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ));
            }
        }
    }
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

impl Walkable for Literal {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
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

impl Walkable for Operation {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Operation::Not(value) => visitor.visit_value(value),
            Operation::Equal(lhs, rhs)
            | Operation::LessThan(lhs, rhs)
            | Operation::GreaterThan(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Add(lhs, rhs) => {
                visitor.visit_value(lhs);
                visitor.visit_value(rhs);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Bit {
    _0,
    _1,
    Unknown,
}
