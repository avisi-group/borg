use {
    crate::dbt::{
        emitter::{Emitter, Type},
        models::{self, execute},
        x86::X86TranslationContext,
        TranslationContext,
    },
    alloc::boxed::Box,
    proc_macro_lib::ktest,
};

pub mod virtio;

#[ktest]
fn static_dynamic_chaos_smoke() {
    fn run(mut register_file: [u64; 3]) -> [u64; 3] {
        let mut ctx = X86TranslationContext::new();
        let model = models::get("aarch64").unwrap();
        let register_file_ptr = register_file.as_mut_ptr() as *mut u8;

        let _val = execute(&*model, "func_corrupted_var", &[], &mut ctx);

        ctx.emitter().leave();
        let translation = ctx.compile();
        log::debug!("{:?}", translation);
        translation.execute(register_file_ptr);

        register_file
    }

    assert_eq!(run([0, 0, 0]), [0, 0, 10]);
    assert_eq!(run([0, 1, 0]), [0, 1, 10]);
    assert_eq!(run([1, 0, 0]), [1, 0, 5]);
    assert_eq!(run([1, 1, 0]), [1, 1, 5]);
}

// #[ktest]
// fn num_of_feature() {
//     let mut ctx = X86TranslationContext::new();
//     let model = models::get("aarch64").unwrap();
//     let mut register_file = alloc::vec![0u8;model.register_file_size()];
//     let register_file_ptr = register_file.as_mut_ptr();

//     execute(&*model, "borealis_register_init", &[], &mut ctx);

//     let r0_offset = ctx
//         .emitter()
//         .constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));

//     let feature = ctx
//         .emitter()
//         .read_register(r0_offset.clone(), Type::Unsigned(0x20));

//     //  execute(&*model, "num_of_Feature", &[feature], &mut ctx);

//     ctx.emitter().leave();
//     let translation = ctx.compile();
//     log::debug!("{:?}", translation);
//     translation.execute(register_file_ptr);
// }

// #[ktest]
// fn decodea64_smoke() {
//     let mut register_file = Box::new([0u8; 104488usize]);
//     let register_file_ptr = register_file.as_mut_ptr();
//     let mut ctx = X86TranslationContext::new();
//     let model = models::get("aarch64").unwrap();

//     //execute(&*model, "borealis_register_init", &[], &mut ctx);

//     // OOM crashes:(
//     // execute(
//     //     &*model,
//     //     "__InitSystem",
//     //     &[ctx.emitter().constant(0, Type::Unsigned(0))],
//     //     &mut ctx,
//     // );

//     let pc = ctx.emitter().constant(0, Type::Unsigned(64));

//     // // add x0,x1,x2
//     // // (x0 = x1 + x2)
//     let opcode = ctx.emitter().constant(0x8b020020, Type::Unsigned(64));

//     execute(&*model, "__DecodeA64", &[pc, opcode], &mut ctx);

//     ctx.emitter().leave();
//     let translation = ctx.compile();
//     log::debug!("\n{:?}", translation);

//     unsafe {
//         let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u32;
//         let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u32;
//         let r2 = register_file_ptr.add(model.reg_offset("R2")) as *mut u32;
//         let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;

//         *see = -1;
//         *r0 = 2;
//         *r1 = 5;
//         *r2 = 10;

//         translation.execute(register_file_ptr);

//         assert_eq!(15, (*r0));
//         assert_eq!(0xe, (*see));
//     }

//     panic!();
// }

// // // #[ktest]
// // // fn fibonacci() {
// // //     let mut register_file = Box::new([0u8; 104488usize]);
// // //     let register_file_ptr = register_file.as_mut_ptr();

// // //     let mut ctx = X86TranslationContext::new();
// // //     let model = models::get("aarch64").unwrap();
// // //     execute(&*model, "borealis_register_init", &[], &mut ctx);
// // //     // OOM crashes:(
// // //     // execute(
// // //     //     &*model,
// // //     //     "__InitSystem",
// // //     //     &[ctx.emitter().constant(0, Type::Unsigned(0))],
// // //     //     &mut ctx,
// // //     // );
// // //     ctx.emitter().leave();
// // //     let translation = ctx.compile();
// // //     translation.execute(register_file_ptr);

