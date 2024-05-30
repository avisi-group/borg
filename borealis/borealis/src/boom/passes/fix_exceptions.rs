//! Fix exceptions

use {
    crate::boom::{control_flow::ControlFlowBlock, passes::Pass, Ast, Definition, Type},
    common::shared::Shared,
    itertools::Itertools,
};

/// Adds registers needed for exceptions
pub struct FixExceptions;

impl FixExceptions {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for FixExceptions {
    fn name(&self) -> &'static str {
        "FixExceptions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let (type_name, type_fields) = ast
            .get()
            .definitions
            .iter()
            .filter_map(|def| {
                if let Definition::Union { name, fields } = def {
                    if name.as_ref() == "exception" {
                        Some((*name, fields.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .exactly_one()
            .unwrap();

        ast.get_mut().registers.insert(
            "have_exception".into(),
            (Shared::new(Type::Bool), ControlFlowBlock::new()),
        );
        ast.get_mut().registers.insert(
            "current_exception".into(),
            (
                Shared::new(Type::Union {
                    name: type_name,
                    fields: type_fields,
                }),
                ControlFlowBlock::new(),
            ),
        );
        ast.get_mut().registers.insert(
            "throw".into(),
            (Shared::new(Type::String), ControlFlowBlock::new()),
        );

        false
    }
}
