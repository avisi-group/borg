use {
    common::rudder::{Model, function::Function},
    log::trace,
    rayon::iter::{IntoParallelRefMutIterator, ParallelIterator},
};

pub mod block_inliner;
pub mod branch_simplification;
pub mod constant_folding;
pub mod constant_propagation;
pub mod dead_stmt_elimination;
pub mod dead_symbol_elimination;
pub mod dead_write_elimination;
pub mod jump_threading;
pub mod phi_analysis;
//pub mod return_propagation;
//pub mod tail_calls;
pub mod variable_elimination;
pub mod vector_folding;
pub mod local_tuple_removal;

pub enum OptLevel {
    Level3,
}

pub type FunctionPassFn = fn(&mut Function) -> bool;
pub type FunctionPass = (&'static str, FunctionPassFn);

static BLOCK_INLINER: FunctionPass = ("block-inliner", block_inliner::run);
static JUMP_THREADING: FunctionPass = ("jump-threading", jump_threading::run);
static VARIABLE_ELIMINATION: FunctionPass = ("var-elimination", variable_elimination::run);
static DEAD_SYMBOL_ELIMINATION: FunctionPass =
    ("dead-symbol-elimination", dead_symbol_elimination::run);
static DEAD_WRITE_ELIMINATION: FunctionPass =
    ("dead-write-elimination", dead_write_elimination::run);
static DEAD_STMT_ELIMINATION: FunctionPass = ("dead-stmt-elimination", dead_stmt_elimination::run);
static CONSTANT_PROPAGATION: FunctionPass = ("constant-propagation", constant_propagation::run);
static CONSTANT_FOLDING: FunctionPass = ("constant-folding", constant_folding::run);
//static RETURN_PROPAGATION: FunctionPass = ("return-propagation",
// return_propagation::run);
static BRANCH_SIMPLIFICATION: FunctionPass = ("branch-simplification", branch_simplification::run);
static PHI_ANALYSIS: FunctionPass = ("phi-analysis", phi_analysis::run);
// static TAIL_CALL: FunctionPass = ("tail-call", tail_calls::run);
static VECTOR_FOLDING: FunctionPass = ("vector-folding", vector_folding::run);
static LOCAL_TUPLE_REMOVAL: FunctionPass = ("local-tuple-removal", local_tuple_removal::run);
// static DESTROY_BITVECTORS: FunctionPass = ("destroy-bitvectors",
// destroy_bitvectors::run); static MATERIALISE_APINTS: FunctionPass =
// ("materialise-apints", materialise_apints::run);

pub fn optimise(ctx: &mut Model, level: OptLevel) {
    let passes: Vec<FunctionPass> = match level {
        OptLevel::Level3 => vec![
            BLOCK_INLINER,
            JUMP_THREADING,
            BRANCH_SIMPLIFICATION,
            //RETURN_PROPAGATION,
            // TAIL_CALL,
            DEAD_SYMBOL_ELIMINATION,
            DEAD_WRITE_ELIMINATION,
            DEAD_STMT_ELIMINATION,
            VARIABLE_ELIMINATION,
            CONSTANT_PROPAGATION,
            CONSTANT_FOLDING,
            LOCAL_TUPLE_REMOVAL,
            // DESTROY_BITVECTORS,
            // MATERIALISE_APINTS,
            VECTOR_FOLDING,
            PHI_ANALYSIS,
        ],
    };

    ctx.functions_mut()
        .par_iter_mut()
        .for_each(|(name, function)| {
            let mut changed = true;

            trace!("optimising function {name:?}");

            while changed {
                changed = false;
                for pass in &passes {
                    trace!("running pass {}", pass.0);

                    while pass.1(function) {
                        changed = true;
                    }
                }
            }
        });
}
