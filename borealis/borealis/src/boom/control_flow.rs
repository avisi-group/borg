//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by block targets being unresolved during building (IE. visiting a jump before the label it references is created):
//!
//! 1. Two sets of types, internal maybe-unresolved and public resolved to enforce resolution at type level.
//! 2. Recursive resolution to convert maybe-unresolved to resolved blocks.

use {
    crate::boom::Statement,
    common::{intern::InternedString, shared_key::SharedKey},
    log::warn,
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

/// Control flow graph of a BOOM function
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    /// Function entrypoint
    pub entry_block: Rc<RefCell<ControlFlowBlock>>,
}

impl ControlFlowGraph {
    /// Creates a control flow graph from a slice of statements.
    ///
    /// This should be called per BOOM function.
    pub fn from_statements(statements: &[Rc<RefCell<Statement>>]) -> Self {
        let mut builder = ControlFlowGraphBuilder::new();

        builder.process_statements(statements);

        // resolves all label targets to block targets
        let entry_block = builder.resolve();

        Self { entry_block }
    }
}

/// Node in a control flow graph, contains a basic block of statements and a terminator
#[derive(Debug, Clone)]
pub struct ControlFlowBlock {
    /// Sequence of statements within the block
    pub statements: Vec<Rc<RefCell<Statement>>>,
    /// Block terminator
    pub terminator: Terminator,
}

impl ControlFlowBlock {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            statements: vec![],
            terminator: Terminator::Return,
        }))
    }
}

/// Block terminator
///
/// Describes how one block conditionally or unconditionally jumps to the next
#[derive(Debug, Clone)]
pub enum Terminator {
    /// Function return
    Return,
    /// If condition evaluates to true, then jump to target, otherwise jump to fallthrough
    Conditional {
        condition: Rc<RefCell<Statement>>,
        target: Rc<RefCell<ControlFlowBlock>>,
        fallthrough: Rc<RefCell<ControlFlowBlock>>,
    },
    /// Unconditionally jump to target
    Unconditional {
        target: Rc<RefCell<ControlFlowBlock>>,
    },
}

/// Builder structure for a control flow graph
///
/// Contains state required to build the control flow graph, resolving labels and block terminators
struct ControlFlowGraphBuilder {
    labels: HashMap<InternedString, Rc<RefCell<MaybeUnresolvedControlFlowBlock>>>,
    resolved_blocks:
        HashMap<SharedKey<MaybeUnresolvedControlFlowBlock>, Rc<RefCell<ControlFlowBlock>>>,
    entry_block: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
    current_block: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
}

impl ControlFlowGraphBuilder {
    fn new() -> Self {
        let entry_block = MaybeUnresolvedControlFlowBlock::new();
        Self {
            labels: HashMap::new(),
            resolved_blocks: HashMap::new(),
            current_block: entry_block.clone(),
            entry_block,
        }
    }

    fn process_statements(&mut self, statements: &[Rc<RefCell<Statement>>]) {
        for statement in statements {
            self.current_block.borrow_mut().add_statement(statement);

            match &*statement.borrow() {
                Statement::Label(label) => {
                    let next = MaybeUnresolvedControlFlowBlock::new();

                    self.labels.insert(*label, next.clone());

                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Resolved {
                                target: next.clone(),
                            },
                        ),
                    );
                    self.current_block = next;
                }
                Statement::Block { body } => self.process_statements(body),
                Statement::If {
                    if_body, else_body, ..
                } => {
                    let if_block = MaybeUnresolvedControlFlowBlock::new();
                    let else_block = MaybeUnresolvedControlFlowBlock::new();
                    let post_block = MaybeUnresolvedControlFlowBlock::new();

                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Conditional {
                            condition: statement.clone(),
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
                Statement::Jump { target, .. } => {
                    let fallthrough_block = MaybeUnresolvedControlFlowBlock::new();
                    self.current_block.borrow_mut().set_terminator(
                        MaybeUnresolvedTerminator::Conditional {
                            condition: statement.clone(),
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
                // statement already added at beginning of for loop, so this arm can be a no-op
                _ => (),
            }
        }
    }

    /// Converts unresolved blocks into resolved blocks, errors if any target labels were not present in the labels map
    fn resolve(mut self) -> Rc<RefCell<ControlFlowBlock>> {
        self.resolve_block(self.entry_block.clone())
    }

    fn resolve_block(
        &mut self,
        unresolved: Rc<RefCell<MaybeUnresolvedControlFlowBlock>>,
    ) -> Rc<RefCell<ControlFlowBlock>> {
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

        // clone the statements across
        resolved.borrow_mut().statements = unresolved.borrow().statements.clone();

        //
        resolved.borrow_mut().terminator = match &unresolved.borrow().terminator {
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
            MaybeUnresolvedTerminator::Unknown => {
                warn!("encountered unknown terminator during resolution");
                Terminator::Return
            }
        };

        resolved
    }

    fn resolve_jump_target(
        &mut self,
        target: &MaybeUnresolvedJumpTarget,
    ) -> Rc<RefCell<ControlFlowBlock>> {
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
        condition: Rc<RefCell<Statement>>,
        target: MaybeUnresolvedJumpTarget,
        fallthrough: MaybeUnresolvedJumpTarget,
    },
    Unconditional(MaybeUnresolvedJumpTarget),
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
