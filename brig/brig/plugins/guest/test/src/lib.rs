#![no_std]

use {
    arch::{Bits, ProductTypebc91b195b0b2a883, ProductTyped54bc449dd09e5bd, State, Tracer},
    borealis_register_init::borealis_register_init,
    core::fmt::Debug,
    plugins_rt::api::{PluginHeader, PluginHost},
    replicate_bits_borealis_internal::replicate_bits_borealis_internal,
    u__InitSystem::u__InitSystem,
    AddWithCarry::AddWithCarry,
    CeilPow2::CeilPow2,
    DecodeBitMasks::DecodeBitMasks,
    FloorPow2::FloorPow2,
    IsPow2::IsPow2,
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "test",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);
    log::info!("running tests");

    addwithcarry_negative();
    addwithcarry_zero();
    addwithcarry_carry();
    addwithcarry_overflow();
    addwithcarry_early_4880_loop();

    replicate_bits();
    ubfx();

    fibonacci();

    ispow2();

    log::info!("tests passed");
}

fn addwithcarry_negative() {
    let mut state = State::init(0x0);
    let tracer = NoopTracer;
    let x = Bits::new(0x0, 0x40);
    let y = Bits::new(-5i128 as u128, 0x40);
    let carry_in = false;

    assert_eq!(
        AddWithCarry(&mut state, &tracer, x, y, carry_in),
        ProductTyped54bc449dd09e5bd {
            _0: Bits::new(-5i64 as u128, 0x40),
            _1: 0b1000
        }
    );
}

fn addwithcarry_zero() {
    let mut state = State::init(0x0);
    let tracer = NoopTracer;
    let x = Bits::new(0x0, 0x40);
    let y = Bits::new(0x0, 0x40);
    let carry_in = false;

    assert_eq!(
        AddWithCarry(&mut state, &tracer, x, y, carry_in),
        ProductTyped54bc449dd09e5bd {
            _0: Bits::new(0x0, 0x40),
            _1: 0b0100
        }
    );
}

fn addwithcarry_carry() {
    let mut state = State::init(0x0);
    let tracer = NoopTracer;
    let x = Bits::new(u64::MAX as u128, 0x40);
    let y = Bits::new(0x1, 0x40);
    let carry_in = false;

    assert_eq!(
        AddWithCarry(&mut state, &tracer, x, y, carry_in),
        ProductTyped54bc449dd09e5bd {
            _0: Bits::new(0x0, 0x40),
            _1: 0b0110
        }
    );
}

fn addwithcarry_overflow() {
    let mut state = State::init(0x0);
    let tracer = NoopTracer;
    let x = Bits::new(u64::MAX as u128 / 2, 0x40);
    let y = Bits::new(u64::MAX as u128 / 2, 0x40);
    let carry_in = false;

    assert_eq!(
        AddWithCarry(&mut state, &tracer, x, y, carry_in),
        ProductTyped54bc449dd09e5bd {
            _0: Bits::new(!0x1, 0x40),
            _1: 0b1001
        }
    );
}

/// Testing the flags of the `0x0000000040234888:  eb01001f      cmp x0, x1`
/// instruction
fn addwithcarry_early_4880_loop() {
    let mut state = State::init(0x0);
    let tracer = NoopTracer;
    let x = Bits::new(0x425a6004, 0x40);
    let y = Bits::new(!0x425a6020, 0x40);
    let carry_in = false;

    assert_eq!(
        AddWithCarry(&mut state, &tracer, x, y, carry_in),
        ProductTyped54bc449dd09e5bd {
            _0: Bits::new(0xffffffffffffffe3, 0x40),
            _1: 0b1000
        }
    );
}

fn replicate_bits() {
    let mut state = State::init(0x0);
    assert_eq!(
        Bits::new(0xffff_ffff, 32),
        replicate_bits_borealis_internal(&mut state, &NoopTracer, Bits::new(0xff, 8), 4)
    );
    assert_eq!(
        Bits::new(0xaa, 8),
        replicate_bits_borealis_internal(&mut state, &NoopTracer, Bits::new(0xaa, 8), 1)
    );
    assert_eq!(
        Bits::new(0xaaaa, 16),
        replicate_bits_borealis_internal(&mut state, &NoopTracer, Bits::new(0xaa, 8), 2)
    );
    assert_eq!(
        Bits::new(0xffff_ffff, 32),
        replicate_bits_borealis_internal(&mut state, &NoopTracer, Bits::new(0x1, 1), 32)
    );
}