// // //     // // hacky, run sail function that goes before the main loop :/

// // //     let program = [
// // //         // <_start>
// // //         0xd2800000, // mov     x0, #0x0 (#0)
// // //         0xd2800021, // mov     x1, #0x1 (#1)
// // //         0xd2800002, // mov     x2, #0x0 (#0)
// // //         0xd2800003, // mov     x3, #0x0 (#0)
// // //         0xd2800144, // mov     x4, #0xa (#10)
// // //         // <loop>
// // //         0xeb04007f, // cmp     x3, x4
// // //         0x540000c0, // b.eq    400104 <done>  // b.none
// // //         0x8b010002, // add     x2, x0, x1
// // //         0xaa0103e0, // mov     x0, x1
// // //         0xaa0203e1, // mov     x1, x2
// // //         0x91000463, // add     x3, x3, #0x1
// // //         0x17fffffa, // b       4000e8 <loop>
// // //         // <done>
// // //         0xaa0203e0, // mov     x0, x2
// // //         0x52800ba8, // mov     w8, #0x5d (#93)
// // //         0xd4000001, // svc     #0x0
// // //     ];
// // //     unsafe {
// // //         let see = register_file_ptr.add(model.reg_offset("SEE")) as
// *mut // i32; //         let branch_taken =
// // //             { register_file_ptr.add(model.reg_offset("__BranchTaken"))
// as // // *mut bool };         let pc = {
// // // register_file_ptr.add(model.reg_offset("_PC")) as *mut u64 };
// let // r0 // = { register_file_ptr.add(model.reg_offset("R0")) as *mut u64 };
// // let // r3 = { register_file_ptr.add(model.reg_offset("R3")) as *mut u64 };

// // //         // bounded just in case
// // //         for _ in 0..100 {
// // //             log::warn!("pc = {}", *pc);

// // //             *see = -1;
// // //             *branch_taken = false;

// // //             // exit before the svc
// // //             if *pc == 0x38 {
// // //                 break;
// // //             }

// // //             let mut ctx = X86TranslationContext::new();
// // //             let model = models::get("aarch64").unwrap();

// // //             {
// // //                 let opcode = ctx
// // //                     .emitter()
// // //                     .constant(program[*pc as usize / 4],
// // Type::Unsigned(64)); //                 let pc =
// ctx.emitter().constant(*pc, // Type::Unsigned(64)); //
// execute(&*model, "__DecodeA64", &[pc, // opcode], &mut ctx); //             }

// // //             ctx.emitter().leave();
// // //             let translation = ctx.compile();
// // //             translation.execute(register_file_ptr);

// // //             // increment PC if no branch was taken
// // //             if !*branch_taken {
// // //                 *pc += 4;
// // //             }
// // //         }

// // //         assert_eq!(89, *r0);
// // //         assert_eq!(10, *r3);
// // //     }
// // // }

#[ktest]
fn addwithcarry_negative() {
    let (sum, flags) = add_with_carry_harness(0, -5i64 as u64, false);

    assert_eq!(sum, -5i64 as u64);
    assert_eq!(flags, 0b1000);
}

#[ktest]
fn addwithcarry_zero() {
    let (sum, flags) = add_with_carry_harness(0, 0, false);
    assert_eq!(sum, 0);
    assert_eq!(flags, 0b0100);
}

#[ktest]
fn addwithcarry_carry() {
    let (sum, flags) = add_with_carry_harness(u64::MAX, 1, false);
    assert_eq!(sum, 0);
    assert_eq!(flags, 0b0110);
}

#[ktest]
fn addwithcarry_overflow() {
    let (sum, flags) = add_with_carry_harness(u64::MAX / 2, u64::MAX / 2, false);
    assert_eq!(sum, !1);
    assert_eq!(flags, 0b1001);
}

// // // Testing the flags of the `0x0000000040234888:  eb01001f      cmp x0,
// x1` // // instruction
// // #[ktest]
// // fn addwithcarry_early_4880_loop() {
// //     let (sum, flags) = add_with_carry_harness(0x425a6004, !0x425a6020,
// // false);     assert_eq!(sum, 0xffffffffffffffe3);
// //     assert_eq!(flags, 0b1000);
// // }

