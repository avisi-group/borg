use {
    crate::dbt::x86::{
        encoder::{
            Instruction, MemoryScale, Opcode, Operand, OperandDirection, OperandKind,
            PhysicalRegister, Register,
        },
        register_allocator::solid_state::SolidStateRegisterAllocator,
    },
    proc_macro_lib::ktest,
};

//pub mod reverse_scan;
pub mod solid_state;

pub trait RegisterAllocator {
    fn process(&mut self, instruction: &mut Instruction);
}

#[ktest]
fn simple_allocation_regression() {
    let mut instrs = [Instruction(Opcode::MOV(
        Operand::mem_base_displ(
            64,
            Register::PhysicalRegister(PhysicalRegister::RBP),
            0x1234,
        ),
        Operand::vreg(64, 1),
    ))];

    let mut instrs = [Instruction(Opcode::MOV(
        Operand::vreg(64, 1),
        Operand::mem_base_displ(
            64,
            Register::PhysicalRegister(PhysicalRegister::RBP),
            0x4321,
        ),
    ))];

    /*Instruction {
            opcode: Opcode::MOV,
            operands: alloc::vec![
                (
                    OperandDirection::In,
                    Operand {
                        kind: OperandKind::Memory {
                            base: Some(Register::PhysicalRegister(PhysicalRegister::RBP)),
                            index: None,
                            scale: MemoryScale::S1,
                            displacement: 0x1234,
                            segment_override: None,
                        },
                        width_in_bits: 0x40,
                    },
                ),
                (
                    OperandDirection::Out,
                    Operand {
                        kind: OperandKind::Register(Register::VirtualRegister(0x1)),
                        width_in_bits: 0x40,
                    },
                ),
            ],
        },
        Instruction {
            opcode: Opcode::MOV,
            operands: alloc::vec![
                (
                    OperandDirection::In,
                    Operand {
                        kind: OperandKind::Register(Register::VirtualRegister(0x1)),
                        width_in_bits: 0x40,
                    },
                ),
                (
                    OperandDirection::Out,
                    Operand {
                        kind: OperandKind::Register(Register::VirtualRegister(0x0)),
                        width_in_bits: 0x40,
                    },
                ),
            ],
        },
        Instruction {
            opcode: Opcode::ADD,
            operands: alloc::vec![
                (
                    OperandDirection::In,
                    Operand {
                        kind: OperandKind::Immediate(0x1),
                        width_in_bits: 0x20,
                    },
                ),
                (
                    OperandDirection::InOut,
                    Operand {
                        kind: OperandKind::Register(Register::VirtualRegister(0x0)),
                        width_in_bits: 0x40,
                    },
                ),
            ],
        },
        Instruction {
            opcode: Opcode::MOV,
            operands: alloc::vec![
                (
                    OperandDirection::In,
                    Operand {
                        kind: OperandKind::Register(Register::VirtualRegister(0x0)),
                        width_in_bits: 0x40,
                    },
                ),
                (
                    OperandDirection::Out,
                    Operand {
                        kind: OperandKind::Memory {
                            base: Some(Register::PhysicalRegister(PhysicalRegister::RBP)),
                            index: None,
                            scale: MemoryScale::S1,
                            displacement: 0x1234,
                            segment_override: None,
                        },
                        width_in_bits: 0x40,
                    },
                ),
            ],
        },
    ];*/

    let mut allocator = SolidStateRegisterAllocator::new(4);
    instrs.iter_mut().rev().for_each(|i| allocator.process(i));

    /*assert_eq!(
        instrs,
        [
            Instruction {
                opcode: Opcode::MOV,
                operands: alloc::vec![
                    (
                        OperandDirection::In,
                        Operand {
                            kind: OperandKind::Memory {
                                base: Some(Register::PhysicalRegister(PhysicalRegister::RBP)),
                                index: None,
                                scale: MemoryScale::S1,
                                displacement: 0x1234,
                                segment_override: None,
                            },
                            width_in_bits: 0x40,
                        },
                    ),
                    (
                        OperandDirection::Out,
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(
                                PhysicalRegister::RCX
                            )),
                            width_in_bits: 0x40,
                        },
                    ),
                ],
            },
            Instruction {
                opcode: Opcode::MOV,
                operands: alloc::vec![
                    (
                        OperandDirection::In,
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(
                                PhysicalRegister::RCX
                            )),
                            width_in_bits: 0x40,
                        },
                    ),
                    (
                        OperandDirection::Out,
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(
                                PhysicalRegister::RAX
                            )),
                            width_in_bits: 0x40,
                        },
                    ),
                ],
            },
            Instruction {
                opcode: Opcode::ADD,
                operands: alloc::vec![
                    (
                        OperandDirection::In,
                        Operand {
                            kind: OperandKind::Immediate(0x1),
                            width_in_bits: 0x20,
                        },
                    ),
                    (
                        OperandDirection::InOut,
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(
                                PhysicalRegister::RAX
                            )),
                            width_in_bits: 0x40,
                        },
                    ),
                ],
            },
            Instruction {
                opcode: Opcode::MOV,
                operands: alloc::vec![
                    (
                        OperandDirection::In,
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(
                                PhysicalRegister::RAX
                            )),
                            width_in_bits: 0x40,
                        },
                    ),
                    (
                        OperandDirection::Out,
                        Operand {
                            kind: OperandKind::Memory {
                                base: Some(Register::PhysicalRegister(PhysicalRegister::RBP)),
                                index: None,
                                scale: MemoryScale::S1,
                                displacement: 0x1234,
                                segment_override: None,
                            },
                            width_in_bits: 0x40,
                        },
                    ),
                ],
            },
        ]
    );*/
}
