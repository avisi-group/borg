use log::trace;

use super::{Context, Function};

pub mod branch_simplification;
pub mod constant_folding;
pub mod constant_propagation;
pub mod dead_stmt_elimination;
pub mod dead_symbol_elimination;
pub mod dead_write_elimination;
pub mod debundler;
pub mod inliner;
pub mod jump_threading;
pub mod phi_analysis;
pub mod return_propagation;
pub mod tail_calls;
pub mod variable_elimination;

pub enum OptLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

pub type FunctionPassFn = fn(Function) -> bool;
pub type FunctionPass = (&'static str, FunctionPassFn);

static INLINER: FunctionPass = ("inliner", inliner::run);
static JUMP_THREADING: FunctionPass = ("jump-threading", jump_threading::run);
static VARIABLE_ELIMINATION: FunctionPass = ("var-elimination", variable_elimination::run);
static DEAD_SYMBOL_ELIMINATION: FunctionPass =
    ("dead-symbol-elimination", dead_symbol_elimination::run);
static DEAD_WRITE_ELIMINATION: FunctionPass =
    ("dead-write-elimination", dead_write_elimination::run);
static DEAD_STMT_ELIMINATION: FunctionPass = ("dead-stmt-elimination", dead_stmt_elimination::run);
static CONSTANT_PROPAGATION: FunctionPass = ("constant-propagation", constant_propagation::run);
static CONSTANT_FOLDING: FunctionPass = ("constant-folding", constant_folding::run);
static DEBUNDLER: FunctionPass = ("debundler", debundler::run);
static RETURN_PROPAGATION: FunctionPass = ("return-propagation", return_propagation::run);
static BRANCH_SIMPLIFICATION: FunctionPass = ("branch-simplification", branch_simplification::run);
static PHI_ANALYSIS: FunctionPass = ("phi-analysis", phi_analysis::run);
static TAIL_CALL: FunctionPass = ("tail-call", tail_calls::run);

pub fn optimise(ctx: &mut Context, level: OptLevel) {
    let passes: Vec<FunctionPass> = match level {
        OptLevel::Level0 => vec![],
        OptLevel::Level1 => vec![
            INLINER,
            JUMP_THREADING,
            BRANCH_SIMPLIFICATION,
            RETURN_PROPAGATION,
            TAIL_CALL,
            DEAD_SYMBOL_ELIMINATION,
            DEAD_WRITE_ELIMINATION,
            DEAD_STMT_ELIMINATION,
            VARIABLE_ELIMINATION,
            CONSTANT_PROPAGATION,
            CONSTANT_FOLDING,
            DEBUNDLER,
            PHI_ANALYSIS,
        ],
        OptLevel::Level2 => vec![
            INLINER,
            JUMP_THREADING,
            BRANCH_SIMPLIFICATION,
            RETURN_PROPAGATION,
            TAIL_CALL,
            DEAD_SYMBOL_ELIMINATION,
            DEAD_WRITE_ELIMINATION,
            DEAD_STMT_ELIMINATION,
            VARIABLE_ELIMINATION,
            CONSTANT_PROPAGATION,
            CONSTANT_FOLDING,
            DEBUNDLER,
            PHI_ANALYSIS,
        ],
        OptLevel::Level3 => vec![
            INLINER,
            JUMP_THREADING,
            BRANCH_SIMPLIFICATION,
            RETURN_PROPAGATION,
            TAIL_CALL,
            DEAD_SYMBOL_ELIMINATION,
            DEAD_WRITE_ELIMINATION,
            DEAD_STMT_ELIMINATION,
            VARIABLE_ELIMINATION,
            CONSTANT_PROPAGATION,
            CONSTANT_FOLDING,
            DEBUNDLER,
            PHI_ANALYSIS,
        ],
    };

    for f in ctx.get_functions() {
        let mut changed = true;

        trace!("optimising function {}", f.0);

        while changed {
            changed = false;
            for pass in &passes {
                trace!("running pass {}", pass.0);

                f.1.update_names();
                while pass.1(f.1.clone()) {
                    changed = true;
                }
            }
        }
    }
}
