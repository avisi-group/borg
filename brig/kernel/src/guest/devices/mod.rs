use {
    crate::{
        dbt::{
            emitter::{Emitter, Type},
            x86::X86TranslationContext,
            TranslationContext,
        },
        guest::devices::aarch64::{
            borealis_register_init::borealis_register_init,
            common::{REG_R0, REG_R1, REG_R2, REG_SEE},
            u__DecodeA64::u__DecodeA64,
            u__InitSystem::u__InitSystem,
        },
    },
    alloc::boxed::Box,
    proc_macro_lib::ktest,
};

pub mod aarch64;
pub mod demoarch;
pub mod virtio;

#[ktest]
fn decodea64_smoke() {
    let mut register_file = Box::new([0u8; 104488usize]);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new();
    borealis_register_init(&mut ctx);

    let unit = ctx.emitter().constant(0, Type::Unsigned(0));
    u__InitSystem(&mut ctx, unit);

    let pc = ctx.emitter().constant(0, Type::Unsigned(64));

    // add x0,x1,x2
    // (x0 = x1 + x2)
    let opcode = ctx.emitter().constant(0x8b020020, Type::Unsigned(64));

    u__DecodeA64(&mut ctx, pc, opcode);

    ctx.emitter().leave();
    let translation = ctx.compile();
    log::debug!("\n{:?}", translation);

    unsafe {
        let r0 = register_file_ptr.add(REG_R0) as *mut u32;
        let r1 = register_file_ptr.add(REG_R1) as *mut u32;
        let r2 = register_file_ptr.add(REG_R2) as *mut u32;
        let see = register_file_ptr.add(REG_SEE) as *mut i32;

        *see = -1;
        *r0 = 2;
        *r1 = 5;
        *r2 = 10;

        translation.execute(register_file_ptr);

        assert_eq!(15, (*r0));
        assert_eq!(0xe, (*see));
    }
}