// // // #[ktest]
// // // fn replicate_bits() {
// // //     let mut register_file = Box::new([0u8; 104488usize]);
// // //     let register_file_ptr = register_file.as_mut_ptr();
// // //     let mut ctx = X86TranslationContext::new();
// // //     let model = models::get("aarch64").unwrap();

// // //     execute(&*model, "borealis_register_init", &[], &mut ctx);

// // //     assert_eq!(
// // //         Bits::new(0xffff_ffff, 32),
// // //         replicate_bits_borealis_internal(&mut state, TRACER,
// // Bits::new(0xff, // 8), 4)     );
// // //     assert_eq!(
// // //         Bits::new(0xaa, 8),
// // //         replicate_bits_borealis_internal(&mut state, TRACER,
// // Bits::new(0xaa, // 8), 1)     );
// // //     assert_eq!(
// // //         Bits::new(0xaaaa, 16),
// // //         replicate_bits_borealis_internal(&mut state, TRACER,
// // Bits::new(0xaa, // 8), 2)     );
// // //     assert_eq!(
// // //         Bits::new(0xffff_ffff, 32),
// // //         replicate_bits_borealis_internal(&mut state, TRACER,
// // Bits::new(0x1, // 1), 32)     );
// // // }

// // #[ktest]
// // fn addwithcarry_linux_regression() {
// //     let (sum, flags) = add_with_carry_harness(0xffffffc0082b3cd0,
// // 0xffffffffffffffd8, false);     assert_eq!(sum, 0xffffffc0082b3ca8);
// //     assert_eq!(flags, 0b1010);
// // }

fn add_with_carry_harness(x: u64, y: u64, carry_in: bool) -> (u64, u8) {
    let mut register_file = Box::new([0u8; 104488usize]);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new();
    let model = models::get("aarch64").unwrap();

    //  execute(&*model, "borealis_register_init", &[], &mut ctx);
    let r0 = unsafe { register_file_ptr.add(model.reg_offset("R0")) as *mut u64 };
    let r1 = unsafe { register_file_ptr.add(model.reg_offset("R1")) as *mut u64 };
    let r2 = unsafe { register_file_ptr.add(model.reg_offset("R2")) as *mut u8 };

    unsafe {
        *r0 = x;
        *r1 = y;
        *r2 = carry_in as u8;
    }

    let r0_offset = ctx
        .emitter()
        .constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    let r1_offset = ctx
        .emitter()
        .constant(model.reg_offset("R1") as u64, Type::Unsigned(0x40));
    let r2_offset = ctx
        .emitter()
        .constant(model.reg_offset("R2") as u64, Type::Unsigned(0x8));

    let x = ctx
        .emitter()
        .read_register(r0_offset.clone(), Type::Unsigned(0x40));
    let y = ctx
        .emitter()
        .read_register(r1_offset.clone(), Type::Unsigned(0x40));
    let carry_in = ctx
        .emitter()
        .read_register(r2_offset.clone(), Type::Unsigned(0x8));

    let res = execute(&*model, "add_with_carry_test", &[x, y, carry_in], &mut ctx);

    {
        let sum = ctx.emitter().access_tuple(res.clone(), 0);
        ctx.emitter().write_register(r0_offset, sum);
    }

    {
        let flags = ctx.emitter().access_tuple(res.clone(), 1);

        // zero extend flags to 64
        ctx.emitter().write_register(r1_offset, flags);
    }

    ctx.emitter().leave();
    let translation = ctx.compile();

    translation.execute(register_file_ptr);

    unsafe { (*r0, *(r1 as *mut u8)) }
}

// // // #[ktest]
// // // fn ubfx() {
// // //     {
// // //         let mut state = State::new(Box::new(NoneEnv));
// // //         // decode bit masks
// // //         assert_eq!(
// // //             ProductTypea79c7f841a890648 {
// // //                 tuple__pcnt_bv__pcnt_bv0: Bits::new(0xFFFF00000000000F,
// // 64), //                 tuple__pcnt_bv__pcnt_bv1: Bits::new(0xF, 64)
// // //             },
// // //             DecodeBitMasks(&mut state, TRACER, true, 0x13, 0x10, false,
// // 0x40) //         );
// // //     }

