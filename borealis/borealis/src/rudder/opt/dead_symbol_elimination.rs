use log::trace;

use crate::rudder::analysis;

pub fn run(f: crate::rudder::Function) -> bool {
    let dfa = analysis::dfa::SymbolUseAnalysis::new(&f);

    let mut changed = false;

    for sym in f.local_variables() {
        if dfa.is_symbol_dead(&sym) {
            trace!("removing dead symbol: {}", sym.name());
            f.remove_local_variable(&sym);
            changed |= true;
        }
    }

    changed
}
