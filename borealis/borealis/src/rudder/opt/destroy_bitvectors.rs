use crate::{
    rudder::{
        analysis::dfa::StatementUseAnalysis,
        statement::{CastOperationKind, Statement},
        Block, Function,
    },
    util::arena::{Arena, Ref},
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter() {
        changed |= run_on_block(f.block_arena(), block);
    }

    changed
}

fn run_on_block(arena: &Arena<Block>, b: Ref<Block>) -> bool {
    let mut changed = false;

    let sua = StatementUseAnalysis::new(arena, b);
    for stmt in b.get(arena).statements() {
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
