use {
    crate::{
        dbt::{
            Translation, bit_insert,
            emitter::{Emitter, Type},
            interpret::{Value, interpret},
            models::{self},
            register_file::RegisterFile,
            translate::{translate, translate_instruction},
            x86::{
                X86TranslationContext,
                emitter::{
                    BinaryOperationKind, CastOperationKind, NodeKind, ShiftOperationKind,
                    X86Emitter, X86Node,
                },
            },
        },
        memory::bump::{BumpAllocator, BumpAllocatorRef},
        timer::Measurement,
    },
    alloc::{alloc::Global, boxed::Box},
    common::{hashmap::HashMap, mask::mask},
    core::panic,
    proc_macro_lib::ktest,
};

#[ktest]
fn init_system() {
    let model = models::get("aarch64").unwrap();

    let _register_file = RegisterFile::init(&*model);
}

#[ktest]
fn static_dynamic_chaos_smoke() {
    fn run(r0_value: u64, r1_value: u64, r2_value: u64) -> (u64, u64, u64) {
        let model = models::get("aarch64").unwrap();

        let register_file = RegisterFile::init(&*model);

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        translate(
            Global,
            &*model,
            "func_corrupted_var",
            &[],
            &mut emitter,
            &register_file,
        )
        .unwrap();

        emitter.leave();
        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        register_file.write("R0", r0_value);
        register_file.write("R1", r1_value);
        register_file.write("R2", r2_value);

        translation.execute(&register_file);

        (
            register_file.read("R0"),
            register_file.read("R1"),
            register_file.read("R2"),
        )
    }

    assert_eq!(run(0, 0, 0), (0, 0, 10));
    assert_eq!(run(0, 1, 0), (0, 1, 10));
    assert_eq!(run(1, 0, 0), (1, 0, 5));
    assert_eq!(run(1, 1, 0), (1, 1, 5));
}

// #[ktest]
// fn num_of_feature_dynamic() {
//     let model = &*models::get("aarch64").unwrap();

//     let  register_file = RegisterFile::init(&*model);
//

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     let feature = emitter.read_register(model.reg_offset("R0"),
// Type::Signed(32));

//     let out = translate(Global,
//         &*model,
//         "num_of_Feature",
//         &[feature],
//         &mut emitter,
//         &register_file,
//     )
//     .unwrap();
//     emitter.write_register(model.reg_offset("R1"), out);
//     emitter.leave();
//     let num_regs = emitter.next_vreg();

//     let translation = ctx.compile(num_regs);

//     unsafe {
//         let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as
// *mut i32;         let r1 = register_file_ptr.add(model.reg_offset("R1") as
// usize) as *mut i64;

//         register_file.write::<u64,_>("R0",4);
//         *r1 = 0;

//         translation.execute(&register_file);

//         assert_eq!(4, ( register_file.read::<u64,_>("R0")));
//         assert_eq!(4, (*r1));
//         //assert_eq!(0xe, (*see)); //// todo: re-implement depending on
// result         // of SEE/cacheable registers work
//     }
// }

#[ktest]
fn num_of_feature_const_123() {
    let model = &*models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let feature = emitter.constant(123, Type::Signed(32));

    let out = translate(
        Global,
        &*model,
        "num_of_Feature",
        &[feature],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    emitter.leave();

    assert_eq!(
        *out.kind(),
        NodeKind::Constant {
            value: 159,
            width: 64
        }
    );
}

#[ktest]
fn statistical_profiling_disabled() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let is_enabled = translate(
        Global,
        &*model,
        "StatisticalProfilingEnabled",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(&register_file);

    assert_eq!(0, register_file.read::<u8>("R0"))
}

// /// Disabling because we enabled all the features, but this should really be
// a const false for the sake of performance #[ktest]
// fn havebrbext_disabled() {
//     let model = models::get("aarch64").unwrap();

//     let  register_file = RegisterFile::init(&*model);
//

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     let is_enabled =
//         translate(Global,&*model, "HaveBRBExt", &[], &mut emitter,
// register_file_ptr).unwrap();

//     emitter.write_register(model.reg_offset("R0"), is_enabled);

//     emitter.leave();
//     let num_regs = emitter.next_vreg();
//     let translation = ctx.compile(num_regs);
//     translation.execute(&register_file);

//     unsafe {
//         assert_eq!(
//             false,
//             *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut
// bool)         )
//     }
// }

#[ktest]
fn using_aarch32_disabled() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let is_enabled = translate(
        Global,
        &*model,
        "UsingAArch32",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(&register_file);

    assert_eq!(0, register_file.read::<u8>("R0"))
}

#[ktest]
fn branchto() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let target = emitter.constant(0xDEADFEED, Type::Unsigned(64));
    let branch_type = emitter.constant(1, Type::Unsigned(32));
    let branch_conditional = emitter.constant(1, Type::Unsigned(1));
    translate(
        Global,
        &*model,
        "BranchTo",
        &[target, branch_type, branch_conditional],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    assert_eq!(0x0, register_file.read::<u64>("_PC"));

    register_file.write("__BranchTaken", false);

    translation.execute(&register_file);

    assert_eq!(0xDEADFEED, register_file.read::<u64>("_PC"));
    assert_eq!(true, register_file.read::<bool>("__BranchTaken"))
}

#[ktest]
fn decodea64_addsub() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x8b020020, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 2);
    register_file.write::<u64>("R1", 5);
    register_file.write::<u64>("R2", 10);

    translation.execute(&register_file);

    assert_eq!(15, register_file.read::<u64>("R0"));
    //assert_eq!(0xe, (*see)); //// todo: re-implement depending on result
    // of SEE/cacheable registers work
}

#[ktest]
fn decodea64_addsub_interpret() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 2);
    register_file.write::<u64>("R1", 5);
    register_file.write::<u64>("R2", 10);

    let pc = crate::dbt::interpret::Value::UnsignedInteger {
        value: 0,
        width: 64,
    };
    let opcode = crate::dbt::interpret::Value::UnsignedInteger {
        value: 0x8b020020,
        width: 32,
    };
    interpret(&*model, "__DecodeA64", &[pc, opcode], &register_file);

    assert_eq!(15, register_file.read::<u64>("R0"));
    //   assert_eq!(0xe, (*see)); // todo: re-implement depending on result
    // of SEE/cacheable registers work
}

#[ktest]
fn decodea64_mov() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    //   aa0103e0        mov     x0, x1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xaa0103e0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 2);
    register_file.write::<u64>("R1", 43);

    translation.execute(&register_file);

    //log::info!("{translation:?}");

    assert_eq!(43, register_file.read::<u64>("R0"));
    assert_eq!(43, register_file.read::<u64>("R1"));
    // assert_eq!(55, (*see));// todo: re-implement depending on result of
    // SEE/cacheable registers work
}

#[ktest]
fn decodea64_branch() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(44, Type::Unsigned(64));
    let opcode = emitter.constant(0x17fffffa, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    //  log::trace!("{translation:?}");

    register_file.write("_PC", 44u64);
    register_file.write("SEE", -1i64);

    translation.execute(&register_file);

    assert_eq!(20, register_file.read::<u64>("_PC"));
    //assert_eq!(67, (*see));// todo: re-implement depending on result of
    // SEE/cacheable registers work
}

#[ktest]
fn branch_if_eq() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x540000c0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);

    translation.execute(&register_file);

    //assert_eq!(0x45, (*see)); // todo: re-implement depending on result of
    // SEE/cacheable registers work

    assert_eq!(0x0, register_file.read::<u64>("_PC"));
    assert_eq!(true, register_file.read::<bool>("__BranchTaken"));
}

#[ktest]
fn branch_uncond_imm_offset_math() {
    let model = models::get("aarch64").unwrap();
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    // s0: read-var imm26:u26
    let s0 = emitter.constant(0x17fffffa & mask(26u32), Type::Unsigned(26));

    // s1: const #0u : u2
    let s1 = emitter.constant(0, Type::Unsigned(2));

    // s2: cast zx s0 -> u28
    let s2 = emitter.cast(s0, Type::Unsigned(28), CastOperationKind::ZeroExtend);

    // s3: const #2u : u16
    let s3 = emitter.constant(2, Type::Unsigned(16));

    // s4: lsl s2 s3
    let s4 = emitter.shift(s2, s3, ShiftOperationKind::LogicalShiftLeft);

    // s5: or s4 s1
    let s5 = emitter.binary_operation(BinaryOperationKind::Or(s4, s1));

    // s9: cast sx s5 -> u64
    let s9 = emitter.cast(s5, Type::Unsigned(64), CastOperationKind::SignExtend);

    let NodeKind::Constant { value, width } = s9.kind() else {
        panic!()
    };
    assert_eq!(*value, 0xffffffffffffffe8);
    assert_eq!(*width, 64);
}

