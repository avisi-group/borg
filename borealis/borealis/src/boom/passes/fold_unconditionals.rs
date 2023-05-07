use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        Ast,
    },
    log::trace,
    std::{cell::RefCell, collections::HashSet, fs::File, rc::Rc},
};

/// Control flow blocks with only one parent and one child (unconditional jump to target) are folded into their parent
#[derive(Debug, Default)]
pub struct FoldUnconditionals {
    visited: HashSet<ControlFlowBlock>,
}

impl FoldUnconditionals {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }

    fn fold(&mut self, node: ControlFlowBlock) {
        trace!("folding {node}");

        if self.visited.contains(&node) {
            return;
        }

        // flag for whether we mutated the current node and therefore need to attempt folding again
        let mut did_change = false;

        // check if the current node jumps unconditionally to the child (target) node
        let terminator = node.terminator();
        if let Terminator::Unconditional { target } = terminator {
            let parents = target.parents();

            // check if the child has only 1 parent (the current node)
            if parents.len() == 1 {
                // smoke test that the child's parent is the current node
                assert_eq!(node, parents[0]);
                // smoke test that the child is not the current node
                assert_ne!(node, target);

                did_change = true;

                // move all statements and the terminator from the child to the parent (child will be deallocated automatically)
                let mut statements = node.statements();
                statements.append(&mut target.statements());
                node.set_statements(statements);
                ControlFlowBlock::set_terminator(&node, target.terminator());
            }
        }

        // if folded, restart on current node
        if did_change {
            self.fold(node.clone());
        }

        // recurse into children
        self.visited.insert(node.clone());
        match node.terminator() {
            Terminator::Return | Terminator::Undefined => (),
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => {
                self.fold(target);
                self.fold(fallthrough);
            }
            Terminator::Unconditional { target } => {
                self.fold(target);
            }
        }
    }
}

impl Pass for FoldUnconditionals {
    fn run(&mut self, ast: Rc<RefCell<Ast>>) {
        ast.borrow().functions.iter().for_each(|(name, def)| {
            trace!("folding {name}");
            def.control_flow
                .as_dot(&mut File::create(format!("target/dot/{name}.dot")).unwrap())
                .unwrap();

            self.fold(def.control_flow.entry_block.clone());

            def.control_flow
                .as_dot(&mut File::create(format!("target/dot/{name}_folded.dot")).unwrap())
                .unwrap();
        });
    }
}
