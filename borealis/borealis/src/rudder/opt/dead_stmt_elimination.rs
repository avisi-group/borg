use {
    crate::rudder::analysis::dfa::StatementUseAnalysis,
    common::rudder::{block::Block, function::Function},
    common::{
        arena::{Arena, Ref},
        id::Id,
    },
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(f.arena_mut(), block);
    }

    changed
}

fn run_on_block(arena: &mut Arena<Block>, b: Ref<Block>) -> bool {
    let mut sua = StatementUseAnalysis::new(arena, b);

    for stmt in b
        .get(sua.block_arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        if sua.is_dead(stmt) {
            let s_arena = b.get(arena).arena();
            trace!("killing dead statement: {}", stmt.to_string(s_arena));
            b.get_mut(arena).kill_statement(stmt);
            return true;
        }
    }

    false
}
