//! Rust module generation

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Ast, Bit, Statement},
        genc::{
            codegen::{
                format::{extract_format, flatten_expression, is_see_assignment},
                instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
            },
            format::InstructionFormat,
        },
        passes::{
            self, builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            fold_unconditionals::FoldUnconditionals, remove_const_branch::RemoveConstBranch,
            remove_exception::RemoveExceptions, remove_redundant_assigns::RemoveRedundantAssigns,
            resolve_bitvectors::ResolveBitvectors, resolve_return_assigns::ResolveReturns,
        },
        rust::emit::Emit,
    },
    color_eyre::eyre::Context,
    common::{intern::InternedString, HashMap},
    dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
    log::info,
    proc_macro2::{Ident, Span, TokenStream},
    quote::quote,
    sail::{
        jib_ast::Definition,
        sail_ast::{self, Expression, ExpressionAux, PatternMatchAux},
    },
    std::{
        cell::RefCell,
        cmp::min,
        collections::LinkedList,
        io::{Cursor, Write},
        rc::Rc,
    },
};

mod emit;

/// Compiles a Sail model to a Brig module
pub fn sail_to_brig<W: Write>(
    writer: &mut W,
    sail_ast: &sail_ast::Ast,
    jib_ast: &LinkedList<Definition>,
) -> color_eyre::Result<()> {
    info!("Converting JIB to BOOM");
    let ast = Ast::from_jib(jib_ast);

    // only run on a subset of the model, for now
    apply_function_denylist(ast.clone());

    info!("Running passes on BOOM");
    passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            CycleFinder::new_boxed(),
            ResolveReturns::new_boxed(),
            RemoveExceptions::new_boxed(),
            ResolveBitvectors::new_boxed(),
            AddBuiltinFns::new_boxed(),
            RemoveRedundantAssigns::new_boxed(),
        ],
    );

    info!("Generating Rust");

    // let reg_fields = TokenStream::from_iter(ast.borrow().registers.iter().map(|(name, typ)| {
    //     let typ_str = Ident::new(&typ.emit_string(), Span::call_site());
    //     quote! {
    //         #name: #typ_str,
    //     }
    // }));
    let reg_fields = quote! {
        pc: u64,
        sp: u64,
        x: [u64; 31],
    };

    let boilerplate = quote! {
         //! BOREALIS GENERATED FILE DO NOT MODIFY

         use super::{CoreState, ExecutionEngine};

         pub struct AArch64Interpreter;

         pub struct AArch64CoreState {
            #reg_fields
         }

         impl CoreState for AArch64CoreState {
             fn pc(&self) -> usize {
                 self.pc
             }

             fn new(pc: usize) -> Self {
                 Self { pc }
             }
         }

         impl ExecutionEngine<AArch64CoreState> for AArch64Interpreter {
             fn step(amount: super::StepAmount, state: &mut AArch64CoreState) -> super::StepResult {
                 let insn_data = fetch(state.pc());
                 log::trace!("fetch @ {} = {:08x}", state.pc(), insn_data);
                 decode(insn_data);
                 todo!("decode -- execute")
             }
         }

         fn fetch(pc: usize) -> u32 {
             unsafe { *(pc as *const u32) }
         }
    };

    let syntax_tree = syn::parse_file(&boilerplate.to_string())
        .wrap_err(format!("failed to parse {:?}", boilerplate.to_string()))?;
    let formatted = prettyplease::unparse(&syntax_tree);
    writeln!(writer, "{formatted}")?;

    generate_decode_fn(writer, ast.clone(), sail_ast);

    for f in generate_fns(
        ast.clone(),
        vec!["integer_arithmetic_addsub_immediate_decode".into()],
    )? {
        writeln!(writer, "{f}\n")?;
    }

    Ok(())
}

