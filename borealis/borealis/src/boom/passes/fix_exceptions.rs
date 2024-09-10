//! Fix exceptions

use {
    crate::boom::control_flow::Terminator,
    crate::boom::{control_flow::ControlFlowBlock, passes::Pass, Ast, Literal, Size, Type, Value},
    common::shared::Shared,
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
        let (width, _) = *ast.get().unions.get(&("exception".into())).unwrap();

        let registers = &mut ast.get_mut().registers;

        let empty_init_body = {
            let block = ControlFlowBlock::new();
            block.set_terminator(Terminator::Return(Value::Literal(Shared::new(
                Literal::Unit,
            ))));
            block
        };

        registers.insert(
            "have_exception".into(),
            (Shared::new(Type::Bool), empty_init_body.clone()),
        );
        registers.insert(
            "current_exception_tag".into(),
            (
                Shared::new(Type::Integer {
                    size: Size::Static(32),
                }),
                empty_init_body.clone(),
            ),
        );
        registers.insert(
            "current_exception_value".into(),
            (Shared::new(Type::Union { width }), empty_init_body.clone()),
        );
        registers.insert(
            "throw".into(),
            (Shared::new(Type::String), empty_init_body.clone()),
        );

        false
    }
}
