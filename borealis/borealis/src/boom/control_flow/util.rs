//! General utility methods on ControlFlowBlocks

use {
    crate::boom::{
        control_flow::{dot, ControlFlowBlock},
        Expression, Statement, Value,
    },
    common::{intern::InternedString, HashSet},
    itertools::Itertools,
    log::trace,
    std::{
        cell::RefCell,
        collections::LinkedList,
        io::{self, Write},
        rc::Rc,
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

    pub fn get_assignment(&self, ident: InternedString) -> Option<Rc<RefCell<Value>>> {
        self.iter()
            .flat_map(|cfb| cfb.statements())
            .filter_map(|statement| {
                let res = {
                    let borrow = statement.borrow();
                    match &*borrow {
                        Statement::Copy { expression, value } => {
                            Some((expression.clone(), value.clone()))
                        }
                        _ => None,
                    }
                };

                res
            })
            .filter_map(|(expr, value)| {
                let Expression::Identifier(assign) = expr else {
                    return None;
                };

                if assign == ident {
                    Some(value)
                } else {
                    None
                }
            })
            .at_most_one()
            .unwrap_or_else(|e| panic!("Multiple assignments to {ident} found: {e}"))
    }

    /// Finds the block and index of a statement in a control flow graph
    pub fn find_statement(
        &self,
        target: Rc<RefCell<Statement>>,
    ) -> Option<(ControlFlowBlock, usize)> {
        self.iter()
            .map(|block| (block.clone(), block.statements()))
            .map(|(block, statements)| {
                (
                    block,
                    statements
                        .iter()
                        .position(|statement| Rc::ptr_eq(statement, &target)),
                )
            })
            .filter_map(|(block, index)| index.map(|index| (block, index)))
            .at_most_one()
            // this is technically possible, should probably be handled properly if it does occur
            .expect("found multiple statements matching target")
    }
}
