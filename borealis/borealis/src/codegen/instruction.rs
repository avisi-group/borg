//! Sail instruction-level syntax (format) and semantic (determined through
//! symbolic execution) extraction

use {
    crate::{
        boom::{self, NamedType},
        codegen::format::process_decode_function_clause,
        genc::format::{InstructionFormat, SegmentContent},
    },
    common::intern::InternedString,
    itertools::Itertools,
    log::warn,
    sail::sail_ast::{self, visitor::Visitor, FunctionClause, IdentifierAux},
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

/// Finds all instructions in a Sail definition
pub fn get_instruction_entrypoint_fns(ast: &sail_ast::Ast) -> Vec<FunctionClause> {
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

/// Generates the execute entrypoint for an individual instruction
///
/// This "adapts" the GenC `instr` struct containing all the operands into a
/// call to the instructions `XXX_decode` function. Since the instruction format
/// may contain a mixture of constants and variables, the original Sail
/// bitvector range access semantics must be reconstructed.
pub fn generate_execute_entrypoint(
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

    #[allow(unstable_name_collisions)]
    let parameters = def
        .signature
        .parameters
        .iter()
        .map(|NamedType { name, .. }| name)
        .map(|name| {
            if format_variables.contains(name) {
                format!("inst.{name}")
            } else {
                let parts = format_variables
                    .iter()
                    .filter(|variable| variable.to_string().split("_part").next().unwrap().starts_with(&name.to_string()))
                    .collect::<Vec<_>>();

                match parts.len() {
                    0 => {
                        if constants.get(name).is_some() {
                            name.to_string()
                        } else {
                            panic!("parameter {name} did not correspond to any format segment or constant")
                        }
                    },
                    1 => format!("inst.{}",parts[0]),
                    _ => {
                        warn!("parameter {name} corresponded to multiple format segments");
                        format!("inst.{}",parts[0])
                    },
                }
            }
        }).intersperse(", ".to_owned()).collect::<String>();

    let execute = {
        let mut buf = String::new();

        for (name, value) in constants {
            buf.push_str(&format!("\tuint64 {name} = {value};\n"));
        }

        buf.push_str(&format!("\t{instruction_name}({parameters});"));

        buf
    };

    (mangled_name, format, execute)
}
