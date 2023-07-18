//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure
//!   if-else, match, and for loops
//! * Builtin function handling

use {
    crate::{
        boom::Ast,
        passes::{
            builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            fold_unconditionals::FoldUnconditionals, remove_const_branch::RemoveConstBranch,
            remove_exception::RemoveExceptions, remove_unit::RemoveUnits,
            replace_bitvectors::ReplaceBitvectors, replace_bools::ReplaceBools,
            resolve_return_assigns::ResolveReturns,
        },
    },
    log::info,
    std::{
        cell::RefCell,
        fs::{create_dir_all, File},
        path::PathBuf,
        rc::Rc,
    },
};

mod any;
mod builtin_fns;
mod cycle_finder;
mod destructure;
mod fold_unconditionals;
mod remove_const_branch;
mod remove_exception;
mod remove_unit;
mod replace_bitvectors;
mod replace_bools;
mod resolve_return_assigns;

/// Executes the optimisation and raising passes on an AST
pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            CycleFinder::new_boxed(),
            ReplaceBools::new_boxed(),
            RemoveUnits::new_boxed(ast),
            ResolveReturns::new_boxed(),
            ReplaceBitvectors::new_boxed(),
            RemoveExceptions::new_boxed(),
            AddBuiltinFns::new_boxed(),
        ],
    );
}

/// Pass that performs an operation on an AST
pub trait Pass {
    /// Gets the name of the pass
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST, returning whether the AST was changed
    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool;
}

/// Run each pass until it does not mutate the AST, and run the whole sequence
/// of passes until no pass mutates the AST
fn run_fixed_point(ast: Rc<RefCell<Ast>>, passes: &mut [Box<dyn Pass>]) {
    // ironically, we *do* want to short-circuit here
    // behaviour is "keep running the passes in order until none change"
    loop {
        if !passes
            .iter_mut()
            .map(|pass| {
                info!("{}", pass.name());
                pass.run(ast.clone())
            })
            .any(|did_change| did_change)
        {
            break;
        }
    }
}

fn _dump_func_dot(ast: &Rc<RefCell<Ast>>, func: &'static str, filename: Option<&'static str>) {
    let path = PathBuf::from(format!("target/dot/{}.dot", filename.unwrap_or(func)));

    create_dir_all(path.parent().unwrap()).unwrap();

    ast.borrow()
        .functions
        .get(&func.into())
        .unwrap()
        .entry_block
        .as_dot(&mut File::create(path).unwrap())
        .unwrap()
}
