use crate::rudder::{
    analysis::dfa::StatementUseAnalysis,
    statement::{CastOperationKind, Statement, StatementKind},
    Block, Function, Type,
};
use std::sync::Arc;

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
        StatementKind::Constant { typ, value } => {
            if typ.is_apint() {
                stmt.replace_kind(StatementKind::Constant {
                    typ: Type::new_primitive(
                        crate::rudder::PrimitiveTypeClass::SignedInteger,
                        value.smallest_width(),
                    ),
                    value,
                });

                true
            } else {
                false
            }
        }
        StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value,
        } => {
            if typ.is_apint() {
                // replace uses of this cast with the original value

                let mut changed = false;

                if sua.has_uses(&stmt) {
                    for u in sua.get_uses(&stmt) {
                        u.replace_use(stmt.clone(), value.clone());
                        changed = true;
                    }
                }

                changed
            } else {
                false
            }
        }
        _ => false,
    }
}
