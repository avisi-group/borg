use {
    crate::rudder::{model::function::Function, model::statement::StatementKind},
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

        if let StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } = terminator_ref.get(&block.statement_arena).kind().clone()
        {
            if let StatementKind::Constant { value, .. } =
                condition.get(&block.statement_arena).kind().clone()
            {
                trace!("found constant branch statement {}", value);

                if value.zero() {
                    terminator_ref
                        .get_mut(&mut block.statement_arena)
                        .replace_kind(StatementKind::Jump {
                            target: false_target,
                        });
                } else {
                    terminator_ref
                        .get_mut(&mut block.statement_arena)
                        .replace_kind(StatementKind::Jump {
                            target: true_target,
                        });
                }

                changed = true;
            } else if true_target == false_target {
                terminator_ref
                    .get_mut(&mut block.statement_arena)
                    .replace_kind(StatementKind::Jump {
                        target: true_target,
                    });
                changed = true;
            }
        }
    }

    changed
}
