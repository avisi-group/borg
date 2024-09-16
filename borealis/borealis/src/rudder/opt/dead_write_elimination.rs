use {
    crate::rudder::{analysis, Function},
    log::trace,
};

pub fn run(f: &mut Function) -> bool {
    let dfa = analysis::dfa::SymbolUseAnalysis::new(f);

    let mut changed = false;

    for sym in f.local_variables() {
        if sym.name().as_ref() == "return" {
            continue;
        };
        if !dfa.symbol_has_reads(&sym) {
            trace!("no reads for symbol {}", sym.name());

            if !dfa.symbol_has_writes(&sym) {
                trace!("no writes to symbol {}", sym.name());
                continue;
            }

            for write in dfa.get_symbol_writes(&sym) {
                write
                    .parent_block()
                    .get_mut(f.block_arena_mut())
                    .kill_statement(write);
                changed |= true;
            }
        }
    }

    changed
}
