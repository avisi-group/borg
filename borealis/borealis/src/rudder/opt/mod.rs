use log::trace;

use super::{Context, Function};

pub mod dead_symbol_elimination;
pub mod jump_threading;
pub mod variable_elimination;

pub enum OptLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

pub type FunctionPassFn = fn(Function) -> bool;
pub type FunctionPass = (&'static str, FunctionPassFn);

static JUMP_THREADING: FunctionPass = ("jump-threading", jump_threading::run);
static VARIABLE_ELIMINATION: FunctionPass = ("var-elimination", variable_elimination::run);
static DEAD_SYMBOL_ELIMINATION: FunctionPass =
    ("dead-symbol-elimination", dead_symbol_elimination::run);

pub fn optimise(ctx: &mut Context, level: OptLevel) {
    let passes: Vec<FunctionPass> = match level {
        OptLevel::Level0 => vec![],
        OptLevel::Level1 => vec![
            JUMP_THREADING,
            DEAD_SYMBOL_ELIMINATION,
            VARIABLE_ELIMINATION,
        ],
        OptLevel::Level2 => vec![
            JUMP_THREADING,
            DEAD_SYMBOL_ELIMINATION,
            VARIABLE_ELIMINATION,
        ],
        OptLevel::Level3 => vec![
            JUMP_THREADING,
            DEAD_SYMBOL_ELIMINATION,
            VARIABLE_ELIMINATION,
        ],
    };

    for f in ctx.get_functions() {
        let mut changed = true;

        trace!("optimising function {}", f.0);

        while changed {
            changed = false;
            for pass in &passes {
                trace!("running pass {}", pass.0);
                changed |= pass.1(f.1.clone());
            }
        }
    }
}
