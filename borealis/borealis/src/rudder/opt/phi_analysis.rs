use {
    crate::rudder::analysis::loopy::LoopAnalysis,
    common::{
        arena::{Arena, Ref},
        id::Id,
    },
    common::{
        intern::InternedString,
        rudder::{
            statement::Statement,
            {block::Block, function::Function},
        },
    },
    common::{HashMap, HashSet},
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let la = LoopAnalysis::new(&f);

    // Cannot run on functions containing loops.
    if la.contains_loop() {
        return false;
    }

    // Compute dominance graph

    // Compute live outs
    let mut live_outs: HashMap<Ref<Block>, HashMap<InternedString, Ref<Statement>>> =
        HashMap::default();

    for block in f.block_iter() {
        for stmt in block.get(f.arena()).statements() {
            if let Statement::WriteVariable { symbol, .. } = stmt.get(block.get(f.arena()).arena())
            {
                live_outs
                    .entry(block)
                    .and_modify(|e| {
                        e.insert(symbol.name(), stmt.clone());
                    })
                    .or_insert({
                        let mut writes = HashMap::default();
                        writes.insert(symbol.name(), stmt.clone());

                        writes
                    });
            }
        }
    }

    // Ignore no live outs, or live outs when there's only one block.
    if live_outs.len() < 2 {
        return false;
    }

    trace!("live-outs in {}:", f.name());
    for (block, writes) in live_outs {
        trace!("  block {}:", block.index());
        for (symbol, write) in writes {
            let arena = block.get(f.arena()).arena();

            trace!("    write: {} = {}", symbol, write.to_string(arena));
        }
    }

    false
}
