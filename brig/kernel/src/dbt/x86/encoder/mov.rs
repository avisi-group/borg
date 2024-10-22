use iced_x86::code_asm::{
    byte_ptr, dword_ptr, word_ptr, AsmMemoryOperand, AsmRegister32, AsmRegister64, AsmRegister8,
    CodeAssembler,
};

use crate::dbt::x86::encoder::{
    memory_operand_to_iced, Operand,
    OperandKind::{Immediate as I, Memory as M, Register as R, Target as T},
    Register::PhysicalRegister as PHYS,
};

pub fn encode(assembler: &mut CodeAssembler, src: &Operand, dst: &Operand) {
    match (src, dst) {
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 33..=64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 33..=64,
            },
        ) => {
            //assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 8,
            },
        ) => {
            //assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmRegister8, AsmRegister8>(dst.into(), src.into())
                .unwrap();
        }
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 8,
            },
        ) => {
            //assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmRegister8, AsmRegister8>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 1..=8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 1..=8,
            },
        ) => {
            //assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmRegister8, AsmRegister8>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 1..=8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 33..=64,
            },
        ) => {
            // todo: check high bits of src are zero?
            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // MOV M -> R
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
                width_in_bits: 64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 64,
            },
        ) => {
            assembler
                .mov::<AsmRegister64, AsmMemoryOperand>(
                    dst.into(),
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                )
                .unwrap();
        }
        // MOV M -> R
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
                width_in_bits: 1..=8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 1..=8,
            },
        ) => {
            assembler
                .mov::<AsmRegister8, AsmMemoryOperand>(
                    dst.into(),
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                )
                .unwrap();
        }
        // MOV M -> R
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
                width_in_bits: 17..=32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 17..=32,
            },
        ) => {
            assembler
                .mov::<AsmRegister32, AsmMemoryOperand>(
                    dst.into(),
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                )
                .unwrap();
        }
        // MOV R -> M
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 1..=8,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 1..=8,
            },
        ) => {
            assembler
                .mov::<AsmMemoryOperand, AsmRegister8>(
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                    src.into(),
                )
                .unwrap();
        }
        // MOV R -> M
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 17..=32,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 17..=32,
            },
        ) => {
            assembler
                .mov::<AsmMemoryOperand, AsmRegister32>(
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                    src.into(),
                )
                .unwrap();
        }
        // MOV R -> M
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: 33..=64,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 33..=64,
            },
        ) => {
            assembler
                .mov::<AsmMemoryOperand, AsmRegister64>(
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                    src.into(),
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(src),
                width_in_bits: 32,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 32,
            },
        ) => {
            // assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    *src as u32,
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(src),
                width_in_bits: 1..=8,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 1..=8,
            },
        ) => {
            // assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmMemoryOperand, u32>(
                    byte_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    u32::try_from(*src).unwrap(),
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(src),
                width_in_bits: 9..=16,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 9..=16,
            },
        ) => {
            // assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmMemoryOperand, u32>(
                    word_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    u32::try_from(*src).unwrap(),
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(src),
                width_in_bits: 33..=64,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 33..=64,
            },
        ) => {
            // lo
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    u32::try_from(*src & u64::from(u32::MAX)).unwrap(),
                )
                .unwrap();
            // hi
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(
                        *base,
                        *index,
                        *scale,
                        *displacement + 4,
                    )),
                    u32::try_from((*src >> 32) & u64::from(u32::MAX)).unwrap(),
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(src),
                width_in_bits: 65..=128,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(base)),
                        index,
                        scale,
                        displacement,
                        ..
                    },
                width_in_bits: 65..=128,
            },
        ) => {
            // lolo
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    u32::try_from(*src & u64::from(u32::MAX)).unwrap(),
                )
                .unwrap();
            // lohi
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(
                        *base,
                        *index,
                        *scale,
                        *displacement + 4,
                    )),
                    u32::try_from((*src >> 32) & u64::from(u32::MAX)).unwrap(),
                )
                .unwrap();
            // hilo
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(
                        *base,
                        *index,
                        *scale,
                        *displacement + 8,
                    )),
                    0,
                )
                .unwrap();
            // hihi
            assembler
                .mov::<AsmMemoryOperand, u32>(
                    dword_ptr(memory_operand_to_iced(
                        *base,
                        *index,
                        *scale,
                        *displacement + 12,
                    )),
                    0,
                )
                .unwrap();
        }
        // MOV I -> M
        (
            Operand {
                kind: I(_src),
                width_in_bits: 0,
            },
            Operand {
                kind:
                    M {
                        base: Some(PHYS(_base)),
                        ..
                    },
                width_in_bits: 0,
            },
        ) => {
            todo!("why");
        }

        // MOV I -> R
        (
            Operand {
                kind: I(src),
                width_in_bits: 1..=8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 1..=8,
            },
        ) => {
            assembler
                .mov::<AsmRegister8, i32>(dst.into(), (*src).try_into().unwrap())
                .unwrap();
        }
        // MOV I -> R
        (
            Operand {
                kind: I(src),
                width_in_bits: 33..=64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: 33..=64,
            },
        ) => {
            assembler
                .mov::<AsmRegister64, u64>(dst.into(), *src)
                .unwrap();
        }
        _ => todo!("mov {src} {dst}"),
    }
}
