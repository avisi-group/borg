//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by block targets being unresolved during building (IE. visiting a jump before the label it references is created):
//!
//! Two sets of types, internal maybe-unresolved and public resolved to enforce resolution at type level.
//!
//! Recursive resolution to convert maybe-unresolved to resolved blocks. BlockKey used to identify blocks by hashing the pointer referenced by the `Rc`.

use {
    crate::boom::Statement,
    common::intern::InternedString,
    log::{trace, warn},
    std::{
        cell::RefCell,
        collections::{hash_map::DefaultHasher, HashMap},
        fmt::{self, Display, Formatter},
        hash::{Hash, Hasher},
        ops::Deref,
        ptr,
        rc::Rc,
    },
};

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub entry_block: Rc<RefCell<ControlFlowBlock>>,
}

impl ControlFlowGraph {
    /// Creates a control flow graph from a slice of statements
    pub fn from_statements(statements: &[Rc<RefCell<Statement>>]) -> Self {
        let entry_block = UnresolvedControlFlowBlock::new();
        let mut builder = ControlFlowGraphBuilder {
            labels: HashMap::new(),
            current_block: entry_block.clone(),
            entry_block,
            resolved_blocks: BlockMap::new(),
        };

        builder.process_statements(statements);

        // resolves all label targets to block targets
        let entry_block = builder.resolve();

        Self { entry_block }
    }
}

struct ControlFlowGraphBuilder {
    labels: HashMap<InternedString, Rc<RefCell<UnresolvedControlFlowBlock>>>,
    entry_block: Rc<RefCell<UnresolvedControlFlowBlock>>,
    current_block: Rc<RefCell<UnresolvedControlFlowBlock>>,
    resolved_blocks: BlockMap,
}

