use common::{intern::InternedString, HashMap};

use crate::{
    fn_is_allowlisted,
    rudder::{
        statement::{Statement, StatementBuilder, StatementKind},
        Block, Function, Model, Symbol,
    },
    util::arena::{Arena, Ref},
};

/// In a function, go through all blocks, looking for function calls
/// if a function call is found, split the block into pre-call and post-call blocks
/// copy the blocks of the called function, pre-call block should jump to entry block, make sure all blocks taht terminate in a return unconditonally jump to the post-call block
///
/// all local variables in the inlined function need to be inserted into the calling function (and mangled)
/// parameter local variables need to be made and arguments copied into them
/// return local variable also needs mangling?
pub fn inline(model: &mut Model) {
    let names = model
        .fns
        .keys()
        .copied()
        .filter(|name| fn_is_allowlisted(*name))
        .collect::<Vec<_>>();

    names.into_iter().for_each(|name| {
        let mut function = model.fns.remove(&name).unwrap();
        run_inliner(&mut function, &model.fns);
        function.update_indices();
        model.fns.insert(name, function);
    });
}

fn run_inliner(function: &mut Function, functions: &HashMap<InternedString, Function>) {
    let block_refs = function.block_iter().collect::<Vec<_>>();

    block_refs.into_iter().for_each(|block_ref| {
        let statements = block_ref.get(function.block_arena()).statements();

        let calls = statements
            .iter()
            .enumerate()
            .filter(|(_, s)| matches!(s.kind(), StatementKind::Call { .. }))
            .collect::<Vec<_>>();

        for (index, call) in calls {
            let (pre, post) = statements.split_at(index);

            let post_block_ref = function.block_arena_mut().insert(Block::new());

            let mut pre_statements = pre.to_owned();
            let post_statements = post.to_owned();

            let StatementKind::Call { target, args, .. } = call.kind() else {
                unreachable!()
            };
            let other_fn = functions.get(&target).unwrap();

            // import the target's blocks, assigning new blockrefs, and replacing returns with jumps to post_block_ref
            let entry_block_ref = import_blocks(
                function.block_arena_mut(),
                other_fn.block_arena(),
                other_fn.entry_block(),
                post_block_ref,
            );

            {
                let mut builder = StatementBuilder::new(block_ref);

                // throwing away builder! may lose intermediate generated statements
                let statement = builder.build(StatementKind::Jump {
                    target: entry_block_ref,
                });

                // for arg in args, pre_statements.push(statement::copy)
                pre_statements.push(statement)
            }

            post_statements[0].replace_kind(StatementKind::ReadVariable {
                symbol: Symbol {
                    name: "borealis_inline_return".into(),
                    typ: other_fn.return_type(),
                },
            });

            block_ref
                .get_mut(function.block_arena_mut())
                .set_statements(pre_statements.into_iter());

            post_block_ref
                .get_mut(function.block_arena_mut())
                .set_statements(post_statements.into_iter());
        }
    });
}

/// returns entry block of imported blocks
fn import_blocks(
    this_arena: &mut Arena<Block>,
    other_arena: &Arena<Block>,
    other_block_ref: Ref<Block>,
    this_exit_block_ref: Ref<Block>, // replace returns with jump to this exit block
) -> Ref<Block> {
    fn import_block_rec(
        ref_map: &mut HashMap<Ref<Block>, Ref<Block>>,
        this_arena: &mut Arena<Block>,
        other_arena: &Arena<Block>,
        other_block_ref: Ref<Block>,
        this_exit_block_ref: Ref<Block>,
    ) -> Ref<Block> {
        if let Some(this_block_ref) = ref_map.get(&other_block_ref) {
            return *this_block_ref;
        }

        let other_block = other_block_ref.get(other_arena).clone();
        let this_block_ref = this_arena.insert(other_block);
        ref_map.insert(other_block_ref, this_block_ref);

        {
            let other_statements = this_block_ref.get_mut(this_arena).statements();

            let mut builder = StatementBuilder::new(this_block_ref);

            for statement in other_statements {
                let kind = match statement.kind() {
                    StatementKind::Jump { target } => StatementKind::Jump {
                        target: import_block_rec(
                            ref_map,
                            this_arena,
                            other_arena,
                            target,
                            this_exit_block_ref,
                        ),
                    },
                    StatementKind::Branch {
                        condition,
                        true_target,
                        false_target,
                    } => StatementKind::Branch {
                        condition,
                        true_target: import_block_rec(
                            ref_map,
                            this_arena,
                            other_arena,
                            true_target,
                            this_exit_block_ref,
                        ),
                        false_target: import_block_rec(
                            ref_map,
                            this_arena,
                            other_arena,
                            false_target,
                            this_exit_block_ref,
                        ),
                    },
                    StatementKind::PhiNode { .. } => todo!(),

                    StatementKind::Return { value } => {
                        builder.build(StatementKind::WriteVariable {
                            symbol: Symbol::new("borealis_inline_return".into(), value.typ()),
                            value,
                        });
                        StatementKind::Jump {
                            target: this_block_ref,
                        }
                    }
                    k => k,
                };
                builder.build(kind);
            }

            this_block_ref
                .get_mut(this_arena)
                .set_statements(builder.finish().into_iter());
        }

        this_block_ref
    }

    let mut ref_map = HashMap::default();
    import_block_rec(
        &mut ref_map,
        this_arena,
        other_arena,
        other_block_ref,
        this_exit_block_ref,
    )
}
