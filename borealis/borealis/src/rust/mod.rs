//! Rust module generation

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Ast, Bit, Statement},
        genc::{
            codegen::{
                format::{
                    extract_format, flatten_expression, is_see_assignment,
                    process_decode_function_clause,
                },
                instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
            },
            format::{InstructionFormat, Segment, SegmentContent},
        },
        passes::{
            self, builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            fold_unconditionals::FoldUnconditionals, remove_const_branch::RemoveConstBranch,
            remove_exception::RemoveExceptions, remove_redundant_assigns::RemoveRedundantAssigns,
            resolve_bitvectors::ResolveBitvectors, resolve_return_assigns::ResolveReturns,
        },
        rust::emit::Emit,
    },
    ansi_term::Colour::{Green, Red},
    color_eyre::eyre::Context,
    common::{identifiable::unique_id, intern::InternedString, HashMap},
    itertools::Itertools,
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
        io::{stdout, Cursor, Write},
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
    // let mut tree = DecodeTree::new();

    // get all instructions and their formats
    let mut insn_formats = get_instruction_entrypoint_fns(sail)
        .iter()
        .map(process_decode_function_clause)
        .map(|instr| {
            (
                instr.mangled_name,
                instr
                    .format
                    .0
                    .iter()
                    .map(|Segment { content, length }| match content {
                        SegmentContent::Variable(_) => vec![Bit::Unknown; *length],
                        SegmentContent::Constant(val) => {
                            let mut bits = vec![Bit::Unknown; *length];
                            //

                            for i in 0..*length {
                                match (val >> i) & 1 {
                                    0 => bits[i] = Bit::Zero,
                                    1 => bits[i] = Bit::One,
                                    _ => unreachable!(),
                                }
                            }

                            bits.reverse();
                            bits
                        }
                    })
                    .flatten()
                    .collect(),
            )
        })
        .collect::<HashMap<InternedString, Vec<Bit>>>();

    let tree = DecodeTree::new();

    for (name, mut bits) in insn_formats {
        bits.reverse();
        tree.insert(name, bits);
    }

    tree.to_rs();
    panic!();
    //tree.codegen(), tree.as_dot()
}

struct DecodeTree {
    root: Node,
}

impl DecodeTree {
    fn new() -> Self {
        Self {
            root: Node::new("root".into(), 0),
        }
    }

    fn insert(&self, name: InternedString, bits: Vec<Bit>) {
        self.root.insert(name, bits)
    }

    fn to_rs(&self) {
        let mut f = std::fs::File::create("./target/brig/aarch64.rs").unwrap();

        let header = quote! {
            //! Aarch64 borealis generated module DO NOT EDIT

            fn bit_extract(value: u32, start_idx: usize, len: usize) -> u32 {
                (value >> start_idx) & ((1 << len) - 1)
            }

            fn main() {
                decode(0x91000420);
                panic!("fallthrough :(");
            }
        };

        writeln!(&mut f, "{}", header.to_string()).unwrap();
        writeln!(&mut f, "fn decode(value: u32) {{").unwrap();
        Self::to_rs_rec(&mut f, self.root.clone());
        writeln!(&mut f, "}}").unwrap();
    }

    fn to_rs_rec<W: Write>(w: &mut W, node: Node) {
        if node.offset() == 32 {
            if !node.constrained().is_empty() || node.unconstrained().is_some() {
                panic!("leaf should not have children");
            }

            writeln!(w, "panic!({:?});", node.name(),).unwrap();
        }

        for constrained in node.constrained() {
            let start_idx = node.offset();
            let len = constrained.len;
            let pattern = constrained.value;

            writeln!(
                w,
                "if bit_extract(value, {start_idx}, {len}) == {pattern} {{"
            )
            .unwrap();
            Self::to_rs_rec(w, constrained.target);

            writeln!(w, "}}").unwrap();
        }

        if let Some(unconstrained) = node.unconstrained() {
            Self::to_rs_rec(w, unconstrained.target);
        }

        // fallthrough to unconstrained if exists
    }

    fn to_dot(&self) {
        let mut graph = graph::Graph::default();

        Self::to_dot_node(self.root.clone(), &mut graph);

        dot::render(&graph, &mut stdout()).unwrap()
    }

    fn to_dot_node(node: Node, graph: &mut graph::Graph) -> u32 {
        let id = unique_id();

        graph.nodes.push(id);
        graph
            .node_labels
            .insert(id, format!("{} | {}", node.name(), node.offset()));

        if let Some(unconstrained) = node.unconstrained() {
            let child_id = Self::to_dot_node(unconstrained.target, graph);
            graph.edges.push((id, child_id));
            graph
                .edge_labels
                .insert((id, child_id), format!("X:{}", unconstrained.len));
        }

        for constrained in node.constrained() {
            let child_id = Self::to_dot_node(constrained.target, graph);
            graph.edges.push((id, child_id));
            graph.edge_labels.insert(
                (id, child_id),
                format!("{:b}:{}", constrained.value, constrained.len),
            );
        }

        id
    }
}

#[derive(Clone)]
struct Node {
    inner: Rc<RefCell<NodeInner>>,
}

impl Node {
    fn new(name: InternedString, offset: usize) -> Node {
        Self {
            inner: Rc::new(RefCell::new(NodeInner {
                name,
                offset,
                unconstrained: None,
                constrained: vec![],
            })),
        }
    }

