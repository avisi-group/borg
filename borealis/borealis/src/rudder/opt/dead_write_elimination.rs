use log::trace;

use crate::rudder::analysis;

pub fn run(f: crate::rudder::Function) -> bool {
    let dfa = analysis::dfa::SymbolUseAnalysis::new(&f);

    let mut changed = false;

    for sym in f.local_variables() {
        if !dfa.symbol_has_reads(&sym) {
            trace!("no reads for symbol {}", sym.name());

            if !dfa.symbol_has_writes(&sym) {
                trace!("no writes to symbol {}", sym.name());
                continue;
            }

            for write in dfa.get_symbol_writes(&sym) {
                write.parent().upgrade().kill_statement(write);
                changed |= true;
            }
        }
    }

    changed
}
