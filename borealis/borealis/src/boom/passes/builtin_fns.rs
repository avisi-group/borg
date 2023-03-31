//! Pass for implementing builtin (as in, provided by the Sail compiler) functions in BOOM

use {
    crate::boom::{passes::Pass, visitor::Visitor, Ast, Statement},
    phf::{phf_map, Map},
};

fn _slice(_ast: &Ast, _node: &Statement) {
    todo!();
}

static _FUNCTIONS: Map<&str, fn(&Ast, &Statement)> = phf_map! {
    "slice" => _slice,
};

pub struct AddBuiltinFns;

impl Pass for AddBuiltinFns {
    fn run(&mut self, ast: &mut Ast) {
        // walk AST, inspecting each function call
        // if the function call references an already-defined function, ignore
        // otherwise, lookup function in functions and execute behaviour (either in place modification or inserting new function definition)
        let mut finder = FunctionFinder { ast };

        ast.functions
            .values()
            .for_each(|def| finder.visit_function_definition(def));
    }
}

struct FunctionFinder<'ast> {
    ast: &'ast Ast,
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

        // FUNCTIONS.get(&name.to_string()).expect(&format!(
        //     "Function call {:?} found without definition or builtin function behaviour",
        //     name
        // ))(self.ast, node);
    }
}
