use log::trace;

use crate::rudder::{analysis, Block, StatementKind};

pub fn run(f: crate::rudder::Function) -> bool {
    let dfa = analysis::dfa::SymbolUseAnalysis::new(f.clone());

    let mut changed = false;

    for block in f.entry_block().iter() {
        changed |= run_on_block(&dfa, block);
    }

    changed
}

fn run_on_block(dfa: &analysis::dfa::SymbolUseAnalysis, block: Block) -> bool {
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

    for stmt in block.statements() {
        if let StatementKind::WriteVariable { symbol, value } = stmt.kind() {
            trace!("considering variable write to {}", symbol.name());
            if dfa.is_symbol_local(&symbol) {
                trace!("write to local symbol {}", symbol.name());
            }
        }
    }

    false
}
