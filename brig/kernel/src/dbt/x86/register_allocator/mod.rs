use {
    crate::dbt::x86::{
        encoder::{
            Instruction, MemoryScale, Opcode, Operand, PhysicalRegister, Register, width::Width,
        },
        register_allocator::naive::FreshAllocator, //solid_state::SolidStateRegisterAllocator,
    },
    proc_macro_lib::ktest,
};

//pub mod reverse_scan;
pub mod naive;
//pub mod solid_state;

pub trait RegisterAllocator {
    fn allocate(&mut self, instructions: &mut [Instruction]);
}

#[ktest]
fn conflicted_physical_allocation() {
    let mut instrs = [
        Instruction(Opcode::MOV(
            Operand::imm(Width::_64, 0xaaaa),
            Operand::vreg(Width::_64, 0),
        )),
        Instruction(Opcode::MOV(
            Operand::imm(Width::_64, 0xaaaa),
            Operand::preg(Width::_64, PhysicalRegister::RAX),
        )),
        Instruction(Opcode::MOV(
            Operand::preg(Width::_64, PhysicalRegister::RAX),
            Operand::mem_base(Width::_64, Register::VirtualRegister(0)),
        )),
        Instruction(Opcode::MOV(
            Operand::vreg(Width::_64, 0),
            Operand::mem_base(Width::_64, Register::VirtualRegister(0)),
        )),
    ];

    let mut allocator = FreshAllocator::new(2);
    allocator.allocate(&mut instrs);
}

#[ktest]
fn shr_full() {
    use {
        crate::dbt::x86::{
            OperandKind::*,
            register_allocator::{PhysicalRegister::*, Register::*},
        },
        Opcode::*,
    };

    let mut instructions = [
        Instruction(MOV(
            Operand {
                kind: Memory {
                    base: Some(PhysicalRegister(RBP)),
                    index: None,
                    scale: MemoryScale::S1,
                    displacement: 12560,
                    segment_override: None,
                },
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(4)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(4)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(5)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(SHR(
            Operand {
                kind: Immediate(0),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(5)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(5)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(3)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Immediate(18446744073709551615),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(6)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(3)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(2)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(AND(
            Operand {
                kind: Register(VirtualRegister(6)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(2)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(2)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(1)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Immediate(64),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(7)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Immediate(0),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(PhysicalRegister(RDX)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(1)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(PhysicalRegister(RAX)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(IDIV(
            Operand {
                kind: Register(PhysicalRegister(RDX)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(PhysicalRegister(RAX)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(7)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Memory {
                    base: Some(PhysicalRegister(RBP)),
                    index: None,
                    scale: MemoryScale::S1,
                    displacement: 4984,
                    segment_override: None,
                },
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(10)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(10)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(11)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(SHR(
            Operand {
                kind: Immediate(0),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(11)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(11)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(9)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Immediate(18446744073709551615),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(12)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(9)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(8)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(AND(
            Operand {
                kind: Register(VirtualRegister(12)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(8)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(8)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(13)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(PhysicalRegister(RDX)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(PhysicalRegister(RCX)),
                width_in_bits: Width::_8,
            },
        )),
        Instruction(SHR(
            Operand {
                kind: Register(PhysicalRegister(RCX)),
                width_in_bits: Width::_8,
            },
            Operand {
                kind: Register(VirtualRegister(13)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(13)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Register(VirtualRegister(0)),
                width_in_bits: Width::_64,
            },
        )),
        Instruction(MOV(
            Operand {
                kind: Register(VirtualRegister(0)),
                width_in_bits: Width::_64,
            },
            Operand {
                kind: Memory {
                    base: Some(PhysicalRegister(RBP)),
                    index: None,
                    scale: MemoryScale::S1,
                    displacement: 12560,
                    segment_override: None,
                },
                width_in_bits: Width::_64,
            },
        )),
    ];

    let mut allocator = FreshAllocator::new(14);
    allocator.allocate(&mut instructions);
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
