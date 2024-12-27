use {
    crate::dbt::{
        bit_insert,
        emitter::{Emitter, Type},
        init_register_file,
        interpret::{interpret, Value},
        models::{self},
        translate::translate,
        x86::{
            emitter::{
                BinaryOperationKind, CastOperationKind, NodeKind, ShiftOperationKind, X86Emitter,
            },
            X86TranslationContext,
        },
        Translation,
    },
    common::{mask::mask, HashMap},
    proc_macro_lib::ktest,
};

#[ktest]
fn init_system() {
    let model = models::get("aarch64").unwrap();

    let _register_file = init_register_file(&*model);
}

#[ktest]
fn static_dynamic_chaos_smoke() {
    fn run(mut register_file: [u64; 3]) -> [u64; 3] {
        let register_file_ptr = register_file.as_mut_ptr() as *mut u8;
        let model = models::get("aarch64").unwrap();

        {
            let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
            let mut emitter = X86Emitter::new(&mut ctx);

            translate(
                &*model,
                "func_corrupted_var",
                &[],
                &mut emitter,
                register_file_ptr,
            )
            .unwrap();

            emitter.leave();
            let num_regs = emitter.next_vreg();
            let translation = ctx.compile(num_regs);
            translation.execute(register_file_ptr);
        }

        register_file
    }

    assert_eq!(run([0, 0, 0]), [0, 0, 10]);
    assert_eq!(run([0, 1, 0]), [0, 1, 10]);
    assert_eq!(run([1, 0, 0]), [1, 0, 5]);
    assert_eq!(run([1, 1, 0]), [1, 1, 5]);
}

#[ktest]
fn num_of_feature() {
    let model = &*models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let feature = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(0x20));

    translate(
        &*model,
        "num_of_Feature",
        &[feature],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);
}

#[ktest]
fn statistical_profiling_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let is_enabled = translate(
        &*model,
        "StatisticalProfilingEnabled",
        &[],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut bool)
        )
    }
}

#[ktest]
fn havebrbext_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let is_enabled =
        translate(&*model, "HaveBRBExt", &[], &mut emitter, register_file_ptr).unwrap();

    emitter.write_register(model.reg_offset("R0"), is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut bool)
        )
    }
}

#[ktest]
fn using_aarch32_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let is_enabled = translate(
        &*model,
        "UsingAArch32",
        &[],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut bool)
        )
    }
}

#[ktest]
fn branchto() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let target = emitter.constant(0xDEADFEED, Type::Unsigned(64));
    let branch_type = emitter.constant(1, Type::Unsigned(32));
    let branch_conditional = emitter.constant(1, Type::Unsigned(1));
    translate(
        &*model,
        "BranchTo",
        &[target, branch_type, branch_conditional],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        assert_eq!(
            0x0,
            *(register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u64)
        );

        *(register_file_ptr.add(model.reg_offset("__BranchTaken") as usize) as *mut bool) = false;
    }

    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            0xDEADFEED,
            *(register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u64)
        );
        assert_eq!(
            true,
            *(register_file_ptr.add(model.reg_offset("__BranchTaken") as usize) as *mut bool)
        )
    }
}

#[ktest]
fn decodea64_addsub() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x8b020020, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u32;
        let r2 = register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *see = -1;
        *r0 = 2;
        *r1 = 5;
        *r2 = 10;

        translation.execute(register_file_ptr);

        assert_eq!(15, (*r0));
        assert_eq!(0xe, (*see));
    }
}

#[ktest]
fn decodea64_addsub_interpret() {
    unsafe {
        let model = models::get("aarch64").unwrap();

        let mut register_file = init_register_file(&*model);
        let register_file_ptr = register_file.as_mut_ptr();

        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u32;
        let r2 = register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *see = -1;
        *r0 = 2;
        *r1 = 5;
        *r2 = 10;

        let pc = crate::dbt::interpret::Value::UnsignedInteger {
            value: 0,
            width: 64,
        };
        let opcode = crate::dbt::interpret::Value::UnsignedInteger {
            value: 0x8b020020,
            width: 32,
        };
        interpret(&*model, "__DecodeA64", &[pc, opcode], register_file_ptr);

        assert_eq!(15, (*r0));
        assert_eq!(0xe, (*see));
    }
}

