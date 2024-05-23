use crate::rudder::{Block, StatementKind};

pub fn run(f: crate::rudder::Function) -> bool {
    let mut changed = false;

    for block in f.entry_block().iter() {
        changed |= run_on_block(block);
    }

    changed
}

fn target_for_threadable(target: &Block) -> Option<Block> {
    if target.size() == 1 {
        if let StatementKind::Jump {
            target: target_target,
        } = target.terminator_statement().unwrap().kind()
        {
            Some(target_target)
        } else {
            None
        }
    } else {
        None
    }
}

fn run_on_block(block: Block) -> bool {
    let terminator = block.terminator_statement().unwrap();

    match terminator.kind() {
        StatementKind::Jump { target } => {
            if let Some(thread_to) = target_for_threadable(&target) {
                terminator.replace_kind(StatementKind::Jump { target: thread_to });
                true
            } else {
                false
            }
        }
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => {
            let mut changed = false;

            let true_target = if let Some(true_thread_to) = target_for_threadable(&true_target) {
                changed = true;
                true_thread_to
            } else {
                true_target
            };

            let false_target = if let Some(false_thread_to) = target_for_threadable(&false_target) {
                changed = true;
                false_thread_to
            } else {
                false_target
            };

            if changed {
                terminator.replace_kind(StatementKind::Branch {
                    condition,
                    true_target,
                    false_target,
                });
            }

            changed
        }
        _ => false,
    }
}
