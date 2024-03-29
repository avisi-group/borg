//! Rust module generation

use {
    crate::{
        boom::{
            passes::{
                self, builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
                fold_unconditionals::FoldUnconditionals, make_exception_bool::MakeExceptionBool,
                remove_const_branch::RemoveConstBranch, resolve_bitvectors::ResolveBitvectors,
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
            // ResolveBitvectors::new_boxed(),
            // AddBuiltinFns::new_boxed(),
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
            struct ConsoleTracer;

            impl Tracer for ConsoleTracer {
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
                let instruction = u32::from_str_radix(arg.trim_start_matches("0x"), 16).unwrap();
                dbg!(decode_execute(instruction, &mut State::default(), &mut ConsoleTracer));
            }
        }
    } else {
        quote! {
            use super::{CoreState, ExecutionEngine};

            pub struct AArch64Interpreter;

            impl CoreState for State {
                fn pc(&self) -> usize {
                    self.pc as usize
                }

                fn new(pc: usize) -> Self {
                    Self { pc: pc as u64, sp: 0, x: [0; 31] }
                }
            }

            fn fetch(pc: usize) -> u32 {
                unsafe { *(pc as *const u32) }
            }

            impl ExecutionEngine<State> for AArch64Interpreter {
                fn step(amount: super::StepAmount, state: &mut State) -> super::StepResult {
                    let insn_data = fetch(state.pc());
                    log::trace!("fetch @ {} = {:08x}", state.pc(), insn_data);

                    match decode_execute(insn_data, state) {
                        ExecuteResult::Ok => {
                            state.pc += 4;
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


        impl<V: core::ops::Sub<Output = V>, L> core::ops::Sub for Bundle<V, L> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value - rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Add<Output = V>, L> core::ops::Add for Bundle<V, L> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value + rhs.value,
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

        impl<V: core::cmp::PartialEq, L> core::cmp::Eq for Bundle<V, L> {

        }




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
