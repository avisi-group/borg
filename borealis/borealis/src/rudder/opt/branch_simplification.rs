use {
    common::rudder::{function::Function, statement::Statement},
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
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
            if let Statement::Constant { value, .. } = condition.get(block.arena()).clone() {
                trace!("found constant branch statement {}", value);

                if value.is_zero() {
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
