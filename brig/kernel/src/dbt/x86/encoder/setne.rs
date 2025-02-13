use {
    crate::dbt::x86::encoder::{
        Operand, OperandKind::Register as R, Register::PhysicalRegister as PHYS, Width,
    },
    iced_x86::code_asm::{AsmRegister32, AsmRegister8, CodeAssembler},
};

pub fn encode(assembler: &mut CodeAssembler, dst: &Operand) {
    match dst {
        Operand {
            kind: R(PHYS(target)),
            width_in_bits: Width::_64,
        } => {
            assembler
                .xor::<AsmRegister32, AsmRegister32>(target.into(), target.into())
                .unwrap();
            assembler.setne::<AsmRegister8>(target.into()).unwrap();
        }
        Operand {
            kind: R(PHYS(target)),
            width_in_bits: Width::_8,
        } => {
            assembler.setne::<AsmRegister8>(target.into()).unwrap();
        }
        _ => todo!("setne {dst}"),
    }
}
