use {
    crate::{
        fn_is_allowlisted,
        rudder::model::{
            block::{Block, BlockIterator},
            function::{Function, Symbol},
            statement::{build, import_statement, Statement},
            types::Type,
            Model,
        },
        util::arena::Ref,
    },
    common::{identifiable::Id, intern::InternedString, HashMap},
};

/// In a function, go through all blocks, looking for function calls
///
/// If a function call is found in a block (the pre block), create a new block
/// (the post block).
///
/// All statements before the call go in the pre block. All statements after the
/// call go into the post block. Copy the blocks of the called function,
/// pre-call block should jump to entry block, make sure all blocks taht
/// terminate in a return unconditonally jump to the post-call block.
///
/// all local variables in the inlined function need to be inserted into the
/// calling function (and mangled) parameter local variables need to be made and
/// arguments copied into them return local variable also needs mangling?
pub fn inline(model: &mut Model, top_level_fns: &[&'static str]) {
    top_level_fns
        .iter()
        .copied()
        .map(InternedString::from_static)
        .for_each(|name| {
            log::warn!("inlining {name}");
            let mut function = model.fns.remove(&name).unwrap();

            // map of function name to imported entry block, used to avoid importing an inlined function more than once
            let mut inlined = HashMap::default();

            loop {
                log::warn!("running inliner pass");
                let did_change = run_inliner(&mut function, &model.fns, &mut inlined);
                if !did_change {
                    break;
                }
            }

            model.fns.insert(name, function);
        });
}

fn run_inliner(
    function: &mut Function,
    functions: &HashMap<InternedString, Function>,
    inlined: &mut HashMap<InternedString, (Ref<Block>, Ref<Block>)>,
) -> bool {
    let mut did_change = false;
    function
        .block_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|block_ref| {
            let statements = block_ref.get(function.arena()).statements();

            let mut calls = statements.iter().enumerate().filter_map(|(i, s)| {
                match s.get(block_ref.get(function.arena()).arena()) {
                    Statement::Call { target, args, .. } => {
                        if fn_is_allowlisted(*target) {
                            Some((i, (*target, args.clone())))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            });

            if let Some((index, (call_name, call_args))) = calls.next() {
                did_change = true;

                log::debug!(
                    "inlining call {call_name:?} in \n{}",
                    block_ref.get(function.arena())
                );

                let symbol_prefix = format!("inline{:x}_", Id::new());

                let (pre_statements, post_statements) = {
                    let (pre, post) = statements.split_at(index);
                    (pre.to_owned(), post.to_owned())
                };

                let pre_block_ref = block_ref;
                let post_block_ref = function.arena_mut().insert(Block::new());

                let other_fn = functions.get(&call_name).unwrap();

                log::debug!("other entry block: {:?}", other_fn.entry_block());

                // import local variables
                other_fn
                    .local_variables()
                    .iter()
                    .chain(other_fn.parameters().iter())
                    .map(|sym| symbol_add_prefix(sym, &symbol_prefix))
                    .for_each(|sym| function.add_local_variable(sym));

                // import the target's blocks, assigning new blockrefs, and replacing returns
                // with jumps to post_block_ref

                let (entry_block_ref, exit_block_ref) = inlined
                    .entry(call_name)
                    .or_insert_with(|| import_blocks(function, other_fn, &symbol_prefix))
                    .clone();

                // set pre-statements and end pre block with jump to inlined function entry
                // block
                pre_block_ref
                    .get_mut(function.arena_mut())
                    .set_statements(pre_statements.into_iter());

                for (symbol, value) in other_fn
                    .parameters()
                    .iter()
                    .map(|sym| symbol_add_prefix(sym, &symbol_prefix))
                    .zip(call_args.iter().copied())
                {
                    build(
                        pre_block_ref,
                        function.arena_mut(),
                        Statement::WriteVariable { symbol, value },
                    );
                }
                build(
                    pre_block_ref,
                    function.arena_mut(),
                    Statement::EnterInlineCall {
                        pre_call_block: pre_block_ref,
                        inline_entry_block: entry_block_ref,
                        inline_exit_block: exit_block_ref,
                        post_call_block: post_block_ref,
                    },
                );
                log::debug!("new pre block\n{}", pre_block_ref.get(function.arena()));

                // import post statenents to new block, replacing call with read variable
                {
                    let mut mapping = HashMap::default();

                    if let Type::Tuple(ts) = other_fn.return_type() {
                        let reads = ts
                            .iter()
                            .enumerate()
                            .map(|(index, typ)| {
                                Symbol::new(
                                    format!("{symbol_prefix}_borealis_inline_return_{index}")
                                        .into(),
                                    typ.clone(),
                                )
                            })
                            .map(|symbol| {
                                function.add_local_variable(symbol.clone());
                                build(
                                    post_block_ref,
                                    function.arena_mut(),
                                    Statement::ReadVariable { symbol },
                                )
                            })
                            .collect::<Vec<_>>();
                        let tuple = build(
                            post_block_ref,
                            function.arena_mut(),
                            Statement::CreateTuple(reads),
                        );
                        mapping.insert(post_statements[0], tuple);
                    } else {
                        let symbol = Symbol::new(
                            format!("{symbol_prefix}_borealis_inline_return").into(),
                            other_fn.return_type(),
                        );
                        function.add_local_variable(symbol.clone());
                        let read_return_ref = build(
                            post_block_ref,
                            function.arena_mut(),
                            Statement::ReadVariable { symbol },
                        );
                        mapping.insert(post_statements[0], read_return_ref); // replace call with read variable of return value so that future statements aren't invalidated
                    }

                    // copy remaining statements from pre block to post block
                    for other_statement in &post_statements[1..] {
                        let this_statement = import_statement(
                            post_block_ref,
                            pre_block_ref,
                            function.arena_mut(),
                            *other_statement,
                            &mapping,
                        );

                        mapping.insert(*other_statement, this_statement);
                    }
                }

                log::debug!("new post block\n{}", post_block_ref.get(function.arena()));
            }
        });

    did_change
}

/// returns entry block of imported blocks
fn import_blocks(
    this_function: &mut Function,
    other_function: &Function,
    symbol_prefix: &str, // insert this prefix into symbol names
) -> (Ref<Block>, Ref<Block>) {
    let other_refs = other_function.block_iter().collect::<Vec<_>>();

    let other_arena = other_function.arena();
    let other_entry = other_function.entry_block();

    let mut mapping = HashMap::default();

    // import each block in the other function
    for other_ref in other_refs {
        let other_block = other_ref.get(other_arena).clone();
        let this_ref = this_function.arena_mut().insert(other_block);
        mapping.insert(other_ref, this_ref);
    }

    // this function now contains all the blocks, and we have a mapping of block
    // refs from other to this

    // we need to apply this mapping to all statements in the imported blocks
    mapping.values().copied().for_each(|r| {
        let block = r.get_mut(this_function.arena_mut());
        for statement in block.statements().iter().copied().collect::<Vec<_>>() {
            let kind = statement.get_mut(block.arena_mut());
            match kind {
                Statement::Jump { target } => *target = *mapping.get(target).unwrap(),
                Statement::Branch {
                    true_target,
                    false_target,
                    ..
                } => {
                    *true_target = *mapping.get(true_target).unwrap();
                    *false_target = *mapping.get(false_target).unwrap();
                }
                Statement::PhiNode { .. } => todo!(),

                Statement::ReadVariable { symbol } => {
                    *symbol = symbol_add_prefix(symbol, symbol_prefix)
                }
                Statement::WriteVariable { symbol, .. } => {
                    *symbol = symbol_add_prefix(symbol, symbol_prefix)
                }
                _ => (),
            }
        }
    });

    let mut maybe_exit_block = None;

    // fix returns to point to the supplied target block
    mapping.values().copied().for_each(|block_ref| {
        let terminator = block_ref
            .get(this_function.arena())
            .terminator_statement()
            .unwrap();

        if let Statement::Return { value } = terminator
            .get(block_ref.get(this_function.arena()).arena())
            .clone()
        {
            let block = block_ref.get_mut(this_function.arena_mut());

            block.kill_statement(terminator);

            if let Type::Tuple(ts) = value.get(block.arena()).typ(block.arena()) {
                for (index, typ) in ts.into_iter().enumerate() {
                    let symbol = Symbol::new(
                        format!("{symbol_prefix}_borealis_inline_return_{index}").into(),
                        typ,
                    );
                    let access = build(
                        block_ref,
                        this_function.arena_mut(),
                        Statement::TupleAccess {
                            index,
                            source: value,
                        },
                    );
                    build(
                        block_ref,
                        this_function.arena_mut(),
                        Statement::WriteVariable {
                            symbol,
                            value: access,
                        },
                    );
                }
            } else {
                let symbol = Symbol::new(
                    format!("{symbol_prefix}_borealis_inline_return").into(),
                    value.get(block.arena()).typ(block.arena()),
                );
                build(
                    block_ref,
                    this_function.arena_mut(),
                    Statement::WriteVariable { symbol, value },
                );
            }

            // insert a jump back to the return target block after
            build(
                block_ref,
                this_function.arena_mut(),
                Statement::ExitInlineCall,
            );

            // we only want one exit block
            assert!(maybe_exit_block.replace(block_ref).is_none())
        }
    });

    let entry_block = *mapping.get(&other_entry).unwrap();
    let exit_block = maybe_exit_block.unwrap();

    (entry_block, exit_block)
}

fn symbol_add_prefix(symbol: &Symbol, prefix: &str) -> Symbol {
    Symbol::new(format!("{prefix}{}", symbol.name()).into(), symbol.typ())
}
