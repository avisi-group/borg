use {
    crate::rudder::{analysis::dfa::SymbolUseAnalysis, Function, Statement, StatementKind},
    log::trace,
};

// execute_aarch64_instrs_branch_conditional_cond

pub fn run(f: Function) -> bool {
    let mut changed = false;

    //trace!("constant propagation {}", f.name());

    // if there is a single write to a variable, and it's a constant value, replace
    // all reads with the constant value

    let sua = SymbolUseAnalysis::new(&f);

    for symbol in f.local_variables() {
        if !sua.symbol_has_writes(&symbol) {
            continue;
        }

        let writes = sua.get_symbol_writes(&symbol);
        if writes.len() == 1 {
            let StatementKind::WriteVariable {
                value: value_written,
                indices: write_indices,
                ..
            } = writes.first().unwrap().kind()
            else {
                panic!("not a write")
            };

            if let StatementKind::Constant { typ, value } = value_written.kind() {
                trace!("identified candidate symbol: {}", symbol);

                // FIXME: DOMINATED READS
                // replace all reads, in all blocks, with the constant
                if sua.symbol_has_reads(&symbol) {
                    for read in sua.get_symbol_reads(&symbol) {
                        let StatementKind::ReadVariable {
                            indices: read_indices,
                            ..
                        } = read.kind()
                        else {
                            panic!("not a read");
                        };

                        if write_indices == read_indices {
                            read.replace_kind(StatementKind::Constant {
                                typ: typ.clone(),
                                value: value.clone(),
                            });

                            changed = true;
                        }
                    }
                }
            }
        }
    }

    changed
}
