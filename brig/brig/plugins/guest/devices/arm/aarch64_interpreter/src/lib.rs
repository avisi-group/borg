#![no_std]

use {
    borealis_register_init::borealis_register_init,
    common::{State, Tracer, REGISTER_NAME_MAP, REG_R0, REG_U_PC},
    core::fmt::Debug,
    log::trace,
    step_model::step_model,
    u__InitSystem::u__InitSystem,
};

pub enum TracerKind {
    Noop,
    Log,
}

pub struct Aarch64Interpreter {
    instructions_retired: u64,
    state: State,
    tracer: TracerKind,
}

impl Aarch64Interpreter {
    pub fn new(
        guest_memory_base: usize,
        initial_pc: usize,
        dtb_phys_address: usize,
        tracer: TracerKind,
    ) -> Self {
        let mut state = State::init(guest_memory_base);

        // sets initial register and letbind state (generated from sail model)
        borealis_register_init(&mut state, &LogTracer);

        // actual ARM model function called in elfmain to initialize system
        u__InitSystem(&mut state, &LogTracer, ());

        state.write_register(REG_U_PC, initial_pc);

        // X0 must contain phys address of DTB https://docs.kernel.org/arch/arm64/booting.html
        // probably doesn't belong here as aarch64 guest shouldn't be linux
        // specific
        state.write_register(REG_R0, dtb_phys_address);

        Self {
            instructions_retired: 0,
            state,
            tracer,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.instructions_retired % 0x10_0000 == 0 {
                log::trace!("instrs: {:x}", self.instructions_retired);
            }

            let pc = self.state.read_register::<u64>(REG_U_PC);
            let insn_data =
                unsafe { *((pc + self.state.guest_memory_base() as u64) as *const u32) };

            // monomorphization goes brrr, only seems to add around 10% to compilation time
            // but saves recompilation when changing tracer
            //
            // todo: expand this with a "detailed" mode where all statements in all blocks
            // are traced
            match self.tracer {
                TracerKind::Noop => {
                    let tracer = &NoopTracer;
                    tracer.begin(insn_data, pc);
                    step_model(&mut self.state, tracer, ());
                    tracer.end();
                }
                TracerKind::Log => {
                    let tracer = &LogTracer;
                    tracer.begin(insn_data, pc);
                    step_model(&mut self.state, tracer, ());
                    tracer.end();
                }
            };

            self.instructions_retired += 1;
        }
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
        match REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                trace!("    R[{}] -> {value:x?}", REGISTER_NAME_MAP[idx].1)
            }
            // we're accessing inside a register
            Err(idx) => {
                // get the register and print the offset from the base
                let (register_offset, name) = REGISTER_NAME_MAP[idx - 1];
                trace!("    R[{name}:{:x}] -> {value:x?}", offset - register_offset);
            }
        }
    }

    fn write_register<T: Debug>(&self, offset: isize, value: T) {
        match REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                trace!("    R[{}] <- {value:x?}", REGISTER_NAME_MAP[idx].1)
            }
            Err(idx) => {
                let (register_offset, name) = REGISTER_NAME_MAP[idx - 1];
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
