use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        Statement, Value,
    },
    common::{intern::InternedString, shared_key::SharedKey},
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

/// Builder structure for a control flow graph
///
/// Contains state required to build the control flow graph, resolving labels and block terminators
pub struct ControlFlowGraphBuilder {
    labels: HashMap<InternedString, Rc<RefCell<MaybeUnresolvedControlFlowBlock>>>,
    resolved_blocks: HashMap<SharedKey<MaybeUnresolvedControlFlowBlock>, ControlFlowBlock>,
    entry_block: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
    current_block: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
}

impl ControlFlowGraphBuilder {
    pub fn from_statements(statements: &[Rc<RefCell<Statement>>]) -> ControlFlowBlock {
        let entry_block = MaybeUnresolvedControlFlowBlock::new();

        let mut celf = Self {
            labels: HashMap::new(),
            resolved_blocks: HashMap::new(),
            current_block: entry_block.clone(),
            entry_block,
        };

        celf.process_statements(statements);

        celf.resolve()
    }

    fn process_statements(&mut self, statements: &[Rc<RefCell<Statement>>]) {
        for statement in statements {
            match &*statement.borrow() {
                Statement::Label(label) => {
                    let next = MaybeUnresolvedControlFlowBlock::new();

                    self.labels.insert(*label, next.clone());

                    next.borrow_mut().label = Some(*label);

                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Resolved {
                                target: next.clone(),
                            },
                        ),
                    );

                    self.current_block = next;
                }
                Statement::If {
                    if_body,
                    else_body,
                    condition,
                } => {
                    let if_block = MaybeUnresolvedControlFlowBlock::new();
                    let else_block = MaybeUnresolvedControlFlowBlock::new();
                    let post_block = MaybeUnresolvedControlFlowBlock::new();

                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Conditional {
                            condition: condition.clone(),
                            target: MaybeUnresolvedJumpTarget::Resolved {
                                target: if_block.clone(),
                            },
                            fallthrough: MaybeUnresolvedJumpTarget::Resolved {
                                target: else_block.clone(),
                            },
                        },
                    );

                    self.current_block = if_block;
                    self.process_statements(if_body);
                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Resolved {
                                target: post_block.clone(),
                            },
                        ),
                    );

                    self.current_block = else_block;
                    self.process_statements(else_body);
                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Resolved {
                                target: post_block.clone(),
                            },
                        ),
                    );

                    self.current_block = post_block;
                }
                Statement::Jump { target, condition } => {
                    let fallthrough_block = MaybeUnresolvedControlFlowBlock::new();
                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Conditional {
                            condition: condition.clone(),
                            target: MaybeUnresolvedJumpTarget::Unresolved { label: *target },
                            fallthrough: MaybeUnresolvedJumpTarget::Resolved {
                                target: fallthrough_block.clone(),
                            },
                        },
                    );

                    self.current_block = fallthrough_block;
                }
                Statement::Goto(label) => {
                    // end current block
                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Unresolved { label: *label },
                        ),
                    );

                    // start new, "detached" block
                    self.current_block = MaybeUnresolvedControlFlowBlock::new();
                }
                Statement::End(_) => {
                    // end current block
                    self.current_block
                        .borrow_mut()
                        .set_terminator(MaybeUnresolvedTerminator::Return);

                    // start new, "detached" block
                    self.current_block = MaybeUnresolvedControlFlowBlock::new();
                }
                Statement::Undefined => {
                    // end current block
                    self.current_block
                        .borrow_mut()
                        .set_terminator(MaybeUnresolvedTerminator::Undefined);

                    // start new, "detached" block
                    self.current_block = MaybeUnresolvedControlFlowBlock::new();
                }
                _ => self.current_block.borrow_mut().add_statement(statement),
            }
        }
    }

    /// Converts unresolved blocks into resolved blocks, errors if any target labels were not present in the labels map
    pub fn resolve(mut self) -> ControlFlowBlock {
        self.resolve_block(self.entry_block.clone())
    }

    fn resolve_block(
        &mut self,
        unresolved: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
    ) -> ControlFlowBlock {
        // if block is already resolved, return that
        if let Some(resolved) = self.resolved_blocks.get(&unresolved.clone().into()) {
            return resolved.clone();
        }

        // create a new control flow block
        let resolved = ControlFlowBlock::new();

        // insert the new resolved control flow block into the map, panicking if it was already
        // resolved
        //
        // it's not populated with the correct statements or terminator at this point, but because
        // it's an Rc-RefCell we can mutate it without modifying the map.
        //
        // we need to insert it here so that the recursive calls as part of the terminator
        // resolution can acquire a reference to the resolved block, if we inserted the correctly
        // resolved block after the recursive call, it would loop forever.
        if let Some(block) = self
            .resolved_blocks
            .insert(unresolved.clone().into(), resolved.clone())
        {
            panic!("unresolved control flow block {unresolved:?} already resolved {block:?} when inserting {resolved:?}")
        }

        resolved.set_label(unresolved.borrow().label);

        // clone the statements across
        resolved.set_statements(unresolved.borrow().statements.clone());

        // resolve each kind of terminator
        let terminator = match &unresolved.borrow().terminator {
            MaybeUnresolvedTerminator::Return => Terminator::Return,
            MaybeUnresolvedTerminator::Conditional {
                condition,
                target,
                fallthrough,
            } => Terminator::Conditional {
                condition: condition.clone(),
                target: self.resolve_jump_target(target),
                fallthrough: self.resolve_jump_target(fallthrough),
            },
            MaybeUnresolvedTerminator::Unconditional(target) => Terminator::Unconditional {
                target: self.resolve_jump_target(target),
            },
            MaybeUnresolvedTerminator::Undefined => Terminator::Undefined,
            MaybeUnresolvedTerminator::Unknown => {
                panic!("encountered unknown terminator during resolution");
            }
        };
        ControlFlowBlock::set_terminator(&resolved, terminator);

        resolved
    }

    fn resolve_jump_target(&mut self, target: &MaybeUnresolvedJumpTarget) -> ControlFlowBlock {
        let block = match target {
            MaybeUnresolvedJumpTarget::Resolved { target } => target.clone(),
            MaybeUnresolvedJumpTarget::Unresolved { label } => self
                .labels
                .get(label)
                .cloned()
                .unwrap_or_else(|| panic!("no entry in label map found for {label:?}")),
        };

        self.resolve_block(block)
    }
}

