use {
    crate::dbt::x86::encoder::{
        Operand, OperandKind::Register as R, Register::PhysicalRegister as PHYS, Width,
    },
    iced_x86::code_asm::{AsmRegister8, AsmRegister64, CodeAssembler},
};

pub fn encode(assembler: &mut CodeAssembler, src: &Operand, dst: &Operand) {
    match (src, dst) {
        // TEST R, R
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
                .test::<AsmRegister64, AsmRegister64>(left.into(), right.into())
                .unwrap();
        }

        // TEST R, R
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
                .test::<AsmRegister8, AsmRegister8>(left.into(), right.into())
                .unwrap();
        }
        _ => todo!("test {src} {dst}"),
    }
}