/// Validated with:
///
/// ```rust
/// use std::arch::asm;
/// fn main() {
///     for (x, y) in [
///         (10, 5),
///         (5, 10),
///         (0, 0),
///         (u64::MAX, u64::MAX),
///         (0x7FFF_FFFF_FFFF_FFFF, -1i64 as u64),
///         (0x7FFF_FFFF_FFFF_FFFF, 1),
///         (0x0000000000000000, 0x8000000000000000),
///         (0x8000000000000000, -1i64 as u64),
///         (-1i64 as u64, 0),
///     ] {
///         println!("{x:x} {y:x}: {:04b}", get_flags(x, y))
///     }
///     println!();
///     println!();
///     for (r0, r2) in [
///         (0xffff_ffff_ffff_ff00, 0x0fff_ffff_ffff_ffc0),
///         (0xffff_ffff_ffff_ff00, 0xffff_ffff_ffff_ffc0),
///     ] {
///         println!("{r0:x} {r2:x}: {:x?}", cmp_csel(r0, r2))
///     }
/// }
/// fn get_flags(x: u64, y: u64) -> u8 {
///     let mut nzcv: u64;
///     unsafe {
///         asm!(
///             "cmp x0, x1",
///             "mrs x2, nzcv",
///             in("x0") x,
///             in("x1") y,
///             out("x2") nzcv,
///         );
///     }
///     u8::try_from(nzcv >> 28).unwrap()
/// }
/// fn cmp_csel(r0: u64, mut r2: u64) -> (u64, u8) {
///     let mut nzcv: u64;
///     unsafe {
///         asm!(
///             "cmp x2, x0",
///             "mrs x1, nzcv",
///             "csel    x2, x2, x0, ls",
///             in("x0") r0,
///             inout("x2") r2,
///             out("x1") nzcv,
///         );
///     }
///     (r2, u8::try_from(nzcv >> 28).unwrap())
/// }
/// ```
#[ktest]
fn cmp_csel() {
    assert_eq!(
        0xffff_ffff_ffff_ff00,
        cmp_csel_inner(0xffff_ffff_ffff_ff00, 0xffff_ffff_ffff_ffc0)
    );

    assert_eq!(
        0x0fff_ffff_ffff_ffc0,
        cmp_csel_inner(0xffff_ffff_ffff_ff00, 0x0fff_ffff_ffff_ffc0)
    );

    fn cmp_csel_inner(pre_r0: u64, pre_r2: u64) -> u64 {
        let model = models::get("aarch64").unwrap();

        let register_file = RegisterFile::init(&*model);

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(model.reg_offset("SEE"), see_value);

        // cmp     x2, x0
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0xeb00005f, Type::Unsigned(32));
        translate(
            Global,
            &*model,
            "__DecodeA64",
            &[pc, opcode],
            &mut emitter,
            &register_file,
        )
        .unwrap();

        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(model.reg_offset("SEE"), see_value);

        // csel    x2, x2, x0, ls  // ls = plast
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0x9a809042, Type::Unsigned(32));
        translate(
            Global,
            &*model,
            "__DecodeA64",
            &[pc, opcode],
            &mut emitter,
            &register_file,
        )
        .unwrap();

        emitter.leave();

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        register_file.write::<u64>("R0", pre_r0);
        register_file.write::<u64>("R2", pre_r2);

        translation.execute(&register_file);

        register_file.read::<u64>("R2")
    }
}

#[ktest]
fn fibonacci_instr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

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
        register_file.write("SEE", -1i64);
        register_file.write("__BranchTaken", false);

        let pc = register_file.read::<u64>("_PC");

        // exit before the svc
        if pc == 0x38 {
            break;
        }

        let model = models::get("aarch64").unwrap();

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        {
            let opcode = emitter.constant(program[pc as usize / 4], Type::Unsigned(32));
            let pc = emitter.constant(pc, Type::Unsigned(64));
            translate(
                Global,
                &*model,
                "__DecodeA64",
                &[pc, opcode],
                &mut emitter,
                &register_file,
            )
            .unwrap();
        }

        emitter.leave();
        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);
        translation.execute(&register_file);

        // increment PC if no branch was taken
        if !register_file.read::<bool>("__BranchTaken") {
            register_file.write("_PC", pc + 4);
        }
    }

    assert_eq!(89, register_file.read::<u64>("R0"));
    assert_eq!(10, register_file.read::<u64>("R3"));
}

///  4000d4:	d2955fe0 	mov	x0, #0xaaff                	// #43775
///  4000d8:	d2800001 	mov	x1, #0x0                   	// #0
///  4000dc:	91500421 	add	x1, x1, #0x401, lsl #12
///  4000e0:	f9000020 	str	x0, [x1]
///  4000e4:	f9400020 	ldr	x0, [x1]
#[ktest]
fn mem() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9000020, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    // log::trace!("translation:\n{translation:?}");

    let mem = alloc::boxed::Box::new(0xdead_c0de_0000_0000u64);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 0xdeadcafe);
    register_file.write::<u64>("R1", &*mem as *const u64 as u64);

    translation.execute(&register_file);

    assert_eq!(*mem, register_file.read::<u64>("R0"));
}

#[ktest]
fn mem_store() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9000020, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    const VALUE: u64 = 0xdead_c0de_0000_0000; // will be overwritten
    let mem = alloc::boxed::Box::new(0xdeadcafeu64);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", VALUE);
    register_file.write::<u64>("R1", &*mem as *const u64 as u64);

    translation.execute(&register_file);

    assert_eq!(*mem, VALUE);
}

#[ktest]
fn mem_load() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    //execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9400020, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    const VALUE: u64 = 0xdead_c0de_0000_0000;
    let mem = alloc::boxed::Box::new(VALUE);

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 0xdeadcafe); // will be overwritten
    register_file.write::<u64>("R1", &*mem as *const u64 as u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), VALUE);
}

/// failing due to cached SEE
#[ktest]
fn fibonacci_block() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let program = [
        // <_start>
        0xd2800000, // mov     x0, #0x0 (#0)
        0xd2800021, // mov     x1, #0x1 (#1)
        0xd2800002, // mov     x2, #0x0 (#0)
        0xd2800003, // mov     x3, #0x0 (#0)
        0xd2800c84, // mov     x4, #0x64 (#100)
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

    let mut blocks = HashMap::<u64, Translation>::default();

    loop {
        let pc_offset = model.reg_offset("_PC");
        let mut current_pc = register_file.read::<u64>("_PC");
        let start_pc = current_pc;
        if let Some(translation) = blocks.get(&start_pc) {
            translation.execute(&register_file);
            continue;
        }

        if current_pc == 56 {
            break;
        }

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        loop {
            register_file.write("SEE", -1i64);

            let _false = emitter.constant(0 as u64, Type::Unsigned(1));
            emitter.write_register(model.reg_offset("__BranchTaken"), _false);

            {
                let opcode = emitter.constant(program[current_pc as usize / 4], Type::Unsigned(32));
                let pc = emitter.constant(current_pc, Type::Unsigned(64));
                let _return_value = translate(
                    Global,
                    &*model,
                    "__DecodeA64",
                    &[pc, opcode],
                    &mut emitter,
                    &register_file,
                );
            }

            if emitter.ctx().get_pc_write_flag() || (current_pc == ((program.len() * 4) - 8) as u64)
            {
                break;
            } else {
                let pc = emitter.read_register(pc_offset, Type::Unsigned(64));
                let _4 = emitter.constant(4, Type::Unsigned(64));
                let pc_inc = emitter.binary_operation(BinaryOperationKind::Add(pc, _4));
                emitter.write_register(pc_offset, pc_inc);

                current_pc += 4;
            }
        }

        // inc PC if branch not taken
        {
            let branch_taken =
                emitter.read_register(model.reg_offset("__BranchTaken"), Type::Unsigned(1));

            let _0 = emitter.constant(0, Type::Unsigned(64));
            let _4 = emitter.constant(4, Type::Unsigned(64));
            let addend = emitter.select(branch_taken, _0, _4);

            let pc = emitter.read_register(pc_offset, Type::Unsigned(64));
            let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
            emitter.write_register(pc_offset, new_pc);
        }

        emitter.leave();
        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        // log::trace!("{translation:?}")

        translation.execute(&register_file);
        blocks.insert(start_pc, translation);

        log::trace!(
            "{} {}",
            register_file.read::<u64>("_PC"),
            register_file.read::<bool>("__BranchTaken")
        );
    }

    assert_eq!(
        1298777728820984005, /* technically this is fib 101, fib 100 = 3736710778780434371,
                              * but this depends whether you treat x0 or x1 as the final
                              * result */
        register_file.read::<u64>("R0")
    );
    assert_eq!(100, register_file.read::<u64>("R3"));
}

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

