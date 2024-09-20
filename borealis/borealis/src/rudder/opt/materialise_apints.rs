use crate::{
    rudder::{
        analysis::dfa::StatementUseAnalysis,
        model::statement::{CastOperationKind, Statement, StatementKind},
        model::types::Type,
        model::{block::Block, function::Function, types::PrimitiveTypeClass},
    },
    util::arena::{Arena, Ref},
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(f.arena_mut(), block);
    }

    changed
}

fn run_on_block(arena: &mut Arena<Block>, b: Ref<Block>) -> bool {
    let mut changed = false;

    let mut sua = StatementUseAnalysis::new(arena, b);

    for stmt in b.get(sua.block_arena()).statements() {
        changed |= run_on_stmt(stmt, b, &mut sua);
    }

    changed
}

fn run_on_stmt(stmt: Ref<Statement>, block: Ref<Block>, sua: &mut StatementUseAnalysis) -> bool {
    match stmt
        .get(&block.get(sua.block_arena()).statement_arena)
        .kind()
        .clone()
    {
        StatementKind::Constant { typ, value } => {
            if typ.is_apint() {
                stmt.get_mut(&mut block.get_mut(sua.block_arena()).statement_arena)
                    .replace_kind(StatementKind::Constant {
                        typ: Type::new_primitive(
                            PrimitiveTypeClass::SignedInteger,
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

                if sua.has_uses(stmt) {
                    for u in sua.get_uses(stmt).clone() {
                        u.get_mut(&mut block.get_mut(sua.block_arena()).statement_arena)
                            .replace_use(stmt.clone(), value.clone());
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
