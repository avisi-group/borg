use {
    crate::rudder::{analysis::dfa::StatementUseAnalysis, opt::OptimizationContext},
    common::{
        arena::{Arena, Ref},
        rudder::{block::Block, function::Function},
    },
    log::trace,
};

pub fn run(ctx: &OptimizationContext, f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(ctx, f.arena_mut(), block);
    }

    changed
}

fn run_on_block(ctx: &OptimizationContext, arena: &mut Arena<Block>, b: Ref<Block>) -> bool {
    let mut sua = StatementUseAnalysis::new(arena, b, &ctx.purity);

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
