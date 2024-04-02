use super::{Context, Function};

pub mod jump_threading;

pub trait FunctionPass: Sized {
    fn run(&self, f: Function) -> bool;
}

pub enum OptLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

pub fn optimise(ctx: &mut Context, level: OptLevel) {
    let passes: Vec<dyn FunctionPass> = match level {
        OptLevel::Level0 => vec![],
        OptLevel::Level1 => vec![jump_threading::JumpThreadingPass],
        OptLevel::Level2 => vec![],
        OptLevel::Level3 => vec![],
    };

    for f in ctx.get_functions() {
        let mut changed = true;

        while changed {
            changed = false;
            for p in passes {
                changed |= p.run(f);
            }
        }
    }
}