fn apply_function_denylist(ast: Rc<RefCell<Ast>>) {
    // filter out non addsub functions
    let funcs = ast
        .borrow()
        .functions
        .clone()
        .into_iter()
        .map(|(k, mut def)| {
            // if it's not an allowlisted function, delete the body
            if ![
                "integer_arithmetic_addsub_immediate_decode",
                "integer_arithmetic_addsub_immediate",
                "u__id",
                "AddWithCarry",
                "IsZero",
                "u__GetSlice_int",
                "integer_logical_shiftedreg_decode",
                "DecodeShift",
                "integer_logical_shiftedreg",
                "ShiftReg",
                "branch_conditional_cond_decode",
                "branch_conditional_cond",
                "integer_insext_insert_movewide_decode",
                "integer_insext_insert_movewide",
                "integer_arithmetic_addsub_shiftedreg_decode",
                "DecodeShift",
                "integer_arithmetic_addsub_shiftedreg",
                "IsZeroBit",
                "ConditionHolds",
                "BranchTo",
                "branch_unconditional_immediate_decode",
                "branch_unconditional_immediate",
                "memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode",
                "memory_single_general_immediate_signed_postidx",
                "ConstrainUnpredictable",
                "system_hints_decode",
                "integer_arithmetic_address_pcrel_decode",
                "integer_arithmetic_address_pcrel",
                "memory_pair_general_preidx_memory_pair_general_postidx__decode",
                "memory_pair_general_postidx",
                "memory_pair_general_offset_memory_pair_general_postidx__decode",
                "integer_arithmetic_addsub_extendedreg_decode",
                "DecodeRegExtend",
                "integer_arithmetic_addsub_extendedreg",
                "ExtendReg",
                "memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode",
                "branch_conditional_compare_decode",
                "branch_conditional_compare",
                "memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode",
                "integer_conditional_select_decode",
                "integer_conditional_select",
                "integer_logical_immediate_decode",
                "DecodeBitMasks",
                "HighestSetBit",
                "integer_logical_immediate",
                "memory_pair_general_postidx_memory_pair_general_postidx__decode",
                "branch_unconditional_register_decode",
                "branch_unconditional_register",
                "system_exceptions_runtime_svc_decode",
                "system_exceptions_runtime_svc",
                "integer_conditional_compare_immediate_decode",
                "integer_conditional_compare_immediate",
                "memory_single_general_register_memory_single_general_register__decode",
                "memory_single_general_register",
                "integer_conditional_compare_register_decode",
                "integer_conditional_compare_register",
                "memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode",
                "memory_single_general_immediate_signed_offset_normal",
                "integer_bitfield_decode",
                "integer_bitfield",
                "branch_conditional_test_decode",
                "branch_conditional_test",
                "system_register_system_decode",
                "AArch64_CheckSystemAccess",
                "system_register_system",
                "u__IMPDEF_boolean",
                "u__IMPDEF_boolean_map",
                "vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode",
                "memory_literal_general_decode",
                "memory_literal_general"

                // // CHECKPOINT

                //"AArch64_SysRegRead",
                // "AArch64_SysRegWrite",
                // "HaveBTIExt",
                // "HasArchVersion",
                // "BranchTargetCheck",
                // "AArch64_ExecutingBROrBLROrRetInstr",
                // "AArch64_ExecutingBTIInstr",
                // "ThisInstr",
                // "HaveNVExt"
            ]
            .contains(&k.as_ref())
            {
                def.entry_block = ControlFlowBlock::new();
                def.entry_block.set_statements(vec![Rc::new(RefCell::new(
                    Statement::FunctionCall {
                        expression: None,
                        name: "trap".into(),
                        arguments: vec![],
                    },
                ))])
            }

            (k, def)
        })
        .collect();
    ast.borrow_mut().functions = funcs;
}

/// Generates Rust functions from all functions in a BOOM AST
pub fn generate_fns(
    ast: Rc<RefCell<Ast>>,
    initial_fns: Vec<InternedString>,
) -> color_eyre::Result<Vec<String>> {
    let mut remaining_fns = initial_fns;
    let mut generated_fns = HashMap::default();

    while let Some(ident) = remaining_fns.pop() {
        // skip if already generated
        if generated_fns.contains_key(&ident) {
            continue;
        }

        let ast = ast.borrow();
        let Some(definition) = ast.functions.get(&ident) else {
            log::trace!("cannot generate GenC for unknown function {ident:?}");
            continue;
        };
        log::trace!("generating {ident}");

        let generated = quote! {
            fn #ident() {
                todo!();
            }
        }
        .to_string();

        // count format the whole file at the end but it might be slow?
        let syntax_tree =
            syn::parse_file(&generated).wrap_err(format!("failed to parse {generated:?}"))?;
        let formatted = prettyplease::unparse(&syntax_tree);

        generated_fns.insert(ident, formatted);

        // find all functions called by the current one, and put them in the remaining list (duplicates caught by check if ident is in `generated_fns`)
        remaining_fns.extend(definition.entry_block.get_functions());
    }

    Ok(generated_fns.into_values().collect())
}

