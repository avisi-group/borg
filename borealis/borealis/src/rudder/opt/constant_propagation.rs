use {
    crate::{
        rudder::{
            analysis::dfa::{StatementUseAnalysis, SymbolUseAnalysis},
            model::statement::Statement,
            model::{block::Block, function::Function},
        },
        util::arena::{Arena, Ref},
    },
    common::HashMap,
    log::trace,
};

// execute_aarch64_instrs_branch_conditional_cond

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    //trace!("constant propagation {}", f.name());

    // if there is a single write to a variable, and it's a constant value, replace
    // all reads with the constant value

    let sua = SymbolUseAnalysis::new(f);

    for symbol in f.local_variables() {
        if !sua.symbol_has_writes(&symbol) {
            continue;
        }

        let writes = sua.get_symbol_writes(&symbol);
        if writes.len() == 1 {
            let (statement, block) = writes.first().unwrap();

            let Statement::WriteVariable {
                value: value_written,
                ..
            } = statement.get(block.get(f.arena()).arena()).clone()
            else {
                panic!("not a write")
            };

            if let Statement::Constant { typ, value } =
                value_written.get(block.get(f.arena()).arena()).clone()
            {
                trace!("identified candidate symbol: {}", symbol);

                // FIXME: DOMINATED READS
                // replace all reads, in all blocks, with the constant
                if sua.symbol_has_reads(&symbol) {
                    for (read, block) in sua.get_symbol_reads(&symbol) {
                        let Statement::ReadVariable { .. } =
                            read.get(block.get(f.arena()).arena()).clone()
                        else {
                            panic!("not a read");
                        };

                        read.get_mut(block.get_mut(f.arena_mut()).arena_mut())
                            .replace_kind(Statement::Constant {
                                typ: typ.clone(),
                                value: value.clone(),
                            });

                        changed = true;
                    }
                }
            }
        }
    }

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= simplify_block_local_writes(f.arena_mut(), block);
    }

    changed
}

fn simplify_block_local_writes(arena: &mut Arena<Block>, block: Ref<Block>) -> bool {
    let mut changed = false;

    let mut most_recent_writes = HashMap::default();

    let mut sua = StatementUseAnalysis::new(arena, block);

    for stmt in block
        .get(sua.block_arena())
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        if let Statement::WriteVariable { symbol, value } =
            stmt.get(block.get(sua.block_arena()).arena()).clone()
        {
            most_recent_writes.insert(symbol.name(), value);
        } else if let Statement::ReadVariable { symbol } =
            stmt.get(block.get(sua.block_arena()).arena()).clone()
        {
            if let Some(most_recent_write) = most_recent_writes.get(&symbol.name()) {
                if sua.has_uses(stmt) {
                    let uses_of_read_variable = sua.get_uses(stmt).clone();
                    for stmt_use in uses_of_read_variable {
                        stmt_use
                            .get_mut(block.get_mut(sua.block_arena()).arena_mut())
                            .replace_use(stmt.clone(), most_recent_write.clone());
                    }

                    changed |= true;
                }
            }
        }
    }

    changed
}
