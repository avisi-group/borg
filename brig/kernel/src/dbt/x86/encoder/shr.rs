use {
    crate::dbt::x86::encoder::{
        memory_operand_to_iced, Operand,
        OperandKind::{Immediate as I, Memory as M, Register as R, Target as T},
        PhysicalRegister,
        Register::PhysicalRegister as PHYS,
    },
    iced_x86::code_asm::{
        byte_ptr, dword_ptr, word_ptr, AsmMemoryOperand, AsmRegister32, AsmRegister64,
        AsmRegister8, CodeAssembler,
    },
};

pub fn encode(assembler: &mut CodeAssembler, amount: &Operand, value: &Operand) {
    match (amount, value) {
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: 1..=8,
            },
        ) => {
            assembler
                .shr::<AsmRegister8, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: 17..=32,
            },
        ) => {
            assembler
                .shr::<AsmRegister32, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: 33..=64,
            },
        ) => {
            assembler
                .shr::<AsmRegister64, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }
        _ => todo!("shr {amount} {value}"),
    }
}
