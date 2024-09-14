use crate::{
    rudder::{Block, Function, StatementKind},
    util::arena::Ref,
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter() {
        changed |= run_on_block(f, block);
    }

    changed
}

fn target_for_threadable(f: &Function, target: Ref<Block>) -> Option<Ref<Block>> {
    if target.get(f.block_arena()).size() == 1 {
        if let StatementKind::Jump {
            target: target_target,
        } = target
            .get(f.block_arena())
            .terminator_statement()
            .unwrap()
            .kind()
        {
            Some(target_target)
        } else {
            None
        }
    } else {
        None
    }
}

fn run_on_block(f: &Function, block: Ref<Block>) -> bool {
    let terminator = block.get(f.block_arena()).terminator_statement().unwrap();

    match terminator.kind() {
        StatementKind::Jump { target } => {
            if let Some(thread_to) = target_for_threadable(f, target) {
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

            let true_target = if let Some(true_thread_to) = target_for_threadable(f, true_target) {
                changed = true;
                true_thread_to
            } else {
                true_target
            };

            let false_target = if let Some(false_thread_to) = target_for_threadable(f, false_target)
            {
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
