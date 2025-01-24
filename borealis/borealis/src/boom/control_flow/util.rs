//! General utility methods on ControlFlowBlocks

use common::boom::control_flow::ControlFlowBlock;

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
