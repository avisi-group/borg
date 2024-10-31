use {
    crate::dbt::{
        emitter::{Emitter, Type},
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
    common::{mask::mask, rudder::Model, HashMap},
    proc_macro_lib::ktest,
};

#[ktest]
fn init_system() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    init(&*model, register_file_ptr);
}

#[ktest]
fn static_dynamic_chaos_smoke() {
    fn run(mut register_file: [u64; 3]) -> [u64; 3] {
        let register_file_ptr = register_file.as_mut_ptr() as *mut u8;
        let model = models::get("aarch64").unwrap();

        {
            let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
            let mut emitter = X86Emitter::new(&mut ctx);

            translate(&*model, "func_corrupted_var", &[], &mut emitter);

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
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    let feature = emitter.read_register(r0_offset.clone(), Type::Unsigned(0x20));

    translate(&*model, "num_of_Feature", &[feature], &mut emitter);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);
}

#[ktest]
fn statistical_profiling_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let unit = emitter.constant(0, Type::Unsigned(0));
    let is_enabled = translate(
        &*model,
        "StatisticalProfilingEnabled",
        &[unit],
        &mut emitter,
    );

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    emitter.write_register(r0_offset, is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0")) as *mut bool)
        )
    }
}

#[ktest]
fn havebrbext_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let unit = emitter.constant(0, Type::Unsigned(0));
    let is_enabled = translate(&*model, "HaveBRBExt", &[unit], &mut emitter);

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    emitter.write_register(r0_offset, is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0")) as *mut bool)
        )
    }
}

#[ktest]
fn using_aarch32_disabled() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let unit = emitter.constant(0, Type::Unsigned(0));
    let is_enabled = translate(&*model, "UsingAArch32", &[unit], &mut emitter);

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    emitter.write_register(r0_offset, is_enabled);

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            false,
            *(register_file_ptr.add(model.reg_offset("R0")) as *mut bool)
        )
    }
}

#[ktest]
fn branchto() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let target = emitter.constant(0xDEADFEED, Type::Unsigned(64));
    let branch_type = emitter.constant(1, Type::Unsigned(32));
    let branch_conditional = emitter.constant(1, Type::Unsigned(1));
    translate(
        &*model,
        "BranchTo",
        &[target, branch_type, branch_conditional],
        &mut emitter,
    );

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        assert_eq!(
            0x0,
            *(register_file_ptr.add(model.reg_offset("_PC")) as *mut u64)
        );

        *(register_file_ptr.add(model.reg_offset("__BranchTaken")) as *mut bool) = false;
    }

    translation.execute(register_file_ptr);

    unsafe {
        assert_eq!(
            0xDEADFEED,
            *(register_file_ptr.add(model.reg_offset("_PC")) as *mut u64)
        );
        assert_eq!(
            true,
            *(register_file_ptr.add(model.reg_offset("__BranchTaken")) as *mut bool)
        )
    }
}

#[ktest]
fn decodea64_addsub() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x8b020020, Type::Unsigned(32));
    translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u32;
        let r2 = register_file_ptr.add(model.reg_offset("R2")) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;

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

        let mut register_file = alloc::vec![0u8; model.register_file_size()];
        let register_file_ptr = register_file.as_mut_ptr();

        init(&*model, register_file_ptr);

        let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u32;
        let r2 = register_file_ptr.add(model.reg_offset("R2")) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i64;

        *see = -1;
        *r0 = 2;
        *r1 = 5;
        *r2 = 10;

        let pc = crate::dbt::interpret::Value::UnsignedInteger {
            value: 0,
            length: 64,
        };
        let opcode = crate::dbt::interpret::Value::UnsignedInteger {
            value: 0x8b020020,
            length: 32,
        };
        interpret(&*model, "__DecodeA64", &[pc, opcode], register_file_ptr);

        assert_eq!(15, (*r0));
        assert_eq!(0xe, (*see));
    }
}

