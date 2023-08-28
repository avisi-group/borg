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
    kinded::Kinded,
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
                    .map(|Parameter { name, typ, .. }| (*name, typ.clone())),
            )
            .find(|(name, ..)| *name == ident)
            .map(|(.., typ)| typ.borrow().clone())
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: InternedString,
    pub typ: Rc<RefCell<Type>>,
    pub is_ref: bool,
}

/// Function parameter and return types
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: InternedString,
    pub parameters: Rc<RefCell<Vec<Parameter>>>,
    pub return_type: Rc<RefCell<Type>>,
}

impl Walkable for FunctionSignature {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.parameters
            .borrow()
            .iter()
            .for_each(|Parameter { typ, .. }| visitor.visit_type(typ.clone()));
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
    pub value: Rc<RefCell<Value>>,
}

impl Walkable for NamedValue {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_value(self.value.clone());
    }
}

/// Type
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // removed before emitting
    Unit,
    String,

    // maybe useful to be distinct?
    Bool,
    Bit,

    Real,
    Float,

    Int {
        signed: bool,
        size: Size,
    },

    Constant(BigInt),

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

impl Type {
    // Gets the size of a type if it is an integer
    pub fn get_size(&self) -> Option<Size> {
        if let Type::Int { size, .. } = self {
            Some(*size)
        } else {
            None
        }
    }

    // Gets a reference to the size of a type if it is an integer
    pub fn get_size_mut(&mut self) -> Option<&mut Size> {
        if let Type::Int { size, .. } = self {
            Some(size)
        } else {
            None
        }
    }
}

/// Size of a boom integer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    /// Size is known statically at borealis compile time
    Static(usize),
    /// Size is not static but in a local variable
    Runtime(InternedString),
    /// Size is unknown (emitted as uint64)
    Unknown,
}

impl Walkable for Rc<RefCell<Type>> {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        use Type::*;

        match &*self.borrow() {
            Unit | Bool | String | Real | Float | Constant(_) | Int { .. } | Bit | Enum { .. } => {
                ()
            }

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
        value: Rc<RefCell<Value>>,
    },
    FunctionCall {
        expression: Option<Expression>,
        name: InternedString,
        arguments: Vec<Rc<RefCell<Value>>>,
    },
    Label(InternedString),
    Goto(InternedString),
    Jump {
        condition: Rc<RefCell<Value>>,
        target: InternedString,
    },
    End(InternedString),
    Undefined,
    If {
        condition: Rc<RefCell<Value>>,
        if_body: Vec<Rc<RefCell<Statement>>>,
        else_body: Vec<Rc<RefCell<Statement>>>,
    },
    Exit(InternedString),
    Comment(InternedString),
}

impl From<Statement> for Rc<RefCell<Statement>> {
    fn from(value: Statement) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl Walkable for Statement {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::TypeDeclaration { typ, .. } => visitor.visit_type(typ.clone()),
            Self::Copy { expression, value } => {
                visitor.visit_expression(expression);
                visitor.visit_value(value.clone());
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
                    .for_each(|argument| visitor.visit_value(argument.clone()));
            }
            Self::Label(_) => (),
            Self::Goto(_) => (),
            Self::Jump { condition, .. } => visitor.visit_value(condition.clone()),
            Self::End(_) => (),
            Self::Undefined => (),
            Self::If {
                condition,
                if_body,
                else_body,
            } => {
                visitor.visit_value(condition.clone());
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
        value: Rc<RefCell<Self>>,
        field_name: InternedString,
    },
    CtorKind {
        value: Rc<RefCell<Self>>,
        identifier: InternedString,
        types: Vec<Rc<RefCell<Type>>>,
    },
    CtorUnwrap {
        value: Rc<RefCell<Self>>,
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

                let value = defs[0].borrow();
                value.evaluate_bool(ctx)
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
            Value::Operation(Operation::Not(value)) => value.borrow().get_ident(),
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
            Value::Field { value, .. } => visitor.visit_value(value.clone()),
            Value::CtorKind { value, types, .. } | Value::CtorUnwrap { value, types, .. } => {
                visitor.visit_value(value.clone());
                types.iter().for_each(|typ| visitor.visit_type(typ.clone()));
            }
        }
    }
}

impl From<Literal> for Rc<RefCell<Value>> {
    fn from(value: Literal) -> Self {
        Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(value)))))
    }
}

impl From<Operation> for Rc<RefCell<Value>> {
    fn from(value: Operation) -> Self {
        Rc::new(RefCell::new(Value::Operation(value)))
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(BigInt),
    // Little-endian order
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

#[derive(Debug, Clone, Kinded)]
pub enum Operation {
    Not(Rc<RefCell<Value>>),
    Complement(Rc<RefCell<Value>>),

    Equal(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    NotEqual(Rc<RefCell<Value>>, Rc<RefCell<Value>>),

    LessThan(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    LessThanOrEqual(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    GreaterThan(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    GreaterThanOrEqual(Rc<RefCell<Value>>, Rc<RefCell<Value>>),

    Subtract(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Add(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Or(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    And(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Xor(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    Divide(Rc<RefCell<Value>>, Rc<RefCell<Value>>),

    Cast(Rc<RefCell<Value>>, Rc<RefCell<Type>>),

    LeftShift(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    RightShift(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    RotateRight(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
    RotateLeft(Rc<RefCell<Value>>, Rc<RefCell<Value>>),
}

impl Walkable for Operation {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Operation::Not(value) | Operation::Complement(value) => {
                visitor.visit_value(value.clone())
            }
            Operation::Equal(lhs, rhs)
            | Operation::NotEqual(lhs, rhs)
            | Operation::LessThan(lhs, rhs)
            | Operation::GreaterThan(lhs, rhs)
            | Operation::LessThanOrEqual(lhs, rhs)
            | Operation::GreaterThanOrEqual(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Add(lhs, rhs)
            | Operation::Or(lhs, rhs)
            | Operation::Xor(lhs, rhs)
            | Operation::And(lhs, rhs)
            | Operation::Divide(lhs, rhs)
            | Operation::LeftShift(lhs, rhs)
            | Operation::RightShift(lhs, rhs)
            | Operation::RotateLeft(lhs, rhs)
            | Operation::RotateRight(lhs, rhs) => {
                visitor.visit_value(lhs.clone());
                visitor.visit_value(rhs.clone());
            }
            Operation::Cast(value, typ) => {
                visitor.visit_value(value.clone());
                visitor.visit_type(typ.clone());
            }
        }
    }
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
