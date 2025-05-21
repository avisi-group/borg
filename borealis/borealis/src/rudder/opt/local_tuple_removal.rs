use {
    crate::rudder::{analysis::dfa::StatementUseAnalysis, opt::OptimizationContext},
    common::{
        arena::{Arena, Ref},
        rudder::{block::Block, function::Function, statement::Statement},
    },
};

pub fn run(ctx: &OptimizationContext, f: &mut Function) -> bool {
    let mut changed = false;

    //trace!("constant folding {}", f.name());
    for block in f.block_iter().collect::<Vec<_>>() {
        changed |= run_on_block(ctx, block, f.arena_mut());
    }

    changed
}

fn run_on_block(ctx: &OptimizationContext, block: Ref<Block>, arena: &mut Arena<Block>) -> bool {
    let mut changed = false;

    let mut sua = StatementUseAnalysis::new(arena, block, &ctx.purity);

    //let block = block.get_mut(sua.block_arena());

    for stmt in block
        .get(sua.block_arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        match stmt.get(block.get(sua.block_arena()).arena()).clone() {
            Statement::TupleAccess { index, source } => {
                // If this tuple access is to a "create-tuple" statement, then
                // its uses can be replaced with the actual
                // thing

                match source.get(block.get(sua.block_arena()).arena()).clone() {
                    Statement::CreateTuple(items) => {
                        // Source is a CreateTuple, so replace uses with the value
                        assert!(index < items.len());

                        if let Some(uses_of_tuple_access) = sua.get_uses(stmt).cloned() {
                            let tuple_element = items[index];
                            uses_of_tuple_access.iter().for_each(|u| {
                                u.get_mut(block.get_mut(sua.block_arena()).arena_mut())
                                    .replace_use(stmt, tuple_element)
                            });

                            changed |= true;
                        }
                    }
                    _ => {
                        // Source is not a CreateTuple
                    }
                }
            }
            _ => {
                //
            }
        }
    }

    changed
}
