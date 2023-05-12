//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

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

pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    let initial_statements = ast.borrow().statements();

    [
        FoldUnconditionals::new_boxed(),
        CycleFinder::new_boxed(),
        AddBuiltinFns::new_boxed(ast.clone()),
    ]
    .into_iter()
    .for_each(|mut pass| {
        info!("{}", pass.name());

        pass.run(ast.clone());
    });

    dump_func_dot(&ast, "__Reset");
    dump_func_dot(&ast, "__EndCycle");
    dump_func_dot(&ast, "__ListConfig");

    // currently, passes should not modify any statements so we smoke test that they are all still there
    assert_eq!(initial_statements, ast.borrow().statements());
}

pub trait Pass {
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST
    fn run(&mut self, ast: Rc<RefCell<Ast>>);
}

fn dump_func_dot(ast: &Rc<RefCell<Ast>>, func: &'static str) {
    let path = PathBuf::from(format!("target/dot/{func}.dot"));

    create_dir_all(path.parent().unwrap()).unwrap();

    ast.borrow()
        .functions
        .get(&func.into())
        .unwrap()
        .entry_block
        .as_dot(&mut File::create(path).unwrap())
        .unwrap()
}
