//! Sail instruction-level syntax (format) and semantic (determined through
//! symbolic execution) extraction

use {
    crate::{
        boom::{self, NamedType},
        codegen::format::{process_decode_function_clause, InstructionDecodeInformation},
        genc_model::format::{InstructionFormat, SegmentContent},
    },
    common::{intern::InternedString, HashMap, HashSet},
    itertools::Itertools,
    sail::sail_ast::{self, visitor::Visitor, FunctionClause, IdentifierAux},
    std::{cell::RefCell, ops::Range, rc::Rc},
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

            if ident.as_ref() == "decode64" {
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
    let InstructionDecodeInformation {
        mangled_name,
        execute_function_name,
        format,
        constants,
        split_variable_ranges,
    } = process_decode_function_clause(instruction);

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

        // get the instruction's entrypoint function
        if let Some(func) = ast.functions.get(&execute_function_name) {
            func.signature
                // iterate over the parameters of this function
                .parameters
                .borrow()
                .iter()
                .map(|NamedType { name, .. }| name)
                .map(|name| {
                    if variables.contains(name) {
                        // if the parameter name is a format variable, access it from the `inst`
                        // struct.
                        format!("inst.{name}")
                    } else {
                        // if the parameter name is a format constant, or exists as a combination of
                        // multiple format constants or variables, access the generated local
                        // variable.
                        name.to_string()
                    }
                })
                .join(", ")
        } else {
            log::debug!("could not find function {:?}", execute_function_name);
            "".to_owned()
        }
    };

    // the body of the execute function contains local variable assignments for
    // constants and combination variables
    let body = {
        let mut buf = String::new();

        for (name, value) in constants {
            buf.push_str(&format!("\tuint64 {name} = {value};\n"));
        }

        // insert bitshifts to calculate spans across format segments
        buf.push_str(&reconstruct_split_vars(&format, &split_variable_ranges));

        buf.push_str(&format!("\t{execute_function_name}({arguments});"));

        buf
    };

    let disasm = {
        let mut buf = String::new();

        buf.push('\"');
        buf.push_str(execute_function_name.as_ref());
        buf.push(' ');

        for segment in &format.0 {
            match segment.content {
                SegmentContent::Variable(name) => buf.push_str(&format!("{name}: %hex64, ")),
                SegmentContent::Constant(val) => buf.push_str(&format!("{val}, ")),
            }
        }

        buf.push('\"');

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

    (execute_function_name, mangled_name, format, body, disasm)
}

/// Segment of a split variable
#[derive(Debug, PartialEq)]
enum VariableSegment {
    Constant {
        /// Value of the constant
        value: u64,
        /// Length of the component in bits
        len: usize,
    },
    Variable {
        /// Format variable name (should end in "_partX")
        name: InternedString,
        /// Range
        range: Range<usize>,
    },
}

impl VariableSegment {
    fn from_segment_range(segment: &SegmentContent, range: Range<usize>) -> Self {
        match *segment {
            SegmentContent::Variable(name) => VariableSegment::Variable { name, range },
            SegmentContent::Constant(value) => {
                let num_bits = range.end - range.start;
                let mask = (1 << num_bits) - 1;

                VariableSegment::Constant {
                    value: (value >> range.start) & mask,
                    len: num_bits,
                }
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            VariableSegment::Constant { len, .. } => *len,
            VariableSegment::Variable { range, .. } => range.len(),
        }
    }
}

fn reconstruct_split_vars(
    format: &InstructionFormat,
    split_variables: &HashMap<InternedString, Range<usize>>,
) -> String {
    if split_variables.is_empty() {
        return "".to_owned();
    }

    let mut buf = String::new();

    for (name, range) in split_variables {
        // flip the range because it starts mirrored
        let range = (32 - range.end)..(32 - range.start);

        // calculate a vec of either a constant or ranges of format variable from which
        // bitshifts and masks are generated
        let segments = get_segments_in_range(format, range.clone());

        let value = segments
            .into_iter()
            .scan(0, |state, seg| {
                let initial = *state;
                *state += seg.len();
                Some((initial, seg))
            })
            // for each segment: mask + shift variable, mask constant
            .map(|(start_pos, segment)| {
                (
                    start_pos,
                    match segment {
                        VariableSegment::Constant { value, len } => {
                            format!("({value} & ((1 << {len}) - 1))")
                        }
                        VariableSegment::Variable { name, range } => {
                            let start = range.start;
                            let len = range.len();
                            format!("((inst.{name} >> {start}) & ((1 << {len}) - 1))")
                        }
                    },
                )
            })
            // shift into position
            .map(|(start_pos, segment_str)| format!("({segment_str} << {start_pos})"))
            // and all together
            .join(" & ");

        buf.push_str(&format!("\tuint64 {name} = {value};\n"));
    }

    buf
}

fn get_segments_in_range(format: &InstructionFormat, range: Range<usize>) -> Vec<VariableSegment> {
    format
        .0
        .iter()
        .scan(0, |state, seg| {
            let initial = *state;
            *state += seg.length;
            Some((seg.content.clone(), initial..*state))
        })
        .filter_map(|(segment, segment_range)| {
            // if the segment exists before the range start or after the range end, remove
            // it
            if range.start >= segment_range.end || range.end <= segment_range.start {
                return None;
            }

            // start with the whole segment range
            let mut final_range = 0..segment_range.len();

            // if the start of the range partially overlaps a segment (first segment)
            if segment_range.contains(&range.start) {
                final_range.start = range.start - segment_range.start;
            }

            // if the end of the range partially overlaps a segment (last segment)
            if segment_range.contains(&range.end) {
                final_range.end = range.end - segment_range.start;
            }

            // otherwise, return the whole segment (middle segment)
            Some(VariableSegment::from_segment_range(&segment, final_range))
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            codegen::instruction::{get_segments_in_range, VariableSegment::*},
            genc_model::format::{InstructionFormat, Segment, SegmentContent},
        },
        once_cell::sync::Lazy,
        pretty_assertions::assert_eq,
    };

    const FORMAT: Lazy<InstructionFormat> = Lazy::new(|| {
        InstructionFormat(vec![
            Segment {
                content: SegmentContent::Constant(1),
                length: 1,
            },
            Segment {
                content: SegmentContent::Variable("size_part0".into()),
                length: 1,
            },
            Segment {
                content: SegmentContent::Constant(7),
                length: 3,
            },
            Segment {
                content: SegmentContent::Constant(0),
                length: 1,
            },
            Segment {
                content: SegmentContent::Constant(0),
                length: 2,
            },
            Segment {
                content: SegmentContent::Variable("A".into()),
                length: 1,
            },
            Segment {
                content: SegmentContent::Variable("R".into()),
                length: 1,
            },
            Segment {
                content: SegmentContent::Constant(1),
                length: 1,
            },
            Segment {
                content: SegmentContent::Variable("Rs".into()),
                length: 5,
            },
            Segment {
                content: SegmentContent::Constant(0),
                length: 1,
            },
            Segment {
                content: SegmentContent::Constant(2),
                length: 3,
            },
            Segment {
                content: SegmentContent::Constant(0),
                length: 2,
            },
            Segment {
                content: SegmentContent::Variable("Rn".into()),
                length: 5,
            },
            Segment {
                content: SegmentContent::Variable("Rt".into()),
                length: 5,
            },
        ])
    });

    #[test]
    fn single_constant_segment() {
        assert_eq!(
            get_segments_in_range(&FORMAT, 0..1),
            vec![Constant { value: 1, len: 1 }]
        );
    }

    #[test]
    fn single_variable_segment() {
        assert_eq!(
            get_segments_in_range(&FORMAT, 1..2),
            vec![Variable {
                name: "size_part0".into(),
                range: 0..1
            }]
        );
    }

    #[test]
    fn multi_segment() {
        assert_eq!(
            get_segments_in_range(&FORMAT, 0..2),
            vec![
                Constant { value: 1, len: 1 },
                Variable {
                    name: "size_part0".into(),
                    range: 0..1
                }
            ]
        );
    }

    #[test]
    fn multi_segment_partial_1() {
        assert_eq!(
            get_segments_in_range(&FORMAT, 3..13),
            vec![
                Constant { value: 3, len: 2 },
                Constant { value: 0, len: 1 },
                Constant { value: 0, len: 2 },
                Variable {
                    name: "A".into(),
                    range: 0..1,
                },
                Variable {
                    name: "R".into(),
                    range: 0..1,
                },
                Constant { value: 1, len: 1 },
                Variable {
                    name: "Rs".into(),
                    range: 0..2
                }
            ]
        );
    }

    #[test]
    fn multi_segment_partial_2() {
        assert_eq!(
            get_segments_in_range(&FORMAT, 25..30),
            vec![
                Variable {
                    name: "Rn".into(),
                    range: 3..5,
                },
                Variable {
                    name: "Rt".into(),
                    range: 0..3,
                },
            ]
        );
    }
}