#[ktest]
fn decodea64_mov() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xaa0103e0, Type::Unsigned(32));
    translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u32;
        let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u32;
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;

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

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let pc = emitter.constant(44, Type::Unsigned(64));
    let opcode = emitter.constant(0x17fffffa, Type::Unsigned(32));
    translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    //  log::trace!("{translation:?}");

    unsafe {
        let pc = register_file_ptr.add(model.reg_offset("_PC")) as *mut u64;
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;

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

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0x540000c0, Type::Unsigned(32));
    translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;
        let pc = register_file_ptr.add(model.reg_offset("_PC")) as *mut u32;
        let branch_taken = register_file_ptr.add(model.reg_offset("__BranchTaken")) as *mut bool;

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

        let mut register_file = alloc::vec![0u8; model.register_file_size()];
        let register_file_ptr = register_file.as_mut_ptr();

        let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
        let mut emitter = X86Emitter::new(&mut ctx);

        init(&*model, register_file_ptr);

        let see_offset = emitter.constant(model.reg_offset("SEE") as u64, Type::Unsigned(64));
        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(see_offset, see_value);

        // cmp     x2, x0
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0xeb00005f, Type::Unsigned(32));
        translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

        let see_offset = emitter.constant(model.reg_offset("SEE") as u64, Type::Unsigned(64));
        let see_value = emitter.constant(-1i32 as u64, Type::Signed(32));
        emitter.write_register(see_offset, see_value);

        // csel    x2, x2, x0, ls  // ls = plast
        let pc = emitter.constant(0, Type::Unsigned(64));
        let opcode = emitter.constant(0x9a809042, Type::Unsigned(32));
        translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

        emitter.leave();

        let num_regs = emitter.next_vreg();
        let translation = ctx.compile(num_regs);

        unsafe {
            let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u64;
            let r2 = register_file_ptr.add(model.reg_offset("R2")) as *mut u64;

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

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    init(&*model, register_file_ptr);

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
        let see = register_file_ptr.add(model.reg_offset("SEE")) as *mut i32;
        let branch_taken =
            { register_file_ptr.add(model.reg_offset("__BranchTaken")) as *mut bool };
        let pc = { register_file_ptr.add(model.reg_offset("_PC")) as *mut u64 };
        let r0 = { register_file_ptr.add(model.reg_offset("R0")) as *mut u64 };
        let r3 = { register_file_ptr.add(model.reg_offset("R3")) as *mut u64 };

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
                translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);
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

