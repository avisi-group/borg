use {
    crate::{
        dbt::{
            emitter::{Emitter, Type, TypeKind},
            x86::{emitter::CastOperationKind, X86TranslationContext},
            TranslationContext,
        },
        guest::devices::aarch64::{
            borealis_register_init, get_R::get_R, u__DecodeA64::u__DecodeA64, u__InitSystem,
            AMEVTYPER0_EL0_initialize,
        },
    },
    alloc::boxed::Box,
    proc_macro_lib::ktest,
};

pub mod aarch64;
pub mod demoarch;
pub mod virtio;

// #[ktest]
// fn signextend() {
//     let mut ctx = X86TranslationContext::new();

//          // b8_s0: const #64s : i8
//          let b8_s0 = ctx
//          .emitter()
//          .constant(
//              64i64 as u64,
//              Type {
//                  kind: TypeKind::Signed,
//                  width: 8,
//              },
//          );
//      // b8_s1: cast sx b8_s0 -> i64
//      let b8_s1 = ctx
//          .emitter()
//          .cast(
//              b8_s0.clone(),
//              Type {
//                  kind: TypeKind::Signed,
//                  width: 64,
//              },
//              CastOperationKind::SignExtend,
//          );

//     assert_eq(64, b8_s1);
// }

#[ktest]
fn decodea64_smoke() {
    let mut register_file = Box::new([0u8; 104488usize]);
    let register_file_ptr = register_file.as_mut_ptr();
    let mut ctx = X86TranslationContext::new();
    // borealis_register_init(&mut ctx);

    // let unit = ctx.emitter().constant(
    //     0,
    //     Type {
    //         kind: TypeKind::Unsigned,
    //         width: 0,
    //     },
    // );
    // u__InitSystem(&mut ctx, unit);

    let pc = ctx.emitter().constant(
        0,
        Type {
            kind: TypeKind::Unsigned,
            width: 64,
        },
    );

    // add x0,x1,x2
    // (x0 = x1 + x2)
    let opcode = ctx.emitter().constant(
        0x8b020020,
        Type {
            kind: TypeKind::Unsigned,
            width: 64,
        },
    );

    u__DecodeA64(&mut ctx, pc, opcode);

    ctx.emitter().leave();
    let translation = ctx.compile();
    log::debug!("\n{:?}", translation);

    unsafe {
        // SEE
        *(register_file_ptr.offset(0x8130) as *mut u64) = 0x20;

        let r0 = register_file_ptr.offset(0x428) as *mut u32;
        let r1 = register_file_ptr.offset(0x1390) as *mut u32;
        let r2 = register_file_ptr.offset(0x195F8) as *mut u32;

        *r0 = 2;
        *r1 = 5;
        *r2 = 10;

        translation.execute(register_file_ptr);

        log::debug!("{} {} {}", *r0, *r1, *r2);
    }

    panic!();
}
