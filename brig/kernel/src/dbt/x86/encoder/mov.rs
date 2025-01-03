use {
    crate::dbt::x86::encoder::{
        memory_operand_to_iced, Operand,
        OperandKind::{Immediate as I, Memory as M, Register as R},
        Register::PhysicalRegister as PHYS,
        Width,
    },
    iced_x86::code_asm::{
        byte_ptr, dword_ptr, word_ptr, AsmMemoryOperand, AsmRegister16, AsmRegister32,
        AsmRegister64, AsmRegister8, CodeAssembler,
    },
};

pub fn encode(assembler: &mut CodeAssembler, src: &Operand, dst: &Operand) {
    match (src, dst) {
        // MOV R -> R
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
            //assert_eq!(src_width_in_bits, dst_width_in_bits);

            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_16,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            // todo: check high bits of src are clear?
            // this should be a movzx
            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
            },
        ) => {
            //assert_eq!(src_width_in_bits, dst_width_in_bits);
            assembler
                .mov::<AsmRegister32, AsmRegister32>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            // todo: check high bits of src are zero?
            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_16,
            },
        ) => {
            // todo: check high bits of src are zero?
            assembler
                .mov::<AsmRegister16, AsmRegister16>(dst.into(), src.into())
                .unwrap();
        }
        // MOV R -> R
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_16,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_16,
            },
        ) => {
            // todo: check high bits of src are zero?
            assembler
                .mov::<AsmRegister16, AsmRegister16>(dst.into(), src.into())
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
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
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
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_16,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_16,
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
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
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
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_16,
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
                width_in_bits: Width::_16,
            },
        ) => {
            assembler
                .mov::<AsmMemoryOperand, AsmRegister16>(
                    memory_operand_to_iced(*base, *index, *scale, *displacement),
                    src.into(),
                )
                .unwrap();
        }
        // MOV R -> M
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_32,
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
                width_in_bits: Width::_32,
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
                width_in_bits: Width::_64,
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
                width_in_bits: Width::_64,
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
                width_in_bits: Width::_32,
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
                width_in_bits: Width::_32,
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
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_16,
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
                width_in_bits: Width::_16,
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
                width_in_bits: Width::_64,
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
                width_in_bits: Width::_64,
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
        // MOV I -> R
        (
            Operand {
                kind: I(src),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
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
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            assembler
                .mov::<AsmRegister64, u64>(dst.into(), *src)
                .unwrap();
        }
        // MOV I -> R
        (
            Operand {
                kind: I(src),
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .mov::<AsmRegister32, u32>(dst.into(), u32::try_from(*src).unwrap())
                .unwrap();
        }
        (
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .mov::<AsmRegister32, AsmRegister32>(dst.into(), src.into())
                .unwrap();
        }
        (
            // todo: move me to movzx
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
            },
        ) => {
            assembler
                .movzx::<AsmRegister32, AsmRegister8>(dst.into(), src.into())
                .unwrap();
        }
        (
            // todo: fix this earlier in DBT
            Operand {
                kind: I(src),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_32,
            },
        ) => {
            // todo: maybe zero extend src here?
            assembler
                .mov::<AsmRegister32, i32>(dst.into(), (*src).try_into().unwrap())
                .unwrap();
        }
        (
            // todo: fix this earlier in DBT
            Operand {
                kind: I(src),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            // no need to write high bits
            assembler
                .mov::<AsmRegister32, i32>(dst.into(), (*src).try_into().unwrap())
                .unwrap();
        }
        (
            // todo: fix this earlier in DBT
            Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            // todo: maybe zero extend src here?
            assembler
                .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap();
        }
        (
            // todo: fix this earlier in DBT
            Operand {
                kind: I(src),
                width_in_bits: Width::_32,
            },
            Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            },
        ) => {
            // don't need to write high bits
            assembler
                .mov::<AsmRegister32, i32>(dst.into(), (*src).try_into().unwrap())
                .unwrap();
        }
        _ => todo!("mov {src} {dst}"),
    }
}
