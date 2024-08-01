#![no_std]

extern crate alloc;

use {
    alloc::boxed::Box,
    borealis_register_init::borealis_register_init,
    common::{
        Bits, ProductType188a1c3bf231c64b, ProductTypea79c7f841a890648, State, Tracer, REG_R0,
        REG_R1, REG_R19, REG_R2, REG_R3, REG_SEE, REG_U_PC, REG_U__BRANCHTAKEN,
    },
    core::fmt::Debug,
    execute_aarch64_instrs_integer_arithmetic_rev::execute_aarch64_instrs_integer_arithmetic_rev,
    place_slice_signed::place_slice_signed,
    plugins_rt::api::{guest::Environment, PluginHeader, PluginHost},
    replicate_bits_borealis_internal::replicate_bits_borealis_internal,
    u__DecodeA64::u__DecodeA64,
    u__InitSystem::u__InitSystem,
    AddWithCarry::AddWithCarry,
    CeilPow2::CeilPow2,
    DecodeBitMasks::DecodeBitMasks,
    FloorPow2::FloorPow2,
    IsPow2::IsPow2,
};

const TRACER: &NoneTracer = &NoneTracer;

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "test",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);
    log::info!("running tests");

    // #[test_case]
    // fn addwithcarry_negative()
    {
        let mut state = State::new(Box::new(NoneEnv));

        let x = Bits::new(0x0, 0x40);
        let y = Bits::new(-5i128 as u128, 0x40);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(-5i64 as u128, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b1000
            }
        );
    }

    // #[test_case]
    // fn addwithcarry_zero()
    {
        let mut state = State::new(Box::new(NoneEnv));
        let x = Bits::new(0x0, 0x40);
        let y = Bits::new(0x0, 0x40);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(0x0, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b0100
            }
        );
    }

    // #[test_case]
    // fn addwithcarry_carry()
    {
        let mut state = State::new(Box::new(NoneEnv));

        let x = Bits::new(u64::MAX as u128, 0x40);
        let y = Bits::new(0x1, 0x40);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(0x0, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b0110
            }
        );
    }

    // #[test_case]
    // fn addwithcarry_overflow()
    {
        let mut state = State::new(Box::new(NoneEnv));

        let x = Bits::new(u64::MAX as u128 / 2, 0x40);
        let y = Bits::new(u64::MAX as u128 / 2, 0x40);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(!0x1, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b1001
            }
        );
    }

    // Testing the flags of the `0x0000000040234888:  eb01001f      cmp x0, x1`
    // instruction
    // #[test_case]
    // fn addwithcarry_early_4880_loop()
    {
        let mut state = State::new(Box::new(NoneEnv));

        let x = Bits::new(0x425a6004, 0x40);
        let y = Bits::new(!0x425a6020, 0x40);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(0xffffffffffffffe3, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b1000
            }
        );
    }

    // #[test_case]
    // fn addwithcarry_linux_regression()
    {
        let mut state = State::new(Box::new(NoneEnv));

        let x = Bits::new(0xffffffc0082b3cd0, 64);
        let y = Bits::new(0xffffffffffffffd8, 64);
        let carry_in = false;

        assert_eq!(
            AddWithCarry(&mut state, TRACER, x, y, carry_in),
            ProductType188a1c3bf231c64b {
                tuple__pcnt_bv__pcnt_bv40: Bits::new(0xffffffc0082b3ca8, 0x40),
                tuple__pcnt_bv__pcnt_bv41: 0b1010
            }
        );
    }

    // #[test_case]
    // fn replicate_bits()
    {
        let mut state = State::new(Box::new(NoneEnv));
        assert_eq!(
            Bits::new(0xffff_ffff, 32),
            replicate_bits_borealis_internal(&mut state, TRACER, Bits::new(0xff, 8), 4)
        );
        assert_eq!(
            Bits::new(0xaa, 8),
            replicate_bits_borealis_internal(&mut state, TRACER, Bits::new(0xaa, 8), 1)
        );
        assert_eq!(
            Bits::new(0xaaaa, 16),
            replicate_bits_borealis_internal(&mut state, TRACER, Bits::new(0xaa, 8), 2)
        );
        assert_eq!(
            Bits::new(0xffff_ffff, 32),
            replicate_bits_borealis_internal(&mut state, TRACER, Bits::new(0x1, 1), 32)
        );
    }

    // #[test_case]
    // fn ubfx()
    {
        {
            let mut state = State::new(Box::new(NoneEnv));
            // decode bit masks
            assert_eq!(
                ProductTypea79c7f841a890648 {
                    tuple__pcnt_bv__pcnt_bv0: Bits::new(0xFFFF00000000000F, 64),
                    tuple__pcnt_bv__pcnt_bv1: Bits::new(0xF, 64)
                },
                DecodeBitMasks(&mut state, TRACER, true, 0x13, 0x10, false, 0x40)
            );
        }

        {
            let mut state = State::new(Box::new(NoneEnv));
            state.write_register::<u64>(REG_R3, 0x8444_c004);

            // ubfx x3, x3, #16, #4
            u__DecodeA64(&mut state, TRACER, 0, 0xd3504c63);
            assert_eq!(0x4, state.read_register::<u64>(REG_R3));
        }
    }

    // #[test_case]
    // fn fibonacci()
    {
        let mut state = State::new(Box::new(NoneEnv));
        borealis_register_init(&mut state, TRACER);
        // hacky, run sail function that goes before the main loop :/
        u__InitSystem(&mut state, TRACER, ());

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

        // bounded just in case
        for _ in 0..100 {
            state.write_register(REG_SEE, 0u64);
            state.write_register(REG_U__BRANCHTAKEN, false);
            let pc = state.read_register::<u64>(REG_U_PC);

            // exit before the svc
            if pc == 0x38 {
                break;
            }

            let instr = program[pc as usize / 4];
            u__DecodeA64(&mut state, TRACER, pc.into(), instr);

            // increment PC if no branch was taken
            if !state.read_register::<bool>(REG_U__BRANCHTAKEN) {
                let pc = state.read_register::<u64>(REG_U_PC);
                state.write_register(REG_U_PC, pc + 4);
            }
        }

        assert_eq!(89, state.read_register::<u64>(REG_R0));
        assert_eq!(10, state.read_register::<u64>(REG_R3));
    }

    // #[test_case]
    // fn rev_d00dfeed()
    {
        let mut state = State::new(Box::new(NoneEnv));
        state.write_register::<u64>(REG_R3, 0xedfe0dd0);
        execute_aarch64_instrs_integer_arithmetic_rev(&mut state, TRACER, 32, 3, 32, 3);
        assert_eq!(0xd00dfeed, state.read_register::<u64>(REG_R3));
    }

    // #[test_case]
    // fn ispow2()
    {
        let mut state = State::new(Box::new(NoneEnv));
        let x = 2048i128;
        assert_eq!(
            FloorPow2(&mut state, TRACER, x),
            CeilPow2(&mut state, TRACER, x)
        );
        assert!(IsPow2(&mut state, TRACER, x));
    }

    // #[test_case]
    // fn udiv()
    {
        let x = 0xffffff8008bfffffu64;
        let y = 0x200000u64;
        let mut state = State::new(Box::new(NoneEnv));
        state.write_register(REG_R19, x);
        state.write_register(REG_R1, y);

        // div
        u__DecodeA64_DataProcReg::u__DecodeA64_DataProcReg(&mut state, TRACER, 0x0, 0x9ac10a73);

        assert_eq!(x / y, state.read_register(REG_R19));
    }

    // #[test_case]
    // fn place_slice()
    {
        let mut state = State::new(Box::new(NoneEnv));
        assert_eq!(
            Bits::new(0xffffffffffffffd8, 64),
            place_slice_signed(&mut state, TRACER, 64, Bits::new(0xffffffd8, 64), 0, 32, 0,)
        );
    }

    {
        let mut state = State::new(Box::new(NoneEnv));
        state.write_register::<u64>(REG_R0, 0xffff_ffff_ffff_ff00);
        state.write_register::<u64>(REG_R2, 0xffff_ffff_ffff_ffc0);

        //   //  let pstate = ProductTypee2f620c8eb69267c::default();

        //     state.write_register::<u64>(REG_PSTATE, pstate);

        //cmp     x2, x0
        u__DecodeA64(&mut state, TRACER, 0x0, 0xeb00005f);

        //  csel    x2, x2, x0, ls  // ls = plast
        u__DecodeA64(&mut state, TRACER, 0x0, 0x9a809042);

        // assert x2
        assert_eq!(state.read_register::<u64>(REG_R2), 0xffff_ffff_ffff_ff00);
    }

    {
        let mut state = State::new(Box::new(NoneEnv));
        state.write_register::<u64>(REG_R0, 0xffff_ffff_ffff_ff00);
        state.write_register::<u64>(REG_R2, 0x0fff_ffff_ffff_ffc0);

        //   //  let pstate = ProductTypee2f620c8eb69267c::default();

        //     state.write_register::<u64>(REG_PSTATE, pstate);

        //cmp     x2, x0
        u__DecodeA64(&mut state, TRACER, 0x0, 0xeb00005f);

        //  csel    x2, x2, x0, ls  // ls = plast
        u__DecodeA64(&mut state, TRACER, 0x0, 0x9a809042);

        // assert x2
        assert_eq!(state.read_register::<u64>(REG_R2), 0x0fff_ffff_ffff_ffc0);
    }

    {
        let mut state = State::new(Box::new(NoneEnv));
        state.write_register::<u64>(REG_R0, 0x0000000000000001);

        // rbit x0
        u__DecodeA64(&mut state, TRACER, 0x0, 0xdac00000);

        // assert bits are reversed
        assert_eq!(state.read_register::<u64>(REG_R0), 0x8000000000000000);
    }

    //  panic!();
    log::info!("all tests passed");
}

struct NoneTracer;

impl Tracer for NoneTracer {
    fn begin(&self, _: u32, _: u64) {}

    fn end(&self) {}

    fn read_register(&self, _: usize, _: &dyn Debug) {}

    fn write_register(&self, _: usize, _: &dyn Debug) {}

    fn read_memory(&self, _: usize, _: &dyn Debug) {}

    fn write_memory(&self, _: usize, _: &dyn Debug) {}
}

struct NoneEnv;

impl Environment for NoneEnv {
    fn read_memory(&self, _: u64, _: &mut [u8]) {}

    fn write_memory(&self, _: u64, _: &[u8]) {}
}
