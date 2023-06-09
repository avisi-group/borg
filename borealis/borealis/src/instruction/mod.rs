//! Sail instruction-level syntax (format) and semantic (determined through symbolic execution) extraction

use {
    crate::{
        boom::{self, NamedType},
        genc::format::{InstructionFormat, SegmentContent},
        instruction::format::process_decode_function_clause,
    },
    common::intern::InternedString,
    log::warn,
    sail::sail_ast::{self, visitor::Visitor, FunctionClause, IdentifierAux},
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

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
    let (mangled_name, instruction_name, format, constants) =
        process_decode_function_clause(instruction);

    let format_variables = format
        .0
        .iter()
        .filter_map(|segment| match segment.content {
            SegmentContent::Variable(s) => Some(s),
            SegmentContent::Constant(_) => None,
        })
        .collect::<HashSet<_>>();

    // compile JIB to GenC for the execute definition
    let ast = ast.borrow();
    let def = ast
        .functions
        .get(&instruction_name)
        .unwrap_or_else(|| panic!("could not find function {:?}", instruction_name));

    let mut parameters_iter = def
        .signature
        .parameters
        .iter()
        .map(|NamedType { name, .. }| name)
        .map(|name| {
            if format_variables.contains(name) {
                name
            } else {
                let parts = format_variables
                    .iter()
                    .filter(|variable| variable.to_string().split("_part").next().unwrap().starts_with(&name.to_string()))
                    .collect::<Vec<_>>();

                match parts.len() {
                    0 => {
                        if constants.get(name).is_some() {
                            name
                        } else {
                            panic!("parameter {name} did not correspond to any format segment or constant")
                        }
                    },
                    1 => parts[0],
                    _ => {
                        warn!("parameter {name} corresponded to multiple format segments");
                        parts[0]
                    },
                }
            }
        });

    let parameters_string = {
        let mut s = String::new();

        if let Some(name) = parameters_iter.next() {
            s += &format!("inst.{name}");
        }
        for name in parameters_iter {
            s += ", ";
            s += &format!("inst.{name}");
        }

        s
    };

    let execute = if mangled_name == "integer_arithmetic_addsub_immediate_decode0".into() {
        format!("\t{mangled_name}({parameters_string});")
    } else {
        "".to_owned()
    };

    (mangled_name, format, execute)
}
