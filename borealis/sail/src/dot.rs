//! Graphviz rendering for AST

use {
    crate::{
        ast::{Ast, Comment, CommentRoot, Definition, Identifier, TypeDefinition},
        visitor::{Visitor, Walkable},
    },
    common::{identifiable::Identifiable, intern::InternedStringKey},
    deepsize::DeepSizeOf,
    dot::{GraphWalk, Labeller},
    std::{
        collections::HashMap,
        io::{self, Write},
    },
};

/// Write the rendered DOT graph of the supplied AST to a writer.
pub fn render<W: Write>(ast: &Ast, w: &mut W) -> io::Result<()> {
    let graph = Graph::new(ast);

    writeln!(
        io::stderr().lock(),
        "COUNTER: {}",
        common::identifiable::unique_id()
    )?;
    writeln!(
        io::stderr().lock(),
        "Graph size: {} labels, {} edges",
        graph.nodes.len(),
        graph.edges.len()
    )?;

    writeln!(
        io::stderr().lock(),
        "Graph size: {} bytes",
        graph.deep_size_of()
    )?;

    dot::render(&graph, w)?;

    Ok(())
}

/// Dot graph constructed from walking AST
#[derive(Debug, DeepSizeOf)]
struct Graph {
    nodes: HashMap<u32, InternedStringKey>,
    edges: Vec<(u32, u32)>,
}

impl Graph {
    fn new(ast: &Ast) -> Self {
        let mut celf = Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        };

        celf.visit_root(ast);

        celf
    }
}

impl Visitor for Graph {
    fn visit_root(&mut self, node: &Ast) {
        self.nodes.insert(node.id(), "AST".into());

        for def in &node.defs {
            self.edges.push((node.id(), def.id()));
        }

        for comment in &node.comments {
            self.edges.push((node.id(), comment.id()));
        }

        node.walk(self);
    }

    fn visit_definition(&mut self, node: &Definition) {
        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_type_definition(&mut self, node: &TypeDefinition) {
        self.nodes.insert(node.id(), "Type Definition".into());

        node.walk(self);
    }

    #[allow(missing_docs)]
    fn visit_identifier(&mut self, node: &Identifier) {
        self.nodes.insert(node.id(), "Identifier".into());
        node.walk(self);
    }

    fn visit_comment_root(&mut self, node: &CommentRoot) {
        self.nodes.insert(node.id(), "Comment root".into());
        node.walk(self);
    }

    fn visit_comment(&mut self, node: &Comment) {
        self.nodes.insert(node.id(), "Comment".into());
        node.walk(self);
    }
}

type NodeId = (u32, InternedStringKey);
type EdgeId = (NodeId, NodeId);

impl<'ast> Labeller<'ast, NodeId, EdgeId> for Graph {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(format!("n{}", n.0)).unwrap()
    }

    fn node_label(&'ast self, n: &NodeId) -> dot::LabelText<'ast> {
        dot::LabelText::LabelStr(n.1.to_string().into())
    }
}

impl<'ast> GraphWalk<'ast, NodeId, EdgeId> for Graph {
    fn nodes(&'ast self) -> dot::Nodes<'ast, NodeId> {
        self.nodes.iter().map(|(&id, &k)| (id, k)).collect()
    }

    fn edges(&'ast self) -> dot::Edges<'ast, EdgeId> {
        self.edges
            .iter()
            .map(|(source, target)| {
                (
                    (*source, *self.nodes.get(source).unwrap()),
                    (*target, *self.nodes.get(target).unwrap()),
                )
            })
            .collect()
    }

    fn source(&'ast self, edge: &EdgeId) -> NodeId {
        edge.0
    }

    fn target(&'ast self, edge: &EdgeId) -> NodeId {
        edge.1
    }
}
