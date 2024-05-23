use {
    crate::rudder::{Function, StatementKind},
    log::trace,
};

pub fn run(f: Function) -> bool {
    // check condition for branch.  if it's const, replace with a jump.  if both
    // targets are the same, replace with a jump

    let mut changed = false;
    for block in f.entry_block().iter() {
        let Some(terminator) = block.terminator_statement() else {
            continue;
        };

        if let StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } = terminator.kind()
        {
            if let StatementKind::Constant { value, .. } = condition.kind() {
                trace!("found constant branch statement {}", value);

                if value.zero() {
                    terminator.replace_kind(StatementKind::Jump {
                        target: false_target,
                    });
                } else {
                    terminator.replace_kind(StatementKind::Jump {
                        target: true_target,
                    });
                }

                changed = true;
            } else if true_target == false_target {
                terminator.replace_kind(StatementKind::Jump {
                    target: true_target,
                });
                changed = true;
            }
        }
    }

    changed
}
