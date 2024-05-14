#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String},
    arch::{decode_execute, ExecuteResult, Tracer, REG_U_PC},
    core::fmt::Debug,
    log::trace,
    plugins_rt::api::{GuestDevice, GuestDeviceFactory, IOMemoryHandler, PluginHeader, PluginHost},
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "aarch64",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);
    log::info!("loading aarch64");

    plugins_rt::get_host().register_device("aarch64", Box::new(Aarch64InterpreterFactory));
}

struct Aarch64InterpreterFactory;

impl GuestDeviceFactory for Aarch64InterpreterFactory {
    // todo: find a way of passing some config to guest device creation: json?
    // key-value?
    fn create(&self, config: BTreeMap<String, String>) -> Box<dyn GuestDevice> {
        const GUEST_MEMORY_BASE: usize = 0;
        const INITIAL_PC: usize = 0x4020_0000;

        let tracer = match config.get("tracer").map(String::as_str) {
            Some("log") => TracerKind::Log(LogTracer),
            Some("noop") | None => TracerKind::Noop(NoopTracer),
            Some(t) => panic!("unknown tracer {t:?}"),
        };

        Box::new(Aarch64Interpreter::new(
            GUEST_MEMORY_BASE,
            INITIAL_PC,
            tracer,
        ))
    }
}

unsafe fn fetch_instruction(pc: usize) -> u32 {
    *(pc as *const u32)
}

enum TracerKind {
    Noop(NoopTracer),
    Log(LogTracer),
}

struct Aarch64Interpreter {
    instructions_retired: u64,
    state: arch::State,
    tracer: TracerKind,
}

impl Aarch64Interpreter {
    pub fn new(guest_memory_base: usize, initial_pc: usize, tracer: TracerKind) -> Self {
        let mut state = arch::State::init(guest_memory_base);

        state.write_register(arch::REG_U_PC, initial_pc);

        state.write_register(arch::REG_MIDR_EL1, 0x410f_0000u32);
        state.write_register(arch::REG_CLIDR_EL1, 0x4u32);
        state.write_register(arch::REG_CCSIDR_EL1, 0x4000u32);
        state.write_register(arch::REG_MPIDR_EL1, 0x4000_0000u32);
        state.write_register(arch::REG_DCZID_EL0, 0x11u32);
        state.write_register(arch::REG_CTR_EL0, 0x0444_c004u32);
        state.write_register(arch::REG_ID_AA64PFR0_EL1, 0x11u32);
        state.write_register(arch::REG_ID_AA64DFR0_EL1, 0x1010_1606u32);
        state.write_register(arch::REG_ID_AA64MMFR0_EL1, 0x0f10_0000u32);

        Self {
            instructions_retired: 0,
            state,
            tracer,
        }
    }
}

// impl guestdevice for architectureexecutor?
impl GuestDevice for Aarch64Interpreter {
    fn start(&mut self) {
        let mut instrs_retired: u64 = 0;
        loop {
            if instrs_retired % 0x10_0000 == 0 {
                log::trace!("instrs: {instrs_retired:x}");
            }

            let pc = self.state.read_register(REG_U_PC);
            let insn_data = unsafe { fetch_instruction(pc) };

            // monomorphization goes brrr, only seems to add around 10% to compilation time
            // but saves recompilation when changing tracer todo expand this
            // with a "detailed" mode where all statements in all blocks are traced
            let exec_result = match &self.tracer {
                TracerKind::Noop(tracer) => decode_execute(insn_data, &mut self.state, tracer),
                TracerKind::Log(tracer) => decode_execute(insn_data, &mut self.state, tracer),
            };

            match exec_result {
                ExecuteResult::Ok | ExecuteResult::EndOfBlock => {
                    self.instructions_retired += 1;
                }

                ExecuteResult::UndefinedInstruction => {
                    panic!("undefined instruction {:08x}", insn_data)
                }
            }

            instrs_retired += 1;
        }
    }
    fn stop(&mut self) {
        todo!()
    }
    fn as_io_handler(self: Box<Self>) -> Option<Box<dyn IOMemoryHandler>> {
        None
    }
}

struct NoopTracer;

impl Tracer for NoopTracer {
    fn begin(&self, _: u32, _: u64) {}

    fn end(&self) {}

    fn read_register<T: core::fmt::Debug>(&self, _: isize, _: T) {}

    fn write_register<T: core::fmt::Debug>(&self, _: isize, _: T) {}

    fn read_memory<T: core::fmt::Debug>(&self, _: usize, _: T) {}

    fn write_memory<T: core::fmt::Debug>(&self, _: usize, _: T) {}
}

struct LogTracer;

impl Tracer for LogTracer {
    fn begin(&self, instruction: u32, pc: u64) {
        trace!("[{pc:x}] {instruction:08x}");
    }

    fn end(&self) {
        trace!("");
    }

    fn read_register<T: Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                trace!("    R[{}] -> {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            // we're accessing inside a register
            Err(idx) => {
                // get the register and print the offset from the base
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                trace!("    R[{name}:{:x}] -> {value:x?}", offset - register_offset);
            }
        }
    }

    fn write_register<T: Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                trace!("    R[{}] <- {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            Err(idx) => {
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                trace!("    R[{name}:{:x}] <- {value:x?}", offset - register_offset);
            }
        }
    }

    fn read_memory<T: core::fmt::Debug>(&self, address: usize, value: T) {
        trace!("    M[{address:x}] -> {value:?}");
    }

    fn write_memory<T: core::fmt::Debug>(&self, address: usize, value: T) {
        trace!("    M[{address:x}] <- {value:?}");
    }
}

