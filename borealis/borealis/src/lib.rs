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
        codegen::{codegen_workspace, workspace::write_workspace},
    },
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{debug, info, trace},
    rkyv::Deserialize,
    sailrs::{
        bytes, create_file_buffered,
        intern::{init_interner, interner, InternedString},
        HashMap,
    },
    sailrs::{
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
pub mod codegen;
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

    init_interner(&strs);

    info!("JIB size: {:.2}", bytes(jib.deep_size_of()));
    info!(
        "INTERNER size: {:.2}, {} strings",
        bytes(interner().current_memory_usage()),
        interner().len()
    );

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
    let msgs = rudder.validate();
    for msg in msgs {
        debug!("{msg}");
    }

    info!("Optimising rudder");
    rudder.optimise(rudder::opt::OptLevel::Level3);

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.opt.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Inlining");
    rudder.function_inline(FN_TOPLEVEL);

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.inlined.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder again");
    let msgs = rudder.validate();
    for msg in msgs {
        debug!("{msg}");
    }

    if matches!(
        &mode,
        GenerationMode::CodeGen | GenerationMode::CodeGenWithIr(_)
    ) {
        info!("Generating Rust");
        let ws = codegen_workspace(rudder, FN_TOPLEVEL);

        info!("Writing workspace to {:?}", &path);
        write_workspace(ws, path);
    }
}

const FN_TOPLEVEL: &[&'static str] = &["borealis_register_init", "__DecodeA64", "__InitSystem"];

fn fn_is_allowlisted(name: InternedString) -> bool {
    const FN_ALLOWLIST: &[&'static str] = &[
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
        "_get_RMR_EL3_Type_AA64",
        "_get_ID_AA64PFR0_EL1_Type_EL3",
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
        "_get_HSCTLR_Type_EE",
        "_get_HSCTLR_Type_TE",
        "SCTLR_read__2",
        "_get_SCTLR_Type_EE",
        "_get_SCTLR_Type_TE",
        "SCTLR_NS_read",
        "AArch32_WriteMode",
        "ELFromM32",
        "SCR_GEN_read",
        "Mk_SCRType",
        "BadMode",
        "HaveAArch32EL",
        "EffectiveSCR_EL3_NSE",
        "HaveRME",
        "_get_SCR_EL3_Type_NSE",
        "EffectiveSCR_EL3_NS",
        "HaveSecureState",
        "_get_SCR_EL3_Type_NS",
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
        "_get_SCRType_NS",
        "ELStateUsingAArch32",
        "ELStateUsingAArch32K",
        "HaveSecureEL2Ext",
        "_get_SCR_EL3_Type_EEL2",
        "VBAR_read__2",
        "Bit",
        "_get_GICD_CTLR_Type_DS",
        "AArch32_IMPDEFResets",
        "_get_PMCR_Type_IMP",
        "integer_subrange",
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
    ];

    if FN_DENYLIST.contains(&name.as_ref()) {
        return false;
    }

    FN_ALLOWLIST.contains(&name.as_ref())
        || FN_TOPLEVEL.contains(&name.as_ref())
        || name.as_ref().ends_with("_read")
        || name.as_ref().ends_with("_write")
        || name.as_ref().starts_with("Mk")
        || name.as_ref().starts_with("_update_")
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
