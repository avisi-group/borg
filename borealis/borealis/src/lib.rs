//! Sail frontend for GenSim

use {
    crate::{
        boom::{
            Ast,
            passes::{
                builtin_fns::HandleBuiltinFunctions, constant_propogation::ConstantPropogation,
                cycle_finder::CycleFinder, destruct_composites::DestructComposites,
                fold_unconditionals::FoldUnconditionals, lower_reals::LowerReals,
                remove_const_branch::RemoveConstBranch, remove_constant_type::RemoveConstantType,
                remove_units::RemoveUnits,
            },
        },
        rudder::{
            opt::{self, OptLevel},
            validator,
        },
    },
    common::{
        hashmap::{HashMap, HashSet},
        intern::InternedString,
        rudder::{
            block::Block,
            constant_value::ConstantValue,
            function::{Function, Symbol},
            statement::Statement,
            types::Type,
        },
    },
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{debug, info},
    once_cell::sync::Lazy,
    sailrs::{
        bytes, create_file_buffered,
        jib_ast::{self, Definition, DefinitionAux, Instruction},
        sail_ast::Location,
        types::{ArchivedListVec, ListVec},
    },
    std::{
        fs::{File, create_dir_all},
        io::{BufRead, BufReader, Write},
        path::{Path, PathBuf},
    },
};

pub mod boom;
pub mod rudder;
pub mod util;

/// Deserializes an AST from an archive.
///
/// Internally, deserialization is performed on a new thread with a sufficient
/// stack size to perform the deserialization.
pub fn load_model(path: &Path) -> ListVec<Definition> {
    let file = File::open(path).map_err(PathCtx::f(path)).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file) }.unwrap();

    info!("deserializing");

    let archived: &ArchivedListVec<Definition> = unsafe { rkyv::access_unchecked(&mmap) };
    let jib: ListVec<Definition> = rkyv::deserialize::<_, rkyv::rancor::Error>(archived).unwrap();

    info!("JIB size: {:.2}", bytes(jib.deep_size_of()));

    jib
}

#[derive(Debug, Clone)]
pub enum GenerationMode {
    CodeGen,
    CodeGenWithIr(PathBuf),
    IrOnly(PathBuf),
}

/// Compiles a Sail model to a Brig module
pub fn sail_to_brig(jib_ast: ListVec<jib_ast::Definition>, path: PathBuf, mode: GenerationMode) {
    let dump_ir = match &mode {
        GenerationMode::CodeGen => None,
        GenerationMode::CodeGenWithIr(p) | GenerationMode::IrOnly(p) => Some(p),
    };

    if let Some(path) = &dump_ir {
        create_dir_all(path).unwrap()
    }

    if let Some(path) = &dump_ir {
        sailrs::jib_ast::pretty_print::print_ast(
            &mut create_file_buffered(path.join("ast.jib")).unwrap(),
            jib_ast.iter(),
        );
    }

    info!("Converting JIB to BOOM");
    let ast = Ast::from_jib(jib_wip_filter(jib_ast));

    // // useful for debugging
    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file_buffered(path.join("ast.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Running passes on BOOM");
    [
        LowerReals::new_boxed(),
        HandleBuiltinFunctions::new_boxed(),
        RemoveConstantType::new_boxed(),
        DestructComposites::new_boxed(),
        RemoveUnits::new_boxed(),
    ]
    .into_iter()
    .for_each(|mut pass| {
        pass.run(ast.clone());
    });
    boom::passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            ConstantPropogation::new_boxed(),
            // MonomorphizeVectors::new_boxed(),
            CycleFinder::new_boxed(),
        ],
    );

    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file_buffered(path.join("ast.processed.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Building rudder");
    let mut rudder = rudder::build::from_boom(&ast.get());

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder");
    let msgs = validator::validate(&rudder);
    for msg in msgs {
        debug!("{msg}");
    }

    info!("Optimising rudder");
    opt::optimise(&mut rudder, OptLevel::Level3);

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.opt.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder again");
    let msgs = validator::validate(&rudder);
    for msg in msgs {
        debug!("{msg}");
    }

    rudder
        .functions_mut()
        .extend(example_functions().into_iter());
    let r0_offset = rudder.reg_offset("R0");
    let r1_offset = rudder.reg_offset("R1");
    let r2_offset = rudder.reg_offset("R2");
    rudder
        .functions_mut()
        .extend(variable_corrupted_example(r0_offset, r1_offset, r2_offset).into_iter());

    let to_remove = rudder
        .functions()
        .keys()
        .copied()
        .filter(|name| !fn_is_allowlisted(*name))
        .collect::<Vec<_>>();
    for name in to_remove {
        let function = rudder.functions_mut().get_mut(&name).unwrap();
        let block = function.new_block();
        function.set_entry_block(block);
    }

    {
        let func = rudder
            .functions()
            .get(&InternedString::from_static(
                "execute_aarch64_instrs_memory_pair_general_post_idx",
            ))
            .unwrap();
        rudder::dot::render(
            &mut create_file_buffered(
                dump_ir
                    .unwrap()
                    .join("execute_aarch64_instrs_memory_pair_general_post_idx.dot"),
            )
            .unwrap(),
            func.arena(),
            func.entry_block(),
        )
        .unwrap();
    }

    if matches!(
        &mode,
        GenerationMode::CodeGen | GenerationMode::CodeGenWithIr(_)
    ) {
        info!("Serializing Rudder");

        let buf = postcard::to_allocvec(&rudder).unwrap();

        info!("Writing {:.2} to {:?}", bytes(buf.len()), &path);
        File::create(path).unwrap().write_all(&buf).unwrap();
    }
}