// Testing the flags of the `0x0000000040234888:  eb01001f      cmp x0,x1`
// instruction
#[ktest]
fn addwithcarry_early_4880_loop() {
    let (sum, flags) = add_with_carry_harness(0x425a6004, !0x425a6020, false);
    assert_eq!(sum, 0xffffffffffffffe3);
    assert_eq!(flags, 0b1000);
}

#[ktest]
fn addwithcarry_linux_regression() {
    let (sum, flags) = add_with_carry_harness(0xffffffc0082b3cd0, 0xffffffffffffffd8, false);
    assert_eq!(sum, 0xffffffc0082b3ca8);
    assert_eq!(flags, 0b1010);
}

fn add_with_carry_harness(x: u64, y: u64, carry_in: bool) -> (u64, u8) {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write::<u64>("R0", x);
    register_file.write::<u64>("R1", y);
    register_file.write::<u64>("R2", carry_in as u64);

    let x = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(0x40));
    let y = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(0x40));
    let carry_in = emitter.read_register(model.reg_offset("R2"), Type::Unsigned(0x1));

    let res = translate(
        Global,
        &*model,
        "add_with_carry_test",
        &[x, y, carry_in],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    let sum = emitter.access_tuple(res.clone(), 0);
    emitter.write_register(model.reg_offset("R0"), sum);

    let flags = emitter.access_tuple(res.clone(), 1);
    emitter.write_register(model.reg_offset("R1"), flags);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    (
        register_file.read::<u64>("R0"),
        register_file.read::<u8>("R1"),
    )
}

#[ktest]
fn decodea64_cmp_first_greater() {
    let flags = decodea64_cmp_harness(10, 5);
    assert_eq!(flags, 0b0010);
}
#[ktest]
fn decodea64_cmp_second_greater() {
    let flags = decodea64_cmp_harness(5, 10);
    assert_eq!(flags, 0b1000);
}

#[ktest]
fn decodea64_cmp_zero() {
    let flags = decodea64_cmp_harness(0, 0);
    assert_eq!(flags, 0b0110);
}

#[ktest]
fn decodea64_cmp_equal() {
    let flags = decodea64_cmp_harness(u64::MAX, u64::MAX);
    assert_eq!(flags, 0b0110);
}

#[ktest]
fn decodea64_cmp_signed_overflow() {
    let flags = decodea64_cmp_harness(0x7fffffffffffffff, 0xffffffffffffffff);
    assert_eq!(flags, 0b1001);
}

#[ktest]
fn decodea64_cmp_positive_overflow() {
    let flags = decodea64_cmp_harness(0x7FFF_FFFF_FFFF_FFFF, 1);
    assert_eq!(flags, 0b0010);
}

#[ktest]
fn decodea64_cmp_negative_overflow() {
    let flags = decodea64_cmp_harness(0, 0x8000000000000000);
    assert_eq!(flags, 0b1001);
}

#[ktest]
fn decodea64_cmp_signed_underflow() {
    let flags = decodea64_cmp_harness(0x8000000000000000, u64::MAX);
    assert_eq!(flags, 0b1000);
}

#[ktest]
fn decodea64_cmp_something() {
    let flags = decodea64_cmp_harness(u64::MAX, 0);
    assert_eq!(flags, 0b1010);
}

/// verified with
/// ```rust
/// fn get_flags(x: u64, y: u64) -> u8 {
///     let mut nzcv: u64;
///     unsafe {
///         asm!(
///             "cmp x0, x1",
///             "mrs x2, nzcv",
///             in("x0") x,
///             in("x1") y,
///             out("x2") nzcv,
///         );
///     }
///     u8::try_from(nzcv >> 28).unwrap()
/// }
/// ```
fn decodea64_cmp_harness(x: u64, y: u64) -> u8 {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write::<u64>("R0", x);
    register_file.write::<u64>("R1", y);

    register_file.write("SEE", -1i64);

    // cmp    x0, x1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xeb01001f, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(&register_file);

    register_file.read::<u8>("PSTATE_N") << 3
        | register_file.read::<u8>("PSTATE_Z") << 2
        | register_file.read::<u8>("PSTATE_C") << 1
        | register_file.read::<u8>("PSTATE_V")
}

#[ktest]
fn shiftreg() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let _1 = emitter.constant(1, Type::Signed(64));
    let shift_type = emitter.constant(1, Type::Signed(32));
    let amount = emitter.constant(0, Type::Signed(64));
    let width = emitter.constant(64, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "ShiftReg",
        &[_1, shift_type, amount, width],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), value);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R0", 0);
    register_file.write::<u64>("R1", 0xdeadfeeddeadfeed);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0xdeadfeeddeadfeed);
    assert_eq!(register_file.read::<u64>("R1"), 0xdeadfeeddeadfeed);
}

#[ktest]
fn floorpow2_constant() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(2048, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "FloorPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(2397, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "FloorPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(4095, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "FloorPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(1231, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "FloorPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 1024,
            width: 64
        }
    );
}

#[ktest]
fn ceilpow2_constant() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(2048, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "CeilPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(2397, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "CeilPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 4096,
            width: 64
        }
    );
    let x = emitter.constant(4095, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "CeilPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 4096,
            width: 64
        }
    );
    let x = emitter.constant(1231, Type::Signed(64));
    let value = translate(
        Global,
        &*model,
        "CeilPow2",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
}

//#[ktest]
fn _ispow2() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.read_register(model.reg_offset("R3"), Type::Unsigned(0x40));

    {
        let value = translate(
            Global,
            &*model,
            "FloorPow2",
            &[x.clone()],
            &mut emitter,
            &register_file,
        )
        .unwrap()
        .unwrap();
        emitter.write_register(model.reg_offset("R0"), value);
    }

    {
        let value = translate(
            Global,
            &*model,
            "CeilPow2",
            &[x.clone()],
            &mut emitter,
            &register_file,
        )
        .unwrap()
        .unwrap();
        emitter.write_register(model.reg_offset("R1"), value);
    }

    {
        let value = translate(
            Global,
            &*model,
            "IsPow2",
            &[x],
            &mut emitter,
            &register_file,
        )
        .unwrap()
        .unwrap();
        emitter.write_register(model.reg_offset("R2"), value);
    }

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    // log::debug!("{translation:?}");

    register_file.write::<u64>("R0", 0);
    register_file.write::<u64>("R1", 0);
    register_file.write::<u64>("R2", 0);
    register_file.write::<u64>("R3", 2048);

    translation.execute(&register_file);

    assert_eq!(
        register_file.read::<u64>("R0"),
        register_file.read::<u64>("R1")
    );
    assert_eq!(1, register_file.read::<u64>("R2"))
}

#[ktest]
fn rbitx0_interpret() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    register_file.write::<u64>("R0", 0x0123_4567_89ab_cdef);
    register_file.write("SEE", -1i64);

    // rbit x0
    let pc = Value::UnsignedInteger {
        value: 0,
        width: 64,
    };
    let opcode = Value::UnsignedInteger {
        value: 0xdac00000,
        width: 32,
    };
    interpret(&*model, "__DecodeA64", &[pc, opcode], &register_file);

    // assert bits are reversed
    assert_eq!(register_file.read::<u64>("R0"), 0xf7b3_d591_e6a2_c480);
}

