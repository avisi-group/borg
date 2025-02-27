use {
    crate::dbt::x86::encoder::{
        Operand,
        OperandKind::{Memory as M, Register as R},
        Register::PhysicalRegister as PHYS,
        Width, memory_operand_to_iced,
    },
    iced_x86::code_asm::{AsmMemoryOperand, AsmRegister64, CodeAssembler, qword_ptr},
};

pub fn encode(assembler: &mut CodeAssembler, src: &Operand, dst: &Operand) {
    match (src, dst) {
        (
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .lea::<AsmRegister64, AsmMemoryOperand>(
                    dst.into(),
                    qword_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                )
                .unwrap();
        }
        _ => todo!("lea {src} {dst}"),
    }
}
