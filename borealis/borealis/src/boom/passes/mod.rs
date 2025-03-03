//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure
//!   if-else, match, and for loops
//! * Builtin function handling

use {
    crate::boom::Ast,
    common::intern::InternedString,
    log::info,
    sailrs::shared::Shared,
    std::{
        fs::{File, create_dir_all},
        path::PathBuf,
    },
};

pub mod any;
pub mod builtin_fns;
pub mod constant_propogation;
pub mod cycle_finder;
pub mod destruct_composites;
pub mod fold_unconditionals;
pub mod lower_reals;
pub mod monomorphize_vectors;
pub mod remove_const_branch;
pub mod remove_constant_type;
pub mod remove_units;
/// Pass that performs an operation on an AST
pub trait Pass {
    /// Gets the name of the pass
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST, returning whether the AST was changed
    fn run(&mut self, ast: Shared<Ast>) -> bool;

    /// Resets any state in a pass to it's initial/empty state
    fn reset(&mut self);
}

/// Run each pass until it does not mutate the AST, and run the whole sequence
/// of passes until no pass mutates the AST
pub fn run_fixed_point(ast: Shared<Ast>, passes: &mut [Box<dyn Pass>]) {
    // ironically, we *do* want to short-circuit here
    // behaviour is "keep running the passes in order until none change"
    loop {
        if !passes
            .iter_mut()
            .map(|pass| {
                info!("{}", pass.name());
                pass.reset();
                pass.run(ast.clone())
            })
            .any(|did_change| did_change)
        {
            break;
        }
    }
}

fn _dump_func_dot(ast: Shared<Ast>, func: &'static str, filename: Option<&'static str>) {
    let path = PathBuf::from(format!("target/dot/{}.dot", filename.unwrap_or(func)));

    create_dir_all(path.parent().unwrap()).unwrap();

    ast.get()
        .functions
        .get(&InternedString::from_static(func))
        .unwrap()
        .entry_block
        .as_dot(&mut File::create(path).unwrap())
        .unwrap()
}
