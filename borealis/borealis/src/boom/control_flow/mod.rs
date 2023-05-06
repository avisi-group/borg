//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by block targets being unresolved during building (IE. visiting a jump before the label it references is created):
//!
//! 1. Two sets of types, internal maybe-unresolved and public resolved to enforce resolution at type level.
//! 2. Recursive resolution to convert maybe-unresolved to resolved blocks.

use {
    crate::boom::{control_flow::builder::ControlFlowGraphBuilder, Statement, Value},
    common::{intern::InternedString, shared_key::SharedKey},
    std::{
        cell::RefCell,
        io::{self, Write},
        rc::{Rc, Weak},
    },
};

mod builder;
mod dot;

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
        let entry_block = ControlFlowGraphBuilder::from_statements(statements);

        Self { entry_block }
    }

    /// Renders a `ControlFlowGraph` to DOT syntax.
    pub fn as_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        dot::render(w, &self.entry_block)
    }
}

/// Node in a control flow graph, contains a basic block of statements and a terminator
#[derive(Debug, Clone)]
pub struct ControlFlowBlock {
    /// Optional block label, otherwise the `SharedKey` `Display` format should be used
    label: Option<InternedString>,
    /// Parents of the current node
    parents: Vec<Weak<RefCell<ControlFlowBlock>>>,
    /// Sequence of statements within the block
    pub statements: Vec<Rc<RefCell<Statement>>>,
    /// Block terminator
    terminator: Terminator,
}

impl ControlFlowBlock {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            label: None,
            parents: vec![],
            statements: vec![],
            terminator: Terminator::Return,
        }))
    }

    pub fn label(block: Rc<RefCell<Self>>) -> String {
        block
            .borrow()
            .label
            .map(|label| label.to_string())
            .unwrap_or(SharedKey::from(block.clone()).to_string())
    }

    /// Gets the parents of this control flow block
    pub fn parents(&self) -> impl Iterator<Item = Rc<RefCell<Self>>> + '_ {
        self.parents.iter().filter_map(Weak::upgrade)
    }

    /// Adds a parent to this control flow block
    pub fn add_parent(&mut self, parent: &Rc<RefCell<Self>>) {
        // remove dropped weak pointers
        self.parents.retain(|weak| weak.upgrade().is_some());

        // add new weak pointer
        self.parents.push(Rc::downgrade(&parent));
    }

    /// Gets the terminator of this control flow block
    pub fn terminator(&self) -> &Terminator {
        &self.terminator
    }

    /// Sets the terminator of this control flow block (and also the weak parental references of any children)
    pub fn set_terminator(block: &Rc<RefCell<Self>>, terminator: Terminator) {
        match &terminator {
            Terminator::Return | Terminator::Undefined => (),
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => {
                target.borrow_mut().add_parent(block);
                fallthrough.borrow_mut().add_parent(block);
            }
            Terminator::Unconditional { target } => target.borrow_mut().add_parent(block),
        }

        block.borrow_mut().terminator = terminator;
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
        condition: Value,
        target: Rc<RefCell<ControlFlowBlock>>,
        fallthrough: Rc<RefCell<ControlFlowBlock>>,
    },
    /// Unconditionally jump to target
    Unconditional {
        target: Rc<RefCell<ControlFlowBlock>>,
    },
    Undefined,
}
