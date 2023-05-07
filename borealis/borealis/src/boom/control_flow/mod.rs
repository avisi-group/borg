//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by block targets being unresolved during building (IE. visiting a jump before the label it references is created):
//!
//! 1. Two sets of types, internal maybe-unresolved and public resolved to enforce resolution at type level.
//! 2. Recursive resolution to convert maybe-unresolved to resolved blocks.

use {
    crate::boom::{control_flow::builder::ControlFlowGraphBuilder, Statement, Value},
    common::intern::InternedString,
    std::{
        cell::RefCell,
        collections::hash_map::DefaultHasher,
        fmt::Display,
        fmt::{self, Formatter},
        hash::{Hash, Hasher},
        io::{self, Write},
        ptr,
        rc::{Rc, Weak},
    },
};

mod builder;
mod dot;

/// Control flow graph of a BOOM function
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    /// Function entrypoint
    pub entry_block: ControlFlowBlock,
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
pub struct ControlFlowBlock(Rc<RefCell<ControlFlowBlockInner>>);

#[derive(Debug)]
struct ControlFlowBlockInner {
    /// Optional block label, otherwise the `SharedKey` `Display` format should be used
    label: Option<InternedString>,
    /// Parents of the current node
    parents: Vec<ControlFlowBlockWeak>,
    /// Sequence of statements within the block
    statements: Vec<Rc<RefCell<Statement>>>,
    /// Block terminator
    terminator: Terminator,
}

impl Hash for ControlFlowBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0.as_ptr(), state)
    }
}

impl PartialEq for ControlFlowBlock {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowBlock {}

impl ControlFlowBlock {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(ControlFlowBlockInner {
            label: None,
            parents: vec![],
            statements: vec![],
            terminator: Terminator::Return,
        })))
    }

    pub fn downgrade(&self) -> ControlFlowBlockWeak {
        ControlFlowBlockWeak(Rc::downgrade(&self.0))
    }

    pub fn label(&self) -> Option<InternedString> {
        self.0.borrow().label
    }

    pub fn set_label(&self, label: Option<InternedString>) {
        self.0.borrow_mut().label = label;
    }

    /// Gets the parents of this control flow block
    pub fn parents(&self) -> Vec<ControlFlowBlock> {
        self.0
            .borrow()
            .parents
            .iter()
            .filter_map(ControlFlowBlockWeak::upgrade)
            .collect()
    }

    /// Adds a parent to this control flow block
    pub fn add_parent(&self, parent: &Self) {
        let parents = &mut self.0.borrow_mut().parents;

        // remove dropped weak pointers
        parents.retain(|weak| weak.upgrade().is_some());

        // add new weak pointer
        parents.push(parent.downgrade());
    }

    /// Gets the terminator of this control flow block
    pub fn terminator(&self) -> Terminator {
        self.0.borrow().terminator.clone()
    }

    /// Sets the terminator of this control flow block (and also the weak parental references of any children)
    pub fn set_terminator(&self, terminator: Terminator) {
        match &terminator {
            Terminator::Return | Terminator::Undefined => (),
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => {
                target.add_parent(self);
                fallthrough.add_parent(self);
            }
            Terminator::Unconditional { target } => target.add_parent(self),
        }

        self.0.borrow_mut().terminator = terminator;
    }

    pub fn statements(&self) -> Vec<Rc<RefCell<Statement>>> {
        self.0.borrow().statements.clone()
    }

    pub fn set_statements(&self, statements: Vec<Rc<RefCell<Statement>>>) {
        self.0.borrow_mut().statements = statements;
    }
}

impl Display for ControlFlowBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0.borrow().label {
            Some(label) => write!(f, "{label}"),
            None => {
                let mut state = DefaultHasher::new();
                self.hash(&mut state);
                write!(f, "{:016X}", state.finish())
            }
        }
    }
}

#[derive(Debug)]
pub struct ControlFlowBlockWeak(Weak<RefCell<ControlFlowBlockInner>>);

impl ControlFlowBlockWeak {
    pub fn upgrade(&self) -> Option<ControlFlowBlock> {
        self.0.upgrade().map(ControlFlowBlock)
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
        target: ControlFlowBlock,
        fallthrough: ControlFlowBlock,
    },
    /// Unconditionally jump to target
    Unconditional {
        target: ControlFlowBlock,
    },
    Undefined,
}
