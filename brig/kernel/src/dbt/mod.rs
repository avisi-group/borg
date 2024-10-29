use {
    crate::arch::x86::memory::ExecutableAllocator,
    alloc::{string::String, vec::Vec},
    common::mask::mask,
    core::fmt::{self, Debug},
    iced_x86::{Formatter, Instruction},
};

pub mod emitter;
pub mod interpret;
pub mod models;
mod tests;
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

fn bit_insert(target: u64, source: u64, start: u64, length: u64) -> u64 {
    // todo: hack
    if start >= 64 {
        if source == 0 {
            return 0;
        } else {
            panic!("attempting to insert {length} bits of {source} into {target} at {start}");
        }
    }

    let length = u32::try_from(length).unwrap();

    let cleared_target = {
        let mask = !(mask(length)
            .checked_shl(u32::try_from(start).unwrap())
            .unwrap_or_else(|| {
                panic!("overflow in shl with {target:b} {source:?} {start:?} {length:?}")
            }));
        target & mask
    };

    let shifted_source = {
        let mask = mask(length);
        let masked_source = source & mask;
        masked_source << start
    };

    cleared_target | shifted_source
}

fn bit_extract(value: u64, start: u64, length: u64) -> u64 {
    (value >> start) & mask(u32::try_from(length).unwrap())
}
