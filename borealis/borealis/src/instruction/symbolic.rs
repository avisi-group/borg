//! Symbolic Sail execution

use sail::{
    ast::{Ast, FunctionClause},
    visitor::Visitor,
};

/// Symbolic executor used for determining instruction semantics
pub struct SymbolicExecutor<'ast> {
    _ast: &'ast Ast,
}

impl<'ast> SymbolicExecutor<'ast> {
    /// Create a new `SymbolicExecutor`
    pub fn new(ast: &'ast Ast) -> Self {
        Self { _ast: ast }
    }

    /// Run the symbolic executor on the supplied function clause
    pub fn run(&mut self, instruction: &FunctionClause) -> String {
        self.visit_function_clause(instruction);
        // execute AST symbolically building GenC implementation of instruction semantics
        "".to_owned()
    }
}

impl<'ast> Visitor for SymbolicExecutor<'ast> {}
