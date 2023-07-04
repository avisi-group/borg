//! GenC function generation from BOOM

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Ast},
        codegen::genc::Render,
        genc_model::HelperFunction,
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
                .join(", "),
            name: ident.to_string(),
            body: generate_fn_body(definition.entry_block.clone()),
        };

        generated_fns.insert(ident, generated);
    }

    generated_fns.into_values().collect()
}

fn generate_fn_body(_entry_block: ControlFlowBlock) -> String {
    // let mut buf = String::new();
    // let mut stack = vec![0u8];
    // let mut indent = 0u32;
    // let mut current_block = entry_block;

    // // if a block is unconditional, emit the statements and go to the next block
    // // if a block is conditional, emit an if, else branch, where the if and else
    // // blocks are indented one more

    // loop {
    //     //TODO: write current block statements to buf here

    //     match current_block.terminator() {
    //         Terminator::Return => todo!(),
    //         Terminator::Conditional {
    //             condition,
    //             target,
    //             fallthrough,
    //         } => todo!(),
    //         Terminator::Unconditional { target } => todo!(),
    //         Terminator::Undefined => todo!(),
    //     }
    // }

    // buf
    "    return;".into()
}