#[ktest]
fn rbitx0() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    // rbit x0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xdac00000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R0", 0x0123_4567_89ab_cdef);
    register_file.write("SEE", -1i64);

    translation.execute(&register_file);

    // assert bits are reversed
    assert_eq!(register_file.read::<u64>("R0"), 0xf7b3_d591_e6a2_c480);
}

#[ktest]
fn bitinsert() {
    for (target, source, start, length) in [
        (0x0, 0xff, 0, 8),
        (0xffff_0000_ffff, 0xffff, 16, 16),
        (0xdeadfeed, 0xaaa, 13, 7),
        (0xbbbb_bbbb_bbbb_bbbb, 0xaaaa_aaaa_aaaa_aaaa, 0, 64),
    ] {
        assert_eq!(
            bit_insert(target, source, start, length),
            harness(target, source, start, length)
        );
    }

    fn harness(target: u64, source: u64, start: u64, length: u64) -> u64 {
        let model = models::get("aarch64").unwrap();

        let register_file = RegisterFile::init(&*model);

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        {
            let target = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));
            let source = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(64));
            let start = emitter.constant(start, Type::Signed(64));
            let length = emitter.constant(length, Type::Signed(64));

            let inserted = emitter.bit_insert(target, source, start, length);

            emitter.write_register(model.reg_offset("R2"), inserted);

            emitter.leave();
        }

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);
        // log::trace!("{translation:?}");

        register_file.write::<u64>("R0", target);
        register_file.write::<u64>("R1", source);

        translation.execute(&register_file);

        register_file.read::<u64>("R2")
    }
}

#[ktest]
fn ubfx() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    // ubfx x3, x3, #16, #4
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd3504c63, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);
    register_file.write("R3", 0x8444_c004u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R3"), 0x4);
}

#[ktest]
fn highest_set_bit() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(0b100, Type::Unsigned(64));
    let res = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 2,
            width: 64
        }
    );

    let x = emitter.constant(u64::MAX, Type::Unsigned(64));
    let res = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[x],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 63,
            width: 64
        }
    );
}

#[ktest]
fn ror() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(0xff00, Type::Unsigned(64));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "ROR",
        &[x, shift],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xff,
            width: 64
        }
    );

    let x = emitter.constant(0xff, Type::Unsigned(64));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "ROR",
        &[x, shift],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xff00_0000_0000_0000,
            width: 64
        }
    );

    let x = emitter.constant(0xff, Type::Unsigned(32));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "ROR",
        &[x, shift],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xff00_0000,
            width: 32
        }
    );
}

#[ktest]
fn extsv() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let m = emitter.constant(32, Type::Signed(64));
    let v = emitter.constant(0xFFFF_FFFF_FFFF_FFFF, Type::Unsigned(64));
    let res = translate(
        Global,
        &*model,
        "extsv",
        &[m, v],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xFFFF_FFFF,
            width: 32
        }
    );
    let m = emitter.constant(64, Type::Signed(64));
    let v = emitter.constant(-1i32 as u64, Type::Unsigned(32));
    let res = translate(
        Global,
        &*model,
        "extsv",
        &[m, v],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: -1i64 as u64,
            width: 64
        }
    );
    let m = emitter.constant(64, Type::Signed(64));
    let v = emitter.constant(1, Type::Unsigned(1));
    let res = translate(
        Global,
        &*model,
        "extsv",
        &[m, v],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: u64::MAX,
            width: 64
        }
    );

    let m = emitter.constant(1, Type::Signed(64));
    let v = emitter.constant(1, Type::Unsigned(1));
    let res = translate(
        Global,
        &*model,
        "extsv",
        &[m, v],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(res.kind(), &NodeKind::Constant { value: 1, width: 1 });
}

#[ktest]
fn zext_ones() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let n = emitter.constant(1, Type::Signed(64));
    let m = emitter.constant(1, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(res.kind(), &NodeKind::Constant { value: 1, width: 1 });

    let n = emitter.constant(64, Type::Signed(64));
    let m = emitter.constant(0, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0,
            width: 64
        }
    );

    let n = emitter.constant(64, Type::Signed(64));
    let m = emitter.constant(32, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xFFFF_FFFF,
            width: 64
        }
    );

    let n = emitter.constant(64, Type::Signed(64));
    let m = emitter.constant(64, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: u64::MAX,
            width: 64
        }
    );
}

#[ktest]
fn decodebitmasks() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    // times out:(
    // assert_eq!(
    //     interpret(
    //         &*model,
    //         "DecodeBitMasks",
    //         &[
    //             Value::UnsignedInteger {
    //                 value: 1,
    //                 length: 1,
    //             },
    //             Value::UnsignedInteger {
    //                 value: 0x13,
    //                 length: 6,
    //             },
    //             Value::UnsignedInteger {
    //                 value: 0x10,
    //                 length: 6,
    //             },
    //             Value::UnsignedInteger {
    //                 value: 0,
    //                 length: 1,
    //             },
    //             Value::SignedInteger {
    //                 value: 0x40,
    //                 length: 64,
    //             },
    //         ],
    //         &register_file,
    //     ),
    //     Value::Tuple(alloc::vec![
    //         Value::UnsignedInteger {
    //             value: 0xFFFF00000000000F,
    //             length: 64
    //         },
    //         Value::UnsignedInteger {
    //             value: 0xF,
    //             length: 64
    //         }
    //     ])
    // );

    let immn = emitter.constant(1, Type::Unsigned(1));
    let imms = emitter.constant(0x13, Type::Unsigned(6));
    let immr = emitter.constant(0x10, Type::Unsigned(6));
    let immediate = emitter.constant(0, Type::Unsigned(1));
    let m = emitter.constant(0x40, Type::Signed(64));
    let res = translate(
        Global,
        &*model,
        "DecodeBitMasks",
        &[immn, imms, immr, immediate, m],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        emitter.access_tuple(res.clone(), 0).kind(),
        &NodeKind::Constant {
            value: 0xFFFF00000000000F,
            width: 64
        }
    );
    assert_eq!(
        emitter.access_tuple(res, 1).kind(),
        &NodeKind::Constant {
            value: 0xF,
            width: 64
        }
    );
}

#[ktest]
fn replicate_bits() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    {
        let value = emitter.constant(0xaa, Type::Unsigned(8));
        let count = emitter.constant(2, Type::Signed(64));
        assert_eq!(
            &NodeKind::Constant {
                value: 0xaaaa,
                width: 16
            },
            translate(
                Global,
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                &register_file,
            )
            .unwrap()
            .unwrap()
            .kind()
        );
    }
    {
        let value = emitter.constant(0x1, Type::Unsigned(1));
        let count = emitter.constant(32, Type::Signed(64));
        assert_eq!(
            &NodeKind::Constant {
                value: 0xffff_ffff,
                width: 32
            },
            translate(
                Global,
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                &register_file,
            )
            .unwrap()
            .unwrap()
            .kind()
        );
    }
    {
        let value = emitter.constant(0xaaff, Type::Unsigned(16));
        let count = emitter.constant(4, Type::Signed(64));
        assert_eq!(
            &NodeKind::Constant {
                value: 0xaaff_aaff_aaff_aaff,
                width: 64
            },
            translate(
                Global,
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                &register_file,
            )
            .unwrap()
            .unwrap()
            .kind()
        );
    }
}

#[ktest]
fn rev_d00dfeed() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let _32 = emitter.constant(32, Type::Signed(64));
    let _3 = emitter.constant(3, Type::Signed(64));
    translate(
        Global,
        &*model,
        "execute_aarch64_instrs_integer_arithmetic_rev",
        &[_32.clone(), _3.clone(), _32, _3],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);
    register_file.write("R3", 0xedfe0dd0u64);

    translation.execute(&register_file);
    assert_eq!(0xd00dfeed, register_file.read::<u64>("R3"));
}

#[ktest]
fn place_slice() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let m = emitter.constant(64, Type::Signed(64));
    let xs = emitter.constant(0xffffffd8, Type::Unsigned(64));
    let i = emitter.constant(0, Type::Signed(64));
    let l = emitter.constant(32, Type::Signed(64));
    let shift = emitter.constant(0, Type::Signed(64));

    let res = translate(
        Global,
        &*model,
        "place_slice_signed",
        &[m, xs, i, l, shift],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xffffffffffffffd8,
            width: 64
        }
    );
}

