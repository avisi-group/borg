//! Fix exceptions

use {
    crate::boom::{passes::Pass, Ast, Size, Type},
    sailrs::{intern::InternedString, shared::Shared},
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
        let (width, _) = *ast
            .get()
            .unions
            .get(&InternedString::from_static("exception"))
            .unwrap();

        let registers = &mut ast.get_mut().registers;

        registers.insert("have_exception".into(), Shared::new(Type::Bool));
        registers.insert(
            "current_exception_tag".into(),
            Shared::new(Type::Integer {
                size: Size::Static(32),
            }),
        );
        registers.insert(
            "current_exception_value".into(),
            Shared::new(Type::Union { width }),
        );
        registers.insert("throw".into(), Shared::new(Type::String));

        false
    }
}
