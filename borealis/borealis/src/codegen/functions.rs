//! GenC function generation from BOOM

use {
    crate::{
        boom::{Ast, NamedType, Type},
        codegen::genc::Render,
        genc::HelperFunction,
    },
    itertools::Itertools,
    std::{cell::RefCell, rc::Rc},
};

/// Generates GenC helper functions from all functions in a BOOM AST
pub fn generate_fns(boom: Rc<RefCell<Ast>>) -> Vec<HelperFunction> {
    #[allow(unstable_name_collisions)]
    boom.borrow()
        .functions
        .iter()
        .filter(|(name, _)| name.to_string().ends_with("_decode"))
        .map(|(name, definition)| HelperFunction {
            return_type: definition.signature.return_type.render(),
            parameters: definition
                .signature
                .parameters
                .iter()
                .filter(|NamedType { typ, .. }| *typ != Type::Unit)
                .map(Render::render)
                .intersperse(", ".to_owned())
                .collect(),
            name: name.to_string(),
            body: "return;".to_owned(),
        })
        .collect()
}