#[ktest]
fn fibonacci_block() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();

    init(&*model, register_file_ptr);

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
            let mut current_pc = *(register_file_ptr.add(pc_offset) as *mut u64);
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
                let see_offset =
                    emitter.constant(model.reg_offset("SEE") as u64, Type::Unsigned(64));
                let neg1 = emitter.constant(-1i32 as u64, Type::Signed(32));
                emitter.write_register(see_offset, neg1);

                let branch_taken_offset =
                    emitter.constant(model.reg_offset("__BranchTaken") as u64, Type::Unsigned(64));
                let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                emitter.write_register(branch_taken_offset, _false);

                {
                    let opcode =
                        emitter.constant(program[current_pc as usize / 4], Type::Unsigned(32));
                    let pc = emitter.constant(current_pc, Type::Unsigned(64));
                    let _return_value =
                        translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);
                }

                if emitter.ctx().get_write_pc() || (current_pc == ((program.len() * 4) - 8) as u64)
                {
                    break;
                } else {
                    let pc_offset = emitter.constant(pc_offset as u64, Type::Unsigned(64));
                    let pc = emitter.read_register(pc_offset.clone(), Type::Unsigned(64));
                    let _4 = emitter.constant(4, Type::Unsigned(64));
                    let pc_inc = emitter.binary_operation(BinaryOperationKind::Add(pc, _4));
                    emitter.write_register(pc_offset, pc_inc);

                    current_pc += 4;
                }
            }

            // inc PC if branch not taken
            {
                let branch_taken_offset =
                    emitter.constant(model.reg_offset("__BranchTaken") as u64, Type::Unsigned(64));
                let branch_taken = emitter.read_register(branch_taken_offset, Type::Unsigned(1));

                let _0 = emitter.constant(0, Type::Unsigned(64));
                let _4 = emitter.constant(4, Type::Unsigned(64));
                let addend = emitter.select(branch_taken, _0, _4);

                let pc_offset = emitter.constant(pc_offset as u64, Type::Unsigned(64));
                let pc = emitter.read_register(pc_offset.clone(), Type::Unsigned(64));
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
                *(register_file_ptr.add(model.reg_offset("_PC")) as *mut u64),
                *(register_file_ptr.add(model.reg_offset("__BranchTaken")) as *mut u8)
            );
        }
    }

    unsafe {
        assert_eq!(
            1298777728820984005,
            *(register_file_ptr.add(model.reg_offset("R0")) as *mut u64)
        );
        assert_eq!(
            100,
            *(register_file_ptr.add(model.reg_offset("R3")) as *mut u64)
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

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let r0 = unsafe { register_file_ptr.add(model.reg_offset("R0")) as *mut u64 };
    let r1 = unsafe { register_file_ptr.add(model.reg_offset("R1")) as *mut u64 };
    let r2 = unsafe { register_file_ptr.add(model.reg_offset("R2")) as *mut u8 };

    unsafe {
        *r0 = x;
        *r1 = y;
        *r2 = carry_in as u8;
    }

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    let r1_offset = emitter.constant(model.reg_offset("R1") as u64, Type::Unsigned(0x40));
    let r2_offset = emitter.constant(model.reg_offset("R2") as u64, Type::Unsigned(0x8));

    let x = emitter.read_register(r0_offset.clone(), Type::Unsigned(0x40));
    let y = emitter.read_register(r1_offset.clone(), Type::Unsigned(0x40));
    let carry_in = emitter.read_register(r2_offset.clone(), Type::Unsigned(0x8));

    let res = translate(
        &*model,
        "add_with_carry_test",
        &[x, y, carry_in],
        &mut emitter,
    );

    let sum = emitter.access_tuple(res.clone(), 0);
    emitter.write_register(r0_offset, sum);

    let flags = emitter.access_tuple(res.clone(), 1);
    emitter.write_register(r1_offset, flags);

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

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    unsafe {
        *(register_file_ptr.add(model.reg_offset("R0")) as *mut u64) = x;
        *(register_file_ptr.add(model.reg_offset("R1")) as *mut u64) = y;
        *(register_file_ptr.add(model.reg_offset("SEE")) as *mut i32) = -1;
    }

    // cmp    x0, x1
    let pc = emitter.constant(0, Type::Unsigned(64));
    let opcode = emitter.constant(0xeb01001f, Type::Unsigned(32));
    translate(&*model, "__DecodeA64", &[pc, opcode], &mut emitter);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    translation.execute(register_file_ptr);

    unsafe {
        *(register_file_ptr.add(model.reg_offset("PSTATE_N")) as *mut u8) << 3
            | *(register_file_ptr.add(model.reg_offset("PSTATE_Z")) as *mut u8) << 2
            | *(register_file_ptr.add(model.reg_offset("PSTATE_C")) as *mut u8) << 1
            | *(register_file_ptr.add(model.reg_offset("PSTATE_V")) as *mut u8)
    }
}

#[ktest]
fn shiftreg() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let _1 = emitter.constant(1, Type::Signed(64));
    let shift_type = emitter.constant(1, Type::Signed(32));
    let amount = emitter.constant(0, Type::Signed(64));
    let width = emitter.constant(64, Type::Signed(64));
    let value = translate(
        &*model,
        "ShiftReg",
        &[_1, shift_type, amount, width],
        &mut emitter,
    );

    let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
    emitter.write_register(r0_offset, value);

    emitter.leave();

    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u64;

        *r0 = 0;
        *r1 = 0xdeadfeeddeadfeed;

        translation.execute(register_file_ptr);

        assert_eq!(*r0, 0xdeadfeeddeadfeed);
        assert_eq!(*r1, 0xdeadfeeddeadfeed);
    }
}

