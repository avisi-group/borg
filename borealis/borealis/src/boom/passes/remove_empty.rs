use {
    crate::boom::{passes::Pass, Ast},
    std::{cell::RefCell, rc::Rc},
};

/// Removes empty control flow blocks
#[derive(Default)]
pub struct RemoveEmpty;

impl RemoveEmpty {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RemoveEmpty {
    fn name(&self) -> &'static str {
        "RemoveEmpty"
    }

    fn run(&mut self, _ast: Rc<RefCell<Ast>>) {}
}
