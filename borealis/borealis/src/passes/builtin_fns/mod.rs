//! Pass for implementing builtin (as in, provided by the Sail compiler)
//! functions in BOOM

use {
    crate::{
        boom::{Ast, FunctionDefinition, Statement},
        passes::Pass,
    },
    std::{cell::RefCell, rc::Rc},
};

pub mod functions;

pub struct AddBuiltinFns;

impl AddBuiltinFns {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for AddBuiltinFns {
    fn name(&self) -> &'static str {
        "AddBuiltinFns"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        // walk AST, inspecting each function call
        // if the function call references an already-defined function, ignore
        // otherwise, lookup function in functions and execute behaviour (either in
        // place modification or inserting new function definition)

        ast.borrow()
            .functions
            .values()
            .for_each(|def| process_function_definition(ast.clone(), def));

        false
    }
}

fn process_function_definition(ast: Rc<RefCell<Ast>>, fn_def: &FunctionDefinition) {
    fn_def.entry_block.iter().for_each(|block| {
        block.statements().into_iter().for_each(|statement| {
            // ignore statements that are not function calls
            let Statement::FunctionCall { name, .. } = *statement.borrow() else {
                return;
            };

            // ignore if the function definition is already in the AST,
            if ast.borrow().functions.contains_key(&name) {
                return;
            }

            // found builtin function function
            let handler = functions::HANDLERS.get(name.as_ref()).unwrap_or_else(|| {
                panic!(
                    "Function call {name:?} found without definition or builtin function behaviour"
                )
            });

            handler(ast.clone(), fn_def.clone(), statement);
        });
    });
}
