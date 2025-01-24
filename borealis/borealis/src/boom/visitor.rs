//! Visitor pattern for BOOM AST
//!
//! Visitor trait has overridable methods

use {
    common::boom::{
        control_flow::ControlFlowBlock, Definition, Expression, FunctionDefinition,
        FunctionSignature, Literal, NamedType, NamedValue, Operation, Parameter, Statement, Type,
        Value,
    },
    common::shared::Shared,
};

/// Trait encapsulating the traversing logic for the AST
pub trait Walkable {
    /// Visit children of the current node
    fn walk<V: Visitor>(&self, visitor: &mut V);
}

/// Visitor trait for interacting with the BOOM AST
#[allow(missing_docs)]
pub trait Visitor: Sized {
    fn visit_definition(&mut self, node: &Definition) {
        node.walk(self);
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        node.walk(self);
    }

    fn visit_function_signature(&mut self, node: &FunctionSignature) {
        node.walk(self);
    }

    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        block.walk(self);
    }

    fn visit_named_type(&mut self, node: &NamedType) {
        node.walk(self);
    }

    fn visit_named_value(&mut self, node: &NamedValue) {
        node.walk(self);
    }

    fn visit_type(&mut self, node: Shared<Type>) {
        node.walk(self);
    }

    fn visit_parameter(&mut self, node: &Parameter) {
        node.walk(self);
    }

    fn visit_statement(&mut self, node: Shared<Statement>) {
        node.get().walk(self);
    }

    fn visit_expression(&mut self, node: &Expression) {
        node.walk(self);
    }

    fn visit_value(&mut self, node: Shared<Value>) {
        node.get().walk(self);
    }

    fn visit_literal(&mut self, node: Shared<Literal>) {
        node.get().walk(self);
    }

    fn visit_operation(&mut self, node: &Operation) {
        node.walk(self);
    }
}

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Pragma { .. } => (),

            Self::Struct { fields, .. } => {
                fields
                    .iter()
                    .for_each(|named_type| visitor.visit_named_type(named_type));
            }

            Self::Union { fields, .. } => fields
                .iter()
                .for_each(|named_type| visitor.visit_named_type(named_type)),
        }
    }
}

impl Walkable for FunctionDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function_signature(&self.signature);
        self.entry_block
            .iter()
            .for_each(|block| visitor.visit_control_flow_block(&block));
    }
}

impl Walkable for Parameter {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type(self.typ.clone());
    }
}

impl Walkable for FunctionSignature {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.parameters
            .get()
            .iter()
            .for_each(|Parameter { typ, .. }| visitor.visit_type(typ.clone()));

        if let Some(return_type) = &self.return_type {
            visitor.visit_type(return_type.clone());
        }
    }
}

impl Walkable for NamedType {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type(self.typ.clone());
    }
}

impl Walkable for NamedValue {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_value(self.value.clone());
    }
}

impl Walkable for Shared<Type> {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        use Type::*;

        match &*self.get() {
            Unit
            | Bool
            | String
            | Real
            | Float
            | Constant(_)
            | Integer { .. }
            | Bits { .. }
            | Bit
            | Union { .. } => {}

            Struct { fields, .. } => fields
                .iter()
                .for_each(|field| visitor.visit_named_type(field)),

            Vector { element_type }
            | FixedVector { element_type, .. }
            | Reference(element_type) => visitor.visit_type(element_type.clone()),

            Tuple(ts) => ts.iter().for_each(|t| visitor.visit_type(t.clone())),
        }
    }
}

impl Walkable for Statement {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::VariableDeclaration { typ, .. } => visitor.visit_type(typ.clone()),
            Self::Copy { expression, value } => {
                visitor.visit_expression(expression);
                visitor.visit_value(value.clone());
            }

            Self::FunctionCall {
                expression: expressions,
                arguments,
                ..
            } => {
                if let Some(expression) = expressions {
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
            Self::Panic(value) => visitor.visit_value(value.clone()),
        }
    }
}

impl Walkable for Expression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Identifier(_) => (),
            Self::Field { expression, .. } | Self::Address(expression) => {
                visitor.visit_expression(expression)
            }
            Self::Tuple(exprs) => exprs.iter().for_each(|e| visitor.visit_expression(e)),
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
            Value::Tuple(values) => values.iter().for_each(|v| visitor.visit_value(v.clone())),
            Value::VectorAccess { value, index } => {
                visitor.visit_value(value.clone());
                visitor.visit_value(index.clone());
            }
            Value::VectorMutate {
                vector,
                element,
                index,
            } => {
                visitor.visit_value(vector.clone());
                visitor.visit_value(element.clone());
                visitor.visit_value(index.clone());
            }
        }
    }
}

impl Walkable for Literal {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
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
            | Operation::Multiply(lhs, rhs)
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

impl Walkable for ControlFlowBlock {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.statements()
            .iter()
            .cloned()
            .for_each(|statement| visitor.visit_statement(statement));
    }
}
