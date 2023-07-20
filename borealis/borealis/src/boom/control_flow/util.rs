//! General utility methods on ControlFlowBlocks

use {
    crate::boom::{
        control_flow::{dot, ControlFlowBlock},
        Statement,
    },
    common::{intern::InternedString, HashSet},
    log::trace,
    std::{
        collections::LinkedList,
        io::{self, Write},
    },
};

impl ControlFlowBlock {
    /// Renders a `ControlFlowBlock` to DOT syntax.
    pub fn as_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        dot::render(w, self)
    }

    /// Determines whether the current `ControlFlowBlock` and it's children
    /// contain any cycles
    pub fn contains_cycles(&self) -> bool {
        let mut processed = HashSet::default();
        let mut currently_processing = HashSet::default();
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

    /// Returns a set of identifiers of all functions called in this block and
    /// its children.
    pub fn get_functions(&self) -> HashSet<InternedString> {
        self.iter()
            .flat_map(|block| block.statements())
            .filter_map(|statement| {
                if let Statement::FunctionCall { name, .. } = *statement.borrow() {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }
}
