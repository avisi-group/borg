use {
    crate::dbt::x86::{
        encoder::{width::Width, Instruction, Opcode, Operand, PhysicalRegister, Register},
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
fn shr_rcx_rcx() {
    let mut instrs = [
        Instruction(Opcode::MOV(
            Operand::imm(Width::_64, 0xaaaa),
            Operand::vreg(Width::_64, 0),
        )),
        Instruction(Opcode::SHR(
            Operand::preg(Width::_8, PhysicalRegister::RCX),
            Operand::vreg(Width::_64, 1),
        )),
        Instruction(Opcode::MOV(
            Operand::vreg(Width::_64, 0),
            Operand::mem_base(Width::_64, Register::VirtualRegister(0)),
        )),
    ];

    let mut allocator = SolidStateRegisterAllocator::new(2);
    instrs.iter_mut().rev().for_each(|i| allocator.process(i));
    todo!()
}

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
