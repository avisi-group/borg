use {
    crate::rudder::opt::OptimizationContext,
    common::rudder::{
        function::Function,
        statement::{Statement, UnaryOperationKind},
    },
    log::trace,
};

pub fn run(_ctx: &OptimizationContext, f: &mut Function) -> bool {
    // check condition for branch.  if it's const, replace with a jump.  if both
    // targets are the same, replace with a jump

    let mut changed = false;
    for block_ref in f.block_iter().collect::<Vec<_>>() {
        let block = block_ref.get_mut(f.arena_mut());
        let Some(terminator_ref) = block.terminator_statement() else {
            continue;
        };

        if let Statement::Branch {
            condition,
            true_target,
            false_target,
        } = terminator_ref.get(block.arena()).clone()
        {
            let condition = condition.get(block.arena()).clone();

            if let Statement::Constant(value) = condition {
                trace!("found constant branch statement {}", value);

                if value.is_zero() == Some(true) {
                    terminator_ref
                        .get_mut(block.arena_mut())
                        .replace_kind(Statement::Jump {
                            target: false_target,
                        });
                } else {
                    terminator_ref
                        .get_mut(block.arena_mut())
                        .replace_kind(Statement::Jump {
                            target: true_target,
                        });
                }

                changed = true;
            } else if let Statement::UnaryOperation { kind, value } = condition {
                match kind {
                    UnaryOperationKind::Not => {
                        let new_true = false_target;
                        let new_false = true_target;

                        terminator_ref
                            .get_mut(block.arena_mut())
                            .replace_kind(Statement::Branch {
                                condition: value,
                                true_target: new_true,
                                false_target: new_false,
                            });

                        changed = true;
                    }
                    _ => {}
                }
            } else if true_target == false_target {
                terminator_ref
                    .get_mut(block.arena_mut())
                    .replace_kind(Statement::Jump {
                        target: true_target,
                    });
                changed = true;
            }
        }
    }

    changed
}
