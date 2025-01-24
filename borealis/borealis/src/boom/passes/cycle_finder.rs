//! Finds cycles in the control flow graph

use {crate::boom::passes::Pass, common::boom::Ast, common::shared::Shared, log::warn};
/// Finds cycles in the control flow graph
#[derive(Debug, Default)]
pub struct CycleFinder {}

impl CycleFinder {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for CycleFinder {
    fn name(&self) -> &'static str {
        "CycleFinder"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get().functions.iter().for_each(|(name, def)| {
            if def.entry_block.contains_cycles() {
                warn!("{name} has cycles");
            }
        });
        false
    }
}