impl ControlFlowGraphBuilder {
    fn process_statements(&mut self, statements: &[Rc<RefCell<Statement>>]) {
        for statement in statements {
            self.current_block.borrow_mut().add_statement(statement);

            match statement.borrow().deref() {
                Statement::Label(label) => {
                    let next = UnresolvedControlFlowBlock::new();

                    self.labels.insert(*label, next.clone());

                    self.current_block.borrow_mut().set_terminator(
                        UnresolvedTerminator::Unconditional(MaybeUnresolvedJumpTarget::Resolved {
                            target: next.clone(),
                        }),
                    );
                    self.current_block = next;
                }
                Statement::Block { body } => self.process_statements(body),
                Statement::If {
                    if_body, else_body, ..
                } => {
                    let if_block = UnresolvedControlFlowBlock::new();
                    let else_block = UnresolvedControlFlowBlock::new();
                    let post_block = UnresolvedControlFlowBlock::new();

                    self.current_block.borrow_mut().set_terminator(
                        UnresolvedTerminator::Conditional {
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
                        UnresolvedTerminator::Unconditional(MaybeUnresolvedJumpTarget::Resolved {
                            target: post_block.clone(),
                        }),
                    );

                    self.current_block = else_block;
                    self.process_statements(else_body);
                    self.current_block.borrow_mut().set_terminator(
                        UnresolvedTerminator::Unconditional(MaybeUnresolvedJumpTarget::Resolved {
                            target: post_block.clone(),
                        }),
                    );

                    self.current_block = post_block;
                }
                Statement::Jump { target, .. } => {
                    let fallthrough_block = UnresolvedControlFlowBlock::new();
                    self.current_block.borrow_mut().set_terminator(
                        UnresolvedTerminator::Conditional {
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
                        UnresolvedTerminator::Unconditional(
                            MaybeUnresolvedJumpTarget::Unresolved { label: *label },
                        ),
                    );

                    // start new, "detached" block
                    self.current_block = UnresolvedControlFlowBlock::new();
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
        unresolved: Rc<RefCell<UnresolvedControlFlowBlock>>,
    ) -> Rc<RefCell<ControlFlowBlock>> {
        trace!("resolving {}", BlockKey(unresolved.clone()));

        if let Some(resolved) = self.resolved_blocks.get(unresolved.clone()) {
            return resolved;
        }

        let resolved = ControlFlowBlock::new();

        self.resolved_blocks
            .insert(unresolved.clone(), resolved.clone());

        resolved.borrow_mut().statements = unresolved.borrow().statements.clone();

        let terminator = match &unresolved.borrow().terminator {
            UnresolvedTerminator::Return => Terminator::Return,
            UnresolvedTerminator::Conditional {
                condition,
                target,
                fallthrough,
            } => Terminator::Conditional {
                condition: condition.clone(),
                target: self.resolve_jump_target(target),
                fallthrough: self.resolve_jump_target(fallthrough),
            },
            UnresolvedTerminator::Unconditional(target) => Terminator::Unconditional {
                target: self.resolve_jump_target(target),
            },
            UnresolvedTerminator::Unknown => {
                warn!("encountered unknown terminator during resolution");
                Terminator::Return
            }
        };
        resolved.borrow_mut().terminator = terminator;

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
                .unwrap_or_else(|| panic!("no entry in label map found for {:?}", label)),
        };

        self.resolve_block(block)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnresolvedControlFlowBlock {
    statements: Vec<Rc<RefCell<Statement>>>,
    terminator: UnresolvedTerminator,
}

impl UnresolvedControlFlowBlock {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    fn add_statement(&mut self, statement: &Rc<RefCell<Statement>>) {
        if self.has_terminator() {
            panic!("attempted to add statement to block with terminator");
        }
        self.statements.push(statement.clone())
    }

    fn has_terminator(&self) -> bool {
        match self.terminator {
            UnresolvedTerminator::Unknown => false,
            _ => true,
        }
    }

    fn set_terminator(&mut self, terminator: UnresolvedTerminator) {
        if !self.has_terminator() {
            self.terminator = terminator
        } else {
            panic!(
                "attempted to set terminator to block with terminator: \n{:#?}\n{:#?}",
                self, terminator
            )
        }
    }
}

/// Possibly-unresolved block terminator statement
#[derive(Debug, Clone)]
pub enum UnresolvedTerminator {
    Return,
    Conditional {
        condition: Rc<RefCell<Statement>>,
        target: MaybeUnresolvedJumpTarget,
        fallthrough: MaybeUnresolvedJumpTarget,
    },
    Unconditional(MaybeUnresolvedJumpTarget),
    Unknown,
}

impl Default for UnresolvedTerminator {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum MaybeUnresolvedJumpTarget {
    Resolved {
        target: Rc<RefCell<UnresolvedControlFlowBlock>>,
    },
    Unresolved {
        label: InternedString,
    },
}

/// Node in a control flow graph, contains a basic block of statements and a terminator
#[derive(Debug, Clone)]
pub struct ControlFlowBlock {
    statements: Vec<Rc<RefCell<Statement>>>,
    terminator: Terminator,
}

impl ControlFlowBlock {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            statements: vec![],
            terminator: Terminator::Return,
        }))
    }
}

/// Block terminator statement, describes how the block jumps to the next
#[derive(Debug, Clone)]
pub enum Terminator {
    Return,
    Conditional {
        condition: Rc<RefCell<Statement>>,
        target: Rc<RefCell<ControlFlowBlock>>,
        fallthrough: Rc<RefCell<ControlFlowBlock>>,
    },
    Unconditional {
        target: Rc<RefCell<ControlFlowBlock>>,
    },
}

/// Map from unresolved to resolved control flow blocks
#[derive(Default)]
struct BlockMap(HashMap<BlockKey, Rc<RefCell<ControlFlowBlock>>>);

impl BlockMap {
    fn new() -> Self {
        Self::default()
    }

    fn get(
        &self,
        unresolved: Rc<RefCell<UnresolvedControlFlowBlock>>,
    ) -> Option<Rc<RefCell<ControlFlowBlock>>> {
        self.0.get(&BlockKey(unresolved)).cloned()
    }

    fn insert(
        &mut self,
        unresolved: Rc<RefCell<UnresolvedControlFlowBlock>>,
        resolved: Rc<RefCell<ControlFlowBlock>>,
    ) {
        if let Some(block) = self
            .0
            .insert(BlockKey(unresolved.clone()), resolved.clone())
        {
            panic!("unresolved control flow block {unresolved:?} already resolved {block:?} when inserting {resolved:?}")
        }
    }
}

#[derive(Debug)]
struct BlockKey(Rc<RefCell<UnresolvedControlFlowBlock>>);

impl Hash for BlockKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0.as_ptr(), state)
    }
}

impl PartialEq for BlockKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for BlockKey {}

impl Display for BlockKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        write!(f, "{:016X}", state.finish())
    }
}

#[cfg(test)]
mod test {
    use std::hash::Hasher;

    use {
        super::{BlockKey, UnresolvedControlFlowBlock},
        std::{collections::hash_map::DefaultHasher, hash::Hash},
    };

    #[test]
    fn not_equal() {
        let a = BlockKey(UnresolvedControlFlowBlock::new());
        let b = BlockKey(UnresolvedControlFlowBlock::new());

        assert_ne!(a, b);

        let state = DefaultHasher::new();

        let a_hash = {
            let mut state = state.clone();
            a.hash(&mut state);
            state.finish()
        };

        let b_hash = {
            let mut state = state.clone();
            b.hash(&mut state);
            state.finish()
        };

        assert_ne!(a_hash, b_hash);
    }

    #[test]
    fn equal() {
        let block = UnresolvedControlFlowBlock::new();
        let a = BlockKey(block.clone());
        let b = BlockKey(block);

        assert_eq!(a, b);

        let state = DefaultHasher::new();

        let a_hash = {
            let mut state = state.clone();
            a.hash(&mut state);
            state.finish()
        };

        let b_hash = {
            let mut state = state.clone();
            b.hash(&mut state);
            state.finish()
        };

        assert_eq!(a_hash, b_hash);
    }
}
