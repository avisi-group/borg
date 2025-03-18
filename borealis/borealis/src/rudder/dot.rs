use {
    common::{
        arena::{Arena, Ref},
        modname::HashMap,
        rudder::block::Block,
    },
    dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
    std::io,
};

pub fn render<W: io::Write>(
    w: &mut W,
    block_arena: &Arena<Block>,
    entry_block: Ref<Block>,
) -> io::Result<()> {
    let mut graph = Graph::new(block_arena);

    graph.process_node(entry_block);

    dot::render(&graph, w)
}

type NodeId = usize;
type EdgeId = (NodeId, NodeId);

struct Graph<'a> {
    arena: &'a Arena<Block>,
    nodes: Vec<NodeId>,
    edges: Vec<EdgeId>,
    node_labels: HashMap<NodeId, String>,
    edge_labels: HashMap<EdgeId, &'static str>,
}

impl<'a> Graph<'a> {
    fn new(arena: &'a Arena<Block>) -> Self {
        Self {
            arena,
            nodes: vec![],
            edges: vec![],
            edge_labels: HashMap::default(),
            node_labels: HashMap::default(),
        }
    }

    fn process_node(&mut self, node: Ref<Block>) {
        let id = node.index();

        if self.nodes.contains(&id) {
            // already visited
            return;
        }

        let node_label = {
            let statements = {
                let mut label = Vec::new();

                let s_arena = node.get(&self.arena).arena();

                for statement in node.get(&self.arena).statements() {
                    label.extend_from_slice(statement.to_string(s_arena).as_bytes());
                    label.extend(b"\\l");
                }

                let mut label = String::from_utf8(label).unwrap();

                label = label
                    .replace('<', r"\<")
                    .replace('>', r"\>")
                    .replace('{', r"\{")
                    .replace('}', r"\}");

                label
            };

            format!("{{{:#x}|{statements}}}", node.index())
        };

        for target in &node.get(&self.arena).targets() {
            let id = (id, target.index());
            self.edges.push(id);
            self.edge_labels.insert(id, "");
        }

        self.nodes.push(id);
        self.node_labels.insert(id, node_label);

        for target in &node.get(&self.arena).targets() {
            self.process_node(*target);
        }
    }
}

impl<'ast, 'a> Labeller<'ast, NodeId, EdgeId> for Graph<'a> {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(format!("n{:x}", n)).unwrap()
    }

    fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
        let label = self.node_labels.get(n).cloned().unwrap_or("?".to_owned());

        LabelText::EscStr(label.into())
    }

    fn node_shape(&'ast self, _: &NodeId) -> Option<LabelText<'ast>> {
        Some(LabelText::LabelStr("record".into()))
    }

    fn edge_label(&'ast self, e: &EdgeId) -> LabelText<'ast> {
        LabelText::LabelStr(self.edge_labels.get(e).copied().unwrap_or("?").into())
    }
}

impl<'ast, 'a> GraphWalk<'ast, NodeId, EdgeId> for Graph<'a> {
    fn nodes(&'ast self) -> Nodes<'ast, NodeId> {
        (&self.nodes).into()
    }

    fn edges(&'ast self) -> Edges<'ast, EdgeId> {
        (&self.edges).into()
    }

    fn source(&'ast self, edge: &EdgeId) -> NodeId {
        edge.0
    }

    fn target(&'ast self, edge: &EdgeId) -> NodeId {
        edge.1
    }
}