// impl ArchitectureExecutor for Aarch64Interpreter {
//     type State = arch::State;

//     fn step(&mut self, _amount: StepAmount, state: &mut Self::State) ->
// StepResult {         let insn_data = unsafe { fetch_instruction(state.pc())
// };

//         match arch::decode_execute(insn_data, state, &LogTracer) {
//             arch::ExecuteResult::Ok | arch::ExecuteResult::EndOfBlock => {
//                 self.instructions_retired += 1;
//                 StepResult::Ok
//             }

//             arch::ExecuteResult::UndefinedInstruction => {
//                 panic!("undefined instruction {:08x}", insn_data)
//             }
//         }
//     }

//     fn new() -> Self {
//         Self {
//             instructions_retired: 0,
//         }
//     }

//     fn instructions_retired(&self) -> u64 {
//         self.instructions_retired
//     }
// }

// struct LogTracer;

// impl Tracer for LogTracer {
//     fn begin(&self, instruction: u32, pc: u64) {
//         trace!("[{instruction:x} @ {pc:x}] ");
//     }

//     fn end(&self) {
//         trace!("");
//     }

//     fn read_register<T: Debug>(&self, offset: isize, value: T) {
//         trace!("    R[{offset:x}] -> {value:?}");
//     }

//     fn write_register<T: Debug>(&self, offset: isize, value: T) {
//         trace!("    R[{offset:x}] <- {value:?}");
//     }
// }

// use {
//     crate::{
//         guest::{devices::GuestDevice, memory::IOMemoryHandler,
// KERNEL_LOAD_BIAS},         tasks::{create_task, Task},
//     },
//     alloc::{boxed::Box, rc::Rc},
//     core::fmt::Debug,
//     log::trace,
//     x86::time::rdtsc,
// };

// pub trait CoreState {
//     fn new(pc: usize) -> Self;
//     fn pc(&self) -> usize;
// }

// pub enum StepAmount {
//     Instruction,
//     BasicBlock,
//     Continuous,
// }

// pub enum StepResult {
//     Ok,
//     Halt,
// }

// pub trait ExecutionEngine<S: CoreState> {
//     fn step(&mut self, amount: StepAmount, state: &mut S) -> StepResult;

//     fn new() -> Self;

//     fn instructions_retired(&self) -> u64;
// }

// pub struct ProcessingUnit {
//     execution_thread: Task,
// }

// impl GuestDevice for ProcessingUnit {
//     fn start(&self) {
//         self.execution_thread.start();
//     }

//     fn stop(&self) {
//         self.execution_thread.stop();
//     }

//     fn as_io_handler(self: Rc<Self>) -> Option<Rc<dyn IOMemoryHandler>> {
//         None
//     }
// }

// impl ProcessingUnit {
//     pub fn new<S: CoreState, E: ExecutionEngine<S>>() -> Self {
//         Self {
//             execution_thread: create_task(execution_thread::<S, E>),
//         }
//     }
// }

// fn execution_thread<S: CoreState, E: ExecutionEngine<S>>() {
//     log::trace!("running guest core");

//     // load arch library

//     let mut state = Box::new(S::new(KERNEL_LOAD_BIAS));
//     let mut engine = E::new();

//     let start_time = unsafe { rdtsc() };

//     loop {
//         engine.step(StepAmount::Instruction, &mut *state);
//         if engine.instructions_retired() % 100_000 == 0 {
//             let delta_time = unsafe { rdtsc() } - start_time;
//             log::trace!(
//                 "{} {} {}",
//                 engine.instructions_retired(),
//                 delta_time,
//                 engine.instructions_retired() / delta_time
//             );
//         }
//     }
// }

// impl CoreState for arch::State {
//     fn pc(&self) -> usize {
//         self.read_register(arch::REG_U_PC)
//     }

//     fn new(pc: usize) -> Self {
//         let mut celf = Self::init(0);

//         celf.write_register(arch::REG_U_PC, pc);

//         celf
//     }
// }

// struct LogTracer;

// impl arch::Tracer for LogTracer {
//     fn begin(&self, instruction: u32, pc: u64) {
//         trace!("[{instruction:x} @ {pc:x}] ");
//     }

//     fn end(&self) {
//         trace!("");
//     }

//     fn read_register<T: Debug>(&self, offset: isize, value: T) {
//         trace!("    R[{offset:x}] -> {value:?}");
//     }

//     fn write_register<T: Debug>(&self, offset: isize, value: T) {
//         trace!("    R[{offset:x}] <- {value:?}");
//     }
// }

// fn fetch(pc: usize) -> u32 {
//     unsafe { *(pc as *const u32) }
// }

// pub struct Interpreter {
//     instructions_retired: u64,
// }

// impl ExecutionEngine<arch::State> for Interpreter {
//     fn step(&mut self, _amount: StepAmount, state: &mut arch::State) ->
// StepResult {         let insn_data = fetch(state.pc());

//         match arch::decode_execute(insn_data, state, &LogTracer) {
//             arch::ExecuteResult::Ok | arch::ExecuteResult::EndOfBlock => {
//                 self.instructions_retired += 1;
//                 StepResult::Ok
//             }

//             arch::ExecuteResult::UndefinedInstruction => {
//                 panic!("undefined instruction {:08x}", insn_data)
//             }
//         }
//     }

//     fn new() -> Self {
//         Self {
//             instructions_retired: 0,
//         }
//     }

//     fn instructions_retired(&self) -> u64 {
//         self.instructions_retired
//     }
// }
