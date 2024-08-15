use {
    crate::dbt::x86::X86LoweringContext,
    alloc::collections::BTreeMap,
    plugins_api::guest::dbt::{
        emitter::{Block, Builder, Context, Type},
        Translation,
    },
};

pub mod x86;

pub struct TranslationManager {
    translations: BTreeMap<usize, BTreeMap<usize, Translation>>,
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
