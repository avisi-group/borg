use {
    crate::{
        guest::{devices::GuestDevice, memory::IOMemoryHandler, KERNEL_LOAD_BIAS},
        tasks::{create_task, Task},
    },
    alloc::rc::Rc,
    core::fmt::Debug,
    log::trace,
    x86::time::rdtsc,
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

    fn as_io_handler(self: Rc<Self>) -> Option<Rc<dyn IOMemoryHandler>> {
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

    let mut state = S::new(KERNEL_LOAD_BIAS);
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
        let mut celf = Self::init(0);

        celf.write_register(arch::REG_U_PC, pc);

        celf
    }
}

struct LogTracer;

impl arch::Tracer for LogTracer {
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

fn fetch(pc: usize) -> u32 {
    unsafe { *(pc as *const u32) }
}

pub struct Interpreter {
    instructions_retired: u64,
}

impl ExecutionEngine<arch::State> for Interpreter {
    fn step(&mut self, _amount: StepAmount, state: &mut arch::State) -> StepResult {
        let insn_data = fetch(state.pc());

        match arch::decode_execute(insn_data, state, &LogTracer) {
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