#[ktest]
fn udiv() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x9ac10a73, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let x = 0xffffff8008bfffffu64;
    let y = 0x200000u64;

    register_file.write("SEE", -1i64);
    register_file.write("R1", y);
    register_file.write("R19", x);

    translation.execute(&register_file);

    assert_eq!(0x7fffffc0045, register_file.read::<u64>("R19"));
}

// #[ktest]
// fn to_real_const() {
//     let model = models::get("aarch64").unwrap();

//     let  register_file = RegisterFile::init(&*model);
//

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     let i = emitter.constant(1, Type::Signed(64));

//     let res = translate(Global,&*model, "to_real", &[i], &mut emitter,
// register_file_ptr);

//     panic!("{res:?}")
// }

// #[ktest]
// fn to_real_dyn() {
//     let model = models::get("aarch64").unwrap();

//     let  register_file = RegisterFile::init(&*model);
//

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     let r = emitter.read_register(0, Type::Signed(64));

//     let res = translate(Global,&*model, "to_real", &[r], &mut emitter,
// register_file_ptr);

//     panic!("{res:?}")
// }

#[ktest]
fn floor() {
    assert_eq!(0, harness(3, 4));
    assert_eq!(1, harness(5, 4));
    assert_eq!(2, harness(8, 4));

    fn harness(n: i64, d: i64) -> i64 {
        let model = models::get("aarch64").unwrap();

        let register_file = RegisterFile::init(&*model);

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        {
            let n = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));
            let d = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(64));

            let real = emitter.create_tuple(alloc::vec![n, d]);
            let floor =
                emitter.unary_operation(crate::dbt::x86::emitter::UnaryOperationKind::Floor(real));
            emitter.write_register(model.reg_offset("R0"), floor);
        }
        emitter.leave();

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        register_file.write("R0", n);
        register_file.write("R1", d);

        translation.execute(&register_file);

        register_file.read::<i64>("R0")
    }
}

#[ktest]
fn ceil() {
    assert_eq!(1, harness(3, 4));
    assert_eq!(2, harness(5, 4));
    assert_eq!(2, harness(8, 4));

    fn harness(n: i64, d: i64) -> i64 {
        let model = models::get("aarch64").unwrap();

        let register_file = RegisterFile::init(&*model);

        let mut ctx = X86TranslationContext::new(&model, false);
        let mut emitter = X86Emitter::new(&mut ctx);

        {
            let n = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));
            let d = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(64));

            let real = emitter.create_tuple(alloc::vec![n, d]);
            let floor =
                emitter.unary_operation(crate::dbt::x86::emitter::UnaryOperationKind::Ceil(real));
            emitter.write_register(model.reg_offset("R0"), floor);
        }
        emitter.leave();

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        register_file.write("R0", n);
        register_file.write("R1", d);

        translation.execute(&register_file);

        register_file.read::<i64>("R0")
    }
}

#[ktest]
fn msr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  d51be000        msr     cntfrq_el0, x0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd51be000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);

    translation.execute(&register_file);
    // todo: test more here
}

#[ktest]
fn stp() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    //  a9bf7bfd        stp     x29, x30, [sp, #-16]!
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xa9bf7bfd, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();
    //__DecodeA64_LoadStore
    // decode_stp_gen_aarch64_instrs_memory_pair_general_pre_idx
    // execute_aarch64_instrs_memory_pair_general_post_idx

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let dst = Box::<(u64, u64)>::new((0, 0));

    register_file.write("SEE", -1i64);
    register_file.write("R29", 0xFEEDu64);
    register_file.write("R30", 0xDEADu64);
    register_file.write("SP_EL3", (((&*dst) as *const (u64, u64)) as u64) + 16);

    translation.execute(&register_file);

    assert_eq!(*dst, (0xFEED, 0xDEAD));
}

#[ktest]
fn ldrsw() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  b9802fe0        ldrsw   x0, [sp, #44]
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xb9802fe0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    // DEBUG [kernel::dbt::translate] translating "__DecodeA64_LoadStore"
    // DEBUG [kernel::dbt::translate] translating
    // "decode_ldrsw_imm_aarch64_instrs_memory_single_general_immediate_unsigned"
    // DEBUG [kernel::dbt::translate] translating
    // "execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx"

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    // verified with this program:
    // let input: u64 = 0x8001_0000;
    // let input_ptr: u64 = (&input as *const u64) as u64;
    // let mut result: u64;
    // unsafe {
    //     asm!(
    //         "
    //             mov sp, {:x}
    //             ldrsw   x0, [sp, #0]
    //             mov {:x}, x0
    //         ",
    //         in(reg) input_ptr,
    //         out(reg) result
    //     )
    // }
    // println!("{result:x}");

    let src = Box::<u32>::new(0x8001_0000); // negative signed 32-bit int

    register_file.write("R0", 0xDEADu64);

    register_file.write("SP_EL3", (((&*src) as *const u32) as u64) - 44);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0xffff_ffff_8001_0000);
}

#[ktest]
fn get_num_event_counters_accessible() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let result = translate(
        Global,
        &*model,
        "AArch64_GetNumEventCountersAccessible",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    emitter.write_register(model.reg_offset("R0"), result);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 31);
}

#[ktest]
fn sub_pc() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  d10043ff    sub                sp, sp, #0x10
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd10043ff, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("SP_EL3", 0xdeadbe90);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("SP_EL3"), 0xdeadbe80);
}

#[ktest]
fn lsrv() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  lsrv              x0, x1, x0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x9ac02420, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("R0", 0x3cu64);
    register_file.write("R1", 0x3u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0x0);
}

#[ktest]
fn mem_load_immediate() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  ldr                w0, 0xdc
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x180006e0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let mut src = Box::<u64>::new(0xBEE5BEE5);

    register_file.write("_PC", (&mut *src) as *mut u64 as u64 - 0xdc);
    register_file.write("R0", 0x0u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0xBEE5BEE5);
}

#[ktest]
fn eret() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  eret
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd69f03e0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("SPSR_EL3_bits", 6);

    assert_eq!(register_file.read::<u8>("PSTATE_EL"), 3);

    register_file.write::<u64>("ELR_EL3", 0x8000_0020);

    // uncommenting causes DBT runtime assert, commenting causes panic on line 2443
    // log::info!("{translation:?}");

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_PC"), 0x8000_0020);
}

#[ktest]
fn clz() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //clz               x9, x9
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xdac01129, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    let translation = ctx.compile(num_regs);

    register_file.write("R9", 0x1u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R9"), 63);
}

#[ktest]
fn highest_set_bit_const() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let bv = emitter.constant(0x1, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 0,
            width: 64
        }
    );

    let bv = emitter.constant(0b1000, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 3,
            width: 64
        }
    );

    let bv = emitter.constant(u64::MAX, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 63,
            width: 64
        }
    );

    let bv = emitter.constant(u8::MAX as u64, Type::Unsigned(8));
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 7,
            width: 64
        }
    );

    let bv = emitter.constant(
        0b0001_0000_0001_1010_1000_1010_1000_1010,
        Type::Unsigned(32),
    );
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 28,
            width: 64
        }
    );
}

#[ktest]
fn count_leading_zero_bits_const() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let bv = emitter.constant(0x0, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "CountLeadingZeroBits",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 64,
            width: 64
        }
    );

    let bv = emitter.constant(0b1000, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "CountLeadingZeroBits",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 60,
            width: 64
        }
    );

    let bv = emitter.constant(u64::MAX, Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "CountLeadingZeroBits",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 0,
            width: 64
        }
    );

    let bv = emitter.constant(u8::MAX as u64, Type::Unsigned(8));
    let n = translate(
        Global,
        &*model,
        "CountLeadingZeroBits",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 0,
            width: 64
        }
    );

    let bv = emitter.constant(
        0b0001_0000_0001_1010_1000_1010_1000_1010,
        Type::Unsigned(32),
    );
    let n = translate(
        Global,
        &*model,
        "CountLeadingZeroBits",
        &[bv],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        *n.kind(),
        NodeKind::Constant {
            value: 3,
            width: 64
        }
    );
}