#[ktest]
fn decodea64_mov() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xaa0103e0, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *see = -1;
        *r0 = 2;
        *r1 = 43;

        translation.execute(register_file_ptr);

        assert_eq!(43, (*r0));
        assert_eq!(43, (*r1));
        assert_eq!(55, (*see));
    }
}

#[ktest]
fn decodea64_branch() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(44, Type::Unsigned(64));
    let opcode = emitter.constant(0x17fffffa, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    //  log::trace!("{translation:?}");

    unsafe {
        let pc = register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *pc = 44;
        *see = -1;

        translation.execute(register_file_ptr);

        assert_eq!(20, (*pc));
        assert_eq!(67, (*see));
    }
}

#[ktest]
fn branch_if_eq() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x540000c0, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;
        let pc = register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u32;
        let branch_taken =
            register_file_ptr.add(model.reg_offset("__BranchTaken") as usize) as *mut bool;

        *see = -1;

        translation.execute(register_file_ptr);

        assert_eq!(0x45, (*see));
        assert_eq!(0x0, (*pc));
        assert_eq!(true, (*branch_taken));
    }
}

#[ktest]
fn branch_uncond_imm_offset_math() {
    let mut ctx = X86TranslationContext::new(0);
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

        let mut register_file = init_register_file(&*model);
        let register_file_ptr = register_file.as_mut_ptr();

        let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
        let mut emitter = X86Emitter::new(&mut ctx);

        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(model.reg_offset("SEE"), see_value);

        // cmp     x2, x0
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0xeb00005f, Type::Unsigned(32));
        translate(
            &*model,
            "__DecodeA64",
            &[pc, opcode],
            &mut emitter,
            register_file_ptr,
        );

        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(model.reg_offset("SEE"), see_value);

        // csel    x2, x2, x0, ls  // ls = plast
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0x9a809042, Type::Unsigned(32));
        translate(
            &*model,
            "__DecodeA64",
            &[pc, opcode],
            &mut emitter,
            register_file_ptr,
        );

        emitter.leave();

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        unsafe {
            let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
            let r2 = register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u64;

            *r0 = pre_r0;
            *r2 = pre_r2;

            translation.execute(register_file_ptr);

            *r2
        }
    }
}

#[ktest]
fn fibonacci_instr() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

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

    unsafe {
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;
        let branch_taken =
            { register_file_ptr.add(model.reg_offset("__BranchTaken") as usize) as *mut bool };
        let pc = { register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u64 };
        let r0 = { register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64 };
        let r3 = { register_file_ptr.add(model.reg_offset("R3") as usize) as *mut u64 };

        // bounded just in case
        for _ in 0..100 {
            *see = -1;
            *branch_taken = false;

            // exit before the svc
            if *pc == 0x38 {
                break;
            }

            let model = models::get("aarch64").unwrap();

            let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
            let mut emitter = X86Emitter::new(&mut ctx);

            {
                let opcode = emitter.constant(program[*pc as usize / 4], Type::Unsigned(32));
                let pc = emitter.constant(*pc, Type::Unsigned(64));
                translate(
                    &*model,
                    "__DecodeA64",
                    &[pc, opcode],
                    &mut emitter,
                    register_file_ptr,
                );
            }

            emitter.leave();
            let num_regs = emitter.next_vreg();
            let translation = ctx.compile(num_regs);
            translation.execute(register_file_ptr);

            // increment PC if no branch was taken
            if !*branch_taken {
                *pc += 4;
            }
        }

        assert_eq!(89, *r0);
        assert_eq!(10, *r3);
    }
}

///  4000d4:	d2955fe0 	mov	x0, #0xaaff                	// #43775
///  4000d8:	d2800001 	mov	x1, #0x0                   	// #0
///  4000dc:	91500421 	add	x1, x1, #0x401, lsl #12
///  4000e0:	f9000020 	str	x0, [x1]
///  4000e4:	f9400020 	ldr	x0, [x1]
//#[ktest]
fn mem() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    //execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9000020, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    // log::trace!("translation:\n{translation:?}");

    unsafe {
        let mem = alloc::boxed::Box::new(0xdead_c0de_0000_0000u64);

        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;

        *see = -1;
        *r0 = 0xdeadcafe;
        *r1 = &*mem as *const u64 as u64;

        log::trace!(
            "see: {:016x}, r0: {:016x}, r1: {:016x}, mem: {:016x}",
            *see,
            *r0,
            *r1,
            *mem
        );

        translation.execute(register_file_ptr);

        log::trace!(
            "see: {:016x}, r0: {:016x}, r1: {:016x}, mem: {:016x}",
            *see,
            *r0,
            *r1,
            *mem
        );

        panic!();
    }
}

