use crate::rudder::{
    analysis::dfa::StatementUseAnalysis,
    statement::{CastOperationKind, Statement},
    Block, Function,
};

pub fn run(f: Function) -> bool {
    let mut changed = false;

    for block in f.entry_block().iter() {
        changed |= run_on_block(block);
    }

    changed
}

fn run_on_block(b: Block) -> bool {
    let mut changed = false;

    let sua = StatementUseAnalysis::new(&b);
    for stmt in b.statements() {
        changed |= run_on_stmt(stmt, &sua);
    }

    changed
}

fn run_on_stmt(stmt: Statement, sua: &StatementUseAnalysis) -> bool {
    match stmt.kind() {
        /*crate::rudder::statement::StatementKind::BitExtract {
            value,
            start,
            length,
        } => match value.kind() {
            crate::rudder::statement::StatementKind::Cast {
                kind: CastOperationKind::ZeroExtend,
                typ,
                value: cast_value,
            } => {
                if typ.is_bits() {
                    stmt.replace_kind(crate::rudder::statement::StatementKind::BitExtract {
                        value: cast_value,
                        length,
                        start,
                    });
                    true
                } else {
                    false
                }
            }
            _ => false,
        },*/
        crate::rudder::statement::StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value: cast_value,
        } => {
            if typ.is_bits() {
                if sua.has_uses(&stmt) {
                    for u in sua.get_uses(&stmt) {
                        u.replace_use(stmt.clone(), cast_value.clone());
                    }

                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
