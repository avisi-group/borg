//! Graphviz rendering for AST

use {
    crate::{
        ast::{
            Ast, Comment, CommentRoot, DecSpec, DefaultSpec, Definition, Expression,
            FieldExpression, FunctionClause, FunctionDefinition, Identifier, IdentifierAux,
            IndexRange, InternalLoopMeasure, KindIdentifier, KindedIdentifier, LValueExpression,
            LetBind, Literal, LoopMeasure, MappingClause, MappingDefinition, MappingPattern,
            MappingPatternExpression, NConstraint, NumericExpression, Order, Pattern, PatternMatch,
            QuantItem, RecursiveAnnotationOpt, ScatteredDefinition, Typ, TypArg, TypPat, TypQuant,
            TypeAnnotationOpt, TypeDefinition, TypeScheme, TypeUnion, Value, ValueSpecification,
        },
        types::EnumWrapper,
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

    dot::render(&graph, w)?;

    Ok(())
}

/// Dot graph constructed from walking AST
#[derive(Debug, DeepSizeOf)]
struct Graph {
    nodes: HashMap<u32, InternedStringKey>,
    edges: Vec<(u32, u32)>,
    current_parent: u32,
}

impl Graph {
    fn new(ast: &Ast) -> Self {
        let mut celf = Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            current_parent: 0,
        };

        celf.visit_root(ast);

        celf
    }
}

impl Visitor for Graph {
    fn visit_root(&mut self, node: &Ast) {
        self.nodes.insert(node.id(), "AST".into());
        node.walk(self);
    }

    fn visit_definition(&mut self, node: &EnumWrapper<Definition>) {
        self.nodes.insert(node.id(), "Definition".into());
        node.walk(self);
    }

    fn visit_type_definition(&mut self, node: &TypeDefinition) {
        self.nodes.insert(node.id(), "Type definition".into());
        node.walk(self);
    }

