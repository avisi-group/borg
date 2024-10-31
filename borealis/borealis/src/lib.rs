//! Sail frontend for GenSim

use {
    crate::{
        boom::{
            passes::{
                builtin_fns::HandleBuiltinFunctions, cycle_finder::CycleFinder,
                destruct_composites::DestructComposites, destruct_unions::DestructUnions,
                fix_exceptions::FixExceptions, fold_unconditionals::FoldUnconditionals,
                remove_const_branch::RemoveConstBranch, remove_constant_type::RemoveConstantType,
            },
            Ast,
        },
        rudder::{
            opt::{self, OptLevel},
            validator,
        },
    },
    common::{
        intern::{self, InternedString},
        rudder::{
            block::Block,
            constant_value::ConstantValue,
            function::{Function, Symbol},
            statement::Statement,
            types::Type,
        },
        HashMap,
    },
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{debug, info, trace},
    rkyv::Deserialize,
    sailrs::{
        bytes, create_file_buffered,
        jib_ast::{self, Definition, DefinitionAux, Instruction},
        sail_ast::Location,
        types::ListVec,
    },
    std::{
        fs::{create_dir_all, File},
        io::Write,
        path::{Path, PathBuf},
    },
};

pub mod boom;
//pub mod codegen;
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

    let (jib, strs): (ListVec<Definition>, _) =
        unsafe { rkyv::archived_root::<(ListVec<Definition>, HashMap<String, u32>)>(&mmap) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

    trace!("initializing interner");

    intern::init(strs);

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
        HandleBuiltinFunctions::new_boxed(),
        RemoveConstantType::new_boxed(),
        DestructComposites::new_boxed(),
        DestructUnions::new_boxed(),
        FixExceptions::new_boxed(),
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
    rudder
        .functions_mut()
        .extend(variable_corrupted_example().into_iter());

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

fn fn_is_allowlisted(name: InternedString) -> bool {
    const FN_ALLOWLIST: &[&'static str] = &[
        "borealis_register_init",
        "__DecodeA64",
        "__InitSystem",
        "add_with_carry_test",
        "num_of_Feature",
        "__DecodeA64_DataProcReg",
        "decode_add_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
        "DecodeShift",
        "execute_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
        "__id",
        "X_read",
        "get_R",
        "read_gpr",
        "ShiftReg",
        "X_set",
        "set_R",
        "write_gpr",
        "TakeReset",
        "InitVariantImplemented",
        "InitFeatureImpl",
        "SetResetVector",
        "Mk_RVBAR_EL1_Type",
        "Mk_RVBAR_EL2_Type",
        "Mk_RVBAR_EL3_Type",
        "Mk_MVBAR_Type",
        "HaveAArch64",
        "IsFeatureImplemented",
        "num_of_Feature",
        "HaveEL",
        "AArch32_TakeReset",
        "FPEXC_read",
        "_update_FPEXC_Type_EN",
        "HSCTLR_read",
        "SCTLR_read__2",
        "SCTLR_NS_read",
        "AArch32_WriteMode",
        "ELFromM32",
        "SCR_GEN_read",
        "Mk_SCRType",
        "BadMode",
        "HaveAArch32EL",
        "EffectiveSCR_EL3_NSE",
        "HaveRME",
        "EffectiveSCR_EL3_NS",
        "HaveSecureState",
        "AArch32_ResetControlRegisters",
        "ResetControlRegisters",
        "AArch32_AutoGen_ArchitectureReset",
        "Mk_VDISR_Type",
        "VDISR_read",
        "VDISR_write",
        "Mk_VDFSR_Type",
        "VDFSR_read",
        "VDFSR_write",
        "FPSCR_read__1",
        "CPACR_read__1",
        "CNTKCTL_read__1",
        "ELUsingAArch32",
        "IsSecureBelowEL3",
        "ELStateUsingAArch32",
        "ELStateUsingAArch32K",
        "HaveSecureEL2Ext",
        "VBAR_read__2",
        "Bit",
        "AArch32_IMPDEFResets",
        "integer_subrange",
        "set_subrange_zeros",
        "set_slice_zeros",
        "slice_mask",
        "sail_mask",
        "AArch64_IMPDEFResets",
        "__DecodeA64_DataProcImm",
        "decode_movz_aarch64_instrs_integer_ins_ext_insert_movewide",
        "execute_aarch64_instrs_integer_ins_ext_insert_movewide",
        "Zeros",
        "decode_subs_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
        "__DecodeA64_BranchExcSys",
        "decode_b_cond_aarch64_instrs_branch_conditional_cond",
        "execute_aarch64_instrs_branch_conditional_cond",
        "ConditionHolds",
        "BranchNotTaken",
        "HaveStatisticalProfiling",
        "SPEBranch",
        "SPEBranch__1",
        "StatisticalProfilingEnabled",
        "StatisticalProfilingEnabled__1",
        "UsingAArch32",
        "ProfilingBufferEnabled",
        "ProfilingBufferOwner",
        "IsSecureEL2Enabled",
        "EL2Enabled",
        "SecurityStateAtEL",
        "Unreachable",
        "HaveVirtHostExt",
        "AArch64_TakeReset",
        "AArch64_ResetControlRegisters",
        "AArch64_AutoGen_ArchitectureReset",
        "EncodePARange",
        "Have56BitPAExt",
        "EncodeVARange",
        "HaveCNTSCExt",
        "IsG1ActivityMonitorImplemented",
        "IsG1ActivityMonitorOffsetImplemented",
        "__Reset",
        "AArch64_ResetGeneralRegisters",
        "AArch64_ResetSIMDFPRegisters",
        "V_set",
        "ImplementedSMEVectorLength",
        "Align_int",
        "fdiv_int",
        "SupportedPowerTwoSVL",
        "ImplementedSVEVectorLength",
        "IsPow2",
        "FloorPow2",
        "CeilPow2",
        "IsSVEEnabled",
        "IsOriginalSVEEnabled",
        "AArch64_ResetSpecialRegisters",
        "ResetExternalDebugRegisters",
        "AArch64_PAMax",
        "is_zero_subrange",
        "extzx",
        "extzv",
        "BranchTo",
        "Hint_Branch",
        "AArch64_BranchAddr",
        "AddrTop",
        "S1TranslationRegime",
        "EffectiveTBI",
        "ConstrainUnpredictableBits",
        "execute_aarch64_instrs_integer_logical_shiftedreg",
        "ROR",
    ];

    const FN_DENYLIST: &[&'static str] = &[
        "AArch64_MemTag_read",
        "MemSingleNF_read",
        "ICV_AP1R_read",
        "ICC_AP1R_S_read",
        "ICC_AP1R_NS_read",
        "sail_mem_write",
        "sail_mem_read",
        "DBGBCR_read",
        "DBGBXVR_read",
        "ICH_LRC_read",
        "ICC_AP1R_EL1_read",
        "AMEVCNTR0_EL0_read",
        "AMEVTYPER0_read",
        "DBGWCR_read",
        "execute_aarch64_instrs_integer_arithmetic_div",
    ];

    if FN_DENYLIST.contains(&name.as_ref()) {
        return false;
    }

    FN_ALLOWLIST.contains(&name.as_ref())
        || name.as_ref().ends_with("_read")
        || name.as_ref().ends_with("_write")
        || name.as_ref().starts_with("Mk")
        || name.as_ref().starts_with("_update_")
        || name.as_ref().starts_with("_get_")
        || name.as_ref().starts_with("Have")
        || name.as_ref().starts_with("decode_")
        || name.as_ref().starts_with("execute_aarch64_instrs")
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
    let mut f1 = Function::new("example_f1".into(), Type::unit(), vec![]);

    {
        let entry_block = f1.entry_block().get_mut(f1.arena_mut());
        let s_arena = entry_block.arena_mut();
        let _unit = s_arena.insert(Statement::Constant {
            typ: Type::unit(),
            value: ConstantValue::UnsignedInteger(0),
        });
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
            return_type: Type::u64(),
        });
        let r2 = s_arena.insert(Statement::ReadRegister {
            typ: Type::u64(),
            offset: _16,
        });
        let call2 = s_arena.insert(Statement::Call {
            target: "example_f2".into(),
            args: vec![call1, r2],
            return_type: Type::u64(),
        });
        let w3 = s_arena.insert(Statement::WriteRegister {
            offset: _24,
            value: call2,
        });
        let ret = s_arena.insert(Statement::Return { value: _unit });
        entry_block.set_statements(
            [_unit, _0, _8, _16, _24, r0, r1, call1, r2, call2, w3, ret].into_iter(),
        );
    }

    let left = Symbol::new("left".into(), Type::u64());
    let right = Symbol::new("right".into(), Type::u64());
    let mut f2 = Function::new(
        "example_f2".into(),
        Type::u64(),
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
        let ret = s_arena.insert(Statement::Return { value: add });
        entry_block.set_statements([left, right, add, ret].into_iter());
    }
    fns.insert(f1.name(), f1);
    fns.insert(f2.name(), f2);

    fns
}

