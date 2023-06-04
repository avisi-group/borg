//! Sail instruction-level syntax (format) and semantic (determined through symbolic execution) extraction

use {
    crate::{
        boom,
        genc::format::InstructionFormat,
        instruction::{execute::boom_fn_to_genc, format::process_decode_function_clause},
    },
    common::intern::InternedString,
    sail::sail_ast::{self, visitor::Visitor, FunctionClause, IdentifierAux},
    std::{cell::RefCell, rc::Rc},
};

pub mod execute;
pub mod format;

/// Finds all instructions in a Sail definition
pub fn get_instructions(ast: &sail_ast::Ast) -> Vec<FunctionClause> {
    struct InstructionFinder {
        clauses: Vec<FunctionClause>,
    }

    impl Visitor for InstructionFinder {
        fn visit_function_clause(&mut self, node: &FunctionClause) {
            let IdentifierAux::Identifier(ident) = node.inner.identifier.inner else {
                return;
            };

            if ident.to_string() == "decode64" {
                self.clauses.push(node.clone());
            }
        }
    }

    let mut finder = InstructionFinder { clauses: vec![] };

    finder.visit_root(ast);

    finder.clauses
}

/// Compiles an individual instruction definition to GenC
pub fn process_instruction(
    ast: Rc<RefCell<boom::Ast>>,
    instruction: &FunctionClause,
) -> (InternedString, InstructionFormat, String) {
    // determine instruction format
    let (mangled_name, instruction_name, format) = process_decode_function_clause(instruction);

    // compile JIB to GenC for the execute definition
    let ast = ast.borrow();
    let def = ast
        .functions
        .get(&instruction_name)
        .unwrap_or_else(|| panic!("could not find function {:?}", instruction_name));
    let execute = boom_fn_to_genc(def);

    (mangled_name, format, execute)
}