/// Calls to these functions will be replaced with units
pub const DELETED_CALLS: &[&str] = &[
    "RestoreTransactionCheckpointParameterised",
    "Z_set",
    "MaybeZeroSVEUppers",
    "ResetSVEState",
];

fn fn_is_allowlisted(name: InternedString) -> bool {
    static FN_DENYLIST: Lazy<HashSet<InternedString>> = Lazy::new(|| {
        BufReader::new(File::open("denylist.txt").unwrap())
            .lines()
            .map(|s| InternedString::from(s.unwrap()))
            .collect()
    });

    !FN_DENYLIST.contains(&name)
}

fn jib_wip_filter(jib_ast: ListVec<Definition>) -> impl Iterator<Item = jib_ast::Definition> {
    jib_ast.into_iter().map(|d| {
        if let DefinitionAux::Fundef(name, ret, parameters, body) = d.def {
            let new_body = if fn_is_allowlisted(name.as_interned()) {
                body
            } else {
                vec![Instruction {
                    inner: jib_ast::InstructionAux::Undefined(jib_ast::Type::Unit),
                    annot: (0, Location::Unknown),
                }]
                .into()
            };

            Definition {
                def: DefinitionAux::Fundef(name, ret, parameters, new_body),
                annot: d.annot,
            }
        } else {
            d
        }
    })
}

fn example_functions() -> HashMap<InternedString, Function> {
    let mut fns = HashMap::default();
    let mut f1 = Function::new("example_f1".into(), None, vec![]);

    {
        let entry_block = f1.entry_block().get_mut(f1.arena_mut());
        let s_arena = entry_block.arena_mut();
        let _0 = s_arena.insert(Statement::Constant {
            typ: Type::u64(),
            value: ConstantValue::UnsignedInteger(0),
        });

        let _8 = s_arena.insert(Statement::Constant {
            typ: Type::u64(),
            value: ConstantValue::UnsignedInteger(8),
        });

        let _16 = s_arena.insert(Statement::Constant {
            typ: Type::u64(),
            value: ConstantValue::UnsignedInteger(16),
        });
        let _24 = s_arena.insert(Statement::Constant {
            typ: Type::u64(),
            value: ConstantValue::UnsignedInteger(24),
        });
        let r0 = s_arena.insert(Statement::ReadRegister {
            typ: Type::u64(),
            offset: _0,
        });
        let r1 = s_arena.insert(Statement::ReadRegister {
            typ: Type::u64(),
            offset: _8,
        });
        let call1 = s_arena.insert(Statement::Call {
            target: "example_f2".into(),
            args: vec![r0, r1],
            return_type: Some(Type::u64()),
        });
        let r2 = s_arena.insert(Statement::ReadRegister {
            typ: Type::u64(),
            offset: _16,
        });
        let call2 = s_arena.insert(Statement::Call {
            target: "example_f2".into(),
            args: vec![call1, r2],
            return_type: Some(Type::u64()),
        });
        let w3 = s_arena.insert(Statement::WriteRegister {
            offset: _24,
            value: call2,
        });
        let ret = s_arena.insert(Statement::Return { value: None });
        entry_block
            .set_statements([_0, _8, _16, _24, r0, r1, call1, r2, call2, w3, ret].into_iter());
    }

    let left = Symbol::new("left".into(), Type::u64());
    let right = Symbol::new("right".into(), Type::u64());
    let mut f2 = Function::new(
        "example_f2".into(),
        Some(Type::u64()),
        vec![left.clone(), right.clone()],
    );
    {
        let entry_block = f2.entry_block().get_mut(f2.arena_mut());
        let s_arena = entry_block.arena_mut();

        let left = s_arena.insert(Statement::ReadVariable { symbol: left });
        let right = s_arena.insert(Statement::ReadVariable { symbol: right });
        let add = s_arena.insert(Statement::BinaryOperation {
            kind: common::rudder::statement::BinaryOperationKind::Add,
            lhs: left,
            rhs: right,
        });
        let ret = s_arena.insert(Statement::Return { value: Some(add) });
        entry_block.set_statements([left, right, add, ret].into_iter());
    }
    fns.insert(f1.name(), f1);
    fns.insert(f2.name(), f2);

    fns
}

