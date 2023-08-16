//! Pass for implementing builtin (as in, provided by the Sail compiler)
//! functions in BOOM

use {
    crate::{
        boom::{Ast, FunctionDefinition, Statement},
        passes::{builtin_fns::functions::HANDLERS, Pass},
    },
    common::{intern::InternedString, HashMap, HashSet},
    once_cell::sync::Lazy,
    std::{cell::RefCell, hash::Hash, rc::Rc},
};

pub mod functions;

type HandlerFunction = fn(Rc<RefCell<Ast>>, FunctionDefinition, Rc<RefCell<Statement>>);

/// Functions to handle in an initial pass, before processing others
static INITIAL_PASS_FNS: Lazy<HashSet<InternedString>> = Lazy::new(|| {
    HashSet::from_iter(
        ["UInt", "pcnt_i___pcnt_i64", "pcnt_i64___pcnt_i"]
            .into_iter()
            .map(InternedString::from_static),
    )
});

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

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        // walk AST, inspecting each function call
        // if the function call references an already-defined function, ignore
        // otherwise, lookup function in functions and execute behaviour (either in
        // place modification or inserting new function definition)

        // perform initial pass on limited set of functions
        ast.borrow().functions.values().for_each(|def| {
            process_function_definition(ast.clone(), def, &difference(&HANDLERS, &INITIAL_PASS_FNS))
        });

        // perform full pass on all functions
        ast.borrow()
            .functions
            .values()
            .for_each(|def| process_function_definition(ast.clone(), def, &HANDLERS));

        false
    }
}

fn process_function_definition(
    ast: Rc<RefCell<Ast>>,
    fn_def: &FunctionDefinition,
    handlers: &HashMap<InternedString, HandlerFunction>,
) {
    fn_def.entry_block.iter().for_each(|block| {
        block.statements().into_iter().for_each(|statement| {
            // ignore statements that are not function calls
            let Statement::FunctionCall { name, .. } = *statement.borrow() else {
                return;
            };

            // if the function has a handler, call it
            if let Some(handler) = handlers.get(&name) {
                handler(ast.clone(), fn_def.clone(), statement);
            }
        });
    });
}

/// Returns the key-value pairs in `map` where the keys exist in `set`
fn difference<K: Eq + PartialEq + Hash + Copy, V: Clone>(
    map: &HashMap<K, V>,
    set: &HashSet<K>,
) -> HashMap<K, V> {
    map.iter()
        .filter(|(k, _)| set.contains(k))
        .map(|(&k, v)| (k, v.clone()))
        .collect()
}
