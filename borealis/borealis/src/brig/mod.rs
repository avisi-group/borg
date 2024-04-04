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
        brig::{
            allowlist::apply_fn_allowlist, bundle::codegen_bundle,
            codegen_interpreter::codegen_functions, state::codegen_state,
        },
        rudder,
    },
    common::create_file,
    log::{info, warn},
    proc_macro2::TokenStream,
    quote::quote,
    sailrs::jib_ast,
    std::io::Write,
};

mod allowlist;
mod bundle;
mod codegen_dbt;
mod codegen_interpreter;
mod state;

const ENTRYPOINT: &str = "__DecodeA64";

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
