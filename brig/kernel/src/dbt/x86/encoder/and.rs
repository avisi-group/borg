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
    iced_x86::code_asm::{
        AsmRegister8, AsmRegister16, AsmRegister32, AsmRegister64, CodeAssembler,
    },
};

pub fn encode<A: Alloc>(assembler: &mut CodeAssembler, src: &Operand<A>, dst: &Operand<A>) {
    match (src, dst) {
        (
            Operand { kind: I(left), .. },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_8,
            },
        ) => {
            if *left > i32::MAX as u64 {
                panic!("AND immediate too large: {left:x}");
            }
            assembler
                .and::<AsmRegister8, i32>(right.into(), *left as i32)
                .unwrap();
        }
        (
            Operand { kind: I(left), .. },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_16,
            },
        ) => {
            if *left > i32::MAX as u64 {
                panic!("AND immediate too large: {left:x}");
            }
            assembler
                .and::<AsmRegister16, i32>(right.into(), *left as i32)
                .unwrap();
        }
        (
            Operand { kind: I(left), .. },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_32,
            },
        ) => {
            if *left > i32::MAX as u64 {
                panic!("AND immediate too large: {left:x}");
            }
            assembler
                .and::<AsmRegister32, i32>(right.into(), *left as i32)
                .unwrap();
        }
        (
            Operand { kind: I(left), .. },
            Operand {
                kind: R(PHYS(right)),
                width_in_bits: Width::_64,
            },
        ) => {
            if *left > i32::MAX as u64 {
                panic!("AND immediate too large: {left:x}");
            }
            assembler
                .and::<AsmRegister64, i32>(right.into(), *left as i32)
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
                .and::<AsmRegister64, AsmRegister64>(right.into(), left.into())
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
                .and::<AsmRegister32, AsmRegister32>(right.into(), left.into())
                .unwrap();
        }
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
                .and::<AsmRegister8, AsmRegister8>(right.into(), left.into())
                .unwrap();
        }

        _ => todo!("and {src} {dst}"),
    }
}