//#[ktest]
fn ispow2() {
    let model = models::get("aarch64").unwrap();

    let mut register_file = alloc::vec![0u8; model.register_file_size()];
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new(model.reg_offset("_PC"));
    let mut emitter = X86Emitter::new(&mut ctx);

    init(&*model, register_file_ptr);

    let r3_offset = emitter.constant(model.reg_offset("R3") as u64, Type::Unsigned(0x40));
    let x = emitter.read_register(r3_offset, Type::Unsigned(0x40));

    {
        let value = translate(&*model, "FloorPow2", &[x.clone()], &mut emitter);
        let r0_offset = emitter.constant(model.reg_offset("R0") as u64, Type::Unsigned(0x40));
        emitter.write_register(r0_offset, value);
    }

    // {
    //     let value = translate(&*model, "CeilPow2", &[x.clone()], &mut emitter);
    //     let r1_offset = emitter.constant(model.reg_offset("R1") as u64,
    // Type::Unsigned(0x40));     emitter.write_register(r1_offset, value);
    // }

    // {
    //     let value = translate(&*model, "IsPow2", &[x], &mut emitter);
    //     let r2_offset = emitter.constant(model.reg_offset("R2") as u64,
    // Type::Unsigned(0x40));     emitter.write_register(r2_offset, value);
    // }

    emitter.leave();
    let num_regs = emitter.next_vreg();
    let translation = ctx.compile(num_regs);
    log::debug!("{translation:?}");

    unsafe {
        let r0 = register_file_ptr.add(model.reg_offset("R0")) as *mut u64;
        let r1 = register_file_ptr.add(model.reg_offset("R1")) as *mut u64;
        let r2 = register_file_ptr.add(model.reg_offset("R2")) as *mut u64;
        let r3 = register_file_ptr.add(model.reg_offset("R3")) as *mut u64;

        *r0 = 0;
        *r1 = 0;
        *r2 = 0;
        *r3 = 2048;

        translation.execute(register_file_ptr);

        assert_eq!(*r0, *r1);
        assert_eq!(1, *r2)
    }
}

//////#[ktest]
// fn rbitx0() {
//     let mut state = State::new(Box::new(NoneEnv));
//     state.write_register::<u64>(REG_R0, 0x0000000000000001);

//     // rbit x0
//     u__DecodeA64(&mut state, TRACER, 0x0, 0xdac00000);

//     // assert bits are reversed
//     assert_eq!(state.read_register::<u64>(REG_R0), 0x8000000000000000);
// }

//////#[ktest]
// fn ubfx() {
//     {
//         let mut state = State::new(Box::new(NoneEnv));
//         // decode bit masks
//         assert_eq!(
//             ProductTypea79c7f841a890648 {
//                 tuple__pcnt_bv__pcnt_bv0: Bits::new(0xFFFF00000000000F, 64),
//                 tuple__pcnt_bv__pcnt_bv1: Bits::new(0xF, 64)
//             },
//             DecodeBitMasks(&mut state, TRACER, true, 0x13, 0x10, false, 0x40)
//         );
//     }

//     {
//         let mut state = State::new(Box::new(NoneEnv));
//         state.write_register::<u64>(REG_R3, 0x8444_c004);

//         // ubfx x3, x3, #16, #4
//         u__DecodeA64(&mut state, TRACER, 0, 0xd3504c63);
//         assert_eq!(0x4, state.read_register::<u64>(REG_R3));
//     }
// }

// //////////#[ktest]
// // // fn replicate_bits() {
// // //     let mut register_file = Box::new([0u8;
// model.register_file_size()]); // //     let register_file_ptr =
// register_file.as_mut_ptr(); // //     let mut ctx =
// X86TranslationContext::new(model.reg_offset("_PC")); // //     let model =
// models::get("aarch64").unwrap();

// // // translate(&*model, "borealis_register_init", &[], &mut ctx);

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

// //////////#[ktest]
// // // fn rev_d00dfeed() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     state.write_register::<u64>(REG_R3, 0xedfe0dd0);
// // // translate_aarch64_instrs_integer_arithmetic_rev(&mut state, TRACER,
// 32, // 3, // 32, 3);     assert_eq!(0xd00dfeed,
// state.read_register::<u64>(REG_R3)); // // }

// //////////#[ktest]
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

// //////////#[ktest]
// // // fn place_slice() {
// // //     let mut state = State::new(Box::new(NoneEnv));
// // //     assert_eq!(
// // //         Bits::new(0xffffffffffffffd8, 64),
// // //         place_slice_signed(&mut state, TRACER, 64,
// Bits::new(0xffffffd8, // 64), // 0, 32, 0,)     );
// // // }

fn init(model: &Model, register_file: *mut u8) {
    interpret(&*model, "borealis_register_init", &[], register_file);
    configure_features(&*model, register_file);
    interpret(&*model, "__InitSystem", &[Value::Unit], register_file);
}

