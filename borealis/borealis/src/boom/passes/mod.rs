//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

use crate::boom::passes::remove_const_branch::RemoveConstBranch;

use {
    crate::boom::{
        passes::{
            builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            fold_unconditionals::FoldUnconditionals,
        },
        Ast,
    },
    log::info,
    std::{
        cell::RefCell,
        fs::{create_dir_all, File},
        path::PathBuf,
        rc::Rc,
    },
};

mod builtin_fns;
mod cycle_finder;
mod fold_unconditionals;
mod remove_const_branch;

pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    //  let initial_statements = ast.borrow().statements();

    dump_func_dot(&ast, "__Reset", None);
    dump_func_dot(&ast, "system_sysops", None);

    [
        FoldUnconditionals::new_boxed(),
        RemoveConstBranch::new_boxed(),
        FoldUnconditionals::new_boxed(),
        CycleFinder::new_boxed(),
        AddBuiltinFns::new_boxed(ast.clone()),
    ]
    .into_iter()
    .for_each(|mut pass| {
        info!("{}", pass.name());

        pass.run(ast.clone())
    });

    dump_func_dot(&ast, "__Reset", Some("__Reset_after"));
    dump_func_dot(&ast, "system_sysops", Some("system_sysops_after"));

    // currently, passes should not modify any statements so we smoke test that they are all still there
    // assert_eq!(initial_statements, ast.borrow().statements());
}

pub trait Pass {
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST
    fn run(&mut self, ast: Rc<RefCell<Ast>>);
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
