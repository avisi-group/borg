//! Visitor pattern for JIB AST
//!
//! Visitor trait has overridable methods

use crate::jib_ast::{
    Definition, Expression, Init, Instruction, Name, Op, Type, TypeDefinition, Value, Vl,
};

/// Visitor trait for interacting with Sail AST
pub trait Visitor: Sized {
    #[allow(missing_docs)]
    fn visit_definition(&mut self, node: &Definition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_instruction(&mut self, node: &Instruction) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_type_definition(&mut self, node: &TypeDefinition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_value(&mut self, node: &Value) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_expression(&mut self, node: &Expression) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_op(&mut self, node: &Op) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_type(&mut self, node: &Type) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_name(&mut self, node: &Name) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_init(&mut self, node: &Init) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_vl(&mut self, node: &Vl) {
        node.walk(self);
    }
}

/// Trait encapsulating the traversing logic for the AST
pub trait Walkable {
    /// Visit children of the current node
    fn walk<V: Visitor>(&self, visitor: &mut V);
}
