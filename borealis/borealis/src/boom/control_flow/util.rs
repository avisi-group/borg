//! General utility methods on ControlFlowBlocks

use {
    crate::boom::{
        control_flow::{dot, ControlFlowBlock},
        Expression, Statement, Value,
    },
    sailrs::{intern::InternedString, shared::Shared, HashSet},
    itertools::Itertools,
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
            if processed.contains(&current.id()) {
                continue;
            }

            processed.insert(current.id());
            currently_processing.insert(current.id());

            // Visit children of the current node
            for child in current.terminator().targets() {
                if currently_processing.contains(&child.id()) {
                    // Found a cycle!
                    return true;
                }
                to_visit.push_back(child);
            }

            currently_processing.remove(&current.id());
        }

        false
    }

    /// Returns a set of identifiers of all functions called in this block and
    /// its children.
    pub fn get_functions(&self) -> HashSet<InternedString> {
        self.iter()
            .flat_map(|block| block.statements())
            .filter_map(|statement| {
                if let Statement::FunctionCall { name, .. } = *statement.get() {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_assignment(&self, ident: InternedString) -> Option<Shared<Value>> {
        self.iter()
            .flat_map(|cfb| cfb.statements())
            .filter_map(|statement| {
                let res = {
                    let borrow = statement.get();
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
    pub fn find_statement(&self, target: Shared<Statement>) -> Option<(ControlFlowBlock, usize)> {
        self.iter()
            .map(|block| (block.clone(), block.statements()))
            .map(|(block, statements)| {
                (
                    block,
                    statements
                        .iter()
                        .position(|statement| Shared::ptr_eq(statement, &target)),
                )
            })
            .filter_map(|(block, index)| index.map(|index| (block, index)))
            .at_most_one()
            // this is technically possible, should probably be handled properly if it does occur
            .expect("found multiple statements matching target")
    }
}

/// Finds and returns the first common child block of blocks `left` and `right`,
/// if it exists.
///
/// Left and right don't actually mean anything, just used to distinguish
/// between blocks, and a common child is a block in which all paths from `left`
/// and `right` go through.
///
/// When emitting BOOM, we could recurse into each sub-tree when emitting
/// if-statements. This is *correct* but results in an explosion, all blocks in
/// each left and right body after the if-statements original scope are
/// duplicated, and this occurs for every if-statement.
///
/// Instead, if we can find the point at which the branches re-join, we can emit
/// only the minimum number of statements in the left and right bodies of the
/// if-statement, and return one indendation level. This avoids the duplication
/// explosion.
///
/// However, finding the block where branches re-join is non-trivial. We need to
/// find all paths from the left and right blocks, then find the first (closest)
/// block which appears in all such paths. Finding all paths first quickly
/// exhausts available memory, so the current implementation iteratively grows
/// all paths in lock-step, searching for common blocks each time.
///
/// The problem is that it still consumes too much memory on larger graphs. At
/// the time of writing, this was solved by culling paths which re-join. I
/// thought about it for a while and I think it's correct, but who knows?
///
/// Update 2023-09-18: Spoke with Al, only need to walk left and right paths
/// once to find rejoining block
///
/// :(
pub fn find_common_block(
    left: ControlFlowBlock,
    right: ControlFlowBlock,
) -> Option<ControlFlowBlock> {
    log::trace!("finding common block of {left} and {right}");

    let left_path = walk(left);
    let right_path = walk(right);

    let result = left_path.into_iter().find(|left_block| {
        right_path
            .iter()
            .any(|right_block| left_block.id() == right_block.id())
    });

    if let Some(common) = &result {
        log::trace!("found common block {}", common);
    } else {
        log::trace!("found no common block");
    }

    result
}

/// Walks the graph, always taking the left child
fn walk(start: ControlFlowBlock) -> Vec<ControlFlowBlock> {
    let mut path = vec![start];

    loop {
        let current = path.last().unwrap();

        match &current.terminator().targets()[..] {
            [] => return path,
            [next] | [next, ..] => path.push(next.clone()),
        }
    }
}