fn configure_features(model: &Model, register_file: *mut u8) {
    let features = [
        "FEAT_AA32EL0_IMPLEMENTED",
        "FEAT_AA32EL1_IMPLEMENTED",
        "FEAT_AA32EL2_IMPLEMENTED",
        "FEAT_AA32EL3_IMPLEMENTED",
        "FEAT_AA64EL0_IMPLEMENTED",
        "FEAT_AA64EL1_IMPLEMENTED",
        "FEAT_AA64EL2_IMPLEMENTED",
        "FEAT_AA64EL3_IMPLEMENTED",
        "FEAT_EL0_IMPLEMENTED",
        "FEAT_EL1_IMPLEMENTED",
        "FEAT_EL2_IMPLEMENTED",
        "FEAT_EL3_IMPLEMENTED",
        "FEAT_AES_IMPLEMENTED",
        "FEAT_AdvSIMD_IMPLEMENTED",
        "FEAT_CSV2_1p1_IMPLEMENTED",
        "FEAT_CSV2_1p2_IMPLEMENTED",
        "FEAT_CSV2_2_IMPLEMENTED",
        "FEAT_CSV2_3_IMPLEMENTED",
        "FEAT_DoubleLock_IMPLEMENTED",
        "FEAT_ETMv4_IMPLEMENTED",
        "FEAT_ETMv4p1_IMPLEMENTED",
        "FEAT_ETMv4p2_IMPLEMENTED",
        "FEAT_ETMv4p3_IMPLEMENTED",
        "FEAT_ETMv4p4_IMPLEMENTED",
        "FEAT_ETMv4p5_IMPLEMENTED",
        "FEAT_ETMv4p6_IMPLEMENTED",
        "FEAT_ETS2_IMPLEMENTED",
        "FEAT_FP_IMPLEMENTED",
        "FEAT_GICv3_IMPLEMENTED",
        "FEAT_GICv3_LEGACY_IMPLEMENTED",
        "FEAT_GICv3_TDIR_IMPLEMENTED",
        "FEAT_GICv3p1_IMPLEMENTED",
        "FEAT_GICv4_IMPLEMENTED",
        "FEAT_GICv4p1_IMPLEMENTED",
        "FEAT_IVIPT_IMPLEMENTED",
        "FEAT_PCSRv8_IMPLEMENTED",
        "FEAT_PMULL_IMPLEMENTED",
        "FEAT_PMUv3_IMPLEMENTED",
        "FEAT_PMUv3_EXT_IMPLEMENTED",
        "FEAT_PMUv3_EXT32_IMPLEMENTED",
        "FEAT_SHA1_IMPLEMENTED",
        "FEAT_SHA256_IMPLEMENTED",
        "FEAT_TRC_EXT_IMPLEMENTED",
        "FEAT_TRC_SR_IMPLEMENTED",
        "FEAT_nTLBPA_IMPLEMENTED",
        "FEAT_CRC32_IMPLEMENTED",
        "FEAT_Debugv8p1_IMPLEMENTED",
        "FEAT_HAFDBS_IMPLEMENTED",
        "FEAT_HPDS_IMPLEMENTED",
        "FEAT_LOR_IMPLEMENTED",
        "FEAT_LSE_IMPLEMENTED",
        "FEAT_PAN_IMPLEMENTED",
        "FEAT_PMUv3p1_IMPLEMENTED",
        "FEAT_RDM_IMPLEMENTED",
        "FEAT_VHE_IMPLEMENTED",
        "FEAT_VMID16_IMPLEMENTED",
        "FEAT_AA32BF16_IMPLEMENTED",
        "FEAT_AA32HPD_IMPLEMENTED",
        "FEAT_AA32I8MM_IMPLEMENTED",
        "FEAT_ASMv8p2_IMPLEMENTED",
        "FEAT_DPB_IMPLEMENTED",
        "FEAT_Debugv8p2_IMPLEMENTED",
        "FEAT_EDHSR_IMPLEMENTED",
        "FEAT_F32MM_IMPLEMENTED",
        "FEAT_F64MM_IMPLEMENTED",
        "FEAT_FP16_IMPLEMENTED",
        "FEAT_HPDS2_IMPLEMENTED",
        "FEAT_I8MM_IMPLEMENTED",
        "FEAT_IESB_IMPLEMENTED",
        "FEAT_LPA_IMPLEMENTED",
        "FEAT_LSMAOC_IMPLEMENTED",
        "FEAT_LVA_IMPLEMENTED",
        "FEAT_MPAM_IMPLEMENTED",
        "FEAT_PAN2_IMPLEMENTED",
        "FEAT_PCSRv8p2_IMPLEMENTED",
        "FEAT_RAS_IMPLEMENTED",
        "FEAT_SHA3_IMPLEMENTED",
        "FEAT_SHA512_IMPLEMENTED",
        "FEAT_SM3_IMPLEMENTED",
        "FEAT_SM4_IMPLEMENTED",
        "FEAT_SPE_IMPLEMENTED",
        "FEAT_SVE_IMPLEMENTED",
        "FEAT_TTCNP_IMPLEMENTED",
        "FEAT_UAO_IMPLEMENTED",
        "FEAT_VPIPT_IMPLEMENTED",
        "FEAT_XNX_IMPLEMENTED",
        "FEAT_CCIDX_IMPLEMENTED",
        "FEAT_CONSTPACFIELD_IMPLEMENTED",
        "FEAT_EPAC_IMPLEMENTED",
        "FEAT_FCMA_IMPLEMENTED",
        "FEAT_FPAC_IMPLEMENTED",
        "FEAT_FPACCOMBINE_IMPLEMENTED",
        "FEAT_JSCVT_IMPLEMENTED",
        "FEAT_LRCPC_IMPLEMENTED",
        "FEAT_NV_IMPLEMENTED",
        "FEAT_PACIMP_IMPLEMENTED",
        "FEAT_PACQARMA3_IMPLEMENTED",
        "FEAT_PACQARMA5_IMPLEMENTED",
        "FEAT_PAuth_IMPLEMENTED",
        "FEAT_SPEv1p1_IMPLEMENTED",
        "FEAT_AMUv1_IMPLEMENTED",
        "FEAT_BBM_IMPLEMENTED",
        "FEAT_CNTSC_IMPLEMENTED",
        "FEAT_DIT_IMPLEMENTED",
        "FEAT_Debugv8p4_IMPLEMENTED",
        "FEAT_DotProd_IMPLEMENTED",
        "FEAT_DoubleFault_IMPLEMENTED",
        "FEAT_FHM_IMPLEMENTED",
        "FEAT_FlagM_IMPLEMENTED",
        "FEAT_IDST_IMPLEMENTED",
        "FEAT_LRCPC2_IMPLEMENTED",
        "FEAT_LSE2_IMPLEMENTED",
        "FEAT_NV2_IMPLEMENTED",
        "FEAT_PMUv3p4_IMPLEMENTED",
        "FEAT_RASSAv1p1_IMPLEMENTED",
        "FEAT_RASv1p1_IMPLEMENTED",
        "FEAT_S2FWB_IMPLEMENTED",
        "FEAT_SEL2_IMPLEMENTED",
        "FEAT_TLBIOS_IMPLEMENTED",
        "FEAT_TLBIRANGE_IMPLEMENTED",
        "FEAT_TRF_IMPLEMENTED",
        "FEAT_TTL_IMPLEMENTED",
        "FEAT_TTST_IMPLEMENTED",
        "FEAT_BTI_IMPLEMENTED",
        "FEAT_CSV2_IMPLEMENTED",
        "FEAT_CSV3_IMPLEMENTED",
        "FEAT_DPB2_IMPLEMENTED",
        "FEAT_E0PD_IMPLEMENTED",
        "FEAT_EVT_IMPLEMENTED",
        "FEAT_ExS_IMPLEMENTED",
        "FEAT_FRINTTS_IMPLEMENTED",
        "FEAT_FlagM2_IMPLEMENTED",
        "FEAT_GTG_IMPLEMENTED",
        "FEAT_MTE_IMPLEMENTED",
        "FEAT_MTE2_IMPLEMENTED",
        "FEAT_PMUv3p5_IMPLEMENTED",
        "FEAT_RNG_IMPLEMENTED",
        "FEAT_RNG_TRAP_IMPLEMENTED",
        "FEAT_SB_IMPLEMENTED",
        "FEAT_SPECRES_IMPLEMENTED",
        "FEAT_SSBS_IMPLEMENTED",
        "FEAT_SSBS2_IMPLEMENTED",
        "FEAT_AMUv1p1_IMPLEMENTED",
        "FEAT_BF16_IMPLEMENTED",
        "FEAT_DGH_IMPLEMENTED",
        "FEAT_ECV_IMPLEMENTED",
        "FEAT_FGT_IMPLEMENTED",
        "FEAT_HPMN0_IMPLEMENTED",
        "FEAT_MPAMv0p1_IMPLEMENTED",
        "FEAT_MPAMv1p1_IMPLEMENTED",
        "FEAT_MTPMU_IMPLEMENTED",
        "FEAT_PAuth2_IMPLEMENTED",
        "FEAT_TWED_IMPLEMENTED",
        "FEAT_AFP_IMPLEMENTED",
        "FEAT_EBF16_IMPLEMENTED",
        "FEAT_HCX_IMPLEMENTED",
        "FEAT_LPA2_IMPLEMENTED",
        "FEAT_LS64_IMPLEMENTED",
        "FEAT_LS64_ACCDATA_IMPLEMENTED",
        "FEAT_LS64_V_IMPLEMENTED",
        "FEAT_MTE3_IMPLEMENTED",
        "FEAT_PAN3_IMPLEMENTED",
        "FEAT_PMUv3p7_IMPLEMENTED",
        "FEAT_RPRES_IMPLEMENTED",
        "FEAT_SPEv1p2_IMPLEMENTED",
        "FEAT_WFxT_IMPLEMENTED",
        "FEAT_XS_IMPLEMENTED",
        "FEAT_CMOW_IMPLEMENTED",
        "FEAT_Debugv8p8_IMPLEMENTED",
        "FEAT_GICv3_NMI_IMPLEMENTED",
        "FEAT_HBC_IMPLEMENTED",
        "FEAT_MOPS_IMPLEMENTED",
        "FEAT_NMI_IMPLEMENTED",
        "FEAT_PMUv3_EXT64_IMPLEMENTED",
        "FEAT_PMUv3_TH_IMPLEMENTED",
        "FEAT_PMUv3p8_IMPLEMENTED",
        "FEAT_SCTLR2_IMPLEMENTED",
        "FEAT_SPEv1p3_IMPLEMENTED",
        "FEAT_TCR2_IMPLEMENTED",
        "FEAT_TIDCP1_IMPLEMENTED",
        "FEAT_ADERR_IMPLEMENTED",
        "FEAT_AIE_IMPLEMENTED",
        "FEAT_ANERR_IMPLEMENTED",
        "FEAT_CLRBHB_IMPLEMENTED",
        "FEAT_CSSC_IMPLEMENTED",
        "FEAT_Debugv8p9_IMPLEMENTED",
        "FEAT_DoubleFault2_IMPLEMENTED",
        "FEAT_ECBHB_IMPLEMENTED",
        "FEAT_FGT2_IMPLEMENTED",
        "FEAT_HAFT_IMPLEMENTED",
        "FEAT_LRCPC3_IMPLEMENTED",
        "FEAT_MTE4_IMPLEMENTED",
        "FEAT_MTE_ASYM_FAULT_IMPLEMENTED",
        "FEAT_MTE_ASYNC_IMPLEMENTED",
        "FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED",
        "FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED",
        "FEAT_MTE_PERM_IMPLEMENTED",
        "FEAT_MTE_STORE_ONLY_IMPLEMENTED",
        "FEAT_MTE_TAGGED_FAR_IMPLEMENTED",
        "FEAT_PCSRv8p9_IMPLEMENTED",
        "FEAT_PFAR_IMPLEMENTED",
        "FEAT_PMUv3_EDGE_IMPLEMENTED",
        "FEAT_PMUv3_ICNTR_IMPLEMENTED",
        "FEAT_PMUv3_SS_IMPLEMENTED",
        "FEAT_PMUv3p9_IMPLEMENTED",
        "FEAT_PRFMSLC_IMPLEMENTED",
        "FEAT_RASSAv2_IMPLEMENTED",
        "FEAT_RASv2_IMPLEMENTED",
        "FEAT_RPRFM_IMPLEMENTED",
        "FEAT_S1PIE_IMPLEMENTED",
        "FEAT_S1POE_IMPLEMENTED",
        "FEAT_S2PIE_IMPLEMENTED",
        "FEAT_S2POE_IMPLEMENTED",
        "FEAT_SPECRES2_IMPLEMENTED",
        "FEAT_SPE_CRR_IMPLEMENTED",
        "FEAT_SPE_FDS_IMPLEMENTED",
        "FEAT_SPEv1p4_IMPLEMENTED",
        "FEAT_SPMU_IMPLEMENTED",
        "FEAT_THE_IMPLEMENTED",
        "FEAT_DoPD_IMPLEMENTED",
        "FEAT_ETE_IMPLEMENTED",
        "FEAT_SVE2_IMPLEMENTED",
        "FEAT_SVE_AES_IMPLEMENTED",
        "FEAT_SVE_BitPerm_IMPLEMENTED",
        "FEAT_SVE_PMULL128_IMPLEMENTED",
        "FEAT_SVE_SHA3_IMPLEMENTED",
        "FEAT_SVE_SM4_IMPLEMENTED",
        "FEAT_TME_IMPLEMENTED",
        "FEAT_TRBE_IMPLEMENTED",
        "FEAT_ETEv1p1_IMPLEMENTED",
        "FEAT_BRBE_IMPLEMENTED",
        "FEAT_ETEv1p2_IMPLEMENTED",
        "FEAT_RME_IMPLEMENTED",
        "FEAT_SME_IMPLEMENTED",
        "FEAT_SME_F64F64_IMPLEMENTED",
        "FEAT_SME_FA64_IMPLEMENTED",
        "FEAT_SME_I16I64_IMPLEMENTED",
        "FEAT_BRBEv1p1_IMPLEMENTED",
        "FEAT_MEC_IMPLEMENTED",
        "FEAT_SME2_IMPLEMENTED",
        "FEAT_ABLE_IMPLEMENTED",
        "FEAT_CHK_IMPLEMENTED",
        "FEAT_D128_IMPLEMENTED",
        "FEAT_EBEP_IMPLEMENTED",
        "FEAT_ETEv1p3_IMPLEMENTED",
        "FEAT_GCS_IMPLEMENTED",
        "FEAT_ITE_IMPLEMENTED",
        "FEAT_LSE128_IMPLEMENTED",
        "FEAT_LVA3_IMPLEMENTED",
        "FEAT_SEBEP_IMPLEMENTED",
        "FEAT_SME2p1_IMPLEMENTED",
        "FEAT_SME_F16F16_IMPLEMENTED",
        "FEAT_SVE2p1_IMPLEMENTED",
        "FEAT_SVE_B16B16_IMPLEMENTED",
        "FEAT_SYSINSTR128_IMPLEMENTED",
        "FEAT_SYSREG128_IMPLEMENTED",
        "FEAT_TRBE_EXT_IMPLEMENTED",
        "FEAT_TRBE_MPAM_IMPLEMENTED",
        "v8Ap0_IMPLEMENTED",
        "v8Ap1_IMPLEMENTED",
        "v8Ap2_IMPLEMENTED",
        "v8Ap3_IMPLEMENTED",
        "v8Ap4_IMPLEMENTED",
        "v8Ap5_IMPLEMENTED",
        "v8Ap6_IMPLEMENTED",
        "v8Ap7_IMPLEMENTED",
        "v8Ap8_IMPLEMENTED",
        "v8Ap9_IMPLEMENTED",
        "v9Ap0_IMPLEMENTED",
        "v9Ap1_IMPLEMENTED",
        "v9Ap2_IMPLEMENTED",
        "v9Ap3_IMPLEMENTED",
        "v9Ap4_IMPLEMENTED",
    ];

    let enabled = [
        "FEAT_AA64EL0_IMPLEMENTED",
        "FEAT_AA64EL1_IMPLEMENTED",
        "FEAT_AA64EL2_IMPLEMENTED",
        "FEAT_AA64EL3_IMPLEMENTED",
        "FEAT_D128_IMPLEMENTED",
    ];

    features
        .iter()
        .map(|name| (name, enabled.contains(name)))
        .for_each(|(name, value)| {
            let offset = model.reg_offset(name);
            unsafe { register_file.add(offset).write(value as u8) };
        });
}