#[ktest]
fn highest_set_bit_dynamic() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let r0 = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));
    let n = translate(
        Global,
        &*model,
        "HighestSetBit",
        &[r0],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();
    emitter.write_register(model.reg_offset("R0"), n);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    //   log::info!("{translation:?}");

    register_file.write::<u64>("R0", 0x1);

    translation.execute(&register_file);
    assert_eq!(register_file.read::<u64>("R0"), 0);
}

#[ktest]
fn msr_daifclr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  d50348ff        msr               daifclr, #0x8
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd50348ff, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("SEE", -1i64);

    translation.execute(&register_file);
    // todo: test more here
}

#[ktest]
fn current_security_state_is_const() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let state = translate(
        Global,
        &*model,
        "CurrentSecurityState",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    assert_eq!(
        *state.unwrap().kind(),
        NodeKind::Constant {
            value: 2,
            width: 32
        }
    )
}

#[ktest]
fn sys_movzx_investigation() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  sys               #3, c7, c4, #1, x8
    // (dc      zva, x8)
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd50b7428, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);

    let mut dst = Box::new(0xAAu8);

    register_file.write::<u64>("R0", (&mut *dst as *mut u8) as u64);

    // memory not set up for tests
    //         panicked at kernel/src/guest/mod.rs:51:18:
    // null pointer dereference occurred
    //   translation.execute(&register_file);

    //   assert_eq!(*dst, 0x0);
}

#[ktest]
fn ttbr1_el1_write() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let val = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));

    translate(
        Global,
        &*model,
        "TTBR1_EL1_write",
        &[val],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R0", 0xF0F0_0000_F0F0_0000);
    register_file.write::<u64>("_TTBR1_EL1_bits", 0x0);
    translation.execute(&register_file);

    assert_eq!(
        register_file.read::<u64>("_TTBR1_EL1_bits"),
        0xF0F0_0000_F0F0_0000
    );
}

#[ktest]
fn aarch64_sysregwrite() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    // [X86NodeRef(X86Node { typ: Unsigned(2), kind: Constant { value: 3, width: 2 }
    // }), X86NodeRef(X86Node { typ: Unsigned(2), kind: Constant { value: 3, width:
    // 2 } }), X86NodeRef(X86Node { typ: Unsigned(3), kind: Constant { value: 0,
    // width: 3 } }), X86NodeRef(X86Node { typ: Unsigned(4), kind: Constant { value:
    // 2, width: 4 } }), X86NodeRef(X86Node { typ: Unsigned(3), kind: Constant {
    // value: 1, width: 3 } }), X86NodeRef(X86Node { typ: Unsigned(4), kind:
    // Constant { value: 0, width: 4 } }), X86NodeRef(X86Node { typ: Signed(64),
    // kind: Constant { value: 1, width: 64 } })]

    let el = emitter.constant(3, Type::Unsigned(2));
    let op0 = emitter.constant(3, Type::Unsigned(2));
    let op1 = emitter.constant(0, Type::Unsigned(3));
    let crn = emitter.constant(2, Type::Unsigned(4));
    let op2 = emitter.constant(1, Type::Unsigned(3));
    let crm = emitter.constant(0, Type::Unsigned(4));
    let t = emitter.constant(1, Type::Signed(64));

    translate(
        Global,
        &*model,
        "TTBR1_EL1_SysRegWrite_949dc27ace2a7dbe",
        &[el, op0, op1, crn, op2, crm, t],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R1", 0x8224e000);
    register_file.write::<u64>("_TTBR1_EL1_bits", 0x0);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_TTBR1_EL1_bits"), 0x8224e000);
}

#[ktest]
fn msr_ttbr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  msr               ttbr1_el1, x1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd5182021, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R1", 0x8224e000);
    register_file.write::<u64>("_TTBR1_EL1_bits", 0x0);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_TTBR1_EL1_bits"), 0x8224e000);
}

#[ktest]
fn branch_link_pc_flag() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    assert!(!ctx.get_pc_write_flag());

    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  bl         0x1134
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x9400044d, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    assert!(ctx.get_pc_write_flag());
}

#[ktest]
fn mrs_mpidr_el1() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    assert_eq!(register_file.read::<u64>("MPIDR_EL1_bits"), 0x80000000);
    register_file.write("SEE", -1i64);

    // mrs     x5, mpidr_el1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd53800a5, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R5"), 0x80000000);
}

#[ktest]
fn mov_300000() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  mov     x4, #0x300000
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd2a00604, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R4"), 0x300000);
}

#[ktest]
fn mrs_ctr_el0() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //          mrs     x3, ctr_el0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd53b0023, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R3"), 0x4_8444_8004);
}

#[ktest]
fn mrs_id_aa64dfr0_el1() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    assert_eq!(
        register_file.read::<u64>("ID_AA64DFR0_EL1_bits"),
        0x112101f5e1e1e91b
    );

    register_file.write("SEE", -1i64);

    // mrs               x1, id_aa64dfr0_el1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd5380501, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R1"), 0x112101f5e1e1e91b);
}

// /// disabled because of failing the second assertion
// /// panicked at kernel/src/dbt/tests.rs:3294:9:
// /// assertion `left == right` failed
// /// left: 1234269928444520731
// /// right: 1373915719029297426
// ///
// /// which I got from the Sail interpreter logs from the mrs instruction
// ///
// /// but now the trace is valid even with the test failing
// ///
// /// leaving off for now but can always come back later
// ///
// #[ktest]
// fn mrs_id_aa64pfr0_el1() {
//     let model = models::get("aarch64").unwrap();

//     let  register_file = RegisterFile::init(&*model);
//
//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     unsafe {
//         let id_aa64pfr0_el1 =
//             *(register_file_ptr.add(model.reg_offset("ID_AA64PFR0_EL1_bits")
// as usize) as *mut u64);         assert_eq!(id_aa64pfr0_el1,
// 0x1311211130111112);

//         let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as
// *mut i64;         register_file.write("SEE", -1i64);
//     }

//     // mrs               x1, id_aa64pfr0_el1
//     let pc = emitter.constant(0, Type::Unsigned(64));
//     let opcode = emitter.constant(0xd5380501, Type::Unsigned(32));
//     translate(Global,
//         &*model,
//         "__DecodeA64",
//         &[pc, opcode],
//         &mut emitter,
//         &register_file,
//     );

//     emitter.leave();

//     let num_regs = emitter.next_vreg();
//     let translation = ctx.compile(num_regs);

//     unsafe {
//         let x1 = register_file_ptr.add(model.reg_offset("R1") as usize) as
// *mut u64;

//         translation.execute(&register_file);

//         assert_eq!(*x1, 0x1311211130111112);
//     }
//}
#[ktest]
fn ldaxr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    // ldaxr            x3, [x0]
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xc85ffc03, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);
}

#[ktest]
fn slow_msr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd5184000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);
}

#[ktest]
fn slow_msr_2() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd5181000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);
}

#[ktest]
fn csinc() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    // csinc		w3, wzr, wzr, ne
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x1a9f17e3, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("PSTATE_Z", 0x1u8);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R3"), 0x1);
}

#[ktest]
fn ldrh() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //   78635823        ldrh    w3, [x1, w3, uxtw #1]
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x78635823, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let src = Box::<u32>::new(0xAAAA_DEAD); // negative signed 32-bit int

    register_file.write("R3", 0xABu64);

    register_file.write(
        "R1",
        ((&*src) as *const u32) as u64 - (register_file.read::<u64>("R3") << 1),
    );

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R3"), 0x0000_DEAD);
}

#[ktest]
fn csneg() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  5a8307e3        csneg   w3, wzr, w3, eq // eq = none
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x5a8307e3, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R3", 0x9);
    register_file.write("PSTATE_Z", 0u8);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R3"), 0xfffffff7);
}

#[ktest]
fn ldp() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    //  a9405400        ldp     x0, x21, [x0]
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xa9405400, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let src = Box::<(u64, u64)>::new((0xBBBB_BBBB_BBBB_BBBB, 0xCCCC_CCCC_CCCC_CCCC));

    register_file.write("SEE", -1i64);
    register_file.write("R0", ((&*src) as *const (u64, u64)) as u64);
    register_file.write("R21", 0xAAAA_AAAA_AAAA_AAAAu64);

    translation.execute(&register_file);

    assert_eq!(
        (
            register_file.read::<u64>("R0"),
            register_file.read::<u64>("R21")
        ),
        *src
    );
}

