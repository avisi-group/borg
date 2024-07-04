use {
    crate::boom::{
        self,
        control_flow::{ControlFlowBlock, Terminator},
    },
    common::{identifiable::Id, shared::Shared, HashMap},
    dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
    std::io,
};

pub fn render<W: io::Write>(w: &mut W, block: &ControlFlowBlock) -> io::Result<()> {
    let mut graph = Graph::new();

    graph.process_node(block.clone());

    dot::render(&graph, w)
}

type NodeId = Id;
type EdgeId = (NodeId, NodeId);

#[derive(Default)]
struct Graph {
    nodes: Vec<NodeId>,
    edges: Vec<EdgeId>,
    node_labels: HashMap<NodeId, String>,
    edge_labels: HashMap<EdgeId, &'static str>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn process_node(&mut self, node: ControlFlowBlock) {
        let id = node.id();

        if self.nodes.contains(&id) {
            // already visited
            return;
        }

        let node_label = {
            let statements = {
                let mut label = Vec::new();

                for statement in node.statements() {
                    boom::pretty_print::print_statement(&mut label, statement);
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

            let terminator = match node.terminator() {
                Terminator::Return(value) => match value {
                    Some(value) => {
                        let value = {
                            let mut buf = Vec::new();
                            boom::pretty_print::print_value(&mut buf, Shared::new(value));
                            String::from_utf8(buf).unwrap()
                        };

                        format!("return ({value})")
                    }
                    None => "return".to_owned(),
                },
                Terminator::Conditional { condition, .. } => {
                    let condition = {
                        let mut buf = Vec::new();
                        boom::pretty_print::print_value(&mut buf, Shared::new(condition));
                        String::from_utf8(buf).unwrap()
                    };

                    format!("if ({condition})")
                }
                Terminator::Unconditional { .. } => "goto".to_owned(),
                Terminator::Panic(_) => "panic".to_owned(),
            };

            format!("{{{}|{statements}|{terminator}}}", node)
        };

        let children = match node.terminator() {
            Terminator::Return(_) | Terminator::Panic(_) => vec![],
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => vec![(target, "if"), (fallthrough, "else")],
            Terminator::Unconditional { target } => vec![(target, "")],
        };

        for child in &children {
            let id = (id, child.0.id());
            self.edges.push(id);
            self.edge_labels.insert(id, child.1);
        }

        self.nodes.push(id);
        self.node_labels.insert(id, node_label);

        for child in children {
            self.process_node(child.0);
        }
    }
}

impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
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

impl<'ast> GraphWalk<'ast, NodeId, EdgeId> for Graph {
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