    fn visit_identifier(&mut self, node: &Identifier) {
        let label = match &node.inner {
            IdentifierAux::Identifier(s) => format!("Identifier({:?})", s),
            IdentifierAux::Operator(s) => format!("Operator({:?})", s),
        };
        self.nodes.insert(node.id(), label.into());
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

    fn visit_typquant(&mut self, node: &TypQuant) {
        self.nodes.insert(node.id(), "Type quantifier".into());
        node.walk(self);
    }

    fn visit_typarg(&mut self, node: &TypArg) {
        self.nodes.insert(node.id(), "Type argument".into());
        node.walk(self);
    }

    fn visit_quantitem(&mut self, node: &QuantItem) {
        self.nodes.insert(node.id(), "Quant item".into());
        node.walk(self);
    }

    fn visit_numeric_expression(&mut self, node: &NumericExpression) {
        self.nodes.insert(node.id(), "Numeric expression".into());
        node.walk(self);
    }

    fn visit_typ(&mut self, node: &Typ) {
        self.nodes.insert(node.id(), "Typ".into());
        node.walk(self);
    }

    fn visit_order(&mut self, node: &Order) {
        self.nodes.insert(node.id(), "Order".into());
        node.walk(self)
    }

    fn visit_nconstraint(&mut self, node: &NConstraint) {
        self.nodes.insert(node.id(), "NConstraint".into());
        node.walk(self);
    }

    fn visit_kinded_identifier(&mut self, node: &KindedIdentifier) {
        self.nodes.insert(node.id(), "Kinded identifier".into());
        node.walk(self);
    }

    fn visit_kind_identifier(&mut self, node: &KindIdentifier) {
        self.nodes.insert(node.id(), "Kind identifier".into());
        node.walk(self);
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.nodes.insert(node.id(), "Function definition".into());
        node.walk(self);
    }

    fn visit_recursive_annotation_opt(&mut self, node: &RecursiveAnnotationOpt) {
        self.nodes
            .insert(node.id(), "Recursive annotation opt".into());
        node.walk(self);
    }

    fn visit_type_annotation_opt(&mut self, node: &TypeAnnotationOpt) {
        self.nodes.insert(node.id(), "Type annotation opt".into());
        node.walk(self);
    }

    fn visit_function_clause(&mut self, node: &FunctionClause) {
        self.nodes.insert(node.id(), "Function clause".into());
        node.walk(self);
    }

    fn visit_default_spec(&mut self, node: &DefaultSpec) {
        self.nodes.insert(node.id(), "Default spec".into());
        node.walk(self);
    }

    fn visit_value_specification(&mut self, node: &ValueSpecification) {
        self.nodes.insert(node.id(), "Value spec".into());
        node.walk(self);
    }

    fn visit_type_scheme(&mut self, node: &TypeScheme) {
        self.nodes.insert(node.id(), "Type scheme".into());
        node.walk(self);
    }

    fn visit_pattern(&mut self, node: &Pattern) {
        self.nodes.insert(node.id(), "Pattern".into());
        node.walk(self);
    }

    fn visit_expression(&mut self, node: &Expression) {
        self.nodes.insert(node.id(), "Expression".into());
        node.walk(self);
    }

    fn visit_literal(&mut self, node: &Literal) {
        self.nodes.insert(node.id(), "Literal".into());
        node.walk(self);
    }

    fn visit_typpat(&mut self, node: &TypPat) {
        self.nodes.insert(node.id(), "Typ pat".into());
        node.walk(self);
    }

    fn visit_pattern_match(&mut self, node: &PatternMatch) {
        self.nodes.insert(node.id(), "pattern match".into());
        node.walk(self);
    }

    fn visit_internal_loop_measure(&mut self, node: &InternalLoopMeasure) {
        self.nodes.insert(node.id(), "internal loop measure".into());
        node.walk(self);
    }

    fn visit_field_expression(&mut self, node: &FieldExpression) {
        self.nodes.insert(node.id(), "field expression".into());
        node.walk(self);
    }

    fn visit_letbind(&mut self, node: &LetBind) {
        self.nodes.insert(node.id(), "Let bind".into());
        node.walk(self);
    }

    fn visit_lvalue_expression(&mut self, node: &LValueExpression) {
        self.nodes.insert(node.id(), "LValue Expression".into());
        node.walk(self);
    }

    fn visit_value(&mut self, node: &EnumWrapper<Value>) {
        self.nodes.insert(node.id(), "Value".into());
        node.walk(self);
    }

    fn visit_typunion(&mut self, node: &TypeUnion) {
        self.nodes.insert(node.id(), "Type union".into());
        node.walk(self);
    }

    fn visit_indexrange(&mut self, node: &IndexRange) {
        self.nodes.insert(node.id(), "Index range".into());
        node.walk(self);
    }

    fn visit_decspec(&mut self, node: &DecSpec) {
        self.nodes.insert(node.id(), "Dec spec".into());
        node.walk(self);
    }

    fn visit_mapping_definition(&mut self, node: &MappingDefinition) {
        self.nodes.insert(node.id(), "mapping definition".into());
        node.walk(self);
    }

    fn visit_scattered_definition(&mut self, node: &ScatteredDefinition) {
        self.nodes.insert(node.id(), "scattered spec".into());
        node.walk(self);
    }

    fn visit_loop_measure(&mut self, node: &LoopMeasure) {
        self.nodes.insert(node.id(), "Loop measure".into());
        node.walk(self);
    }

    fn visit_mapping_clause(&mut self, node: &MappingClause) {
        self.nodes.insert(node.id(), "mapping clause".into());
        node.walk(self);
    }

    fn visit_mapping_pattern_expression(&mut self, node: &MappingPatternExpression) {
        self.nodes
            .insert(node.id(), "Mappign pattern expression".into());
        node.walk(self);
    }

    fn visit_mapping_pattern(&mut self, node: &MappingPattern) {
        self.nodes.insert(node.id(), "Mapping pattern".into());
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
