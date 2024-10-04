use {
    crate::{fn_is_allowlisted, rudder::analysis::dfa::StatementUseAnalysis},
    common::{
        arena::Ref,
        id::Id,
        intern::InternedString,
        rudder::{
            block::Block,
            function::{Function, Symbol},
            statement::{build, build_at, import_statement, Location, Statement},
            types::Type,
            Model,
        },
        HashMap,
    },
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
            let mut function = model.functions_mut().remove(&name).unwrap();

            // map of function name to imported entry block and exit block, used to avoid
            // importing an inlined function more than once
            let mut inlined = HashMap::default();

            // EnterInlineCall statements have to reference the exit block of the inlined
            // function, in order to set up the mapping in the DBT from exit block to the
            // call site post block ExitInlineCall statements lookup the current
            // block index in that mapping and jump back to whereever the inline call
            // started
            //
            // However, if we have a block that contains calls and ends with an
            // ExitInlineCall, after inlining the ExitInlineCall statement will be in a
            // different block, invalidating those references
            //
            // This is a map of old block refs to new exitinlinecall block refs
            let mut exit_inline_call_rewrites = HashMap::default();

            loop {
                let did_change = run_inliner(
                    &mut function,
                    model.functions(),
                    &mut inlined,
                    &mut exit_inline_call_rewrites,
                );

                fix_exit_refs(&mut function, &mut inlined, &exit_inline_call_rewrites);

                if !did_change {
                    break;
                }
            }

            model.functions_mut().insert(name, function);
        });
}

struct ImportedFunction {
    symbol_prefix: String,
    entry_block: Ref<Block>,
    exit_block: Ref<Block>,
}

