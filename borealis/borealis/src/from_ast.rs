use {
    crate::{
        format::DecodeStringVisitor,
        genc::{Description, Instruction},
    },
    sail::{ast::Ast, visitor::Visitor},
};

impl From<&Ast> for Description {
    fn from(ast: &Ast) -> Self {
        let formats = {
            let mut visitor = DecodeStringVisitor::new();
            visitor.visit_root(ast);
            visitor.formats
        };

        let mut description = Description::empty();

        description.instructions = formats
            .into_iter()
            .map(|(name, format)| {
                (
                    name.to_string(),
                    Instruction {
                        format,
                        execute: "".to_owned(),
                    },
                )
            })
            .collect();

        description
    }
}
