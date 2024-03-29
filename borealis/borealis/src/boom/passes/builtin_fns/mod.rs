//! Pass for implementing builtin (as in, provided by the Sail compiler)
//! functions in BOOM

use {
    crate::boom::{
        passes::{any::AnyExt, builtin_fns::handlers::HANDLERS, Pass},
        Ast, FunctionDefinition, Statement,
    },
    common::{intern::InternedString, HashMap},
    std::{cell::RefCell, rc::Rc},
};

mod handlers;

type HandlerFunction = fn(Rc<RefCell<Ast>>, FunctionDefinition, Rc<RefCell<Statement>>) -> bool;

/// Pass for implementing builtin (as in, provided by the Sail compiler)
/// functions in BOOM
pub struct AddBuiltinFns;

impl AddBuiltinFns {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for AddBuiltinFns {
    fn name(&self) -> &'static str {
        "AddBuiltinFns"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        // walk AST, inspecting each function call
        // if the function call references an already-defined function, ignore
        // otherwise, lookup function in functions and execute behaviour (either in
        // place modification or inserting new function definition)

        // perform full pass on all functions
        ast.borrow()
            .functions
            .values()
            .map(|def| process_function_definition(ast.clone(), def, &HANDLERS))
            .any()
    }
}

fn process_function_definition(
    ast: Rc<RefCell<Ast>>,
    fn_def: &FunctionDefinition,
    handlers: &HashMap<InternedString, HandlerFunction>,
) -> bool {
    fn_def
        .entry_block
        .iter()
        .map(|block| {
            block
                .statements()
                .into_iter()
                .map(|statement| {
                    // ignore statements that are not function calls
                    let Statement::FunctionCall { name, .. } = *statement.borrow() else {
                        return false;
                    };

                    // if the function has a handler, call it
                    if let Some(handler) = handlers.get(&name) {
                        handler(ast.clone(), fn_def.clone(), statement)
                    } else {
                        false
                    }
                })
                .any()
        })
        .any()
}

// /// Returns the key-value pairs in `map` where the keys exist in `set`
// fn difference<K: Eq + PartialEq + Hash + Copy, V: Clone>(
//     map: &HashMap<K, V>,
//     set: &HashSet<K>,
// ) -> HashMap<K, V> {
//     map.iter()
//         .filter(|(k, _)| set.contains(k))
//         .map(|(&k, v)| (k, v.clone()))
//         .collect()
// }
