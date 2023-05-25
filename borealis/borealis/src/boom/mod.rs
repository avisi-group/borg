//! Borealis Object Of Machine (BOOM)
//!
//! Internal intermediate representation used to convert JIB AST to GenC AST.

#![allow(missing_docs)]

use {
    crate::boom::{
        control_flow::ControlFlowBlock,
        convert::BoomEmitter,
        visitor::{Visitor, Walkable},
    },
    common::{intern::InternedString, shared_key::SharedKey},
    num_bigint::BigInt,
    sail::jib_ast,
    std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
        rc::Rc,
    },
};

pub mod control_flow;
pub mod convert;
pub mod pretty_print;
pub mod visitor;

/// BOOM AST
#[derive(Debug, Clone, Default)]
pub struct Ast {
    /// Sequence of definitions
    pub definitions: Vec<Definition>,
    /// Register types by identifier
    pub registers: HashMap<InternedString, Type>,
    /// Function definitions by identifier
    pub functions: HashMap<InternedString, FunctionDefinition>,
}

impl Ast {
    pub fn from_jib<'a, I: IntoIterator<Item = &'a jib_ast::Definition>>(
        iter: I,
    ) -> Rc<RefCell<Self>> {
        let mut emitter = BoomEmitter::new();
        emitter.process(iter);
        Rc::new(RefCell::new(emitter.finish()))
    }

    /// Collects all statements in the AST into a HashSet
    ///
    /// Used to verify that none have been lost during the passes
    pub fn statements(&self) -> HashSet<SharedKey<Statement>> {
        let mut statements = HashSet::new();

        self.definitions.iter().for_each(|def| {
            if let Definition::Let { body, .. } = def {
                statements.extend(body.iter().cloned().map(Into::into));
            }
        });

        self.functions.values().for_each(
            |FunctionDefinition {
                 entry_block: control_flow,
                 ..
             }| {
                let mut visited = HashSet::new();
                let mut to_visit = vec![control_flow.clone()];

                while let Some(current) = to_visit.pop() {
                    if visited.contains(&current) {
                        return;
                    }

                    visited.insert(current.clone());
                    to_visit.extend(current.terminator().targets());

                    statements.extend(current.statements().into_iter().map(Into::into));
                }
            },
        );

        statements
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
        body: Vec<Rc<RefCell<Statement>>>,
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
                    .for_each(|statement| visitor.visit_statement(statement.clone()));
            }
        }
    }
}

/// Function signature and body
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    /// Function type signature
    pub signature: FunctionSignature,
    /// Entry block into the control flow graph
    pub entry_block: ControlFlowBlock,
}

impl Walkable for FunctionDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_signature(&self.signature);
    }
}

/// Function parameter and return types
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: InternedString,
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
    Try {
        body: Vec<Rc<RefCell<Statement>>>,
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
        if_body: Vec<Rc<RefCell<Statement>>>,
        else_body: Vec<Rc<RefCell<Statement>>>,
    },
    Exit(InternedString),
    Comment(InternedString),
}

impl Walkable for Statement {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::TypeDeclaration { typ, .. } => visitor.visit_type(typ),
            Self::Try { body } => body
                .iter()
                .for_each(|statement| visitor.visit_statement(statement.clone())),
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
                    .for_each(|statement| visitor.visit_statement(statement.clone()));
                else_body
                    .iter()
                    .for_each(|statement| visitor.visit_statement(statement.clone()));
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

impl Value {
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
                        } = &*statement.borrow()
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
                if defs.len() == 0 {
                    return None;
                }

                // variable should be assigned to exactly once
                assert!(defs.len() == 1);

                defs[0].evaluate_bool(ctx)
            }
            Self::Literal(Literal::Bool(value)) => Some(*value),
            Self::Literal(_) => None,
            // Self::Operation(op) => op.evaluate_bool(),
            _ => None,
        }
    }
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

/// Enum containing all possible node kinds
#[derive(Debug, Clone)]
pub enum NodeKind {
    Statement(Rc<RefCell<Statement>>),
    Expression(Rc<RefCell<Expression>>),
    Value(Rc<RefCell<Value>>),
    Literal(Rc<RefCell<Literal>>),
    Operation(Rc<RefCell<Operation>>),
    Type(Rc<RefCell<Type>>),
    NamedValue(Rc<RefCell<NamedValue>>),
}
