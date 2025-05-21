use {
    crate::rudder::analysis::pure::PurityAnalysis,
    common::rudder::{Model, function::Function},
    log::trace,
    rayon::iter::{IntoParallelRefMutIterator, ParallelIterator},
};

mod block_inliner;
mod branch_simplification;
mod constant_folding;
mod constant_propagation;
mod dead_stmt_elimination;
mod dead_symbol_elimination;
mod dead_write_elimination;
mod jump_threading;
mod phi_analysis;
//mod return_propagation;
//mod tail_calls;
mod local_tuple_removal;
mod panic_asserter;
mod remove_unused_parameters;
mod useless_cast_elimination;
mod variable_elimination;
mod vector_folding;

pub enum OptLevel {
    Level3,
}

type FunctionPassFn = fn(&OptimizationContext, &mut Function) -> bool;
type FunctionPass = (&'static str, FunctionPassFn);

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
static PANIC_ASSERTER: FunctionPass = ("panic-asserter", panic_asserter::run);
static USELESS_CAST_ELIMINATION: FunctionPass =
    ("useless-cast-elimination", useless_cast_elimination::run);
// static DESTROY_BITVECTORS: FunctionPass = ("destroy-bitvectors",
// destroy_bitvectors::run); static MATERIALISE_APINTS: FunctionPass =
// ("materialise-apints", materialise_apints::run);

struct OptimizationContext {
    purity: PurityAnalysis,
}

pub fn optimise(model: &mut Model, level: OptLevel) {
    let passes: Vec<FunctionPass> = match level {
        OptLevel::Level3 => vec![
            BLOCK_INLINER,
            JUMP_THREADING,
            BRANCH_SIMPLIFICATION,
            PANIC_ASSERTER,
            //RETURN_PROPAGATION,
            // TAIL_CALL,
            DEAD_SYMBOL_ELIMINATION,
            DEAD_WRITE_ELIMINATION,
            DEAD_STMT_ELIMINATION,
            USELESS_CAST_ELIMINATION,
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

    let purity = PurityAnalysis::new(&model);

    let mut context = OptimizationContext { purity };

    model
        .functions_mut()
        .par_iter_mut()
        .for_each(|(name, function)| {
            let mut changed = true;

            trace!("optimising function {name:?}");

            while changed {
                changed = false;
                for pass in &passes {
                    trace!("running pass {}", pass.0);

                    while pass.1(&context, function) {
                        changed = true;
                    }
                }
            }
        });

    remove_unused_parameters::run(&mut context, model);
}