#[derive(Debug, Clone, Default)]
struct MaybeUnresolvedControlFlowBlock {
    label: Option<InternedString>,
    statements: Vec<Rc<RefCell<Statement>>>,
    terminator: MaybeUnresolvedTerminator,
}

impl MaybeUnresolvedControlFlowBlock {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    fn add_statement(&mut self, statement: &Rc<RefCell<Statement>>) {
        assert!(
            !self.has_terminator(),
            "attempted to add statement to block with terminator"
        );
        self.statements.push(statement.clone());
    }

    fn has_terminator(&self) -> bool {
        !matches!(self.terminator, MaybeUnresolvedTerminator::Unknown)
    }

    fn set_terminator(&mut self, terminator: MaybeUnresolvedTerminator) {
        if self.has_terminator() {
            panic!(
                "attempted to set terminator to block with terminator: \n{self:#?}\n{terminator:#?}"
            )
        } else {
            self.terminator = terminator;
        }
    }
}

/// Possibly-unresolved block terminator statement
#[derive(Debug, Clone)]
enum MaybeUnresolvedTerminator {
    Return,
    Conditional {
        condition: Value,
        target: MaybeUnresolvedJumpTarget,
        fallthrough: MaybeUnresolvedJumpTarget,
    },
    Unconditional(MaybeUnresolvedJumpTarget),
    Undefined,
    Unknown,
}

impl Default for MaybeUnresolvedTerminator {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
enum MaybeUnresolvedJumpTarget {
    Resolved {
        target: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
    },
    Unresolved {
        label: InternedString,
    },
}
