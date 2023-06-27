//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by
//! block targets being unresolved during building (IE. visiting a jump before
//! the label it references is created):
//!
//! 1. Two sets of types, internal maybe-unresolved and public resolved to
//! enforce resolution at type level. 2. Recursive resolution to convert
//! maybe-unresolved to resolved blocks.

use {
    crate::boom::{control_flow::builder::ControlFlowGraphBuilder, Statement, Value},
    common::intern::InternedString,
    log::trace,
    std::{
        cell::RefCell,
        collections::{hash_map::DefaultHasher, HashSet, LinkedList},
        fmt::{self, Display, Formatter},
        hash::{Hash, Hasher},
        io::{self, Write},
        ptr,
        rc::{Rc, Weak},
    },
};

mod builder;
mod dot;

/// Creates a control flow graph from a slice of statements.
///
/// This should be called per BOOM function.
pub fn build_graph(statements: &[Rc<RefCell<Statement>>]) -> ControlFlowBlock {
    ControlFlowGraphBuilder::from_statements(statements)
}

/// Node in a control flow graph, contains a basic block of statements and a
/// terminator
#[derive(Debug, Clone)]
pub struct ControlFlowBlock {
    inner: Rc<RefCell<ControlFlowBlockInner>>,
}

#[derive(Debug)]
struct ControlFlowBlockInner {
    /// Optional block label, otherwise the `SharedKey` `Display` format should
    /// be used
    label: Option<InternedString>,
    /// Parents of the current node
    parents: HashSet<ControlFlowBlockWeak>,
    /// Sequence of statements within the block
    statements: Vec<Rc<RefCell<Statement>>>,
    /// Block terminator
    terminator: Terminator,
}

impl Hash for ControlFlowBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.inner.as_ptr(), state)
    }
}

impl PartialEq for ControlFlowBlock {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for ControlFlowBlock {}

impl Display for ControlFlowBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.inner.borrow().label {
            Some(label) => write!(f, "{label}"),
            None => {
                let mut state = DefaultHasher::new();
                self.hash(&mut state);
                write!(f, "{:016X}", state.finish())
            }
        }
    }
}

impl ControlFlowBlock {
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ControlFlowBlockInner {
                label: None,
                parents: HashSet::new(),
                statements: vec![],
                terminator: Terminator::Return,
            })),
        }
    }

    /// Creates a new weak pointer to a `ControlFlowBlock`
    pub fn downgrade(&self) -> ControlFlowBlockWeak {
        ControlFlowBlockWeak(Rc::downgrade(&self.inner))
    }

    /// Gets the label of a `ControlFlowBlock`
    pub fn label(&self) -> Option<InternedString> {
        self.inner.borrow().label
    }

    /// Sets the label of a `ControlFlowBlock`
    pub fn set_label(&self, label: Option<InternedString>) {
        self.inner.borrow_mut().label = label;
    }

    /// Gets the parents of this control flow block
    pub fn parents(&self) -> Vec<ControlFlowBlock> {
        self.inner
            .borrow()
            .parents
            .iter()
            .filter_map(ControlFlowBlockWeak::upgrade)
            .collect()
    }

    /// Adds a parent to this control flow block
    pub fn add_parent(&self, parent: &Self) {
        let parents = &mut self.inner.borrow_mut().parents;

        // remove dropped weak pointers
        parents.retain(|weak| weak.upgrade().is_some());

        // add new weak pointer
        parents.insert(parent.downgrade());
    }

    fn remove_parent(&self, parent_to_remove: &Self) {
        let parent_to_remove = parent_to_remove.downgrade();
        let parents = &mut self.inner.borrow_mut().parents;

        assert!(parents.contains(&parent_to_remove));

        parents.retain(|p| *p != parent_to_remove);
    }

    /// Gets the terminator of this control flow block
    pub fn terminator(&self) -> Terminator {
        self.inner.borrow().terminator.clone()
    }

    /// Sets the terminator of this control flow block (and also the weak
    /// parental references of any children)
    pub fn set_terminator(&self, terminator: Terminator) {
        // remove ourselves as a parent from all children
        self.terminator()
            .targets()
            .iter()
            .for_each(|child| child.remove_parent(self));

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

        self.inner.borrow_mut().terminator = terminator;
    }

    /// Gets the statements of a `ControlFlowBlock`
    pub fn statements(&self) -> Vec<Rc<RefCell<Statement>>> {
        self.inner.borrow().statements.clone()
    }

    /// Sets the statements of a `ControlFlowBlock`
    pub fn set_statements(&self, statements: Vec<Rc<RefCell<Statement>>>) {
        self.inner.borrow_mut().statements = statements;
    }

    /// Renders a `ControlFlowBlock` to DOT syntax.
    pub fn as_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        dot::render(w, self)
    }

    /// Determines whether the current `ControlFlowBlock` and it's children
    /// contain any cycles
    pub fn contains_cycles(&self) -> bool {
        let mut processed = HashSet::new();
        let mut currently_processing = HashSet::new();
        let mut to_visit = LinkedList::new();
        to_visit.push_back(self.clone());

        // continue until all no nodes are left to visit
        while let Some(current) = to_visit.pop_front() {
            trace!("processing {}", current);

            // continue if we have already processed the current node
            if processed.contains(&current) {
                continue;
            }

            processed.insert(current.clone());
            currently_processing.insert(current.clone());

            // Visit children of the current node
            for child in current.terminator().targets() {
                if currently_processing.contains(&child) {
                    // Found a cycle!
                    return true;
                }
                to_visit.push_back(child);
            }

            currently_processing.remove(&current);
        }

        false
    }
}

/// Non-owning reference to a `ControlFlowBlock`
#[derive(Debug)]
pub struct ControlFlowBlockWeak(Weak<RefCell<ControlFlowBlockInner>>);

impl ControlFlowBlockWeak {
    /// Attempts to upgrade to a strong reference to a `ControlFlowBlock`
    pub fn upgrade(&self) -> Option<ControlFlowBlock> {
        self.0.upgrade().map(|inner| ControlFlowBlock { inner })
    }
}

impl Hash for ControlFlowBlockWeak {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0.as_ptr(), state)
    }
}

impl PartialEq for ControlFlowBlockWeak {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowBlockWeak {}

/// Block terminator
///
/// Describes how one block conditionally or unconditionally jumps to the next
#[derive(Debug, Clone)]
pub enum Terminator {
    /// Function return
    Return,
    /// If condition evaluates to true, then jump to target, otherwise jump to
    /// fallthrough
    Conditional {
        /// Condition
        condition: Value,
        /// Target if condition is `true`
        target: ControlFlowBlock,
        /// Fallthrough if condition is `false`
        fallthrough: ControlFlowBlock,
    },
    /// Unconditionally jump to target
    Unconditional {
        /// Target to branch to
        target: ControlFlowBlock,
    },
    /// Undefined terminator, distinct from the internal `Unknown` variant.
    Undefined,
}

impl Terminator {
    /// Gets the targets of a terminator
    pub fn targets(&self) -> Vec<ControlFlowBlock> {
        match self {
            Terminator::Return | Terminator::Undefined => vec![],
            Terminator::Conditional {
                target,
                fallthrough,
                ..
            } => vec![target.clone(), fallthrough.clone()],
            Terminator::Unconditional { target } => vec![target.clone()],
        }
    }
}
