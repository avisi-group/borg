#![no_std]

extern crate alloc;

use {
    alloc::boxed::Box,
    borealis_register_init::borealis_register_init,
    common::{
        lookup_register_by_offset, State, Structe2f620c8eb69267c, Tracer, REG_PSTATE, REG_U_PC,
    },
    core::fmt::Debug,
    log::trace,
    plugins_rt::api::guest::Environment,
    step_model::step_model,
    u__FetchInstr::u__FetchInstr,
    u__InitSystem::u__InitSystem,
    u__SetConfig::u__SetConfig,
    ThisInstrAddr::ThisInstrAddr,
};

#[derive(Debug)]
pub enum TracerKind {
    Noop,
    Log,
    Sail,
    Qemu,
}

#[derive(Debug)]
pub struct Aarch64Interpreter {
    instructions_retired: u64,
    state: State,
    tracer_kind: TracerKind,
}

impl Aarch64Interpreter {
    pub fn new(
        initial_pc: u64,
        tracer_kind: TracerKind,
        environment: Box<dyn Environment>,
    ) -> Self {
        let mut state = State::new(environment);

        // sets initial register and letbind state (generated from sail model)
        borealis_register_init(&mut state, &NoopTracer);

        // actual ARM model function called in elfmain to initialize system
        u__InitSystem(&mut state, &NoopTracer, ());

        // from boot.sh command line args to `armv9` binary
        u__SetConfig(&mut state, &NoopTracer, "cpu.cpu0.RVBAR", 0x8000_0000);
        u__SetConfig(&mut state, &NoopTracer, "cpu.has_tlb", 0x0);

        state.write_register(REG_U_PC, initial_pc);

        // X0 must contain phys address of DTB https://docs.kernel.org/arch/arm64/booting.html
        // probably doesn't belong here as aarch64 guest shouldn't be linux
        // specific
        // state.write_register(REG_R0, dtb_phys_address);
        // 2024-06-01 this is done by the bootloader so not needed here

        Self {
            instructions_retired: 0,
            state,
            tracer_kind,
        }
    }

    pub fn run(&mut self) {
        loop {
            let pc =
                u64::try_from(ThisInstrAddr(&mut self.state, &NoopTracer, 64).value()).unwrap();

            let insn_data = u__FetchInstr(&mut self.state, &NoopTracer, pc)
                .tuple__pcnt_enum_z__InstrEnc__pcnt_bv321;

            // if self.instructions_retired == 3854400 {
            //     self.tracer_kind = TracerKind::Log;
            // }

            // if self.instructions_retired == 2000 {
            //     panic!();
            // }

            // monomorphization goes brrr, only seems to add around 10% to compilation time
            // but saves recompilation when changing tracer
            //
            // todo: expand this with a "detailed" mode where all statements in all blocks
            // are traced
            match self.tracer_kind {
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
                TracerKind::Sail => {
                    let nzcv = {
                        let pstate = self
                            .state
                            .read_register::<Structe2f620c8eb69267c>(REG_PSTATE);

                        (pstate.N as u8) << 3
                            | (pstate.Z as u8) << 2
                            | (pstate.C as u8) << 1
                            | (pstate.V as u8)
                    };

                    trace!(
                        "[Sail] {} PC={:#x} NZCV={:#x}",
                        self.instructions_retired + 1,
                        pc,
                        nzcv,
                    );

                    // print all registers
                    trace!(
                        "R00 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R0)
                    );
                    trace!(
                        "R01 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R1)
                    );
                    trace!(
                        "R02 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R2)
                    );
                    trace!(
                        "R03 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R3)
                    );
                    trace!(
                        "R04 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R4)
                    );
                    trace!(
                        "R05 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R5)
                    );
                    trace!(
                        "R06 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R6)
                    );
                    trace!(
                        "R07 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R7)
                    );
                    trace!(
                        "R08 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R8)
                    );
                    trace!(
                        "R09 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R9)
                    );
                    trace!(
                        "R10 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R10)
                    );
                    trace!(
                        "R11 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R11)
                    );
                    trace!(
                        "R12 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R12)
                    );
                    trace!(
                        "R13 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R13)
                    );
                    trace!(
                        "R14 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R14)
                    );
                    trace!(
                        "R15 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R15)
                    );
                    trace!(
                        "R16 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R16)
                    );
                    trace!(
                        "R17 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R17)
                    );
                    trace!(
                        "R18 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R18)
                    );
                    trace!(
                        "R19 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R19)
                    );
                    trace!(
                        "R20 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R20)
                    );
                    trace!(
                        "R21 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R21)
                    );
                    trace!(
                        "R22 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R22)
                    );
                    trace!(
                        "R23 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R23)
                    );
                    trace!(
                        "R24 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R24)
                    );
                    trace!(
                        "R25 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R25)
                    );
                    trace!(
                        "R26 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R26)
                    );
                    trace!(
                        "R27 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R27)
                    );
                    trace!(
                        "R28 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R28)
                    );
                    trace!(
                        "R29 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R29)
                    );
                    trace!(
                        "R30 = {:016x}",
                        self.state.read_register::<u64>(common::REG_R30)
                    );

                    step_model(&mut self.state, &SailTracer, ());
                }
                TracerKind::Qemu => {
                    unimplemented!("QEMU-style tracing output not supported")
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
    fn read_register(&self, _: usize, _: &dyn Debug) {}
    fn write_register(&self, _: usize, _: &dyn Debug) {}
    fn read_memory(&self, _: usize, _: &dyn Debug) {}
    fn write_memory(&self, _: usize, _: &dyn Debug) {}
}

struct LogTracer;

impl Tracer for LogTracer {
    fn begin(&self, instruction: u32, pc: u64) {
        trace!("[{pc:x}] {instruction:08x}");
    }

    fn end(&self) {
        trace!("");
    }

    fn read_register(&self, offset: usize, value: &dyn Debug) {
        let reg = lookup_register_by_offset(offset).unwrap();
        trace!("    R[{}:{:x}] -> {value:x?}", reg.name, reg.offset);
    }

    fn write_register(&self, offset: usize, value: &dyn Debug) {
        let reg = lookup_register_by_offset(offset).unwrap();
        trace!("    R[{}:{:x}] <- {value:x?}", reg.name, reg.offset);
    }

    fn read_memory(&self, address: usize, value: &dyn Debug) {
        trace!("    M[{address:x}] -> {value:?}");
    }

    fn write_memory(&self, address: usize, value: &dyn Debug) {
        trace!("    M[{address:x}] <- {value:?}");
    }
}

struct SailTracer;

impl Tracer for SailTracer {
    fn begin(&self, _: u32, _: u64) {}
    fn end(&self) {}
    fn read_register(&self, _: usize, _: &dyn Debug) {}
    fn write_register(&self, _: usize, _: &dyn Debug) {}
    fn read_memory(&self, _address: usize, _value: &dyn Debug) {
        // uncomment for mem tracing
        //  trace!("[Sail] mem {:016x?} -> {:016x?}", _address, _value);
    }
    fn write_memory(&self, _address: usize, _value: &dyn Debug) {
        // uncomment for mem tracing
        //  trace!("[Sail] mem {:016x?} <- {:016x?}", _address, _value);
    }
}
