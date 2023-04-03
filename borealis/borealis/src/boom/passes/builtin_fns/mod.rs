//! Pass for implementing builtin (as in, provided by the Sail compiler) functions in BOOM

use {
    crate::boom::{passes::Pass, visitor::Visitor, Ast, Statement},
    regex::Regex,
};

pub mod functions;
pub mod generic_functions;

pub struct AddBuiltinFns;

impl Pass for AddBuiltinFns {
    fn run(&mut self, ast: &mut Ast) {
        // walk AST, inspecting each function call
        // if the function call references an already-defined function, ignore
        // otherwise, lookup function in functions and execute behaviour (either in place modification or inserting new function definition)
        let mut finder = FunctionFinder {
            ast,
            generic_fn_regex: Regex::new("([a-z_]+)<(.+)>").unwrap(),
        };

        ast.functions
            .values()
            .for_each(|def| finder.visit_function_definition(def));
    }
}

struct FunctionFinder<'ast> {
    ast: &'ast Ast,
    generic_fn_regex: Regex,
}

impl<'ast> Visitor for FunctionFinder<'ast> {
    fn visit_statement(&mut self, node: &Statement) {
        // ignore statements that are not function calls
        let Statement::FunctionCall { name, .. } = node else {
            return;
        };

        // ignore if the function definition is already in the AST,
        if self.ast.functions.contains_key(name) {
            return;
        }

        let name = name.to_string();

        match self.generic_fn_regex.captures(&name) {
            Some(captures) => {
                let name = captures.get(1).unwrap().as_str();
                let typ = captures.get(2).unwrap().as_str();

                // found generic function
                generic_functions::HANDLERS.get(name).unwrap_or_else(|| panic!(
                    "Generic function call \'{}<{}>\' found without definition or builtin function behaviour",
                    name, typ
                ))(self.ast, node, typ)
            }
            None => {
                // found non-generic function
                functions::HANDLERS.get(&name).unwrap_or_else(|| {
                    panic!(
                        "Function call {:?} found without definition or builtin function behaviour",
                        name
                    )
                })(self.ast, node)
            }
        }
    }
}
