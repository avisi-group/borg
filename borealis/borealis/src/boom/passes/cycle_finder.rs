use {
    crate::boom::{passes::Pass, Ast},
    log::warn,
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug, Default)]
pub struct CycleFinder {}

impl CycleFinder {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for CycleFinder {
    fn name(&self) -> &'static str {
        "IfRaiser"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) {
        ast.borrow().functions.iter().for_each(|(name, def)| {
            if def.entry_block.contains_cycles() {
                warn!("{name} has cycles");
            }
        });
    }
}
