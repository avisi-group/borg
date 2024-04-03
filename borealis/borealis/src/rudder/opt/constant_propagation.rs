use log::trace;

use crate::rudder::{analysis::dfa::SymbolUseAnalysis, Function, StatementKind};

pub fn run(f: Function) -> bool {
    let mut changed = false;

    trace!("constant propagation {}", f.name());

    // if there is a single write to a variable, and it's a constant value, replace all reads with the constant value

    let sua = SymbolUseAnalysis::new(&f);

    for symbol in f.local_variables() {
        if !sua.symbol_has_writes(&symbol) {
            continue;
        }

        let writes = sua.get_symbol_writes(&symbol);
        if writes.len() == 1 {
            let StatementKind::WriteVariable {
                value: value_written,
                ..
            } = writes.first().unwrap().kind()
            else {
                panic!("not a write")
            };

            if let StatementKind::Constant { typ, value } = value_written.kind() {
                trace!("identified candidate symbol: {}", symbol);

                // TODO
            }
        }
    }

    changed
}
