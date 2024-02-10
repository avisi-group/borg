use {
    crate::{
        boom::{self, Bit},
        genc::{
            codegen::{
                format::process_decode_function_clause, instruction::get_instruction_entrypoint_fns,
            },
            format::{Segment, SegmentContent},
        },
        rudder,
    },
    common::{identifiable::unique_id, intern::InternedString, HashMap},
    sail::sail_ast,
    std::{
        cell::RefCell,
        collections::HashSet,
        io::{stdout, Write},
        rc::Rc,
    },
};

pub fn generate_decode_fns<W: Write>(
    writer: &mut W,
    sail: &sail_ast::Ast,
    boom: Rc<RefCell<boom::Ast>>,
) {
    // retrieve all `decode64` function clauses
    let epfns = get_instruction_entrypoint_fns(sail);

    // process clauses to extract bits from segments
    let insn_formats = epfns
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

    tree.codegen(writer);

    let boom_functions = &boom.borrow().functions;
    let mut seen = HashSet::new();
    for (name, _) in insn_formats {
        if seen.contains(&name.0) {
            continue;
        }

        seen.insert(name.0);
        let boomfn = boom_functions.get(&name.0);

        generate_decode_fn(writer, name.0, boomfn.unwrap());
    }
}

fn generate_decode_fn<W: Write>(
    writer: &mut W,
    name: InternedString,
    boom_function: &boom::FunctionDefinition,
) {
    let _rudderfn = rudder::Function::from_boom(boom_function);

    writeln!(
        writer,
        "fn execute_{}(state: &mut AArch64CoreState) -> ExecuteResult {{ log::trace!({:?}); ExecuteResult::Ok }}",
        name, name
    )
    .unwrap();
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

    pub fn codegen<W: Write>(&self, writer: &mut W) {
        writeln!(
            writer,
            "fn execute_instruction(value: u32, state: &mut AArch64CoreState) -> ExecuteResult {{"
        )
        .unwrap();
        Self::to_rs_rec(writer, self.root.clone());
        writeln!(writer, "ExecuteResult::UndefinedInstruction").unwrap();
        writeln!(writer, "}}").unwrap();
    }

    fn to_rs_rec<W: Write>(w: &mut W, node: Node) {
        if node.offset() == 32 {
            if !node.constrained().is_empty() || node.unconstrained().is_some() {
                panic!("leaf should not have children");
            }

            writeln!(w, "return execute_{}(state);", node.name()).unwrap();
        }

        let mut first = true;
        for constrained in node.constrained() {
            let start_idx = node.offset();
            let len = constrained.len;

            let mask = ((1u64 << len) - 1) << start_idx;
            let pattern = constrained.value << start_idx;

            if start_idx + len > 32 {
                panic!("bit extract overflow")
            } else if len == 32 {
                writeln!(w, "if value == 0x{pattern:08x} {{").unwrap();
            } else {
                if !first {
                    write!(w, "else ").unwrap();
                }

                writeln!(w, "if (value & 0x{mask:08x}) == 0x{pattern:08x} {{").unwrap();
            }

            Self::to_rs_rec(w, constrained.target);

            writeln!(w, "}}").unwrap();

            if first {
                first = false;
            }
        }

        if let Some(unconstrained) = node.unconstrained() {
            Self::to_rs_rec(w, unconstrained.target);
        }

        // fallthrough to unconstrained if exists
    }

    /// Emits a decode tree as a DOT graph to `stdout`
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
