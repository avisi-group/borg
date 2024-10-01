use {
    crate::rudder::analysis::dfa::StatementUseAnalysis,
    common::{
        arena::{Arena, Ref},
        rudder::{
            block::Block,
            function::Function,
            statement::{CastOperationKind, Statement},
            types::{PrimitiveTypeClass, Type},
        },
    },
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

    for stmt in b
        .get(sua.block_arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        changed |= run_on_stmt(stmt, b, &mut sua);
    }

    changed
}

fn run_on_stmt(stmt: Ref<Statement>, block: Ref<Block>, sua: &mut StatementUseAnalysis) -> bool {
    match stmt.get(block.get(sua.block_arena()).arena()).clone() {
        Statement::Constant { typ, value } => {
            if typ.is_apint() {
                stmt.get_mut(block.get_mut(sua.block_arena()).arena_mut())
                    .replace_kind(Statement::Constant {
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
        Statement::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value,
        } => {
            if typ.is_apint() {
                // replace uses of this cast with the original value

                let mut changed = false;

                if let Some(uses) = sua.get_uses(stmt).cloned() {
                    for u in uses {
                        u.get_mut(block.get_mut(sua.block_arena()).arena_mut())
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