// // //     {
// // //         let mut state = State::new(Box::new(NoneEnv));
// // //         state.write_register::<u64>(REG_R3, 0x8444_c004);

// // //         // ubfx x3, x3, #16, #4
// // //         u__DecodeA64(&mut state, TRACER, 0, 0xd3504c63);
// // //         assert_eq!(0x4, state.read_register::<u64>(REG_R3));
// // //     }
// // // }

// // // #[ktest]
// // // fn rev_d00dfeed() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register::<u64>(REG_R3, 0xedfe0dd0);
// // //     execute_aarch64_instrs_integer_arithmetic_rev(&mut state, TRACER,
// 32, // 3, // 32, 3);     assert_eq!(0xd00dfeed,
// state.read_register::<u64>(REG_R3)); // // }

// // // #[ktest]
// // // fn ispow2() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     let x = 2048i128;
// // //     assert_eq!(
// // //         FloorPow2(&mut state, TRACER, x),
// // //         CeilPow2(&mut state, TRACER, x)
// // //     );
// // //     assert!(IsPow2(&mut state, TRACER, x));
// // // }

// // // #[ktest]
// // // fn udiv() {
// // //     let x = 0xffffff8008bfffffu64;
// // //     let y = 0x200000u64;
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register(REG_R19, x);
// // //     state.write_register(REG_R1, y);

// // //     // div
// // //     u__DecodeA64_DataProcReg::u__DecodeA64_DataProcReg(&mut state,
// TRACER, // // 0x0, 0x9ac10a73);

// // //     assert_eq!(x / y, state.read_register(REG_R19));
// // // }

// // // #[ktest]
// // // fn place_slice() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     assert_eq!(
// // //         Bits::new(0xffffffffffffffd8, 64),
// // //         place_slice_signed(&mut state, TRACER, 64,
// Bits::new(0xffffffd8, // 64), // 0, 32, 0,)     );
// // // }

// // // fn cmp_csel() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register::<u64>(REG_R0, 0xffff_ffff_ffff_ff00);
// // //     state.write_register::<u64>(REG_R2, 0xffff_ffff_ffff_ffc0);

// // //     //   //  let pstate = ProductTypee2f620c8eb69267c::default();

// // //     //     state.write_register::<u64>(REG_PSTATE, pstate);

// // //     //cmp     x2, x0
// // //     u__DecodeA64(&mut state, TRACER, 0x0, 0xeb00005f);

// // //     //  csel    x2, x2, x0, ls  // ls = plast
// // //     u__DecodeA64(&mut state, TRACER, 0x0, 0x9a809042);

// // //     // assert x2
// // //     assert_eq!(state.read_register::<u64>(REG_R2),
// 0xffff_ffff_ffff_ff00); // // }

// // // #[ktest]
// // // fn cmp_csel_2() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register::<u64>(REG_R0, 0xffff_ffff_ffff_ff00);
// // //     state.write_register::<u64>(REG_R2, 0x0fff_ffff_ffff_ffc0);

// // //     //   //  let pstate = ProductTypee2f620c8eb69267c::default();

// // //     //     state.write_register::<u64>(REG_PSTATE, pstate);

// // //     //cmp     x2, x0
// // //     u__DecodeA64(&mut state, TRACER, 0x0, 0xeb00005f);

// // //     //  csel    x2, x2, x0, ls  // ls = plast
// // //     u__DecodeA64(&mut state, TRACER, 0x0, 0x9a809042);

// // //     // assert x2
// // //     assert_eq!(state.read_register::<u64>(REG_R2),
// 0x0fff_ffff_ffff_ffc0); // // }

// // // #[ktest]
// // // fn rbitx0() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register::<u64>(REG_R0, 0x0000000000000001);

// // //     // rbit x0
// // //     u__DecodeA64(&mut state, TRACER, 0x0, 0xdac00000);

// // //     // assert bits are reversed
// // //     assert_eq!(state.read_register::<u64>(REG_R0), 0x8000000000000000);
// // // }
