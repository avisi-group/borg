//! Branches with a constant condition can be removed

use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            Ast,
        },
        passes::{any::AnyExt as _, Pass},
    },
    log::{debug, trace},
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

/// Control flow blocks with only one parent and one child (unconditional jump
/// to target) are folded into their parent
#[derive(Debug, Default)]
pub struct RemoveConstBranch;

impl RemoveConstBranch {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RemoveConstBranch {
    fn name(&self) -> &'static str {
        "RemoveConstBranch"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .iter()
            .map(|(name, def)| {
                debug!("removing const branch {name}");
                remove_const_branch(def.entry_block.clone())
            })
            .any()
    }
}

fn remove_const_branch(entry_block: ControlFlowBlock) -> bool {
    let mut did_change = false;
    let mut processed = HashSet::new();
    let mut to_visit = vec![entry_block];

    // continue until all no nodes are left to visit
    while let Some(current) = to_visit.pop() {
        trace!("processing {current}");

        // continue (try next unvisited node) if we have already processed the current
        // node
        if processed.contains(&current) {
            continue;
        }

        // mark current node as processed
        processed.insert(current.clone());

        // push children to visit
        to_visit.extend(current.terminator().targets());

        let Terminator::Conditional {
            condition,
            target,
            fallthrough,
        } = current.terminator() else {
            continue;
        };

        let Some(condiiton) = condition.evaluate_bool(&current) else {
            continue;
        };

        if condiiton {
            current.set_terminator(Terminator::Unconditional { target });
        } else {
            current.set_terminator(Terminator::Unconditional {
                target: fallthrough,
            });
        }

        did_change = true;
    }

    did_change
}
