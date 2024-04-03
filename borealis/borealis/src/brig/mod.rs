//! Rust module generation

use {
    crate::{
        boom::{
            passes::{
                self, cycle_finder::CycleFinder, fold_unconditionals::FoldUnconditionals,
                make_exception_bool::MakeExceptionBool, remove_const_branch::RemoveConstBranch,
                resolve_return_assigns::ResolveReturns,
            },
            Ast,
        },
        brig::{bundle::codegen_bundle, functions::codegen_functions, state::codegen_state},
        rudder,
    },
    common::create_file,
    log::{info, warn},
    proc_macro2::TokenStream,
    quote::quote,
    sailrs::{jib_ast, types::ListVec},
    std::io::Write,
};

mod bundle;
mod functions;
mod state;

const ENTRYPOINT: &str = "__DecodeA64";

const FN_ALLOWLIST: &[&str] = &[
    ENTRYPOINT,
    "__DecodeA64_DataProcImm",
    "decode_add_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate",
    "place_slice",
    "slice_mask",
    "sail_ones",
    "extzv",
    "execute_aarch64_instrs_integer_arithmetic_add_sub_immediate",
    "X_read",
    "neq_int",
    "get_R",
    "read_gpr",
    "AddWithCarry",
    "integer_subrange",
    "IsZero",
    "X_set",
    "set_R",
    "write_gpr",
    "decode_movz_aarch64_instrs_integer_ins_ext_insert_movewide",
    "execute_aarch64_instrs_integer_ins_ext_insert_movewide",
    "Zeros",
    "__id",
    "__DecodeA64_DataProcReg",
    "decode_subs_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
    "DecodeShift",
    "execute_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
    "ShiftReg",
    "__DecodeA64_BranchExcSys",
    "decode_b_cond_aarch64_instrs_branch_conditional_cond",
    "execute_aarch64_instrs_branch_conditional_cond",
    "ConditionHolds",
    "BranchNotTaken",
    "HaveStatisticalProfiling",
    "IsFeatureImplemented",
    "num_of_Feature",
    "decode_add_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg",
    "decode_orr_log_shift_aarch64_instrs_integer_logical_shiftedreg",
    "execute_aarch64_instrs_integer_logical_shiftedreg",
    "decode_b_uncond_aarch64_instrs_branch_unconditional_immediate",
    "execute_aarch64_instrs_branch_unconditional_immediate",
    "PC_read",
    "BranchTo",
    "Hint_Branch",
    "UsingAArch32",
    "HaveAArch32",
    "HaveAArch64",
    "AArch64_BranchAddr",
    "AddrTop",
    "HaveEL",
    "S1TranslationRegime",
    "ELUsingAArch32",
    "IsSecureBelowEL3",
    "SCR_GEN_read",
    "Mk_SCRType",
    "_get_SCRType_NS",
    "ELStateUsingAArch32",
    "ELStateUsingAArch32K",
    "HaveAArch32EL",
    "HaveVirtHostExt",
    "EffectiveTBI",
    "_get_TCR_EL1_Type_TBI0",
    "HavePACExt",
    "HaveBRBExt",
    "decode_svc_aarch64_instrs_system_exceptions_runtime_svc",
    "execute_aarch64_instrs_system_exceptions_runtime_svc",
    "AArch64_CheckForSVCTrap",
    "HaveFGTExt",
    "AArch64_CallSupervisor",
    "SSAdvance",
    "decode_hvc_aarch64_instrs_system_exceptions_runtime_hvc",
    "DebugTarget",
    "CurrentSecurityState",
    "SecurityStateAtEL",
    "HaveRME",
    "DebugTargetFrom",
    "HaveSecureEL2Ext",
    "_get_MDSCR_EL1_Type_SS",
    "EL2Enabled",
    "decode_ccmp_imm_aarch64_instrs_integer_conditional_compare_immediate",
    "IsSecureEL2Enabled",
    "execute_aarch64_instrs_integer_conditional_compare_immediate",
    "decode_bl_aarch64_instrs_branch_unconditional_immediate",
    "HaveGCS",
    "decode_mrs_aarch64_instrs_system_register_system",
    "NextInstrAddr",
    "ThisInstrLength",
    "AArch64_CheckSystemAccess",
    "HaveBTIExt",
    "fdiv_int",
    "HaveTME",
    "execute_aarch64_instrs_system_register_system",
    "AArch64_SysRegRead",
    "AArch64_AutoGen_SysRegRead",
    "CurrentEL_SysRegRead_94be544516d41cc8",
    "_get_HCR_EL2_Type_NV",
    "AArch64_CheckNVCondsIfCurrentEL",
    "__UNKNOWN_boolean",
    "decode_subs_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate",
    "SCTLR_EL1_SysRegRead_ce1d601189e8030e",
    "_get_HCR_EL2_Type_TRVM",
    "_get_SCR_EL3_Type_FGTEn",
    "_get_HFGRTR_EL2_Type_SCTLR_EL1",
    "_get_HCR_EL2_Type_E2H",
    "decode_tbnz_aarch64_instrs_branch_conditional_test",
    "execute_aarch64_instrs_branch_conditional_test",
    "decode_ands_log_imm_aarch64_instrs_integer_logical_immediate",
    "DecodeBitMasks",
    "_get_SCR_EL3_Type_NS",
    "HighestSetBit",
    "zext_ones",
    "extsv",
    "ROR",
];

