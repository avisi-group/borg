use {
    crate::boom::control_flow::{ControlFlowBlock, Terminator},
    common::shared_key::SharedKey,
    dot::{GraphWalk, Labeller},
    std::{
        cell::RefCell,
        collections::HashMap,
        io::{self, Write},
        rc::Rc,
    },
};

pub fn render<W: Write>(w: &mut W, block: &Rc<RefCell<ControlFlowBlock>>) -> io::Result<()> {
    let mut graph = Graph::new();

    graph.process_node(block.clone());

    dot::render(&graph, w)
}

#[derive(Default)]
struct Graph {
    nodes: Vec<NodeId>,
    edges: Vec<EdgeId>,
    labels: HashMap<NodeId, String>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn process_node(&mut self, node: Rc<RefCell<ControlFlowBlock>>) {
        let id: NodeId = node.clone().into();

        if self.nodes.contains(&id) {
            // already visited
            return;
        }

        let (children, label) = match &node.borrow().terminator {
            Terminator::Return => (vec![], "return".to_owned()),
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => (
                vec![target.clone(), fallthrough.clone()],
                format!("conditional"),
            ),
            Terminator::Unconditional { target } => {
                (vec![target.clone()], "unconditional".to_owned())
            }
        };

        for child in &children {
            self.edges.push((id.clone(), child.clone().into()));
        }
        self.nodes.push(id.clone());
        self.labels.insert(id, label);

        for child in children {
            self.process_node(child);
        }
    }
}

type NodeId = SharedKey<ControlFlowBlock>;
type EdgeId = (NodeId, NodeId);

impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(format!("n{}", n)).unwrap()
    }

    fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
        dot::LabelText::LabelStr(self.labels.get(n).map(String::as_str).unwrap_or("?").into())
    }
}

impl<'ast> GraphWalk<'ast, NodeId, EdgeId> for Graph {
    fn nodes(&'ast self) -> dot::Nodes<'ast, NodeId> {
        (&self.nodes).into()
    }

    fn edges(&'ast self) -> dot::Edges<'ast, EdgeId> {
        (&self.edges).into()
    }

    fn source(&'ast self, edge: &EdgeId) -> NodeId {
        edge.0.clone()
    }

    fn target(&'ast self, edge: &EdgeId) -> NodeId {
        edge.1.clone()
    }
}
