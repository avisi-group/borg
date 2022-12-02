//! Visitor pattern for Sail AST
//!
//! Vistitor trait has overridable methods

use {
    crate::{
        ast::{Ast, Comment, Definition},
        types::OCamlString,
    },
    std::collections::LinkedList,
};

/// Visitor trait for interacting with Sail AST
pub trait Visitor: Sized {
    #[allow(missing_docs)]
    fn visit_root(&mut self, node: &Ast) {
        walk_root(self, node);
    }

    #[allow(missing_docs)]
    fn visit_definition(&mut self, node: &Definition) {
        walk_definition(self, node);
    }

    #[allow(missing_docs)]
    fn visit_comment_root(&mut self, node: &(OCamlString, LinkedList<Comment>)) {
        walk_comment_root(self, node);
    }

    #[allow(missing_docs)]
    fn visit_comment(&mut self, node: &Comment) {
        walk_comment(self, node);
    }
}

/// Root walk function for walking an AST
pub fn walk_root<V: Visitor>(v: &mut V, node: &Ast) {
    node.defs.iter().for_each(|d| v.visit_definition(d));
    node.comments.iter().for_each(|c| v.visit_comment_root(c));
}

#[allow(missing_docs)]
pub fn walk_definition<V: Visitor>(_v: &mut V, _node: &Definition) {}

#[allow(missing_docs)]
pub fn walk_comment_root<V: Visitor>(v: &mut V, node: &(OCamlString, LinkedList<Comment>)) {
    node.1.iter().for_each(|c| v.visit_comment(c))
}

#[allow(missing_docs)]
pub fn walk_comment<V: Visitor>(_v: &mut V, _node: &Comment) {}
