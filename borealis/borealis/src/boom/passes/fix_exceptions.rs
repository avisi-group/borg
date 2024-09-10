//! Fix exceptions

use {
    crate::boom::{control_flow::ControlFlowBlock, passes::Pass, Ast, Size, Type},
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

        registers.insert(
            "have_exception".into(),
            (Shared::new(Type::Bool), ControlFlowBlock::new()),
        );
        registers.insert(
            "current_exception_tag".into(),
            (
                Shared::new(Type::Integer {
                    size: Size::Static(32),
                }),
                ControlFlowBlock::new(),
            ),
        );
        registers.insert(
            "current_exception_value".into(),
            (Shared::new(Type::Union { width }), ControlFlowBlock::new()),
        );
        registers.insert(
            "throw".into(),
            (Shared::new(Type::String), ControlFlowBlock::new()),
        );

        false
    }
}