fn variable_corrupted_example() -> HashMap<InternedString, Function> {
    let mut fns = HashMap::default();
    let mut func = Function::new("func_corrupted_var".into(), Type::u64(), vec![]);
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
            let _0 = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(0),
            });
            let read = s_arena.insert(Statement::ReadRegister {
                typ: Type::u64(),
                offset: _0,
            });
            let branch = s_arena.insert(Statement::Branch {
                condition: read,
                true_target: b,
                false_target: c,
            });
            a.set_statements([_0, read, branch].into_iter());
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
            let _8 = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(8),
            });
            let read = s_arena.insert(Statement::ReadRegister {
                typ: Type::u64(),
                offset: _8,
            });
            let branch = s_arena.insert(Statement::Branch {
                condition: read,
                true_target: e,
                false_target: f,
            });
            d.set_statements([_8, read, branch].into_iter());
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
            let _16 = s_arena.insert(Statement::Constant {
                typ: Type::u64(),
                value: ConstantValue::UnsignedInteger(16),
            });
            let w = s_arena.insert(Statement::WriteRegister {
                offset: _16,
                value: read,
            });
            let ret = s_arena.insert(Statement::Return { value: read });
            g.set_statements([read, _16, w, ret].into_iter());
        }
    }

    fns.insert(func.name(), func);
    fns
}
