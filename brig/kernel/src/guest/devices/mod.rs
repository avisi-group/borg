use proc_macro_lib::ktest;

use crate::dbt::{
    emitter::{Emitter, Type, TypeKind},
    x86::X86TranslationContext,
    TranslationContext,
};

pub mod aarch64;
pub mod demoarch;
pub mod virtio;

#[ktest]
fn decodea64_smoke() {
    let mut ctx = X86TranslationContext::new();

    let pc = ctx.emitter().constant(
        0,
        Type {
            kind: TypeKind::Unsigned,
            width: 64,
        },
    );

    let opcode = ctx.emitter().constant(
        0x8b020020,
        Type {
            kind: TypeKind::Unsigned,
            width: 64,
        },
    );

    aarch64::u__DecodeA64(&mut ctx, pc, opcode);
    ctx.emitter().leave();
    let translation = ctx.compile();
    log::debug!("\n{:?}", translation);
    panic!();
}
