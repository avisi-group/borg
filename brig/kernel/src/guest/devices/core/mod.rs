use crate::{
    guest::{devices::GuestDevice, memory::IOMemoryHandler},
    tasks::{create_task, Task},
};

pub mod aarch64;

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
    fn step(amount: StepAmount, state: &mut S) -> StepResult;
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
    loop {
        E::step(StepAmount::Instruction, &mut state);
    }
}

// struct Interpreter<S> {
//     state: S,
// }

// impl<I: Interpeter<A>, A: Architecture> Interpreter<A::State> {
//     pub fn new() -> Self {
//         Self {
//             state: A::initial_state(),
//         }
//     }

//     pub fn run(self) {
//         loop {
//             A::step_instr(self.state)
//         }
//     }
// }