#[ktest]
fn mem_store() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9000020, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        const VALUE: u64 = 0xdead_c0de_0000_0000; // will be overwritten
        let mem = alloc::boxed::Box::new(0xdeadcafeu64);

        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;

        *see = -1;
        *r0 = VALUE;
        *r1 = &*mem as *const u64 as u64;

        translation.execute(register_file_ptr);

        assert_eq!(*mem, VALUE);
    }
}

#[ktest]
fn mem_load() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    //execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xf9400020, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        const VALUE: u64 = 0xdead_c0de_0000_0000;
        let mem = alloc::boxed::Box::new(VALUE);

        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;

        *see = -1;
        *r0 = 0xdeadcafe; // will be overwritten
        *r1 = &*mem as *const u64 as u64;

        translation.execute(register_file_ptr);

        assert_eq!(*r0, VALUE);
    }
}

#[ktest]
fn fibonacci_block() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

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
        unsafe {
            let pc_offset = model.reg_offset("_PC");
            let mut current_pc = *(register_file_ptr.add(pc_offset as usize) as *mut u64);
            let start_pc = current_pc;
            if let Some(translation) = blocks.get(&start_pc) {
                translation.execute(register_file.as_mut_ptr());
                continue;
            }

            if current_pc == 56 {
                break;
            }

            let mut ctx = X86TranslationContext::new(pc_offset);
            let mut emitter = X86Emitter::new(&mut ctx);

            loop {
                let neg1 = emitter.constant(-1i32 as u64, Type::Signed(32));
                emitter.write_register(model.reg_offset("SEE"), neg1);

                let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                emitter.write_register(model.reg_offset("__BranchTaken"), _false);

                {
                    let opcode =
                        emitter.constant(program[current_pc as usize / 4], Type::Unsigned(32));
                    let pc = emitter.constant(current_pc, Type::Unsigned(64));
                    let _return_value = translate(
                        &*model,
                        "__DecodeA64",
                        &[pc, opcode],
                        &mut emitter,
                        register_file_ptr,
                    );
                }

                if emitter.ctx().get_write_pc() || (current_pc == ((program.len() * 4) - 8) as u64)
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

            translation.execute(register_file.as_mut_ptr());
            blocks.insert(start_pc, translation);

            log::trace!(
                "{} {}",
                *(register_file_ptr.add(model.reg_offset("_PC") as usize) as *mut u64),
                *(register_file_ptr.add(model.reg_offset("__BranchTaken") as usize) as *mut u8)
            );
        }
    }

    unsafe {
        assert_eq!(
            1298777728820984005,
            *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64)
        );
        assert_eq!(
            100,
            *(register_file_ptr.add(model.reg_offset("R3") as usize) as *mut u64)
        );
    }
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let r0 = unsafe { register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64 };
    let r1 = unsafe { register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64 };
    let r2 = unsafe { register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u8 };

    unsafe {
        *r0 = x;
        *r1 = y;
        *r2 = carry_in as u8;
    }

    let x = emitter.read_register(model.reg_offset("R0"), Type::Unsigned(0x40));
    let y = emitter.read_register(model.reg_offset("R1"), Type::Unsigned(0x40));
    let carry_in = emitter.read_register(model.reg_offset("R2"), Type::Unsigned(0x8));

    let res = translate(
        &*model,
        "add_with_carry_test",
        &[x, y, carry_in],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();

    let sum = emitter.access_tuple(res.clone(), 0);
    emitter.write_register(model.reg_offset("R0"), sum);

    let flags = emitter.access_tuple(res.clone(), 1);
    emitter.write_register(model.reg_offset("R1"), flags);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    translation.execute(register_file_ptr);

    unsafe { (*r0, *(r1 as *mut u8)) }
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    unsafe {
        *(register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64) = x;
        *(register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64) = y;
        *(register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64) = -1;
    }

    // cmp    x0, x1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xeb01001f, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        *(register_file_ptr.add(model.reg_offset("PSTATE_N") as usize) as *mut u8) << 3
            | *(register_file_ptr.add(model.reg_offset("PSTATE_Z") as usize) as *mut u8) << 2
            | *(register_file_ptr.add(model.reg_offset("PSTATE_C") as usize) as *mut u8) << 1
            | *(register_file_ptr.add(model.reg_offset("PSTATE_V") as usize) as *mut u8)
    }
}