fn run_inliner(
    function: &mut Function,
    functions: &HashMap<InternedString, Function>,
    inlined: &mut HashMap<InternedString, ImportedFunction>,
    exit_inline_call_rewrites: &mut HashMap<Ref<Block>, Ref<Block>>,
) -> bool {
    let mut did_change = false;
    function
        .block_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|block_ref| {
            let mut calls = block_ref
                .get(function.arena())
                .statements()
                .iter()
                .enumerate()
                .filter_map(
                    |(i, s)| match s.get(block_ref.get(function.arena()).arena()) {
                        Statement::Call { target, args, .. } => {
                            if fn_is_allowlisted(*target) {
                                Some((i, (*target, args.clone())))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                );

            // we only do one call at a time for simplicity, todo: make this more efficient
            if let Some((mut index, (call_name, call_args))) = calls.next() {
                did_change = true;

                log::debug!(
                    "inlining call {call_name:?} in \n{}",
                    block_ref.get(function.arena())
                );

                // if the block we're about to split contains an exit inline call, we'll need to
                // replace any EnterInlineCall's that reference this exit block to the new exit
                // block (the post call block)
                let needs_exit_rewrite = if let Statement::ExitInlineCall = block_ref
                    .get(function.arena())
                    .terminator_statement()
                    .unwrap()
                    .get(block_ref.get(function.arena()).arena())
                {
                    Some(block_ref)
                } else {
                    None
                };

                // determine if there are dependencies between pre and post, and write-read to a
                // new local variable if so
                {
                    let (pre_statements, post_statements) = {
                        let (pre, post) =
                            block_ref.get(function.arena()).statements().split_at(index);
                        (pre.to_owned(), post.to_owned())
                    };

                    let mut dependencies = vec![];
                    let sua = StatementUseAnalysis::new(function.arena_mut(), block_ref);
                    for pre_statement in &pre_statements {
                        if let Some(uses) = sua.get_uses(*pre_statement) {
                            // ignoring the call statement itself
                            for post_statement in &post_statements[1..] {
                                if uses.contains(post_statement) {
                                    //  panic!("{pre_statement} uses {post_statement}")
                                    dependencies.push((*pre_statement, *post_statement));
                                }
                            }
                        }
                    }

                    // to account for the additional writes
                    index += dependencies.len();

                    for (pre, post) in dependencies {
                        let arena = block_ref.get(function.arena()).arena();
                        let symbol = Symbol::new(
                            format!("bridged_{:x}", Id::new()).into(),
                            pre.get(arena).typ(arena),
                        );
                        function.add_local_variable(symbol.clone());

                        // todo: add a Location::After
                        build_at(
                            block_ref,
                            function.arena_mut(),
                            Statement::WriteVariable {
                                symbol: symbol.clone(),
                                value: pre,
                            },
                            Location::Before(post_statements[0]),
                        );

                        let read_var = build_at(
                            block_ref,
                            function.arena_mut(),
                            Statement::ReadVariable { symbol },
                            Location::Before(post),
                        );
                        post.get_mut(block_ref.get_mut(function.arena_mut()).arena_mut())
                            .replace_use(pre, read_var);
                    }
                };

                log::debug!(
                    "after bridging statement dependencies in \n{}",
                    block_ref.get(function.arena())
                );

                // split statements at the supplied index
                let (pre_statements, post_statements) = {
                    let (pre, post) = block_ref.get(function.arena()).statements().split_at(index);
                    (pre.to_owned(), post.to_owned())
                };

                // the pre block is the current block which will be trimmed to only contain
                // statements before the call
                let pre_block_ref = block_ref;

                // post block is a new block containing all statements after the call
                let post_block_ref = function.arena_mut().insert(Block::new());

                let other_fn = functions.get(&call_name).unwrap();

                log::debug!("other entry block: {:?}", other_fn.entry_block());

                // import the target's blocks, assigning new blockrefs, and replacing returns
                // with jumps to post_block_ref
                let imported_function = inlined.entry(call_name).or_insert_with(|| {
                    let symbol_prefix = format!("inline{:x}_", Id::new());

                    // import local variables and the parameters as new local variables in the
                    // current function (namespaced using the unique prefix)
                    other_fn
                        .local_variables()
                        .iter()
                        .chain(other_fn.parameters().iter())
                        .map(|sym| symbol_add_prefix(sym, &symbol_prefix))
                        .for_each(|sym| function.add_local_variable(sym));

                    let (entry_block, exit_block) =
                        import_blocks(function, other_fn, &symbol_prefix);

                    ImportedFunction {
                        symbol_prefix,
                        entry_block,
                        exit_block,
                    }
                });

                // set pre-statements and end pre block with jump to inlined function entry
                // block
                pre_block_ref
                    .get_mut(function.arena_mut())
                    .set_statements(pre_statements.into_iter());

                // write arguments into the parameter local vars
                for (symbol, value) in other_fn
                    .parameters()
                    .iter()
                    .map(|sym| symbol_add_prefix(sym, &imported_function.symbol_prefix))
                    .zip(call_args.iter().copied())
                {
                    build(
                        pre_block_ref,
                        function.arena_mut(),
                        Statement::WriteVariable { symbol, value },
                    );
                }

                // finish the pre-block with the call to the imported entry block
                build(
                    pre_block_ref,
                    function.arena_mut(),
                    Statement::EnterInlineCall {
                        pre_call_block: pre_block_ref,
                        inline_entry_block: imported_function.entry_block,
                        inline_exit_block: imported_function.exit_block,
                        post_call_block: post_block_ref,
                    },
                );
                log::debug!("new pre block\n{}", pre_block_ref.get(function.arena()));

                // import post statenents to new block, replacing call with read variable
                {
                    let mut mapping = HashMap::default();

                    // return value constructed by reading the `borealis_inline_return` local
                    // variable(s)
                    let return_value = if let Type::Tuple(ts) = other_fn.return_type() {
                        let reads = ts
                            .iter()
                            .enumerate()
                            .map(|(index, typ)| {
                                Symbol::new(
                                    format!(
                                        "{}_borealis_inline_return_{index}",
                                        imported_function.symbol_prefix
                                    )
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
                        build(
                            post_block_ref,
                            function.arena_mut(),
                            Statement::CreateTuple(reads),
                        )
                    } else {
                        let symbol = Symbol::new(
                            format!("{}_borealis_inline_return", imported_function.symbol_prefix)
                                .into(),
                            other_fn.return_type(),
                        );
                        function.add_local_variable(symbol.clone());
                        build(
                            post_block_ref,
                            function.arena_mut(),
                            Statement::ReadVariable { symbol },
                        )
                    };

                    // replace call with read variable of return value so that future statements
                    // aren't invalidated
                    mapping.insert(post_statements[0], return_value);

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

                if let Some(old) = needs_exit_rewrite {
                    exit_inline_call_rewrites.insert(old, post_block_ref);
                }
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

fn fix_exit_refs(
    function: &mut Function,
    inlined: &mut HashMap<InternedString, ImportedFunction>,
    rewrites: &HashMap<Ref<Block>, Ref<Block>>,
) {
    inlined
        .values_mut()
        .for_each(|ImportedFunction { exit_block, .. }| {
            if let Some(new) = rewrites.get(exit_block) {
                *exit_block = *new;
            }
        });
    function
        .block_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|block_ref| {
            block_ref
                .get_mut(function.arena_mut())
                .statements()
                .to_owned()
                .into_iter()
                .for_each(|r| {
                    if let Statement::EnterInlineCall {
                        inline_exit_block, ..
                    } = r.get_mut(block_ref.get_mut(function.arena_mut()).arena_mut())
                    {
                        if let Some(new) = rewrites.get(inline_exit_block) {
                            *inline_exit_block = *new;
                        }
                    }
                });
        });
}