/// Compiles a Sail model to a Brig module
pub fn sail_to_brig<I: Iterator<Item = jib_ast::Definition>>(
    jib_ast: I,
    standalone: bool,
) -> TokenStream {
    info!("Converting JIB to BOOM");
    let ast = Ast::from_jib(apply_fn_allowlist(jib_ast));

    // useful for debugging
    crate::boom::pretty_print::print_ast(
        &mut create_file("target/ast_raw.boom").unwrap(),
        ast.clone(),
    );

    info!("Running passes on BOOM");
    passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            ResolveReturns::new_boxed(),
            MakeExceptionBool::new_boxed(),
            CycleFinder::new_boxed(),
        ],
    );

    // useful for debugging
    crate::boom::pretty_print::print_ast(
        &mut create_file("target/ast_processed.boom").unwrap(),
        ast.clone(),
    );

    info!("Building rudder");

    let mut rudder = rudder::build::from_boom(&ast.borrow());

    writeln!(&mut create_file("target/ast.rudder").unwrap(), "{rudder}").unwrap();

    info!("Validating rudder");
    let msgs = rudder.validate();
    for msg in msgs {
        warn!("{msg}");
    }

    info!("Optimising rudder");
    rudder.optimise(rudder::opt::OptLevel::Level3);

    writeln!(
        &mut create_file("target/ast.opt.rudder").unwrap(),
        "{rudder}"
    )
    .unwrap();

    info!("Validating rudder again");
    let msgs = rudder.validate();
    for msg in msgs {
        warn!("{msg}");
    }

    info!("Generating Rust");

    let state = codegen_state(&rudder);

    let body = codegen_functions(rudder, ENTRYPOINT.into());

    let prelude = if standalone {
        quote! {
            extern crate alloc;

            struct PrintlnTracer;

            impl Tracer for PrintlnTracer {
                fn begin(&self, pc: u64) {
                    println!("begin @ {pc:x}");
                }

                fn end(&self) {
                    println!("end");
                }

                fn read_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
                    println!("read-register {offset:x} = {value:?}");
                }
                fn write_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
                    println!("write-register {offset:x} = {value:?}");
                }
            }

            fn main() {
                let arg = std::env::args().skip(1).next().unwrap();
                let text = std::fs::read(arg).unwrap();

                let mut state = State::init();

                loop {
                    let pc = state.read_register::<u64>(REG_U_PC);
                    let insr = unsafe { *(text.as_ptr().offset(pc as isize) as *mut u32) };
                    dbg!(decode_execute(insr, &mut state, &mut PrintlnTracer));
                }
            }
        }
    } else {
        quote! {
            use super::{CoreState, ExecutionEngine};

            pub struct Interpreter;

            impl CoreState for State {
                fn pc(&self) -> usize {
                    self.read_register(REG_U_PC)
                }

                fn new(pc: usize) -> Self {
                    let mut celf = State::init();

                    celf.write_register(REG_U_PC, pc);

                    celf
                }
            }

            struct LogTracer;

            impl Tracer for LogTracer {
                fn begin(&self, pc: u64) {
                    log::trace!("begin @ {pc:x}");
                }

                fn end(&self) {
                    log::trace!("end");
                }

                fn read_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
                    log::trace!("read-register {offset:x} = {value:?}");
                }
                fn write_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
                    log::trace!("write-register {offset:x} = {value:?}");
                }
            }


            fn fetch(pc: usize) -> u32 {
                unsafe { *(pc as *const u32) }
            }

            impl ExecutionEngine<State> for Interpreter {
                fn step(amount: super::StepAmount, state: &mut State) -> super::StepResult {
                    let insn_data = fetch(state.pc());
                    log::trace!("fetch @ {} = {:08x}", state.pc(), insn_data);

                    match decode_execute(insn_data, state, &mut LogTracer) {
                        ExecuteResult::Ok => {
                                                       super::StepResult::Ok
                        },
                        ExecuteResult::EndOfBlock => {
                            super::StepResult::Ok
                        },
                        ExecuteResult::UndefinedInstruction => {
                            panic!("undefined instruction {:08x}", insn_data)
                        }
                    }
                }
            }
        }
    };

    let bundle = codegen_bundle();

    quote! {
        #![allow(non_snake_case)]
        #![allow(unused_assignments)]
        #![allow(unused_mut)]
        #![allow(unused_parens)]
        #![allow(unused_variables)]
        #![allow(dead_code)]
        #![allow(unreachable_code)]
        #![allow(unused_doc_comments)]

        //! BOREALIS GENERATED FILE DO NOT MODIFY

        #state

        trait Tracer {
            fn begin(&self, pc: u64);
            fn end(&self);
            fn read_register<T: core::fmt::Debug>(&self, offset: usize, value: T);
            fn write_register<T: core::fmt::Debug>(&self, offset: usize, value: T);
        }

        #[derive(Debug)]
        enum ExecuteResult {
            Ok,
            EndOfBlock,
            UndefinedInstruction
        }

        #bundle

        #prelude

        #body
    }
}

/// Stub out functions not in the allowlist
fn apply_fn_allowlist<I: Iterator<Item = jib_ast::Definition>>(
    iter: I,
) -> impl Iterator<Item = jib_ast::Definition> {
    use sailrs::{
        jib_ast::{Definition, Instruction, InstructionAux, Type, Value, Vl},
        sail_ast::Location,
    };

    iter.map(|def| {
        if let Definition::Fundef(name, idk, arguments, body) = def {
            let body = if FN_ALLOWLIST.contains(&name.as_interned().as_ref()) {
                body
            } else {
                ListVec::from(vec![
                    // throw so that panic is created in rudder
                    Instruction {
                        inner: InstructionAux::Throw(Value::Lit(Vl::Unit, Type::Unit)),
                        annot: (0, Location::Unknown),
                    },
                ])
            };

            Definition::Fundef(name, idk, arguments, body)
        } else {
            def
        }
    })
}
