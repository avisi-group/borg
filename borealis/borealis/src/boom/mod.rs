//! Borealis Object Of Machine, Internal intermediate representation used to
//! convert JIB AST to GenC AST

#![allow(missing_docs)]

use {
    crate::boom::{
        control_flow::ControlFlowBlock,
        convert::BoomEmitter,
        pretty_print::BoomPrettyPrinter,
        visitor::{Visitor, Walkable},
    },
    common::{intern::InternedString, shared_key::SharedKey, HashMap, HashSet},
    num_bigint::BigInt,
    sail::jib_ast,
    std::{
        cell::RefCell,
        fmt::{self, Debug, Display, Formatter},
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
    pub registers: HashMap<InternedString, Rc<RefCell<Type>>>,
    /// Function definitions by identifier
    pub functions: HashMap<InternedString, FunctionDefinition>,
}

impl Ast {
    /// Converts JIB AST into BOOM AST
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
        let mut statements = HashSet::default();

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
                let mut visited = HashSet::default();
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
        self.entry_block
            .iter()
            .for_each(|block| visitor.visit_control_flow_block(&block));
    }
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
                if let Statement::TypeDeclaration { name, typ } = &*statement.borrow() {
                    Some((*name, typ.clone()))
                } else {
                    None
                }
            })
            .chain(
                self.signature
                    .parameters
                    .borrow()
                    .iter()
                    .map(|NamedType { name, typ }| (*name, typ.clone())),
            )
            .find(|(name, ..)| *name == ident)
            .map(|(.., typ)| typ.borrow().clone())
    }
}

/// Function parameter and return types
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: InternedString,
    pub parameters: Rc<RefCell<Vec<NamedType>>>,
    pub return_type: Rc<RefCell<Type>>,
}

impl Walkable for FunctionSignature {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.parameters
            .borrow()
            .iter()
            .for_each(|parameter| visitor.visit_named_type(parameter));
        visitor.visit_type(self.return_type.clone());
    }
}

/// Name and type of a union field, struct field, or function parameter
#[derive(Debug, Clone, PartialEq)]
pub struct NamedType {
    pub name: InternedString,
    pub typ: Rc<RefCell<Type>>,
}

impl Walkable for NamedType {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type(self.typ.clone());
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
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Bool,

    String,

    Real,
    Float,
    Constant(BigInt),
    FixedInt(isize), // int64_t
    LargeInt,

    Bit,
    FixedBits(isize, bool), // uint64_t
    LargeBits(bool),

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
        element_type: Rc<RefCell<Self>>,
    },

    Vector {
        element_type: Rc<RefCell<Self>>,
    },

    FixedVector {
        length: isize,
        element_type: Rc<RefCell<Self>>,
    },

    Reference(Rc<RefCell<Self>>),
}

impl Walkable for Rc<RefCell<Type>> {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        use Type::*;

        match &*self.borrow() {
            Unit
            | Bool
            | String
            | Real
            | Float
            | Constant(_)
            | LargeInt
            | FixedInt(_)
            | FixedBits(_, _)
            | LargeBits(_)
            | Bit
            | Enum { .. } => (),

            Union { fields, .. } | Struct { fields, .. } => fields
                .iter()
                .for_each(|field| visitor.visit_named_type(field)),

            List { element_type }
            | Vector { element_type }
            | FixedVector { element_type, .. }
            | Reference(element_type) => visitor.visit_type(element_type.clone()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut visitor = BoomPrettyPrinter::new(f);
        //TODO: this is probably bad and slow
        visitor.visit_type(Rc::new(RefCell::new(self.clone())));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    TypeDeclaration {
        name: InternedString,
        typ: Rc<RefCell<Type>>,
    },
    Copy {
        expression: Expression,
        value: Value,
    },
    FunctionCall {
        expression: Option<Expression>,
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
            Self::TypeDeclaration { typ, .. } => visitor.visit_type(typ.clone()),
            Self::Copy { expression, value } => {
                visitor.visit_expression(expression);
                visitor.visit_value(value);
            }

            Self::FunctionCall {
                expression,
                arguments,
                ..
            } => {
                if let Some(expression) = expression {
                    visitor.visit_expression(expression);
                }
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
    Literal(Rc<RefCell<Literal>>),
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
        types: Vec<Rc<RefCell<Type>>>,
    },
    CtorUnwrap {
        value: Box<Self>,
        identifier: InternedString,
        types: Vec<Rc<RefCell<Type>>>,
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
                if defs.is_empty() {
                    return None;
                }

                // variable should be assigned to exactly once
                assert!(defs.len() == 1);

                defs[0].evaluate_bool(ctx)
            }
            Self::Literal(literal) => match &*literal.borrow() {
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
            Value::Operation(Operation::Not(value)) => value.get_ident(),
            _ => None,
        }
    }
}

impl Walkable for Value {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Value::Identifier(_) => (),
            Value::Literal(literal) => visitor.visit_literal(literal.clone()),
            Value::Operation(operation) => visitor.visit_operation(operation),
            Value::Struct { fields, .. } => fields
                .iter()
                .for_each(|field| visitor.visit_named_value(field)),
            Value::Field { value, .. } => visitor.visit_value(value),
            Value::CtorKind { value, types, .. } | Value::CtorUnwrap { value, types, .. } => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ.clone()));
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

impl Walkable for Rc<RefCell<Literal>> {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Not(Box<Value>),
    Complement(Box<Value>),
    Equal(Box<Value>, Box<Value>),
    LessThan(Box<Value>, Box<Value>),
    GreaterThan(Box<Value>, Box<Value>),
    Subtract(Box<Value>, Box<Value>),
    Add(Box<Value>, Box<Value>),
    Or(Box<Value>, Box<Value>),
    And(Box<Value>, Box<Value>),
    LeftShift(Box<Value>, Box<Value>),
    RightShift(Box<Value>, Box<Value>),
}

impl Walkable for Operation {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Operation::Not(value) | Operation::Complement(value) => visitor.visit_value(value),
            Operation::Equal(lhs, rhs)
            | Operation::LessThan(lhs, rhs)
            | Operation::GreaterThan(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Add(lhs, rhs)
            | Operation::Or(lhs, rhs)
            | Operation::And(lhs, rhs)
            | Operation::LeftShift(lhs, rhs)
            | Operation::RightShift(lhs, rhs) => {
                visitor.visit_value(lhs);
                visitor.visit_value(rhs);
            }
        }
    }
}

pub enum OperationKind {
    Not,
    Complement,
    Equal,
    LessThan,
    GreaterThan,
    Subtract,
    Add,
    Or,
    And,
    LeftShift,
    RightShift,
}

/// Bit
#[derive(PartialEq, Eq, Clone, Copy)]
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

impl Debug for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Unknown => write!(f, "x"),
        }
    }
}

/// Converts a sequence of bits to an integer
pub fn bits_to_int<B: AsRef<[Bit]>>(bits: B) -> u64 {
    let bits = bits.as_ref();

    assert!(bits.iter().all(Bit::is_fixed));

    bits.iter().rev().fold(0, |acc, bit| acc << 1 | bit.value())
}
