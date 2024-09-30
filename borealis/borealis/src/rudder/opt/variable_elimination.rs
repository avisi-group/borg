use {
    crate::rudder::analysis,
    common::rudder::{block::Block, function::Function, statement::Statement},
    common::{
        arena::{Arena, Ref},
        id::Id,
    },
    common::{Entry, HashMap, HashSet},
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let symbol_ua = analysis::dfa::SymbolUseAnalysis::new(f);

    let mut changed = false;

    trace!("running on function {}", f.name());
    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(&symbol_ua, f.arena_mut(), block);
    }

    changed
}

fn run_on_block(
    symbol_ua: &analysis::dfa::SymbolUseAnalysis,
    arena: &mut Arena<Block>,
    block: Ref<Block>,
) -> bool {
    // collapse multiple reads
    //
    // 1: write-var SYM
    // 2: read-var SYM
    // 3: read-var SYM
    //
    // Into
    //
    // 1: write-var SYM
    // 2: read-var SYM
    // 3: <kill>, replace 3 with 2

    // if we see a write to a local symbol, then all reads until the next write can
    // be replaced.

    let mut stmt_ua = analysis::dfa::StatementUseAnalysis::new(arena, block);

    let mut live_writes = HashMap::default();

    let mut changed = false;
    for stmt in block
        .get(stmt_ua.block_arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        if let Statement::WriteVariable { symbol, value } =
            stmt.get(block.get(stmt_ua.block_arena()).arena())
        {
            // Ignore global symbols (for now)
            if !symbol_ua.is_symbol_local(&symbol) {
                continue;
            }

            trace!("considering variable write to {}", symbol.name());
            match live_writes.entry(symbol.name()) {
                Entry::Occupied(mut e) => {
                    trace!(
                        "already live write to symbol {}, updating live value",
                        symbol.name(),
                    );
                    e.insert(value.clone());
                }
                Entry::Vacant(e) => {
                    trace!("starting live range for symbol {}", symbol.name(),);
                    e.insert(value.clone());
                }
            }
        } else if let Statement::ReadVariable { symbol } =
            stmt.get(block.get(stmt_ua.block_arena()).arena()).clone()
        {
            if !symbol_ua.is_symbol_local(&symbol) {
                continue;
            }

            trace!("considering variable read from {}", symbol.name());
            let Some(live_value) = live_writes.get(&(symbol.name())) else {
                trace!("no live range for read of symbol");
                continue;
            };

            if stmt_ua.is_dead(stmt) {
                trace!("read is dead -- will be collected later");
                continue;
            }

            // replace uses of read with live value
            if let Some(uses) = stmt_ua.get_uses(stmt).cloned() {
                for u in uses {
                    let arena = stmt_ua.block_arena();
                    trace!("replacing use in {}", u.to_string(block.get(arena).arena()));

                    u.get_mut(block.get_mut(arena).arena_mut())
                        .replace_use(stmt.clone(), live_value.clone());
                    changed = true;
                }
            }
        }
    }

    changed
}
