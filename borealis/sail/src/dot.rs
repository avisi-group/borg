//! Graphviz rendering for AST

use {
    crate::{
        ast::{Ast, Comment, Definition},
        types::OCamlString,
        visitor::{walk_comment, walk_comment_root, walk_definition, walk_root, Visitor},
    },
    common::identifiable::Identifiable,
    deepsize::DeepSizeOf,
    dot::{GraphWalk, Labeller},
    std::{
        collections::{HashMap, LinkedList},
        io::{self, Write},
    },
};

/// Write the rendered DOT graph of the supplied AST to a writer.
pub fn render<W: Write>(ast: &Ast, w: &mut W) -> io::Result<()> {
    let graph = Graph::new(ast);

    writeln!(io::stderr().lock(), "COUNTER: {}", 0)?;
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
    nodes: HashMap<u64, String>,
    edges: Vec<(u64, u64)>,
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
        self.nodes.insert(node.id(), "AST".to_owned());
        walk_root(self, node);
    }

    fn visit_definition(&mut self, node: &Definition) {
        self.nodes.insert(0, "Definition".to_owned());
        walk_definition(self, node);
    }

    fn visit_comment_root(&mut self, node: &(OCamlString, LinkedList<Comment>)) {
        self.nodes.insert(0, "Comment root".to_owned());
        walk_comment_root(self, node);
    }

    fn visit_comment(&mut self, node: &Comment) {
        self.nodes.insert(0, "Comment".to_owned());
        walk_comment(self, node);
    }
}

type NodeId<'a> = (u64, &'a str);
type EdgeId<'a> = (NodeId<'a>, NodeId<'a>);

impl<'ast> Labeller<'ast, NodeId<'ast>, EdgeId<'ast>> for Graph {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("AST").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(format!("n{}", n.0)).unwrap()
    }

    fn node_label(&'ast self, n: &NodeId<'ast>) -> dot::LabelText<'ast> {
        dot::LabelText::LabelStr(n.1.into())
    }
}

impl<'ast> GraphWalk<'ast, NodeId<'ast>, EdgeId<'ast>> for Graph {
    fn nodes(&'ast self) -> dot::Nodes<'ast, NodeId> {
        self.nodes.iter().map(|(i, s)| (*i, s.as_str())).collect()
    }

    fn edges(&'ast self) -> dot::Edges<'ast, EdgeId> {
        self.edges
            .iter()
            .map(|(source, target)| {
                (
                    (*source, self.nodes.get(source).unwrap().as_str()),
                    (*target, self.nodes.get(target).unwrap().as_str()),
                )
            })
            .collect()
    }

    fn source(&'ast self, edge: &EdgeId<'ast>) -> NodeId<'ast> {
        edge.0
    }

    fn target(&'ast self, edge: &EdgeId<'ast>) -> NodeId<'ast> {
        edge.1
    }
}
