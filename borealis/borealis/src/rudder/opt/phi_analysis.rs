use {
    crate::rudder::{analysis::loopy::LoopAnalysis, Block, Function, Statement, StatementKind},
    common::{intern::InternedString, HashMap},
    log::trace,
};

pub fn run(f: Function) -> bool {
    let la = LoopAnalysis::new(&f);

    // Cannot run on functions containing loops.
    if la.contains_loop() {
        return false;
    }

    // Compute dominance graph

    // Compute live outs
    let mut live_outs: HashMap<Block, HashMap<InternedString, Statement>> = HashMap::default();

    for block in f.entry_block().iter() {
        for stmt in block.statements() {
            if let StatementKind::WriteVariable { symbol, .. } = stmt.kind() {
                live_outs
                    .entry(block.clone())
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
            trace!("    write: {} = {}", symbol, write);
        }
    }

    false
}
