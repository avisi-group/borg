use {
    crate::{
        rudder::{analysis::loopy::LoopAnalysis, statement::Statement, Block, Function, StatementKind},
        util::arena::Ref,
    },
    common::{intern::InternedString, HashMap},
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
    let mut live_outs: HashMap<Ref<Block>, HashMap<InternedString, Ref<Statement>>> = HashMap::default();

    for block in f.block_iter() {
        for stmt in block.get(f.block_arena()).statements() {
            if let StatementKind::WriteVariable { symbol, .. } = stmt.get(block.get(f.block_arena()).arena()).kind() {
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
        trace!("  block {}:", block.get(f.block_arena()).index());
        for (symbol, write) in writes {
            let arena = block.get(f.block_arena()).arena();

            trace!("    write: {} = {}", symbol, write.get(arena).to_string(arena));
        }
    }

    false
}
