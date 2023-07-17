//! Pass for implementing builtin (as in, provided by the Sail compiler)
//! functions in BOOM

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock,
            visitor::{Visitor, Walkable},
            Ast, Statement,
        },
        passes::Pass,
    },
    regex::Regex,
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

pub mod functions;
pub mod generic_functions;

pub struct AddBuiltinFns {
    ast: Rc<RefCell<Ast>>,
    generic_fn_regex: Regex,
    visited_blocks: HashSet<ControlFlowBlock>,
}

impl AddBuiltinFns {
    pub fn new_boxed(ast: Rc<RefCell<Ast>>) -> Box<dyn Pass> {
        Box::new(Self {
            ast,
            generic_fn_regex: Regex::new("([a-z_]+)<(.+)>").expect("failed to build regex"),
            visited_blocks: HashSet::new(),
        })
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
            .for_each(|def| self.visit_function_definition(def));

        false
    }
}

impl Visitor for AddBuiltinFns {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        // ignore statements that are not function calls
        let Statement::FunctionCall { name, .. } = *node.borrow() else {
            return;
        };

        // ignore if the function definition is already in the AST,
        if self.ast.borrow().functions.contains_key(&name) {
            return;
        }

        match self.generic_fn_regex.captures(name.as_ref()) {
            Some(captures) => {
                let name = captures.get(1).unwrap().as_str();
                let typ = captures.get(2).unwrap().as_str();

                // found generic function
                generic_functions::HANDLERS.get(name).unwrap_or_else(||
                    panic!(
                        "Generic function call \'{name}<{typ}>\' found without definition or builtin function behaviour"
                    )
                )(&self.ast.borrow(), &node.borrow(), typ);
            }
            None => {
                // found non-generic function
                functions::HANDLERS.get(name.as_ref()).unwrap_or_else(|| {
                    panic!(
                        "Function call {name:?} found without definition or builtin function behaviour"
                    )
                })(&self.ast.borrow(), &node.borrow());
            }
        }

        node.borrow().walk(self);
    }

    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool {
        self.visited_blocks.contains(block)
    }

    fn set_block_visited(&mut self, block: &ControlFlowBlock) {
        self.visited_blocks.insert(block.clone());
    }
}