#[ktest]
fn mem_load_32_bit() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  ldr		w0, [x0]
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xb9400000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    let mut src = Box::<u32>::new(0xF1F0F1F0);

    register_file.write::<u64>("R0", ((&mut *src) as *mut u32) as u64);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0xF1F0F1F0);
}

#[ktest]
fn ccmp() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  ccmp x5, #0x0, #0x0, eq
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xfa4008a0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("R5", 0x0u64);
    register_file.write::<u8>("PSTATE_N", 1);
    register_file.write::<u8>("PSTATE_Z", 0);
    register_file.write::<u8>("PSTATE_C", 0);
    register_file.write::<u8>("PSTATE_V", 0);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u8>("PSTATE_N"), 0);
    assert_eq!(register_file.read::<u8>("PSTATE_Z"), 0);
    assert_eq!(register_file.read::<u8>("PSTATE_C"), 0);
    assert_eq!(register_file.read::<u8>("PSTATE_V"), 0);
    assert_eq!(register_file.read::<u64>("R5"), 0);
}

#[ktest]
fn msr_elr_el2() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);
    // msr		elr_el2, x4
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd51c4024, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R4", 0x82080000);

    // uncommenting causes DBT runtime assert, commenting causes panic on line 2443
    //log::info!("{translation:?}");

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("ELR_EL2"), 0x82080000);
}

#[ktest]
fn eret_3() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //  eret
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd69f03e0, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("SPSR_EL3_bits", 0x3c9); // PSTATE.EL  = spsr<3:2>;
    register_file.write("SCR_EL3_bits", 0b1); // SCR_EL3.NS = 0
    assert_eq!(register_file.read::<u8>("PSTATE_EL"), 3);
    register_file.write::<u64>("ELR_EL3", 0x80000004);

    // log::info!("{translation:?}");

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_PC"), 0x80000004);
    assert_eq!(register_file.read::<u8>("PSTATE_EL"), 2);
}

#[ktest]
fn exception_return() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let new_pc = emitter.constant(0x80000004, Type::Unsigned(64));
    let spsr = emitter.constant(0x3c9, Type::Unsigned(64));
    translate(
        Global,
        &*model,
        "AArch64_ExceptionReturn",
        &[new_pc, spsr],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    let translation = ctx.compile(num_regs);

    assert_eq!(register_file.read::<u8>("PSTATE_EL"), 3);
    register_file.write("SCR_EL3_bits", 0b1);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_PC"), 0x80000004);
    assert_eq!(register_file.read::<u8>("PSTATE_EL"), 2);
    //  assert_eq!(*el, 2); todo: find out why this assertion fails
}

#[ktest]
fn illegal_exception_return() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let spsr = emitter.constant(0x3c9, Type::Unsigned(64));
    let illegal_psr_state = translate(
        Global,
        &*model,
        "IllegalExceptionReturn",
        &[spsr],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), illegal_psr_state);
    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    assert_eq!(register_file.read::<u64>("R0"), 0x0);
    register_file.write("SCR_EL3_bits", 0x5b1);
    register_file.write("SCTLR_EL2_bits", 0x30c50830);
    register_file.write("CPTR_EL2_bits", 0x33ff);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0x0);
}

#[ktest]
fn el_from_spsr() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let spsr = emitter.constant(0b1111001001, Type::Unsigned(64));
    let valid_target_tuple = translate(
        Global,
        &*model,
        "ELFromSPSR",
        &[spsr],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    let valid = emitter.access_tuple(valid_target_tuple.clone(), 0);
    let target = emitter.access_tuple(valid_target_tuple, 1);
    emitter.write_register(model.reg_offset("R0"), valid);
    emitter.write_register(model.reg_offset("R1"), target);
    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    assert_eq!(register_file.read::<u64>("R0"), 0x0);
    register_file.write("SCR_EL3_bits", 0x5b1);
    register_file.write("SCTLR_EL2_bits", 0x30c50830);
    register_file.write("CPTR_EL2_bits", 0x33ff);

    translation.execute(&register_file);

    // valid = true
    assert_eq!(register_file.read::<u64>("R0"), 0x1);

    // EL should be 2 afterwards
    assert_eq!(register_file.read::<u64>("R1"), 0x2);
}

#[ktest]
fn el_state_using_aarch32k() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let el = emitter.constant(1, Type::Unsigned(2));

    let secure = emitter.node(X86Node {
        typ: Type::Unsigned(1),
        kind: NodeKind::BinaryOperation(BinaryOperationKind::CompareEqual(
            emitter.node(X86Node {
                typ: Type::Unsigned(1),
                kind: NodeKind::Cast {
                    value: emitter.node(X86Node {
                        typ: Type::Unsigned(1),
                        kind: NodeKind::BinaryOperation(BinaryOperationKind::And(
                            emitter.node(X86Node {
                                typ: Type::Unsigned(1),
                                kind: NodeKind::Cast {
                                    value: emitter.node(X86Node {
                                        typ: Type::Unsigned(64),
                                        kind: NodeKind::Shift {
                                            value: emitter.node(X86Node {
                                                typ: Type::Unsigned(64),
                                                kind: NodeKind::GuestRegister { offset: 7696 },
                                            }),
                                            amount: emitter.node(X86Node {
                                                typ: Type::Signed(64),
                                                kind: NodeKind::Constant {
                                                    value: 0,
                                                    width: 64,
                                                },
                                            }),
                                            kind: ShiftOperationKind::LogicalShiftRight,
                                        },
                                    }),
                                    kind: CastOperationKind::Truncate,
                                },
                            }),
                            emitter.node(X86Node {
                                typ: Type::Unsigned(1),
                                kind: NodeKind::Constant { value: 1, width: 1 },
                            }),
                        )),
                    }),
                    kind: CastOperationKind::Truncate,
                },
            }),
            emitter.node(X86Node {
                typ: Type::Unsigned(1),
                kind: NodeKind::Constant { value: 0, width: 1 },
            }),
        )),
    });
    let known_aarch32_tuple = translate(
        Global,
        &*model,
        "ELStateUsingAArch32K",
        &[el, secure],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    let known = emitter.access_tuple(known_aarch32_tuple.clone(), 0);
    let aarch32 = emitter.access_tuple(known_aarch32_tuple, 1);
    emitter.write_register(model.reg_offset("R0"), known);
    emitter.write_register(model.reg_offset("R1"), aarch32);
    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 1);
    assert_eq!(register_file.read::<u64>("R1"), 0);
}

#[ktest]
fn el_state_using_aarch32k_dynamic() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let target = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(2));

    let tuple = translate(
        Global,
        &*model,
        "ELUsingAArch32K",
        &[target],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    let known = emitter.access_tuple(tuple.clone(), 0);
    emitter.write_register(model.reg_offset("R1"), known);

    let aarch32 = emitter.access_tuple(tuple, 1);
    emitter.write_register(model.reg_offset("R2"), aarch32);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    // EL2 target?
    register_file.write::<u64>("R0", 2);

    translation.execute(&register_file);

    // known
    assert_eq!(register_file.read::<u64>("R1"), 1);
    // target_el_is_aarch32
    assert_eq!(register_file.read::<u64>("R2"), 0);
}

#[ktest]
fn have_aarch64() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let have_aarch64 = translate(
        Global,
        &*model,
        "HaveAArch64",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        *have_aarch64.kind(),
        NodeKind::Constant { value: 1, width: 1 }
    )
}

#[ktest]
fn xpaclri() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);
    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    // nop
    translate_instruction(
        Global,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0x0,
        0xd503201f,
    )
    .unwrap();

    translate_instruction(
        Global,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0x0,
        0xd50320ff,
    )
    .unwrap();

    // nop
    translate_instruction(
        Global,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0x0,
        0xd503201f,
    )
    .unwrap();

    // nop
    translate_instruction(
        Global,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0x0,
        0xd503201f,
    )
    .unwrap();

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);
}

// todo: enable me
// #[ktest]
fn _brk() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    // brk              #0x800
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd4210000, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);
}

#[ktest]
fn lsr() {
    //d360fd08        lsr     x8, x8, #32
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //   d360fd08        lsr     x8, x8, #32
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd360fd08, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write("R8", 0x1);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R8"), 0x1 >> 32);
}

