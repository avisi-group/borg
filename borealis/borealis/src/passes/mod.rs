//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

use {
    crate::{
        boom::Ast,
        passes::{
            builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            fold_unconditionals::FoldUnconditionals, remove_const_branch::RemoveConstBranch,
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
mod fold_unconditionals;
mod remove_const_branch;

/// Executes the optimisation and raising passes on an AST
pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    dump_func_dot(&ast, "__Reset", None);
    dump_func_dot(&ast, "system_sysops", None);

    run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            CycleFinder::new_boxed(),
            AddBuiltinFns::new_boxed(ast.clone()),
        ],
    );

    dump_func_dot(&ast, "__Reset", Some("__Reset_after"));
    dump_func_dot(&ast, "system_sysops", Some("system_sysops_after"));
}

/// Pass that performs an operation on an AST
pub trait Pass {
    /// Gets the name of the pass
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST, returning whether the AST was changed
    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool;
}

/// Run each pass until it does not mutate the AST, and run the whole sequence of passes until no pass mutates the AST
fn run_fixed_point(ast: Rc<RefCell<Ast>>, passes: &mut [Box<dyn Pass>]) {
    loop {
        let mut ast_did_change = false;

        passes.into_iter().for_each(|pass| loop {
            info!("{}", pass.name());

            let pass_ast_did_change = pass.run(ast.clone());

            // finish once the pass no longer mutates the AST
            if !pass_ast_did_change {
                break;
            } else {
                ast_did_change = true;
            }
        });

        if !ast_did_change {
            break;
        }
    }
}

fn dump_func_dot(ast: &Rc<RefCell<Ast>>, func: &'static str, filename: Option<&'static str>) {
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