    fn insert(&self, name: InternedString, bits: Vec<Bit>) {
        if self.offset() == 32 {
            log::trace!("offset 32, finished");
            return;
        }

        let prefix = get_prefix(&bits, self.offset());
        log::trace!("inserting {name} = {bits:?} prefix {prefix:?}");

        match prefix {
            Prefix::Constrained { value, len } => {
                // find existing transition
                let constraineds = self.constrained();
                let existing = constraineds
                    .iter()
                    .find(|t| t.len == len && t.value == value);

                if let Some(existing) = existing {
                    existing.target.insert(name, bits);
                } else {
                    let new_node = Node::new(name, self.offset() + len);
                    new_node.insert(name, bits);
                    self.insert_constrained(ConstrainedTransition {
                        value,
                        len,
                        target: new_node,
                    });
                }
            }
            Prefix::Unconstrained { len } => {
                let unconstrained = self.unconstrained();
                match unconstrained {
                    None => {
                        let new_node = Node::new(name, self.offset() + len);
                        new_node.insert(name, bits);

                        self.set_unconstrained(UnconstrainedTransition {
                            len,
                            target: new_node,
                        });
                    }
                    Some(transition) => {
                        if len >= transition.len {
                            // first `len` bits are already handled, so just proceed
                            transition.target.insert(name, bits);
                        } else {
                            // transition is shorter so need to break transition in twain, and pass new node to first part

                            // length of first transition is the prefix length
                            let t_len_a = len;

                            // length of second transition is the remaining length of the original transition
                            let t_len_b = transition.len - len;

                            if transition.len <= t_len_a {
                                panic!("who knows???")
                            }

                            // create new intermediate node
                            let intermediate =
                                Node::new("intermediate".into(), self.offset() + t_len_a);

                            // transition from current node to intermediate
                            let t_a = UnconstrainedTransition {
                                len: t_len_a,
                                target: intermediate.clone(),
                            };

                            // transition from intermediate to original target
                            let t_b = UnconstrainedTransition {
                                len: t_len_b,
                                target: transition.target,
                            };

                            self.set_unconstrained(t_a);
                            intermediate.set_unconstrained(t_b);

                            intermediate.insert(name, bits);
                        }
                    }
                }
            }
        }
    }

    fn offset(&self) -> usize {
        self.inner.borrow().offset
    }

    fn name(&self) -> InternedString {
        self.inner.borrow().name
    }

    fn unconstrained(&self) -> Option<UnconstrainedTransition> {
        self.inner.borrow().unconstrained.clone()
    }

    fn constrained(&self) -> Vec<ConstrainedTransition> {
        self.inner.borrow().constrained.clone()
    }

    fn set_unconstrained(&self, t: UnconstrainedTransition) {
        self.inner.borrow_mut().unconstrained = Some(t);
    }

    fn insert_constrained(&self, t: ConstrainedTransition) {
        self.inner.borrow_mut().constrained.push(t);
    }
}

struct NodeInner {
    name: InternedString,
    offset: usize,
    unconstrained: Option<UnconstrainedTransition>,
    constrained: Vec<ConstrainedTransition>,
}

#[derive(Clone)]
struct UnconstrainedTransition {
    len: usize,
    target: Node,
}

#[derive(Clone)]
struct ConstrainedTransition {
    value: u64,
    len: usize,
    target: Node,
}

#[derive(Debug)]
enum Prefix {
    Constrained { value: u64, len: usize },
    Unconstrained { len: usize },
}

fn get_prefix(bits: &[Bit], offset: usize) -> Prefix {
    let first_bit = bits[offset];

    let biterator = bits[offset..]
        .iter()
        .take_while(|bit| bit.is_unknown() == first_bit.is_unknown());

    match first_bit {
        Bit::Unknown => Prefix::Unconstrained {
            len: biterator.count(),
        },
        _ => {
            let (value, len) = biterator
                .enumerate()
                .fold((0, 0), |(value, len), (idx, bit)| {
                    (value + (bit.value() << idx), len + 1)
                });
            Prefix::Constrained { value, len }
        }
    }
}

mod graph {
    use common::HashMap;
    use dot::{Edges, GraphWalk, LabelText, Labeller, Nodes};

    type NodeId = u32;
    type EdgeId = (NodeId, NodeId);

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<NodeId>,
        pub edges: Vec<EdgeId>,
        pub edge_labels: HashMap<EdgeId, String>,
        pub node_labels: HashMap<NodeId, String>,
    }

    impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
        fn graph_id(&'ast self) -> dot::Id<'ast> {
            dot::Id::new("Decoder").unwrap()
        }

        fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
            dot::Id::new(format!("node{n}")).unwrap()
        }

        fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
            LabelText::EscStr(
                self.node_labels
                    .get(n)
                    .cloned()
                    .unwrap_or(String::from("?"))
                    .into(),
            )
            .into()
        }

        fn node_shape(&'ast self, _: &NodeId) -> Option<LabelText<'ast>> {
            Some(LabelText::LabelStr("Mrecord".into()))
        }

        fn edge_label(&'ast self, e: &EdgeId) -> LabelText<'ast> {
            LabelText::LabelStr(
                self.edge_labels
                    .get(e)
                    .cloned()
                    .unwrap_or(String::from("?"))
                    .into(),
            )
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
}
