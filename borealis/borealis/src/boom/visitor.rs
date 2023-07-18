//! Visitor pattern for BOOM AST
//!
//! Visitor trait has overridable methods

use {
    crate::boom::{
        control_flow::ControlFlowBlock, Definition, Expression, FunctionDefinition,
        FunctionSignature, Literal, NamedType, NamedValue, Operation, Statement, Type, Value,
    },
    std::{cell::RefCell, rc::Rc},
};

/// Visitor trait for interacting with the BOOM AST
pub trait Visitor: Sized {
    #[allow(missing_docs)]
    fn visit_definition(&mut self, node: &Definition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_function_signature(&mut self, node: &FunctionSignature) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        block.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_named_type(&mut self, node: &NamedType) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_named_value(&mut self, node: &NamedValue) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        node.borrow().walk(self);
    }

    #[allow(missing_docs)]
    fn visit_expression(&mut self, node: &Expression) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_value(&mut self, node: &Value) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_literal(&mut self, node: Rc<RefCell<Literal>>) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_operation(&mut self, node: &Operation) {
        node.walk(self);
    }
}

/// Trait encapsulating the traversing logic for the AST
pub trait Walkable {
    /// Visit children of the current node
    fn walk<V: Visitor>(&self, visitor: &mut V);
}
