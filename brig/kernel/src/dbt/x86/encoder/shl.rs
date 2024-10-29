use {
    crate::dbt::x86::encoder::{
        Operand,
        OperandKind::{Immediate as I, Register as R},
        Register::PhysicalRegister as PHYS,
        Width,
    },
    iced_x86::code_asm::{AsmRegister32, AsmRegister64, AsmRegister8, CodeAssembler},
};

pub fn encode(assembler: &mut CodeAssembler, amount: &Operand, value: &Operand) {
    match (amount, value) {
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: Width::_8,
            },
        ) => {
            assembler
                .shl::<AsmRegister8, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .shl::<AsmRegister32, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(amount), ..
            },
            Operand {
                kind: R(PHYS(value)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .shl::<AsmRegister64, u32>(value.into(), u32::try_from(*amount).unwrap())
                .unwrap();
        }

        _ => todo!("shl {amount} {value}"),
    }
}
