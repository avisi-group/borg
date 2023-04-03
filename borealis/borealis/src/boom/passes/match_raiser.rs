use crate::boom::{passes::Pass, Ast};

pub struct MatchRaiser;

impl Pass for MatchRaiser {
    fn run(&mut self, _ast: &mut Ast) {
        // todo!()
    }
}
