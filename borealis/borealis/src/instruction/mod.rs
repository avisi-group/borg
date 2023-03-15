//! Sail instruction-level syntax (format) and semantic (determined through symbolic execution) extraction

use {
    crate::{
        genc::format::InstructionFormat,
        instruction::{execute::jib_func_to_genc, format::process_decode_function_clause},
    },
    common::intern::InternedString,
    sail::{
        jib_ast::Definition,
        sail_ast::{visitor::Visitor, Ast, FunctionClause, IdentifierAux},
    },
    std::collections::LinkedList,
};

pub mod execute;
pub mod format;

/// Finds all instructions in a Sail definition
pub fn get_instructions(ast: &Ast) -> Vec<FunctionClause> {
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
    jib: &LinkedList<Definition>,
    instruction: &FunctionClause,
) -> (InternedString, InstructionFormat, String) {
    // determine instruction format
    let (name, instruction_name, format) = process_decode_function_clause(instruction);

    // compile JIB to GenC for the execute definition
    let execute = jib_func_to_genc(instruction_name, jib);

    (name, format, execute)
}
