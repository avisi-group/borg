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
        brig::codegen::codegen,
        rudder,
    },
    common::create_file,
    log::info,
    proc_macro2::TokenStream,
    quote::quote,
    sailrs::{jib_ast, types::ListVec},
    std::io::Write,
};

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
];

mod codegen;

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

    let rudder = rudder::build::from_boom(&ast.borrow());

    writeln!(&mut create_file("target/ast.rudder").unwrap(), "{rudder}").unwrap();

    info!("Generating Rust");

    let registers_len = {
        let (last_register, last_offset) = rudder
            .get_registers()
            .into_iter()
            .max_by_key(|(typ, offset)| offset + typ.width_bytes())
            .unwrap();

        last_offset + last_register.width_bytes()
    };

    let state = quote! {
        // todo check this is necessary
        #[repr(align(8))]
        pub struct State {
            pc: u64,
            data: [u8; #registers_len],
        }

        impl Default for State {
            fn default() -> Self {
                Self {
                    pc: 0,
                    data: [0; #registers_len],
                }
            }
        }
    };

    let body = codegen(rudder, ENTRYPOINT.into());

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

                let mut state = State::default();

                // set enabled features
                unsafe {
                    *(state.data.as_mut_ptr().byte_offset(0x18ed0) as *mut [bool; 259]) = [
                        false, false, false, false, true, true, true, false, false, true, true, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false,
                    ]
                };

                loop {
                    let pc_ptr = unsafe { (state.data.as_mut_ptr().byte_offset(12704) as *mut u64) };
                    let pc = dbg!(unsafe { *pc_ptr });
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
                    unsafe { *(self.data.as_ptr().byte_offset(12704) as *const usize) }
                }

                fn new(pc: usize) -> Self {
                    let mut celf = State::default();
                    unsafe { *(celf.data.as_mut_ptr().byte_offset(12704) as *mut usize) = pc };

                    // set enabled features
                    unsafe {
                        *(celf.data.as_mut_ptr().byte_offset(0x18ed0) as *mut [bool; 259]) = [
                            false, false, false, false, true, true, true, false, false, true, true, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false,
                        ]
                    };

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

    quote! {
        #![allow(non_snake_case)]
        #![allow(unused_assignments)]
        #![allow(unused_mut)]
        #![allow(unused_parens)]
        #![allow(unused_variables)]
        #![allow(dead_code)]
        #![allow(unreachable_code)]

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

        #[derive(Default, Clone, Copy, Debug)]
        struct Bundle<V, L> {
            value: V,
            length: L,
        }

        impl<V: core::ops::BitAnd<Output = V>, L> core::ops::BitAnd for Bundle<V, L> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value & rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::BitOr<Output = V>, L> core::ops::BitOr for Bundle<V, L> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value | rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::BitXor<Output = V>, L> core::ops::BitXor for Bundle<V, L> {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value ^ rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Shl<Output = V>, L> core::ops::Shl for Bundle<V, L> {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value << rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Shr<Output = V>, L> core::ops::Shr for Bundle<V, L> {
            type Output = Self;

            fn shr(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value >> rhs.value,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Sub for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Sub<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) - core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Add for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Add<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) + core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Div for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Div<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) / core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Not<Output = V>, L> core::ops::Not for Bundle<V, L> {
            type Output = Self;
            fn not(self) -> Self {
                Self {
                    value: !self.value,
                    length: self.length
                }
            }
        }

        impl<V: core::cmp::PartialOrd, L> core::cmp::PartialOrd for Bundle<V, L> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<V: core::cmp::PartialEq, L> core::cmp::PartialEq for Bundle<V, L> {
            fn eq(&self, other: &Self) -> bool {
                self.value.eq(&other.value)
            }
        }

        impl<V: core::cmp::PartialEq, L> core::cmp::Eq for Bundle<V, L> {}

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
                    Instruction {
                        inner: InstructionAux::Throw(Value::Lit(Vl::Unit, Type::Unit)),
                        annot: (0, Location::Unknown),
                    },
                    Instruction {
                        inner: InstructionAux::Undefined(Type::Unit),
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
