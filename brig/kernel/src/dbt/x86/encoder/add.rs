use {
    crate::dbt::{
        Alloc,
        x86::encoder::{
            Operand,
            OperandKind::{Immediate as I, Register as R},
            Register::PhysicalRegister as PHYS,
            Width,
        },
    },
    iced_x86::code_asm::{AsmRegister64, CodeAssembler},
};

pub fn encode<A: Alloc>(assembler: &mut CodeAssembler, src: &Operand<A>, dst: &Operand<A>) {
    match (src, dst) {
        // ADD R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .add::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // ADD IMM -> R
        (
            Operand {
                kind: I(src),
                width_in_bits: _,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .add::<AsmRegister64, i32>(dst.into(), i32::try_from(*src as i64).unwrap())
                .unwrap();
        }
        _ => todo!("add {src} {dst}"),
    }
}
