use {
    crate::{
        rudder::{analysis::dfa::StatementUseAnalysis, Block, Function},
        util::arena::{Arena, Ref},
    },
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(f.block_arena_mut(), block);
    }

    changed
}

fn run_on_block(arena: &mut Arena<Block>, b: Ref<Block>) -> bool {
    let mut sua = StatementUseAnalysis::new(arena, b);

    for stmt in b.get(sua.block_arena()).statements() {
        if sua.is_dead(stmt) {
            let s_arena = &b.get(arena).statement_arena;
            trace!("killing dead statement: {}", stmt.get(s_arena).to_string(s_arena));
            b.get_mut(arena).kill_statement(stmt);
            return true;
        }
    }

    false
}