fn ubfx() {
    {
        let mut state = State::init(0x0);
        // decode bit masks
        assert_eq!(
            ProductTypebc91b195b0b2a883 {
                _0: Bits::new(0xFFFF00000000000F, 64),
                _1: Bits::new(0xF, 64)
            },
            DecodeBitMasks(&mut state, &NoopTracer, true, 0x13, 0x10, false, 0x40)
        );
    }

    {
        let mut state = State::init(0x0);
        state.write_register::<u64>(arch::REG_R3, 0x8444_c004);

        // ubfx x3, x3, #16, #4
        arch::decode_execute(0xd3504c63, &mut state, &NoopTracer);
        assert_eq!(0x4, state.read_register::<u64>(arch::REG_R3));
    }
}

fn fibonacci() {
    let mut state = State::init(0x0);
    borealis_register_init(&mut state, &LogTracer);
    // hacky, run sail function that goes before the main loop :/
    u__InitSystem(&mut state, &LogTracer, ());

    let program = [
        // <_start>
        0xd2800000, // mov     x0, #0x0 (#0)
        0xd2800021, // mov     x1, #0x1 (#1)
        0xd2800002, // mov     x2, #0x0 (#0)
        0xd2800003, // mov     x3, #0x0 (#0)
        0xd2800144, // mov     x4, #0xa (#10)
        // <loop>
        0xeb04007f, // cmp     x3, x4
        0x540000c0, // b.eq    400104 <done>  // b.none
        0x8b010002, // add     x2, x0, x1
        0xaa0103e0, // mov     x0, x1
        0xaa0203e1, // mov     x1, x2
        0x91000463, // add     x3, x3, #0x1
        0x17fffffa, // b       4000e8 <loop>
        // <done>
        0xaa0203e0, // mov     x0, x2
        0x52800ba8, // mov     w8, #0x5d (#93)
        0xd4000001, // svc     #0x0
    ];

    loop {
        let pc = state.read_register::<usize>(arch::REG_U_PC);

        // exit before the svc
        if pc == 0x38 {
            break;
        }

        let instr = program[pc / 4];
        arch::decode_execute(instr, &mut state, &NoopTracer);
    }

    assert_eq!(89, state.read_register::<u64>(arch::REG_R0));
    assert_eq!(10, state.read_register::<u64>(arch::REG_R3));
}

fn ispow2() {
    let mut state = State::init(0x0);
    let x = 2048i128;
    assert_eq!(
        FloorPow2(&mut state, &LogTracer, x),
        CeilPow2(&mut state, &LogTracer, x)
    );
    assert!(IsPow2(&mut state, &LogTracer, x));
}

struct NoopTracer;

impl Tracer for NoopTracer {
    fn begin(&self, _: u32, _: u64) {}

    fn end(&self) {}

    fn read_register<T: Debug>(&self, _: isize, _: T) {}

    fn write_register<T: Debug>(&self, _: isize, _: T) {}

    fn read_memory<T: Debug>(&self, _: usize, _: T) {}

    fn write_memory<T: Debug>(&self, _: usize, _: T) {}
}

struct LogTracer;

impl Tracer for LogTracer {
    fn begin(&self, instruction: u32, pc: u64) {
        log::trace!("[{pc:x}] {instruction:08x}");
    }

    fn end(&self) {
        log::trace!("");
    }

    fn read_register<T: core::fmt::Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                log::trace!("    R[{}] -> {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            // we're accessing inside a register
            Err(idx) => {
                // get the register and print the offset from the base
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                log::trace!("    R[{name}:{:x}] -> {value:x?}", offset - register_offset);
            }
        }
    }

    fn write_register<T: core::fmt::Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                log::trace!("    R[{}] <- {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            Err(idx) => {
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                log::trace!("    R[{name}:{:x}] <- {value:x?}", offset - register_offset);
            }
        }
    }

    fn read_memory<T: core::fmt::Debug>(&self, address: usize, value: T) {
        log::trace!("    M[{address:x}] -> {value:?}");
    }

    fn write_memory<T: core::fmt::Debug>(&self, address: usize, value: T) {
        log::trace!("    M[{address:x}] <- {value:?}");
    }
}
