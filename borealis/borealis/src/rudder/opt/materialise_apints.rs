use crate::{
    rudder::{
        analysis::dfa::StatementUseAnalysis,
        statement::{CastOperationKind, Statement, StatementKind},
        Block, Function, Type,
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
