use {
    crate::rudder::{analysis::dfa::StatementUseAnalysis, Block},
    log::trace,
};

pub fn run(f: crate::rudder::Function) -> bool {
    let mut changed = false;

    for block in f.entry_block().iter() {
        changed |= run_on_block(block);
    }

    changed
}

fn run_on_block(b: Block) -> bool {
    let sua = StatementUseAnalysis::new(&b);

    for stmt in b.statements() {
        if sua.is_dead(&stmt) {
            trace!("killing dead statement: {}", stmt);
            b.kill_statement(&stmt);
            return true;
        }
    }

    false
}
