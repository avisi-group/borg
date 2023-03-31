//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

use crate::boom::{passes::builtin_fns::AddBuiltinFns, Ast};

pub mod builtin_fns;

pub fn execute_passes(ast: &mut Ast) {
    [Box::new(AddBuiltinFns)]
        .iter_mut()
        .for_each(|pass| pass.run(ast));
}

pub trait Pass {
    /// Run the pass on the supplied AST
    fn run(&mut self, ast: &mut Ast);
}
