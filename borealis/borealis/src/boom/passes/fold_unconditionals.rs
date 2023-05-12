use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        Ast,
    },
    log::{debug, trace},
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

/// Control flow blocks with only one parent and one child (unconditional jump to target) are folded into their parent
#[derive(Debug, Default)]
pub struct FoldUnconditionals;

impl FoldUnconditionals {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for FoldUnconditionals {
    fn name(&self) -> &'static str {
        "FoldUnconditionals"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) {
        ast.borrow().functions.iter().for_each(|(name, def)| {
            debug!("folding {name}");
            fold_graph(def.entry_block.clone());
        });
    }
}

fn fold_graph(entry_block: ControlFlowBlock) {
    let mut processed = HashSet::new();
    let mut to_visit = vec![entry_block];

    // continue until all no nodes are left to visit
    while let Some(current) = to_visit.pop() {
        trace!("processing {current}");

        // continue if we have already processed the current node
        if processed.contains(&current) {
            continue;
        }

        if let Terminator::Unconditional { target: child } = current.terminator() {
            trace!("node is unconditional with child {child}");

            // check if the child has only 1 parent (the current node)
            if child.parents().len() == 1 {
                trace!("only has one parent {}", child.parents()[0]);

                // smoke test that the child's parent is the current node
                assert_eq!(current, child.parents()[0]);
                // smoke test that the child is *not* the current node
                assert_ne!(current, child);

                // move all statements and the terminator from the child to the current node
                let mut statements = current.statements();
                statements.append(&mut child.statements());
                current.set_statements(statements);
                ControlFlowBlock::set_terminator(&current, child.terminator());

                // modified the node so visit it again
                to_visit.push(current.clone());
                continue;
            }

            // if the current node is unconditional, and empty, it can be removed
            if current.statements().is_empty() {
                trace!("node is empty!");
                // set all parents that reference the current node to the child
                for parent in current.parents() {
                    trace!("parent {parent}");

                    let new_terminator = match parent.terminator() {
                        Terminator::Conditional {
                            target,
                            fallthrough,
                            condition,
                        } => {
                            if target == current {
                                Terminator::Conditional {
                                    condition,
                                    target: child.clone(),
                                    fallthrough,
                                }
                            } else if fallthrough == current {
                                Terminator::Conditional {
                                    condition,
                                    target,
                                    fallthrough: child.clone(),
                                }
                            } else {
                                panic!("neither child ({target}, {fallthrough}) of parent ({parent}) of current node ({current}) was current node");
                            }
                        }
                        Terminator::Unconditional { target } => {
                            if target == current {
                                Terminator::Unconditional {
                                    target: child.clone(),
                                }
                            } else {
                                panic!("child of parent of current node was not current node");
                            }
                        }
                        Terminator::Return | Terminator::Undefined => {
                            panic!("parent of current node has no children")
                        }
                    };

                    parent.set_terminator(new_terminator);
                }

                // revisit parents
                to_visit.extend(current.parents());
                continue;
            }
        }

        // mark current node as processed
        processed.insert(current.clone());

        // push children to visit
        to_visit.extend(current.terminator().targets());
    }
}
