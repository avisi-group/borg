//! Visitor pattern for Sail AST
//!
//! Vistitor trait has overridable methods

use crate::ast::{Ast, Comment, CommentRoot, Definition, Identifier, TypeDefinition};

/// Visitor trait for interacting with Sail AST
pub trait Visitor: Sized {
    #[allow(missing_docs)]
    fn visit_root(&mut self, node: &Ast) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_definition(&mut self, node: &Definition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_type_definition(&mut self, node: &TypeDefinition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_identifier(&mut self, node: &Identifier) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_comment_root(&mut self, node: &CommentRoot) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_comment(&mut self, node: &Comment) {
        node.walk(self);
    }
}

/// Trait encapsulating the traversing logic for the AST
pub trait Walkable {
    /// Visit children of the current node
    fn walk<V: Visitor>(&self, visitor: &mut V);
}
