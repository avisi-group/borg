use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        Ast,
    },
    common::shared_key::SharedKey,
    log::trace,
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

#[derive(Debug, Default)]
pub struct FoldUnconditionals {
    visited: HashSet<SharedKey<ControlFlowBlock>>,
}

impl FoldUnconditionals {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }

    fn fold(&mut self, node: Rc<RefCell<ControlFlowBlock>>) {
        let key = SharedKey::from(node.clone());
        trace!("folding {key}");

        if self.visited.contains(&key) {
            return;
        }
        self.visited.insert(SharedKey::from(node.clone()));

        // flag for whether we mutated the current node and therefore need to attempt folding again
        let mut did_change = false;

        // check if the current node jumps unconditionally to the child (target) node
        let terminator = node.borrow().terminator().clone();
        if let Terminator::Unconditional { target } = terminator {
            let parents = target.borrow().parents().collect::<Vec<_>>();

            // check if the child has only 1 parent (the current node)
            if parents.len() == 1 {
                // smoke test that the child's parent is the current node
                assert_eq!(
                    SharedKey::from(node.clone()),
                    SharedKey::from(parents[0].clone())
                );
                // smoke test that the child is not the current node
                assert_ne!(
                    SharedKey::from(node.clone()),
                    SharedKey::from(target.clone())
                );

                did_change = true;

                // move all statements and the terminator from the child to the parent (child will be deallocated automatically)
                node.borrow_mut()
                    .statements
                    .append(&mut target.borrow_mut().statements);
                ControlFlowBlock::set_terminator(&node, target.borrow().terminator().clone());
            }
        }

        // if folded, restart on current node
        if did_change {
            self.fold(node.clone());
        }

        // recurse into children
        match node.borrow().terminator().clone() {
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
        let ast = ast.borrow();
        let def = ast
            .functions
            .get(&("integer_arithmetic_addsub_immediate_decode".into()))
            .unwrap();

        def.control_flow
            .as_dot(&mut std::fs::File::create("target/controlflow.dot").unwrap())
            .unwrap();

        self.fold(def.control_flow.entry_block.clone());

        def.control_flow
            .as_dot(&mut std::fs::File::create("target/controlflow_folded.dot").unwrap())
            .unwrap();
    }
}
