use {
    crate::rudder::{Function, StatementKind},
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    // If a block contains a call followed by a return, optimise this into a tail
    // call

    let mut changed = false;
    for block in f.block_iter().map(|b| b.get(f.block_arena())) {
        if block.statements().len() < 2 {
            continue;
        }

        let terminator = block.terminator_statement().unwrap();

        if let StatementKind::Return { value } = terminator.kind() {
            let second_last = block.statements().iter().rev().nth(1).unwrap().clone();

            if value != second_last {
                continue;
            }

            if let StatementKind::Call { target, args, .. } = second_last.kind() {
                trace!("candidate for tail call");

                second_last.replace_kind(StatementKind::Call {
                    target,
                    args,
                    tail: true,
                });
                block.kill_statement(&terminator);
                changed = true;
            }
        }
    }

    changed
}
