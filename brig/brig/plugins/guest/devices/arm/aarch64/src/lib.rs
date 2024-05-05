#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, rc::Rc},
    arch::Tracer,
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

    plugins_rt::host::get().register_device("aarch64", Box::new(Aarch64InterpreterFactory));
}

struct Aarch64InterpreterFactory;

impl GuestDeviceFactory for Aarch64InterpreterFactory {
    // todo: find a way of passing some config to guest device creation: json?
    // key-value?
    fn create(&self) -> Box<dyn GuestDevice> {
        const GUEST_MEMORY_BASE: usize = 0;
        const INITIAL_PC: usize = 0x8000_0000;
        Box::new(Aarch64Interpreter::new(GUEST_MEMORY_BASE, INITIAL_PC))
    }
}

unsafe fn fetch_instruction(pc: usize) -> u32 {
    *(pc as *const u32)
}

struct Aarch64Interpreter {
    instructions_retired: u64,
    state: arch::State,
}

impl Aarch64Interpreter {
    pub fn new(guest_memory_base: usize, initial_pc: usize) -> Self {
        let mut state = arch::State::init(guest_memory_base);

        state.write_register(arch::REG_U_PC, initial_pc);

        Self {
            instructions_retired: 0,
            state,
        }
    }
}

// impl guestdevice for architectureexecutor?
impl GuestDevice for Aarch64Interpreter {
    fn start(&mut self) {
        // if engine.instructions_retired() % 100_000 == 0 {
        //                 let delta_time = unsafe { rdtsc() } - start_time;
        //                 log::trace!(
        //                     "{} {} {}",
        //                     engine.instructions_retired(),
        //                     delta_time,
        //                     engine.instructions_retired() / delta_time
        //                 );
        //             }
        loop {
            let pc = self.state.read_register(arch::REG_U_PC);
            let insn_data = unsafe { fetch_instruction(pc) };

            match arch::decode_execute(insn_data, &mut self.state, &LogTracer) {
                arch::ExecuteResult::Ok | arch::ExecuteResult::EndOfBlock => {
                    self.instructions_retired += 1;
                }

                arch::ExecuteResult::UndefinedInstruction => {
                    panic!("undefined instruction {:08x}", insn_data)
                }
            }
        }
    }
    fn stop(&mut self) {
        todo!()
    }
    fn as_io_handler(self: Box<Self>) -> Option<Box<dyn IOMemoryHandler>> {
        None
    }
}

struct LogTracer;

impl Tracer for LogTracer {
    fn begin(&self, instruction: u32, pc: u64) {
        trace!("[{instruction:x} @ {pc:x}] ");
    }

    fn end(&self) {
        trace!("");
    }

    fn read_register<T: Debug>(&self, offset: isize, value: T) {
        trace!("    R[{offset:x}] -> {value:?}");
    }

    fn write_register<T: Debug>(&self, offset: isize, value: T) {
        trace!("    R[{offset:x}] <- {value:?}");
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
