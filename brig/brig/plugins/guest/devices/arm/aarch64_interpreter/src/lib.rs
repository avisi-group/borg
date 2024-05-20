#![no_std]

use {
    arch::{
        decode_execute, ExecuteResult, ProductTypec98939056e929b9c, Tracer, REG_CCSIDR_EL1,
        REG_CLIDR_EL1, REG_CTR_EL0, REG_DCZID_EL0, REG_EL0, REG_EL1, REG_EL2, REG_EL3,
        REG_FEATUREIMPL, REG_ID_AA64DFR0_EL1, REG_ID_AA64MMFR0_EL1, REG_ID_AA64PFR0_EL1,
        REG_MIDR_EL1, REG_MPIDR_EL1, REG_PSTATE, REG_R0, REG_U_PC, REG_U__SUPPORTED_PA_SIZE,
        REG_U__SUPPORTED_VA_SIZE,
    },
    core::fmt::Debug,
    log::trace,
};

pub enum TracerKind {
    Noop,
    Log,
}

pub struct Aarch64Interpreter {
    instructions_retired: u64,
    state: arch::State,
    tracer: TracerKind,
}

impl Aarch64Interpreter {
    pub fn new(
        guest_memory_base: usize,
        initial_pc: usize,
        dtb_phys_address: usize,
        tracer: TracerKind,
    ) -> Self {
        let mut state = arch::State::init(guest_memory_base);

        state.write_register(REG_U_PC, initial_pc);

        // X0 must contain phys address of DTB https://docs.kernel.org/arch/arm64/booting.html
        // probably doesn't belong here as aarch64 guest shouldn't be linux
        // specific
        state.write_register(REG_R0, dtb_phys_address);

        state.write_register(REG_MIDR_EL1, 0x410f_0000u32);
        state.write_register(REG_CLIDR_EL1, 0x4u32);
        state.write_register(REG_CCSIDR_EL1, 0x4000u32);
        state.write_register(REG_MPIDR_EL1, 0x4000_0000u32);
        state.write_register(REG_DCZID_EL0, 0x11u32);
        state.write_register(REG_CTR_EL0, 0x0444_c004u32);
        state.write_register(REG_ID_AA64PFR0_EL1, 0x11u32);
        state.write_register(REG_ID_AA64DFR0_EL1, 0x1010_1606u32);
        state.write_register(REG_ID_AA64MMFR0_EL1, 0x0f10_0000u32);

        state.write_register(REG_U__SUPPORTED_PA_SIZE, 56u32);
        state.write_register(REG_U__SUPPORTED_VA_SIZE, 56u32);

        state.write_register(REG_EL0, 0u8);
        state.write_register(REG_EL1, 1u8);
        state.write_register(REG_EL2, 2u8);
        state.write_register(REG_EL3, 3u8);

        // set to EL1 on boot
        state.write_register(
            REG_PSTATE,
            ProductTypec98939056e929b9c {
                _0: false,
                _1: false,
                _2: 0,
                _3: false,
                _4: false,
                _5: false,
                _6: false,
                _7: 1,
                _8: false,
                _9: false,
                _10: 0,
                _11: false,
                _12: false,
                _13: 0,
                _14: false,
                _15: 0,
                _16: false,
                _17: false,
                _18: false,
                _19: false,
                _20: false,
                _21: false,
                _22: false,
                _23: false,
                _24: false,
                _25: false,
                _26: false,
                _27: false,
                _28: false,
                _29: false,
                _30: false,
                _31: false,
            },
        );

        let mut features = [true; 259];

        // register FEAT_AA32EL1_IMPLEMENTED : bool = false
        // register FEAT_AA32EL2_IMPLEMENTED : bool = false
        // register FEAT_AA32EL3_IMPLEMENTED : bool = false
        features[1] = false;
        features[2] = false;
        features[3] = false;

        // register FEAT_ETMv4_IMPLEMENTED : bool = false
        features[19] = false;

        // FEAT_CONSTPACFIELD_IMPLEMENTED
        features[87] = false;

        // register FEAT_EPAC_IMPLEMENTED : bool = false
        features[88] = false;

        // register FEAT_PACIMP_IMPLEMENTED : bool = false
        features[95] = false;

        // register FEAT_PACQARMA3_IMPLEMENTED : bool = false
        features[96] = false;

        // we don't want statistical profiling
        features[99] = false;
        features[164] = false;
        features[177] = false;
        features[216] = false;

        // magic Tom values
        state.write_register(REG_FEATUREIMPL, features);

        Self {
            instructions_retired: 0,
            state,
            tracer,
        }
    }

    pub fn run(&mut self) {
        let mut instrs_retired: u64 = 0;
        loop {
            if instrs_retired % 0x10_0000 == 0 {
                log::trace!("instrs: {instrs_retired:x}");
            }

            let pc = self.state.read_register::<usize>(REG_U_PC);
            let insn_data = unsafe { *((pc + self.state.guest_memory_base()) as *const u32) };

            // monomorphization goes brrr, only seems to add around 10% to compilation time
            // but saves recompilation when changing tracer todo expand this
            // with a "detailed" mode where all statements in all blocks are traced
            let exec_result = match self.tracer {
                TracerKind::Noop => decode_execute(insn_data, &mut self.state, &NoopTracer),
                TracerKind::Log => decode_execute(insn_data, &mut self.state, &LogTracer),
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
