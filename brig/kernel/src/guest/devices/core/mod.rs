use core::arch::x86_64::__rdtscp;

use x86::time::rdtsc;

use crate::{
    guest::{devices::GuestDevice, memory::IOMemoryHandler},
    tasks::{create_task, Task},
};

pub trait CoreState {
    fn new(pc: usize) -> Self;
    fn pc(&self) -> usize;
}

pub enum StepAmount {
    Instruction,
    BasicBlock,
    Continuous,
}

pub enum StepResult {
    Ok,
    Halt,
}

pub trait ExecutionEngine<S: CoreState> {
    fn step(&mut self, amount: StepAmount, state: &mut S) -> StepResult;

    fn new() -> Self;

    fn instructions_retired(&self) -> u64;
}

pub struct GenericCore {
    execution_thread: Task,
}

impl GuestDevice for GenericCore {
    fn start(&self) {
        self.execution_thread.start();
    }

    fn stop(&self) {
        self.execution_thread.stop();
    }

    fn as_io_handler(self: alloc::rc::Rc<Self>) -> Option<alloc::rc::Rc<dyn IOMemoryHandler>> {
        None
    }
}

impl GenericCore {
    pub fn new<S: CoreState, E: ExecutionEngine<S>>() -> Self {
        Self {
            execution_thread: create_task(execution_thread::<S, E>),
        }
    }
}

fn execution_thread<S: CoreState, E: ExecutionEngine<S>>() {
    log::trace!("running guest core");

    let mut state = S::new(0x80000000);
    let mut engine = E::new();

    let start_time = unsafe { rdtsc() };

    loop {
        engine.step(StepAmount::Instruction, &mut state);
        if engine.instructions_retired() % 100_000 == 0 {
            let delta_time = unsafe { rdtsc() } - start_time;
            log::trace!(
                "{} {} {}",
                engine.instructions_retired(),
                delta_time,
                engine.instructions_retired() / delta_time
            );
        }
    }
}

impl CoreState for arch::State {
    fn pc(&self) -> usize {
        self.read_register(arch::REG_U_PC)
    }

    fn new(pc: usize) -> Self {
        let mut celf = Self::init();

        celf.write_register(arch::REG_U_PC, pc);

        celf
    }
}

struct LogTracer;

impl arch::Tracer for LogTracer {
    fn begin(&self, pc: u64) {
        // log::trace!("begin @ {pc:x}");
    }

    fn end(&self) {
        //log::trace!("end");
    }

    fn read_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
        //  log::trace!("read-register {offset:x} = {value:?}");
    }
    fn write_register<T: core::fmt::Debug>(&self, offset: usize, value: T) {
        // log::trace!("write-register {offset:x} = {value:?}");
    }
}

fn fetch(pc: usize) -> u32 {
    unsafe { *(pc as *const u32) }
}

pub struct Interpreter {
    instructions_retired: u64,
}

impl ExecutionEngine<arch::State> for Interpreter {
    fn step(&mut self, amount: StepAmount, state: &mut arch::State) -> StepResult {
        let insn_data = fetch(state.pc());
        // log::trace!("fetch @ {:x} = {:08x}", state.pc(), insn_data);

        match arch::decode_execute(insn_data, state, &mut LogTracer) {
            arch::ExecuteResult::Ok | arch::ExecuteResult::EndOfBlock => {
                self.instructions_retired += 1;
                StepResult::Ok
            }

            arch::ExecuteResult::UndefinedInstruction => {
                panic!("undefined instruction {:08x}", insn_data)
            }
        }
    }

    fn new() -> Self {
        Self {
            instructions_retired: 0,
        }
    }

    fn instructions_retired(&self) -> u64 {
        self.instructions_retired
    }
}