fn generate_decode_fn<W: Write>(writer: &mut W, boom: Rc<RefCell<Ast>>, sail: &sail_ast::Ast) {
    let mut tree = DecodeTree::new();

    // set up entrypoints in GenC execute behaviours
    let insn_formats = get_instruction_entrypoint_fns(sail)
        .into_iter()
        .map(|clause| {
            log::trace!(
                "Processing decode function clause @ {}",
                clause.annotation.location
            );

            let (pat, body) = match &clause.inner.pattern_match.inner {
                PatternMatchAux::Expression(pat, body) => (pat, body),
                PatternMatchAux::When(pat, _, body) => {
                    log::debug!("Function clause has condition, ignoring...");
                    (pat, body)
                }
            };

            let ExpressionAux::Block(expressions) = &*body.inner else {
                panic!("Body was not Block");
            };

            assert_eq!(expressions.len(), 2);

            assert!(is_see_assignment(expressions.front().unwrap()));

            let expressions = flatten_expression(expressions.back().unwrap());

            let execute_function_name = {
                let Some(Expression { inner, .. }) = expressions.last() else {
                    panic!()
                };

                let ExpressionAux::Application(ident, ..) = &**inner else {
                    panic!()
                };

                ident.as_interned()
            };

            (execute_function_name, extract_format(&pat.inner))
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    for (name, bits) in insn_formats {
        count += 1;
        tree.insert(name, bits);

        if count == 20 {
            println!("{}", tree.root.unwrap().to_dot());
            panic!();
        }
    }

    todo!();
    // quote! {
    //     fn decode(insn_data: u32) {

    //     }
    // };
}

#[derive(Debug)]
struct DecodeTree {
    root: Option<DecodeNode>,
}

impl DecodeTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn dump(&self) {
        let Some(ref root) = self.root else {
            return;
        };

        root.dump(vec![]);
    }

    fn insert(&mut self, name: InternedString, bits: Vec<Bit>) {
        //println!("{name: >64} {bits:?}");

        match &mut self.root {
            None => {
                self.root = Some(DecodeNode {
                    name,
                    bits,
                    zero: None,
                    one: None,
                    x: None,
                })
            }
            Some(ref mut root) => root.insert(name, bits),
        }
    }
}

#[derive(Debug, Clone)]
struct DecodeNode {
    pub name: InternedString,
    pub bits: Vec<Bit>,
    pub zero: Option<Box<DecodeNode>>,
    pub one: Option<Box<DecodeNode>>,
    pub x: Option<Box<DecodeNode>>,
}

impl DecodeNode {
    pub fn insert_child(&mut self, name: InternedString, bits: Vec<Bit>) {
        match bits[0] {
            Bit::Zero => match &mut self.zero {
                None => {
                    self.zero = Some(Box::new(DecodeNode {
                        name,
                        bits: bits,
                        zero: None,
                        one: None,
                        x: None,
                    }))
                }
                Some(ref mut child) => child.insert(name, bits),
            },
            Bit::One => match &mut self.one {
                None => {
                    self.one = Some(Box::new(DecodeNode {
                        name,
                        bits: bits,
                        zero: None,
                        one: None,
                        x: None,
                    }))
                }
                Some(ref mut child) => child.insert(name, bits),
            },
            Bit::Unknown => match &mut self.x {
                None => {
                    self.x = Some(Box::new(DecodeNode {
                        name,
                        bits: bits,
                        zero: None,
                        one: None,
                        x: None,
                    }))
                }
                Some(ref mut child) => child.insert(name, bits),
            },
        }
    }

    pub fn insert(&mut self, name: InternedString, bits: Vec<Bit>) {
        println!("inserting {name:?} = {bits:?}");

        for i in 0..min(self.bits.len(), bits.len()) {
            if self.bits[i] != bits[i] {
                let (shared_bits, old_bits) = self.bits.split_at(i);
                let old_bits = old_bits.to_owned();

                let (_, new_bits) = bits.split_at(i);
                let new_bits = new_bits.to_owned();

                println!("matched chunk differs @ {i}, shared_bits: {shared_bits:?}, old_bits: {old_bits:?}, new_bits: {new_bits:?}");

                self.bits = shared_bits.to_owned();
                let old_name = self.name;
                self.name = format!("partial{}", common::identifiable::unique_id()).into();

                // two new children
                // first child has current name, old_bits, and current children

                let child_a = DecodeNode {
                    name: old_name,
                    bits: old_bits.clone(),
                    zero: self.zero.clone(),
                    one: self.one.clone(),
                    x: self.x.clone(),
                };

                match old_bits[0] {
                    Bit::Zero => {
                        self.zero = Some(Box::new(child_a));
                        self.one = None;
                        self.x = None;
                    }
                    Bit::One => {
                        self.zero = None;
                        self.one = Some(Box::new(child_a));
                        self.x = None;
                    }
                    Bit::Unknown => {
                        self.zero = None;
                        self.one = None;
                        self.x = Some(Box::new(child_a));
                    }
                }

                // insert new child with new name and pattern
                self.insert_child(name, new_bits);

                return;
            }
        }

        println!("found no difference, inserting in child");

        self.insert_child(name, bits.split_at(self.bits.len()).1.to_owned());

        //   panic!("attempted to insert decode with equal pattern as existing???")
    }

    pub fn dump(&self, mut prefix: Vec<Bit>) {
        prefix.extend_from_slice(&self.bits);

        if prefix.len() == 32 {
            println!("{}: {prefix:?}", self.name);
        } else {
            if let Some(child) = &self.zero {
                child.dump(prefix.clone());
            }
            if let Some(child) = &self.one {
                child.dump(prefix.clone());
            }
            if let Some(child) = &self.x {
                child.dump(prefix.clone());
            }
        }
    }

    pub fn to_dot(&self) -> String {
        type NodeId = InternedString;
        type EdgeId = (NodeId, NodeId);

        #[derive(Default)]
        struct Graph {
            nodes: Vec<NodeId>,
            edges: Vec<EdgeId>,
            node_labels: HashMap<NodeId, Vec<Bit>>,
        }

        impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
            fn graph_id(&'ast self) -> dot::Id<'ast> {
                dot::Id::new("Decoder").unwrap()
            }

            fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
                dot::Id::new((*n).clone().to_string()).unwrap()
            }

            fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
                let label = self.node_labels.get(n).cloned().unwrap();

                LabelText::EscStr(format!("{n} | {label:?}").into())
            }

            fn node_shape(&'ast self, _: &NodeId) -> Option<LabelText<'ast>> {
                Some(LabelText::LabelStr("record".into()))
            }

            fn edge_label(&'ast self, e: &EdgeId) -> LabelText<'ast> {
                LabelText::LabelStr("".into())
            }
        }

        impl<'ast> GraphWalk<'ast, NodeId, EdgeId> for Graph {
            fn nodes(&'ast self) -> Nodes<'ast, NodeId> {
                (&self.nodes).into()
            }

            fn edges(&'ast self) -> Edges<'ast, EdgeId> {
                (&self.edges).into()
            }

            fn source(&'ast self, edge: &EdgeId) -> NodeId {
                edge.0.clone()
            }

            fn target(&'ast self, edge: &EdgeId) -> NodeId {
                edge.1.clone()
            }
        }

        let mut graph = Graph::default();

        fn recurse(graph: &mut Graph, node: &DecodeNode) {
            graph.nodes.push(node.name);
            graph.node_labels.insert(node.name, node.bits.clone());

            if let Some(child) = &node.zero {
                graph.edges.push((node.name, child.name));
                recurse(graph, child);
            }
            if let Some(child) = &node.one {
                graph.edges.push((node.name, child.name));
                recurse(graph, child);
            }
            if let Some(child) = &node.x {
                graph.edges.push((node.name, child.name));
                recurse(graph, child);
            }
        }

        recurse(&mut graph, self);

        let mut out = vec![];
        dot::render(&graph, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{boom::Bit, rust::DecodeTree};

    #[test]
    fn identitree() {
        let insns = vec![
            (
                "vector_crypto_aes_round_decode",
                [
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                ],
            ),
            (
                "vector_arithmetic_unary_add_pairwise_decode",
                [
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Unknown,
                    Bit::Zero,
                ],
            ),
            (
                "vector_arithmetic_unary_cmp_float_lessthan_simd_decode",
                [
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Unknown,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Unknown,
                    Bit::Zero,
                ],
            ),
            (
                "vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode",
                [
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Unknown,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Unknown,
                    Bit::Zero,
                ],
            ),
        ];

        // insert many patterns
        let mut tree = DecodeTree::new();
        for (name, bits) in insns {
            println!("top level insert {name:?} {bits:?}");
            tree.insert(name.into(), bits.to_vec());
            println!("dot: {}", tree.root.as_ref().unwrap().to_dot());
            println!();
            println!();
            println!();
        }

        panic!();

        // test that they are always recovered
    }
}
