//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

use {
    crate::boom::{
        passes::{
            builtin_fns::AddBuiltinFns, fold_unconditionals::FoldUnconditionals,
            if_raiser::IfRaiser, remove_empty::RemoveEmpty,
        },
        Ast,
    },
    std::{cell::RefCell, rc::Rc},
};

mod builtin_fns;
mod fold_unconditionals;
mod if_raiser;
mod remove_empty;

pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    [
        RemoveEmpty::new_boxed(),
        FoldUnconditionals::new_boxed(),
        IfRaiser::new_boxed(),
        AddBuiltinFns::new_boxed(ast.clone()),
    ]
    .into_iter()
    .for_each(|mut pass| pass.run(ast.clone()));
}

pub trait Pass {
    /// Run the pass on the supplied AST
    fn run(&mut self, ast: Rc<RefCell<Ast>>);
}