// #[ktest]
// fn br_btype() {
//     let model = models::get("aarch64").unwrap();

//     let register_file = RegisterFile::init(&*model);

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     register_file.write("SEE", -1i64);

//     //   0xd61f0100  br                x8
//     translate_instruction(
//         Global,
//         &*model,
//         "__DecodeA64",
//         &mut emitter,
//         &register_file,
//         0,
//         0xd61f0100,
//     )
//     .unwrap();

//     emitter.leave();

//     let num_regs = emitter.next_vreg();
//     let translation = ctx.compile(num_regs);

//     register_file.write::<u64>("R8", 0xffffffc008250254);
//   //  assert_eq!(register_file.read::<u8>("BTypeNext"), 0x0);

//     translation.execute(&register_file);

//     assert_eq!(register_file.read::<u64>("_PC"), 0xffffffc008250254);
//     // assert_eq!(register_file.read::<u8>("BTypeNext"), 0x1);
//     // assert_eq!(register_file.read::<u8>("PSTATE_BTYPE"), 0x1);
// }

// #[ktest]
// fn bl_btype() {
//     let model = models::get("aarch64").unwrap();

//     let register_file = RegisterFile::init(&*model);

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     register_file.write("SEE", -1i64);

//     //   0x97ffffdf bl      0xffff_ffff_ffff_ff7c
//     translate_instruction(
//         Global,
//         &*model,
//         "__DecodeA64",
//         &mut emitter,
//         &register_file,
//         0x1000,
//         0x97ffffdf,
//     )
//     .unwrap();

//     emitter.leave();

//     let num_regs = emitter.next_vreg();
//     let translation = ctx.compile(num_regs);

//     register_file.write::<u64>("_PC", 0x1000);
//     register_file.write::<u8>("BTypeNext", 0x3);
//     register_file.write::<u8>("PSTATE_BTYPE", 0x3);

//     translation.execute(&register_file);

//     assert_eq!(register_file.read::<u64>("_PC"), 0x1000 - 132); // jumping
// back 132     assert_eq!(register_file.read::<u64>("R30"), 0x1000 + 4); //
// next instruction     // assert_eq!(register_file.read::<u8>("PSTATE_BTYPE"),
// 0x0); // todo     // assert_eq!(register_file.read::<u8>("BTypeNext"), 0x0);
// }

// #[ktest]
// fn mrs_btype() {
//     let model = models::get("aarch64").unwrap();

//     let register_file = RegisterFile::init(&*model);

//     let mut ctx = X86TranslationContext::new(&model, false);
//     let mut emitter = X86Emitter::new(&mut ctx);

//     register_file.write("SEE", -1i64);

//     //   d538d080        mrs     x0, tpidr_el1
//     translate_instruction(
//         Global,
//         &*model,
//         "__DecodeA64",
//         &mut emitter,
//         &register_file,
//         0,
//         0xd538d080,
//     )
//     .unwrap();

//     emitter.leave();

//     let num_regs = emitter.next_vreg();
//     let translation = ctx.compile(num_regs);

//     register_file.write::<u8>("BTypeNext", 0x3);
//     register_file.write::<u8>("PSTATE_BTYPE", 0x3);

//     translation.execute(&register_file);

//     // assert_eq!(register_file.read::<u8>("PSTATE_BTYPE"), 0x0);// todo
// }

#[ktest]
fn udf() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //   00000000        udf
    translate_instruction(
        Global,
        &*model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0,
        0x0,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    //  translation.execute(&register_file); // todo
}

#[ktest]
fn eret_post_exception() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    //   0xd69f03e0        eret
    translate_instruction(
        Global,
        &*model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0x82205034,
        0xd69f03e0,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("ELR_EL1", 0x8225_0008);
    register_file.write::<u8>("PSTATE_EL", 0x1);
    register_file.write::<u64>("SPSR_EL1_bits", 0x3c5);
    register_file.write::<u64>("SPSR_EL2_bits", 0x3c5);
    register_file.write::<u64>("SPSR_EL3_bits", 0x3c9);
    register_file.write::<u64>("_PC", 0x82205034);

    translation.execute(&register_file);

    //  assert_eq!(register_file.read::<u64>("_PC"), 0x8225_0008); // todo
}

#[ktest]
fn check_eret_trap() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);

    let pac = emitter.constant(0, Type::Unsigned(1));
    let use_key_a = emitter.constant(1, Type::Unsigned(1));
    let res = translate(
        Global,
        &*model,
        "AArch64_CheckForERetTrap",
        &[pac, use_key_a],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    assert!(res.is_none())
}

#[ktest]
fn exceptionreturn_post_exception() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    register_file.write("SEE", -1i64);
    register_file.write::<u8>("have_exception", 0);

    let new_pc = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(64));
    let spsr = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(64));

    //
    translate(
        Global,
        &*model,
        "AArch64_ExceptionReturn",
        &[new_pc, spsr],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    register_file.write::<u64>("R0", 0x8225_0008);
    register_file.write::<u64>("R1", 0x3c5);

    register_file.write::<u64>("ELR_EL1", 0x8225_0008);
    register_file.write::<u8>("PSTATE_EL", 0x1);
    register_file.write::<u64>("SPSR_EL1_bits", 0x3c5);
    register_file.write::<u64>("SPSR_EL2_bits", 0x3c5);
    register_file.write::<u64>("SPSR_EL3_bits", 0x3c9);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("_PC"), 0x8225_0008);
}

#[ktest]
fn leave_with_cache() {
    let model = models::get("aarch64").unwrap();

    let _register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    let aaaa = emitter.constant(0xAAAA, Type::Unsigned(64));
    emitter.write_register(model.reg_offset("_PC"), aaaa);

    emitter.leave_with_cache(0x1234);

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);
}

#[ktest]
fn end_cycle() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    translate(
        Global,
        &*model,
        "__EndCycle",
        &[],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let _translation = ctx.compile(num_regs);
}

#[ktest]
fn decodea64_profiling() {
    let model = models::get("aarch64").unwrap();

    let mut measure = Measurement::start();

    let allocator = BumpAllocator::new(1 * 1024 * 1024 * 1024);
    let allocator_ref = BumpAllocatorRef::new(&allocator);

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new_with_allocator(allocator_ref, &model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    measure.trigger("init");

    translate_instruction(
        allocator_ref,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0,
        0xd51be000,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    measure.trigger("translation");

    let translation = ctx.compile(num_regs);

    measure.trigger("compilation");

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 2);
    register_file.write::<u64>("R1", 43);

    translation.execute(&register_file);

    measure.trigger("execution");

    //log::info!("{translation:?}");

    // assert_eq!(55, (*see));// todo: re-implement depending on result of
    // SEE/cacheable registers work
}

#[ktest]
fn branch_profiling() {
    let model = models::get("aarch64").unwrap();

    let mut measure = Measurement::start();

    let allocator = BumpAllocator::new(1 * 1024 * 1024 * 1024);
    let allocator_ref = BumpAllocatorRef::new(&allocator);

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new_with_allocator(allocator_ref, &model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    measure.trigger("init");

    translate_instruction(
        allocator_ref,
        &model,
        "__DecodeA64",
        &mut emitter,
        &register_file,
        0,
        0xd61f0100,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();

    measure.trigger("translation");

    let translation = ctx.compile(num_regs);

    crate::println!("{translation:?}");

    measure.trigger("compilation");

    register_file.write("SEE", -1i64);
    register_file.write::<u64>("R0", 2);
    register_file.write::<u64>("R1", 43);

    translation.execute(&register_file);

    measure.trigger("execution");
}

#[ktest]
fn mrs_timer() {
    let model = models::get("aarch64").unwrap();

    let register_file = RegisterFile::init(&*model);

    let mut ctx = X86TranslationContext::new(&model, false);
    let mut emitter = X86Emitter::new(&mut ctx);

    assert_eq!(register_file.read::<u64>("MPIDR_EL1_bits"), 0x80000000);
    register_file.write("SEE", -1i64);

    // mrs     x0, cntvct_el0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd53be040, Type::Unsigned(32));
    translate(
        Global,
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        &register_file,
    )
    .unwrap();

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(&register_file);

    assert_eq!(register_file.read::<u64>("R0"), 0x1234);
}
