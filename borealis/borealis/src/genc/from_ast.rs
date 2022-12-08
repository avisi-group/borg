use {
    crate::{decode::DecodeStringVisitor, genc::Description},
    sail::{ast::Ast, visitor::Visitor},
};

impl From<&Ast> for Description {
    fn from(ast: &Ast) -> Self {
        DecodeStringVisitor::new().visit_root(ast);
        todo!();
    }
}
