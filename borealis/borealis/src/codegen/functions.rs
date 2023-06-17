//! GenC function generation from BOOM

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Ast},
        codegen::genc::Render,
        genc::HelperFunction,
    },
    common::intern::InternedString,
    itertools::Itertools,
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

/// Generates GenC helper functions from all functions in a BOOM AST
pub fn generate_fns(
    ast: Rc<RefCell<Ast>>,
    initial_fns: Vec<InternedString>,
) -> Vec<HelperFunction> {
    let mut remaining_fns = initial_fns;
    let mut generated_fns = HashMap::new();

    while let Some(ident) = remaining_fns.pop() {
        let ast = ast.borrow();
        let definition = ast
            .functions
            .get(&ident)
            .expect("cannot generate GenC for unknown function");

        #[allow(unstable_name_collisions)]
        let generated = HelperFunction {
            return_type: definition.signature.return_type.render(),
            parameters: definition
                .signature
                .parameters
                .iter()
                .map(Render::render)
                .intersperse(", ".to_owned())
                .collect(),
            name: ident.to_string(),
            body: generate_fn_body(definition.entry_block.clone()),
        };

        generated_fns.insert(ident, generated);
    }

    generated_fns.into_values().collect()
}

fn generate_fn_body(_entry_block: ControlFlowBlock) -> String {
    "return;".to_owned()
}
