use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        pretty_print::BoomPrettyPrinter,
        visitor::Visitor,
        Statement,
    },
    common::shared_key::SharedKey,
    dot::{Edges, GraphWalk, LabelText, Labeller, Nodes},
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
    node_labels: HashMap<NodeId, String>,
    edge_labels: HashMap<EdgeId, &'static str>,
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

        let node_label = {
            let statements = {
                let mut label = vec![];
                let mut printer = BoomPrettyPrinter::new(&mut label);

                for statement in &node.borrow().statements {
                    if let Statement::If { condition, .. } = &*statement.borrow() {
                        printer.visit_value(condition);
                    } else {
                        printer.visit_statement(statement.clone());
                    }
                }
                String::from_utf8(label).unwrap()
            };

            let terminator = match &node.borrow().terminator {
                Terminator::Return => "return".to_owned(),
                Terminator::Conditional { condition, .. } => {
                    let mut buf = vec![];
                    BoomPrettyPrinter::new(&mut buf).visit_value(condition);
                    format!("if {}", String::from_utf8_lossy(&buf))
                }
                Terminator::Unconditional { .. } => "goto".to_owned(),
            };

            format!(
                "{{{}|{statements}|{terminator}}}",
                ControlFlowBlock::label(node.clone())
            )
        };

        let children = match &node.borrow().terminator {
            Terminator::Return => vec![],
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => vec![
                (target.clone(), "target"),
                (fallthrough.clone(), "fallthrough"),
            ],
            Terminator::Unconditional { target } => vec![(target.clone(), "")],
        };

        for child in &children {
            let id = (id.clone(), child.0.clone().into());
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
