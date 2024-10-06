use {
    crate::{arch::x86::memory::ExecutableAllocator, dbt::emitter::Emitter},
    alloc::{collections::BTreeMap, string::String, vec::Vec},
    core::fmt::{self, Debug},
    iced_x86::{Formatter, Instruction},
};

pub mod emitter;
pub mod models;
mod trampoline;
pub mod x86;

pub struct TranslationManager {
    translations: BTreeMap<usize, BTreeMap<usize, Translation>>,
}

impl TranslationManager {
    pub fn register_translation(_gpa: usize, _txln: Translation) {
        todo!()
    }

    pub fn lookup_translation(_gpa: usize) -> Option<Translation> {
        todo!()
    }

    pub fn invalidate_all() {
        todo!()
    }

    pub fn invalidate_region(_gpa: usize) {
        todo!()
    }

    pub fn collect_garbage() {
        todo!()
    }
}

pub struct Translation {
    pub code: Vec<u8, ExecutableAllocator>,
}

impl Translation {
    pub fn execute(&self, register_file: *mut u8) {
        let code_ptr = self.code.as_ptr();

        unsafe { trampoline::execute(code_ptr, register_file) };
    }
}

impl Debug for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut decoder = iced_x86::Decoder::with_ip(64, &self.code, 0, 0);

        let mut formatter = iced_x86::GasFormatter::new();

        let mut output = String::new();

        let mut instr = Instruction::default();

        while decoder.can_decode() {
            output.clear();
            decoder.decode_out(&mut instr);
            formatter.format(&instr, &mut output);
            writeln!(f, "{:016x} {output}", instr.ip())?;
        }

        Ok(())
    }
}

pub trait TranslationContext {
    type Emitter: Emitter;

    fn emitter(&mut self) -> &mut Self::Emitter;
    fn create_block(
        &mut self,
        id: u32,
    ) -> <<Self as TranslationContext>::Emitter as Emitter>::BlockRef;
    fn create_symbol(&mut self) -> <<Self as TranslationContext>::Emitter as Emitter>::SymbolRef;
    fn compile(self) -> Translation;

    fn dump(&self);
}
