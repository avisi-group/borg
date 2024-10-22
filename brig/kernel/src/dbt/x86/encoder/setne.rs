use iced_x86::code_asm::{
    byte_ptr, dword_ptr, word_ptr, AsmMemoryOperand, AsmRegister32, AsmRegister64, AsmRegister8,
    CodeAssembler,
};

use crate::dbt::x86::encoder::{
    memory_operand_to_iced, Operand,
    OperandKind::{Immediate as I, Memory as M, Register as R, Target as T},
    Register::PhysicalRegister as PHYS,
};

pub fn encode(assembler: &mut CodeAssembler, dst: &Operand) {
    match dst {
        Operand {
            kind: R(PHYS(target)),
            width_in_bits: 64,
        } => {
            assembler
                .xor::<AsmRegister64, AsmRegister64>(target.into(), target.into())
                .unwrap();
            assembler.setne::<AsmRegister8>(target.into()).unwrap();
        }
        Operand {
            kind: R(PHYS(target)),
            width_in_bits: 1..=8,
        } => {
            assembler.setne::<AsmRegister8>(target.into()).unwrap();
        }
        _ => todo!("setne {dst}"),
    }
}
