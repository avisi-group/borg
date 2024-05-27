use alloc::collections::BTreeMap;

use self::{
    emitter::{Block, Builder, Context, Type},
    x86::X86LoweringContext,
};

pub mod emitter;
pub mod x86;

pub struct TranslationManager {
    translations: BTreeMap<usize, BTreeMap<usize, Translation>>,
}

pub struct Translation {
    code: *const u8,
    size: usize,
}

impl TranslationManager {
    pub fn register_translation(gpa: usize, txln: Translation) {
        todo!()
    }

    pub fn lookup_translation(gpa: usize) -> Option<Translation> {
        todo!()
    }

    pub fn invalidate_all() {
        todo!()
    }

    pub fn invalidate_region(gpa: usize) {
        todo!()
    }

    pub fn collect_garbage() {
        todo!()
    }
}

pub fn test_translator() -> Translation {
    let mut ctx = Context::new();

    let b0 = ctx.create_block();
    let b1 = ctx.create_block();

    let mut builder = Builder::new(b0);

    // B0
    let c0 = builder.const_u32(0);
    let c1 = builder.const_u32(1);
    let c2 = builder.read_register(c0, Type::u32());
    let c3 = builder.read_register(c1, Type::u32());
    let c4 = builder.add(c2, c3);
    let c5 = builder.const_u32(2);
    builder.write_register(c5, c4);
    builder.jump(Block::downgrade(&b1));

    // B1
    builder.set_insert_point(b1);
    builder.leave();

    ctx.lower(X86LoweringContext::new())
}
