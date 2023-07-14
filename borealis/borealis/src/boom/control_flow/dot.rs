use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            Statement,
        },
        codegen::emit::Emit,
    },
    dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
    std::{
        collections::HashMap,
        io::{self, Write},
    },
};

pub fn render<W: Write>(w: &mut W, block: &ControlFlowBlock) -> io::Result<()> {
    let mut graph = Graph::new();

    graph.process_node(block.clone());

    dot::render(&graph, w)
}

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
        let id: NodeId = node.clone();

        if self.nodes.contains(&id) {
            // already visited
            return;
        }

        let node_label = {
            let statements = {
                let mut label = String::new();

                for statement in node.statements() {
                    if let Statement::If { condition, .. } = &*statement.borrow() {
                        condition.emit(&mut label).unwrap();
                    } else {
                        statement.emit(&mut label).unwrap();
                    }
                }

                label
            };

            let terminator = match node.terminator() {
                Terminator::Return(value) => {
                    format!(
                        "return {:?}",
                        value.as_ref().map_or("none".to_owned(), Emit::emit_string)
                    )
                }
                Terminator::Conditional { condition, .. } => {
                    format!("if {}", condition.emit_string())
                }
                Terminator::Unconditional { .. } => "goto".to_owned(),
            };

            format!("{{{}|{statements}|{terminator}}}", node)
        };

        let children = match node.terminator() {
            Terminator::Return(_) => vec![],
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => vec![(target, "if"), (fallthrough, "else")],
            Terminator::Unconditional { target } => vec![(target, "")],
        };

        for child in &children {
            let id = (id.clone(), child.0.clone());
            self.edges.push(id.clone());
            self.edge_labels.insert(id, child.1);
        }

        self.nodes.push(id.clone());
        self.node_labels.insert(id, node_label);

        for child in children {
            self.process_node(child.0);
        }
    }
}

type NodeId = ControlFlowBlock;
type EdgeId = (NodeId, NodeId);

impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(format!("n{}", n)).unwrap()
    }

    fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
        let label = self
            .node_labels
            .get(n)
            .map(|s| {
                s.replace('%', "pcnt")
                    .replace("->", "_to_")
                    .replace('<', r#"\<"#)
                    .replace('>', r#"\>"#)
                    .replace('\n', r#"\l"#)
            })
            .unwrap_or("?".to_owned());

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
        edge.0.clone()
    }

    fn target(&'ast self, edge: &EdgeId) -> NodeId {
        edge.1.clone()
    }
}
