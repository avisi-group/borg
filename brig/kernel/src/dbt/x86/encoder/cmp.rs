use {
    crate::dbt::x86::encoder::{
        Operand,
        OperandKind::{Immediate as I, Register as R},
        Register::PhysicalRegister as PHYS,
        Width,
    },
    iced_x86::code_asm::{AsmRegister8, AsmRegister32, AsmRegister64, CodeAssembler},
};

pub fn encode(assembler: &mut CodeAssembler, left: &Operand, right: &Operand) {
    match (left, right) {
        (
            Operand {
                kind: R(PHYS(left)),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_8,
            },
        ) => {
            assembler
                .cmp::<AsmRegister8, AsmRegister8>(right.into(), left.into())
                .unwrap();
        }
        (
            Operand {
                kind: R(PHYS(left)),
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .cmp::<AsmRegister32, AsmRegister32>(right.into(), left.into())
                .unwrap();
        }
        (
            Operand {
                kind: R(PHYS(left)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .cmp::<AsmRegister64, AsmRegister64>(right.into(), left.into())
                .unwrap();
        }
        (
            Operand {
                kind: I(left),
                width_in_bits: _,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .cmp::<AsmRegister64, i32>(right.into(), (*left).try_into().unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(left),
                width_in_bits: _,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .cmp::<AsmRegister32, i32>(right.into(), (*left).try_into().unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: I(left),
                width_in_bits: _,
            },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_8,
            },
        ) => {
            assembler
                .cmp::<AsmRegister8, i32>(right.into(), (*left).try_into().unwrap())
                .unwrap();
        }

        _ => todo!("cmp {left} {right}"),
    }
}
