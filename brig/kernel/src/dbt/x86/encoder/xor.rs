use {
    crate::dbt::x86::encoder::{
        Operand, OperandKind::Register as R, Register::PhysicalRegister as PHYS, Width,
    },
    iced_x86::code_asm::{AsmRegister64, AsmRegister8, CodeAssembler},
};

pub fn encode(assembler: &mut CodeAssembler, src: &Operand, dst: &Operand) {
    match (src, dst) {
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
                .xor::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
            },
        ) => {
            assembler
                .xor::<AsmRegister8, AsmRegister8>(dst.into(), src.into())
                .unwrap();
        }
        _ => todo!("xor {src} {dst}"),
    }
}
