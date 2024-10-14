use {
    crate::arch::x86::memory::ExecutableAllocator,
    alloc::{string::String, vec::Vec},
    core::fmt::{self, Debug},
    iced_x86::{Formatter, Instruction},
};

pub mod emitter;
pub mod interpret;
pub mod models;
mod trampoline;
pub mod translate;
pub mod x86;

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
