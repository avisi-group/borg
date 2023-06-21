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
) -> (
    InternedString,
    InternedString,
    InstructionFormat,
    String,
    String,
) {
    // determine instruction format
    let (mangled_name, instruction_name, format, constants) =
        process_decode_function_clause(instruction);

    // generates the *arguments* to the instruction's execute function from the
    // function signatures *parameters*.
    #[allow(unstable_name_collisions)]
    let arguments = {
        // collect names of all format variables
        let variables = format
            .0
            .iter()
            .filter_map(|segment| match segment.content {
                SegmentContent::Variable(s) => Some(s),
                SegmentContent::Constant(_) => None,
            })
            .collect::<HashSet<_>>();

        let ast = ast.borrow();
        ast.functions
            // get the instruction's entrypoint function
            .get(&instruction_name)
            .unwrap_or_else(|| panic!("could not find function {:?}", instruction_name))
            .signature
            // iterate over the parameters of this function
            .parameters
            .iter()
            .map(|NamedType { name, .. }| name)
            .map(|name| {
                if variables.contains(name) {
                    // if the parameter name is a format variable, access it from the `inst` struct.
                    format!("inst.{name}")
                } else {
                    // if the parameter name is a format constant, or exists as a combination of
                    // multiple format constants or variables, access the generated local variable.
                    name.to_string()
                }
            })
            .intersperse(", ".to_owned())
            .collect::<String>()
    };

    // the body of the execute function contains local variable assignments for
    // constants and combination variables
    let body = {
        let mut buf = String::new();

        for (name, value) in constants {
            buf.push_str(&format!("\tuint64 {name} = {value};\n"));
        }

        // insert bitshifts to calculate spans across format segments
        // nevermind, apparently we do not need this

        buf.push_str(&format!("\t{instruction_name}({arguments});"));

        buf
    };

    let disasm = {
        let mut buf = String::new();

        buf.push_str("\"");
        buf.push_str(instruction_name.as_ref());
        buf.push_str(" ");

        for segment in &format.0 {
            match segment.content {
                SegmentContent::Variable(name) => buf.push_str(&format!("{name}: %hex64, ")),
                SegmentContent::Constant(val) => buf.push_str(&format!("{val}, ")),
            }
        }

        buf.push_str("\"");

        let vars = format
            .0
            .iter()
            .filter_map(|segment| {
                if let SegmentContent::Variable(name) = segment.content {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .join(", ");

        if !vars.is_empty() {
            buf.push_str(", ");
        }

        buf.push_str(&vars);

        buf
    };

    (instruction_name, mangled_name, format, body, disasm)
}
