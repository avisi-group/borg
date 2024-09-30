use {
    crate::{
        rudder::{
            model::statement::{import_statement, Statement},
            model::{block::Block, function::Function},
        },
        util::arena::Ref,
    },
    common::{HashMap, HashSet},
};

const INLINE_SIZE_THRESHOLD: usize = 5;

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_inliner_block(f, block);
    }

    changed
}

fn run_inliner_block(f: &mut Function, source_block: Ref<Block>) -> bool {
    // if a block ends in a jump statement, and the target block is "small", inline
    // it.
    let terminator = source_block.get(f.arena()).terminator_statement().unwrap();

    let Statement::Jump {
        target: target_block,
    } = terminator.get(source_block.get(f.arena()).arena()).clone()
    else {
        return false;
    };

    if target_block.get(f.arena()).size() > INLINE_SIZE_THRESHOLD
        || target_block // don't inline return statements, we only want one per function when inlining functions
            .get(f.arena())
            .terminator_statement()
            .is_some_and(|r| {
                matches!(
                    r.get(target_block.get(f.arena()).arena()),
                    Statement::Return { .. }
                )
            })
    {
        return false;
    }

    // kill the jump statement, copy target block statements in.
    source_block
        .get_mut(f.arena_mut())
        .kill_statement(terminator);

    let mut mapping = HashMap::default();

    for target_statement in target_block
        .get(f.arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        let source_statement = import_statement(
            source_block,
            target_block,
            f.arena_mut(),
            target_statement,
            &mapping,
        );

        mapping.insert(target_statement, source_statement);
    }

    true
}