#[ktest]
fn shiftreg() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let _1 = emitter.constant(1, Type::Signed(64));
    let shift_type = emitter.constant(1, Type::Signed(32));
    let amount = emitter.constant(0, Type::Signed(64));
    let width = emitter.constant(64, Type::Signed(64));
    let value = translate(
        &*model,
        "ShiftReg",
        &[_1, shift_type, amount, width],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();

    emitter.write_register(model.reg_offset("R0"), value);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;

        *r0 = 0;
        *r1 = 0xdeadfeeddeadfeed;

        translation.execute(register_file_ptr);

        assert_eq!(*r0, 0xdeadfeeddeadfeed);
        assert_eq!(*r1, 0xdeadfeeddeadfeed);
    }
}

#[ktest]
fn floorpow2_constant() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(2048, Type::Signed(64));
    let value = translate(&*model, "FloorPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(2397, Type::Signed(64));
    let value = translate(&*model, "FloorPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(4095, Type::Signed(64));
    let value = translate(&*model, "FloorPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(1231, Type::Signed(64));
    let value = translate(&*model, "FloorPow2", &[x], &mut emitter, register_file_ptr).unwrap();
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(2048, Type::Signed(64));
    let value = translate(&*model, "CeilPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
    let x = emitter.constant(2397, Type::Signed(64));
    let value = translate(&*model, "CeilPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 4096,
            width: 64
        }
    );
    let x = emitter.constant(4095, Type::Signed(64));
    let value = translate(&*model, "CeilPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 4096,
            width: 64
        }
    );
    let x = emitter.constant(1231, Type::Signed(64));
    let value = translate(&*model, "CeilPow2", &[x], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        value.kind(),
        &NodeKind::Constant {
            value: 2048,
            width: 64
        }
    );
}

//#[ktest]
fn ispow2() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.read_register(model.reg_offset("R3"), Type::Unsigned(0x40));

    {
        let value = translate(
            &*model,
            "FloorPow2",
            &[x.clone()],
            &mut emitter,
            register_file_ptr,
        )
        .unwrap();
        emitter.write_register(model.reg_offset("R0"), value);
    }

    {
        let value = translate(
            &*model,
            "CeilPow2",
            &[x.clone()],
            &mut emitter,
            register_file_ptr,
        )
        .unwrap();
        emitter.write_register(model.reg_offset("R1"), value);
    }

    {
        let value = translate(&*model, "IsPow2", &[x], &mut emitter, register_file_ptr).unwrap();
        emitter.write_register(model.reg_offset("R2"), value);
    }

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    log::debug!("{translation:?}");

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;
        let r2 = register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u64;
        let r3 = register_file_ptr.add(model.reg_offset("R3") as usize) as *mut u64;

        *r0 = 0;
        *r1 = 0;
        *r2 = 0;
        *r3 = 2048;

        translation.execute(register_file_ptr);

        assert_eq!(*r0, *r1);
        assert_eq!(1, *r2)
    }
}

#[ktest]
fn rbitx0_interpret() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *r0 = 0x0123_4567_89ab_cdef;
        *see = -1;

        // rbit x0
        let pc = Value::UnsignedInteger {
            value: 0,
            width: 64,
        };
        let opcode = Value::UnsignedInteger {
            value: 0xdac00000,
            width: 32,
        };
        interpret(&*model, "__DecodeA64", &[pc, opcode], register_file_ptr);

        // assert bits are reversed
        assert_eq!(*r0, 0xf7b3_d591_e6a2_c480);
    }
}

