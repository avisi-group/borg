use {
    crate::{
        rudder::{analysis::dfa::StatementUseAnalysis, Block, Function},
        util::arena::{Arena, Ref},
    },
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter() {
        changed |= run_on_block(f.block_arena(), block);
    }

    changed
}

fn run_on_block(arena: &Arena<Block>, b: Ref<Block>) -> bool {
    let sua = StatementUseAnalysis::new(arena, b);

    for stmt in b.get(arena).statements() {
        if sua.is_dead(&stmt) {
            trace!("killing dead statement: {}", stmt);
            b.get(arena).kill_statement(&stmt);
            return true;
        }
    }

    false
}
