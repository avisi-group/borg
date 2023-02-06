//! Sail instruction-level syntax (format) and semantic (determined through symbolic execution) extraction

use {
    crate::{
        genc::format::InstructionFormat,
        instruction::{format::process_decode_function_clause, symbolic::SymbolicExecutor},
    },
    common::intern::InternedStringKey,
    sail::{
        ast::{Ast, FunctionClause, IdentifierAux},
        visitor::Visitor,
    },
};

pub mod format;
pub mod symbolic;

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

/// Compiles an individual instruction definition into an intermediate format
pub fn process_instruction(
    ast: &Ast,
    instruction: &FunctionClause,
) -> (InternedStringKey, InstructionFormat, String) {
    let (name, format) = process_decode_function_clause(instruction);

    let execute = SymbolicExecutor::new(ast).run(instruction);

    (name, format, execute)
}
