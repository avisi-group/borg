use {
    crate::{
        rudder::{
            statement::{build, import_statement, Statement, StatementKind},
            Block, Function,
        },
        util::arena::{Arena, Ref},
    },
    common::HashMap,
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
    let terminator = source_block
        .get(f.block_arena())
        .terminator_statement()
        .unwrap();

    let StatementKind::Jump {
        target: target_block,
    } = terminator
        .get(&source_block.get(f.block_arena()).statement_arena)
        .kind()
        .clone()
    else {
        return false;
    };

    if target_block.get(f.block_arena()).size() > INLINE_SIZE_THRESHOLD {
        return false;
    }

    // kill the jump statement, copy target block statements in.
    source_block
        .get_mut(f.block_arena_mut())
        .kill_statement(terminator);

    let mut mapping = HashMap::default();

    for target_statement in target_block.get(f.block_arena()).statements() {
        let source_statement = import_statement(
            source_block,
            target_block,
            f.block_arena_mut(),
            target_statement,
            &mapping,
        );

        mapping.insert(target_statement, source_statement);
    }

    true
}