fn variable_corrupted_example(
    r0_offset: u64,
    r1_offset: u64,
    r2_offset: u64,
) -> HashMap<InternedString, Function> {
    let mut fns = HashMap::default();
    let mut func = Function::new("func_corrupted_var".into(), Some(Type::u64()), vec![]);
    let ret_val = Symbol::new("x".into(), Type::u64());
    func.add_local_variable(ret_val.clone());

    {
        let a = func.arena_mut().insert(Block::new());
        let b = func.arena_mut().insert(Block::new());
        let c = func.arena_mut().insert(Block::new());
        let d = func.arena_mut().insert(Block::new());
        let e = func.arena_mut().insert(Block::new());
        let f = func.arena_mut().insert(Block::new());
        let g = func.arena_mut().insert(Block::new());

        {
            let entry_block = func.entry_block().get_mut(func.arena_mut());
            let s_arena = entry_block.arena_mut();
            let jump = s_arena.insert(Statement::Jump { target: a });
            entry_block.set_statements([jump].into_iter());
        }

        {
            let a = a.get_mut(func.arena_mut());
            let s_arena = a.arena_mut();
            let r0_offset = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(r0_offset),
            });
            let read = s_arena.insert(Statement::ReadRegister {
                typ: Type::u64(),
                offset: r0_offset,
            });
            let branch = s_arena.insert(Statement::Branch {
                condition: read,
                true_target: b,
                false_target: c,
            });
            a.set_statements([r0_offset, read, branch].into_iter());
        }

        {
            let b = b.get_mut(func.arena_mut());
            let s_arena = b.arena_mut();
            let _5 = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(5),
            });
            let w = s_arena.insert(Statement::WriteVariable {
                symbol: ret_val.clone(),
                value: _5,
            });
            let jump = s_arena.insert(Statement::Jump { target: d });
            b.set_statements([_5, w, jump].into_iter());
        }

        {
            let c = c.get_mut(func.arena_mut());
            let s_arena = c.arena_mut();
            let _10 = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(10),
            });
            let w = s_arena.insert(Statement::WriteVariable {
                symbol: ret_val.clone(),
                value: _10,
            });
            let jump = s_arena.insert(Statement::Jump { target: d });
            c.set_statements([_10, w, jump].into_iter());
        }

        {
            let d = d.get_mut(func.arena_mut());
            let s_arena = d.arena_mut();
            let r1_offset = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(r1_offset),
            });
            let read = s_arena.insert(Statement::ReadRegister {
                typ: Type::u64(),
                offset: r1_offset,
            });
            let branch = s_arena.insert(Statement::Branch {
                condition: read,
                true_target: e,
                false_target: f,
            });
            d.set_statements([r1_offset, read, branch].into_iter());
        }

        {
            let e = e.get_mut(func.arena_mut());
            let s_arena = e.arena_mut();
            let jump = s_arena.insert(Statement::Jump { target: g });
            e.set_statements([jump].into_iter());
        }

        {
            let f = f.get_mut(func.arena_mut());
            let s_arena = f.arena_mut();
            let jump = s_arena.insert(Statement::Jump { target: g });
            f.set_statements([jump].into_iter());
        }

        {
            let g = g.get_mut(func.arena_mut());
            let s_arena = g.arena_mut();
            let read = s_arena.insert(Statement::ReadVariable {
                symbol: ret_val.clone(),
            });
            let r2_offset = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(r2_offset),
            });
            let w = s_arena.insert(Statement::WriteRegister {
                offset: r2_offset,
                value: read,
            });
            let ret = s_arena.insert(Statement::Return { value: Some(read) });
            g.set_statements([read, r2_offset, w, ret].into_iter());
        }
    }

    fns.insert(func.name(), func);
    fns
}