#[ktest]
fn rbitx0() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    // rbit x0
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xdac00000, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *r0 = 0x0123_4567_89ab_cdef;
        *see = -1;

        translation.execute(register_file_ptr);

        // assert bits are reversed
        assert_eq!(*r0, 0xf7b3_d591_e6a2_c480);
    }
}

#[ktest]
fn bitinsert() {
    for (target, source, start, length) in [
        (0x0, 0xff, 0, 8),
        (0xffff_0000_ffff, 0xffff, 16, 16),
        (0xdeadfeed, 0xaaa, 13, 7),
    ] {
        assert_eq!(
            bit_insert(target, source, start, length),
            harness(target, source, start, length)
        );
    }

    fn harness(target: u64, source: u64, start: u64, length: u64) -> u64 {
        let model = models::get("aarch64").unwrap();

        let mut register_file = init_register_file(&*model);
        let register_file_ptr = register_file.as_mut_ptr();
        let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
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

        unsafe {
            let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut u64;
            let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;
            let r2 = register_file_ptr.add(model.reg_offset("R2") as usize) as *mut u64;

            *r0 = target;
            *r1 = source;

            translation.execute(register_file_ptr);

            *r2
        }
    }
}

#[ktest]
fn ubfx() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    // ubfx x3, x3, #16, #4
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd3504c63, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r3 = register_file_ptr.add(model.reg_offset("R3") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *r3 = 0x8444_c004;
        *see = -1;

        translation.execute(register_file_ptr);

        assert_eq!(*r3, 0x4);
    }
}

#[ktest]
fn highest_set_bit() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(0b100, Type::Unsigned(64));
    let res = translate(
        &*model,
        "HighestSetBit",
        &[x],
        &mut emitter,
        register_file_ptr,
    )
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
        &*model,
        "HighestSetBit",
        &[x],
        &mut emitter,
        register_file_ptr,
    )
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let x = emitter.constant(0xff00, Type::Unsigned(64));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(&*model, "ROR", &[x, shift], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xff,
            width: 64
        }
    );

    let x = emitter.constant(0xff, Type::Unsigned(64));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(&*model, "ROR", &[x, shift], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xff00_0000_0000_0000,
            width: 64
        }
    );

    let x = emitter.constant(0xff, Type::Unsigned(32));
    let shift = emitter.constant(8, Type::Signed(64));
    let res = translate(&*model, "ROR", &[x, shift], &mut emitter, register_file_ptr).unwrap();
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let m = emitter.constant(32, Type::Signed(64));
    let v = emitter.constant(0xFFFF_FFFF_FFFF_FFFF, Type::Unsigned(64));
    let res = translate(&*model, "extsv", &[m, v], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: 0xFFFF_FFFF,
            width: 32
        }
    );
    let m = emitter.constant(64, Type::Signed(64));
    let v = emitter.constant(-1i32 as u64, Type::Unsigned(32));
    let res = translate(&*model, "extsv", &[m, v], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: -1i64 as u64,
            width: 64
        }
    );
    let m = emitter.constant(64, Type::Signed(64));
    let v = emitter.constant(1, Type::Unsigned(1));
    let res = translate(&*model, "extsv", &[m, v], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(
        res.kind(),
        &NodeKind::Constant {
            value: u64::MAX,
            width: 64
        }
    );

    let m = emitter.constant(1, Type::Signed(64));
    let v = emitter.constant(1, Type::Unsigned(1));
    let res = translate(&*model, "extsv", &[m, v], &mut emitter, register_file_ptr).unwrap();
    assert_eq!(res.kind(), &NodeKind::Constant { value: 1, width: 1 });
}

