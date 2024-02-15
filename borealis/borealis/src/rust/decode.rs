use {
    crate::{
        boom::Bit,
        genc::{
            codegen::{
                format::process_decode_function_clause, instruction::get_instruction_entrypoint_fns,
            },
            format::{Segment, SegmentContent},
        },
    },
    common::{identifiable::unique_id, intern::InternedString, HashMap},
    proc_macro2::{Literal, TokenStream},
    quote::{quote, TokenStreamExt},
    sail::sail_ast,
    std::{cell::RefCell, io::stdout, rc::Rc},
};

/// Generates the instruction decode function from the Sail `decode64` clauses
/// using a decode tree
pub fn generate_fn(sail: &sail_ast::Ast) -> TokenStream {
    // retrieve all `decode64` function clauses and process clauses to extract bits
    // from segments
    let insn_formats = get_instruction_entrypoint_fns(sail)
        .iter()
        .map(process_decode_function_clause)
        .map(|instr| {
            (
                (instr.execute_function_name, instr.mangled_name),
                instr
                    .format
                    .0
                    .iter()
                    .map(|Segment { content, length }| match content {
                        SegmentContent::Variable(_) => vec![Bit::Unknown; *length],
                        SegmentContent::Constant(val) => {
                            let mut bits = vec![Bit::Unknown; *length];

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
        .collect::<HashMap<(InternedString, InternedString), Vec<Bit>>>();

    let tree = DecodeTree::new();

    for (name, bits) in insn_formats.iter() {
        let mut bits = bits.clone();
        bits.reverse();
        tree.insert(name.0.clone(), bits);
    }

    tree.codegen()
}

pub struct DecodeTree {
    root: Node,
}

impl DecodeTree {
    pub fn new() -> Self {
        Self {
            root: Node::new("root".into(), 0),
        }
    }

    pub fn insert(&self, name: InternedString, bits: Vec<Bit>) {
        self.root.insert(name, bits)
    }

    pub fn codegen(&self) -> TokenStream {
        let body = self.root.codegen();
        quote! {
            fn decode_execute(value: u32, state: &mut AArch64CoreState) -> ExecuteResult {
                #body
                ExecuteResult::UndefinedInstruction
            }
        }
    }

    /// Emits a decode tree as a DOT graph to `stdout`
    #[allow(unused)] // (might need for debugging)
    pub fn to_dot(&self) {
        fn to_dot_node(node: Node, graph: &mut graph::Graph) -> u32 {
            let id = unique_id();

            graph.nodes.push(id);
            graph
                .node_labels
                .insert(id, format!("{} | {}", node.name(), node.offset()));

            if let Some(unconstrained) = node.unconstrained() {
                let child_id = to_dot_node(unconstrained.target, graph);
                graph.edges.push((id, child_id));
                graph
                    .edge_labels
                    .insert((id, child_id), format!("X:{}", unconstrained.len));
            }

            for constrained in node.constrained() {
                let child_id = to_dot_node(constrained.target, graph);
                graph.edges.push((id, child_id));
                graph.edge_labels.insert(
                    (id, child_id),
                    format!("{:b}:{}", constrained.value, constrained.len),
                );
            }

            id
        }

        let mut graph = graph::Graph::default();

        to_dot_node(self.root.clone(), &mut graph);

        dot::render(&graph, &mut stdout()).unwrap()
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
                let existing = self
                    .constrained()
                    .into_iter()
                    .find(|t| t.len == len && t.value == value);

                match existing {
                    // if transition already exists, continue insertion on its target
                    Some(existing) => {
                        existing.target.insert(name, bits);
                    }
                    // otherwise, create a new one
                    None => {
                        let new_node = Node::new(name, self.offset() + len);
                        new_node.insert(name, bits);
                        self.insert_constrained(ConstrainedTransition {
                            value,
                            len,
                            target: new_node,
                        });
                    }
                }
            }
            Prefix::Unconstrained { len } => {
                match self.unconstrained() {
                    // current node does not have an unconstrained transition, insert one
                    None => {
                        let new_node = Node::new(name, self.offset() + len);
                        new_node.insert(name, bits);

                        self.set_unconstrained(UnconstrainedTransition {
                            len,
                            target: new_node,
                        });
                    }
                    // current node does have an unconstrained transition
                    Some(transition) => {
                        // existing unconstrained transition already covers the prefix length
                        if len >= transition.len {
                            // first `len` bits are already handled so simply continue
                            transition.target.insert(name, bits);
                        }
                        // existing unconstrained transition only covers part of the prefix
                        else {
                            // transition is shorter so need to break transition in twain, and pass
                            // new node to first part

                            // length of first transition is the prefix length
                            let first_transition_length = len;

                            // length of second transition is the remaining length of the original
                            // transition
                            let second_transition_length = transition.len - len;

                            if transition.len <= first_transition_length {
                                // this was a condition in the original gensim implementation
                                // todo: figure out what it was for
                                panic!("???")
                            }

                            // create new intermediate node
                            let intermediate = Node::new(
                                "intermediate".into(),
                                self.offset() + first_transition_length,
                            );

                            // transition from current node to intermediate
                            let first_transition = UnconstrainedTransition {
                                len: first_transition_length,
                                target: intermediate.clone(),
                            };

                            // transition from current node
                            self.set_unconstrained(first_transition);

                            // transition from intermediate to original target
                            let second_transition = UnconstrainedTransition {
                                len: second_transition_length,
                                target: transition.target,
                            };

                            // transition from intermediate
                            intermediate.set_unconstrained(second_transition);

                            // now recurse and insert on intermediate2
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

    fn codegen(&self) -> TokenStream {
        // check if it is a leaf node
        if self.constrained().is_empty() && self.unconstrained().is_none() {
            if self.offset() != 32 {
                panic!("leaf should have length 32");
            }

            let fn_name = self.name();

            return quote!(return #fn_name(state););
        }

        let mut constrained_handlers = quote!();
        let mut first = true;
        for constrained in self.constrained() {
            let start_idx = self.offset();

            let mask = u32::try_from(((1u64 << constrained.len) - 1) << start_idx).unwrap();
            let pattern = u32::try_from(constrained.value << start_idx).unwrap();

            if start_idx + constrained.len > 32 {
                panic!("bit extract overflow")
            }

            let mask = format!("0x{mask:08x}").parse::<Literal>().unwrap();
            let pattern = format!("0x{pattern:08x}").parse::<Literal>().unwrap();

            if constrained.len == 32 {
                let inner = constrained.target.codegen();

                constrained_handlers.append_all(quote! {
                    if value == #pattern {
                        #inner
                    }
                });
            } else {
                if !first {
                    constrained_handlers.append_all(quote! {else});
                }

                let inner = constrained.target.codegen();

                constrained_handlers.append_all(quote! {
                    if (value & #mask) == #pattern {
                        #inner
                    }
                });
            }

            if first {
                first = false;
            }
        }

        // fallthrough to unconstrained if exists
        let unconstrained_handler = if let Some(unconstrained) = self.unconstrained() {
            unconstrained.target.codegen()
        } else {
            quote!()
        };

        quote! {
            #constrained_handlers
            #unconstrained_handler
        }
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
        // get first homogenous segment
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
    use {
        common::HashMap,
        dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
    };

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
