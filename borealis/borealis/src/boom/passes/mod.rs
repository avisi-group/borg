//! Infrastructure for executing passes over BOOM.
//!
//! Includes:
//! * Logic for "raising" unstructured BOOM control flow back into structure if-else, match, and for loops
//! * Builtin function handling

use {
    crate::boom::{
        passes::{
            builtin_fns::AddBuiltinFns, fold_unconditionals::FoldUnconditionals,
            if_raiser::IfRaiser,
        },
        Ast,
    },
    log::info,
    std::{
        cell::RefCell,
        fs::{create_dir_all, File},
        path::Path,
        rc::Rc,
    },
};

mod builtin_fns;
mod fold_unconditionals;
mod if_raiser;

pub fn execute_passes(ast: Rc<RefCell<Ast>>) {
    dump_func_dot(
        &ast,
        "integer_arithmetic_addsub_immediate_decode",
        "target/dot/addsub_initial.dot",
    );

    let initial_statements = ast.borrow().statements();

    [
        FoldUnconditionals::new_boxed(),
        IfRaiser::new_boxed(),
        AddBuiltinFns::new_boxed(ast.clone()),
    ]
    .into_iter()
    .for_each(|mut pass| {
        info!("{}", pass.name());

        pass.run(ast.clone());

        dump_func_dot(
            &ast,
            "integer_arithmetic_addsub_immediate_decode",
            format!("target/dot/addsub_{}.dot", pass.name()),
        );
    });

    // currently, passes should not modify any statements so we smoke test that they are all still there
    assert_eq!(initial_statements, ast.borrow().statements());
}

pub trait Pass {
    fn name(&self) -> &'static str;

    /// Run the pass on the supplied AST
    fn run(&mut self, ast: Rc<RefCell<Ast>>);
}

fn dump_func_dot<P: AsRef<Path>>(ast: &Rc<RefCell<Ast>>, func: &'static str, path: P) {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent).unwrap();
    }

    ast.borrow()
        .functions
        .get(&func.into())
        .unwrap()
        .control_flow
        .as_dot(&mut File::create(path).unwrap())
        .unwrap()
}
