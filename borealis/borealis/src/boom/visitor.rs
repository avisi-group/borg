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

    /// Returns whether the supplied ControlFlowBlock has been visited before
    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool;

    /// Marks the supplied ControlFlowBlock as visited
    fn set_block_visited(&mut self, block: &ControlFlowBlock);

    #[allow(missing_docs)]
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        if self.is_block_visited(block) {
            return;
        }

        self.set_block_visited(block);
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
