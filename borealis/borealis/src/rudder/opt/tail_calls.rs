use log::trace;

use crate::rudder::{Function, StatementKind};

pub fn run(f: Function) -> bool {
    // If a block contains a call followed by a return, optimise this into a tail
    // call

    let mut changed = false;
    for block in f.entry_block().iter() {
        if block.statements().len() < 2 {
            continue;
        }

        let terminator = block.terminator_statement().unwrap();

        if let StatementKind::Return { value } = terminator.kind() {
            let second_last = block
                .statements()
                .iter()
                .rev()
                .skip(1)
                .next()
                .unwrap()
                .clone();

            if let Some(return_value) = value {
                if return_value != second_last {
                    continue;
                }
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