#[ktest]
fn zext_ones() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let n = emitter.constant(1, Type::Signed(64));
    let m = emitter.constant(1, Type::Signed(64));
    let res = translate(
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        register_file_ptr,
    )
    .unwrap();
    assert_eq!(res.kind(), &NodeKind::Constant { value: 1, width: 1 });

    let n = emitter.constant(64, Type::Signed(64));
    let m = emitter.constant(0, Type::Signed(64));
    let res = translate(
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        register_file_ptr,
    )
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
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        register_file_ptr,
    )
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
        &*model,
        "zext_ones",
        &[n, m],
        &mut emitter,
        register_file_ptr,
    )
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
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
    //         register_file_ptr,
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
        &*model,
        "DecodeBitMasks",
        &[immn, imms, immr, immediate, m],
        &mut emitter,
        register_file_ptr,
    )
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    {
        let value = emitter.constant(0xaa, Type::Unsigned(8));
        let count = emitter.constant(2, Type::Signed(64));
        assert_eq!(
            &NodeKind::Constant {
                value: 0xaaaa,
                width: 16,
            },
            translate(
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                register_file_ptr,
            )
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
                width: 32,
            },
            translate(
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                register_file_ptr,
            )
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
                width: 64,
            },
            translate(
                &model,
                "replicate_bits_borealis_internal",
                &[value, count],
                &mut emitter,
                register_file_ptr,
            )
            .unwrap()
            .kind()
        );
    }
}

#[ktest]
fn rev_d00dfeed() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let _32 = emitter.constant(32, Type::Signed(64));
    let _3 = emitter.constant(3, Type::Signed(64));
    translate(
        &*model,
        "execute_aarch64_instrs_integer_arithmetic_rev",
        &[_32.clone(), _3.clone(), _32, _3],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r3 = register_file_ptr.add(model.reg_offset("R3") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *r3 = 0xedfe0dd0;
        *see = -1;

        translation.execute(register_file_ptr);
        assert_eq!(0xd00dfeed, *r3);
    }
}

#[ktest]
fn place_slice() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let m = emitter.constant(64, Type::Signed(64));
    let xs = emitter.constant(0xffffffd8, Type::Unsigned(64));
    let i = emitter.constant(0, Type::Signed(64));
    let l = emitter.constant(32, Type::Signed(64));
    let shift = emitter.constant(0, Type::Signed(64));

    let res = translate(
        &*model,
        "place_slice_signed",
        &[m, xs, i, l, shift],
        &mut emitter,
        register_file_ptr,
    )
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

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x9ac10a73, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut u64;
        let r19 = register_file_ptr.add(model.reg_offset("R19") as usize) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        let x = 0xffffff8008bfffffu64;
        let y = 0x200000u64;

        *see = -1;
        *r1 = y;
        *r19 = x;

        translation.execute(register_file_ptr);

        assert_eq!(x / y, (*r19));
    }
}

#[ktest]
fn floor() {
    assert_eq!(0, harness(3, 4));
    assert_eq!(1, harness(5, 4));
    assert_eq!(2, harness(8, 4));

    fn harness(n: i64, d: i64) -> i64 {
        let model = models::get("aarch64").unwrap();

        let mut register_file = init_register_file(&*model);
        let register_file_ptr = register_file.as_mut_ptr();

        let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
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

        unsafe {
            let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut i64;
            let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut i64;

            *r0 = n;
            *r1 = d;

            translation.execute(register_file_ptr);

            *r0
        }
    }
}

#[ktest]
fn ceil() {
    assert_eq!(1, harness(3, 4));
    assert_eq!(2, harness(5, 4));
    assert_eq!(2, harness(8, 4));

    fn harness(n: i64, d: i64) -> i64 {
        let model = models::get("aarch64").unwrap();

        let mut register_file = init_register_file(&*model);
        let register_file_ptr = register_file.as_mut_ptr();

        let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
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

        unsafe {
            let r0 = register_file_ptr.add(model.reg_offset("R0") as usize) as *mut i64;
            let r1 = register_file_ptr.add(model.reg_offset("R1") as usize) as *mut i64;

            *r0 = n;
            *r1 = d;

            translation.execute(register_file_ptr);

            *r0
        }
    }
}

#[ktest]
fn msr() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = init_register_file(&*model);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    //  d51be000        msr     cntfrq_el0, x0
    // todo: instruction takes multiple minutes and GB to translate, fix it
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xd51be000, Type::Unsigned(32));
    translate(
        &*model,
        "__DecodeA64",
        &[pc, opcode],
        &mut emitter,
        register_file_ptr,
    );

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let see = register_file_ptr.add(model.reg_offset("SEE") as usize) as *mut i64;

        *see = -1;

        translation.execute(register_file_ptr);
        // todo: test more here
    }
}
