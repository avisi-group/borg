use {
    crate::rudder::{analysis, Block, Function, StatementKind, SymbolKind},
    common::HashMap,
    log::trace,
    std::collections::hash_map::Entry,
};

pub fn run(f: Function) -> bool {
    let symbol_ua = analysis::dfa::SymbolUseAnalysis::new(&f);

    let mut changed = false;

    trace!("running on function {}", f.name());
    for block in f.entry_block().iter() {
        changed |= run_on_block(&symbol_ua, block);
    }

    changed
}

fn run_on_block(symbol_ua: &analysis::dfa::SymbolUseAnalysis, block: Block) -> bool {
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

    let stmt_ua = analysis::dfa::StatementUseAnalysis::new(&block);

    let mut live_writes = HashMap::default();

    let mut changed = false;
    for stmt in block.statements() {
        // todo: ask tom about fields
        if let StatementKind::WriteVariable {
            symbol,
            value,
            indices,
        } = stmt.kind()
        {
            // Ignore global symbols (for now)
            if symbol.kind() == SymbolKind::Parameter || !symbol_ua.is_symbol_local(&symbol) {
                continue;
            }

            trace!("considering variable write to {}", symbol.name());
            match live_writes.entry((symbol.name(), indices.clone())) {
                Entry::Occupied(mut e) => {
                    trace!(
                        "already live write to symbol {}:{:?}, updating live value",
                        symbol.name(),
                        &indices
                    );
                    e.insert(value.clone());
                }
                Entry::Vacant(e) => {
                    trace!(
                        "starting live range for symbol {}:{:?}",
                        symbol.name(),
                        &indices
                    );
                    e.insert(value.clone());
                }
            }
        } else if let StatementKind::ReadVariable { symbol, indices } = stmt.kind() {
            if symbol.kind() == SymbolKind::Parameter || !symbol_ua.is_symbol_local(&symbol) {
                continue;
            }

            trace!("considering variable read from {}", symbol.name());
            let Some(live_value) = live_writes.get(&(symbol.name(), indices)) else {
                trace!("no live range for read of symbol");
                continue;
            };

            if stmt_ua.is_dead(&stmt) {
                trace!("read is dead -- will be collected later");
                continue;
            }

            // replace uses of read with live value
            for use_ in stmt_ua.get_uses(&stmt) {
                trace!("replacing use in {}", use_);

                use_.replace_use(stmt.clone(), live_value.clone());
                changed = true;
            }
        }
    }

    changed
}
