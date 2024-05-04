use alloc::vec::Vec;

pub enum Opcode {
    MOV,
    ADD,
    SUB,
}

pub enum PhysicalRegister {
    RAX,
    RCX,
    RDX,
    RBX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub enum SegmentRegister {
    FS,
    GS,
}

pub enum Register {
    PhysicalRegister(PhysicalRegister),
    VirtualRegister(usize),
}

impl Into<iced_x86::Register> for PhysicalRegister {
    fn into(self) -> iced_x86::Register {
        match self {
            PhysicalRegister::RAX => iced_x86::Register::RAX,
            _ => todo!(),
        }
    }
}

pub enum MemoryScale {
    S1,
    S2,
    S4,
    S8,
}

pub enum OperandKind {
    Immediate(u64),
    Memory {
        base: Option<Register>,
        index: Option<Register>,
        scale: MemoryScale,
        displacement: i32,
        segment_override: Option<SegmentRegister>,
    },
    Register(Register),
}

pub enum OperandDirection {
    In,
    Out,
    InOut,
}

pub struct Operand {
    kind: OperandKind,
    width_in_bits: u8,
}

impl Operand {
    pub fn imm(width_in_bits: u8, value: u64) -> Operand {
        Operand {
            kind: OperandKind::Immediate(value),
            width_in_bits,
        }
    }

    pub fn preg(width_in_bits: u8, reg: PhysicalRegister) -> Operand {
        Operand {
            kind: OperandKind::Register(Register::PhysicalRegister(reg)),
            width_in_bits,
        }
    }

    pub fn vreg(width_in_bits: u8, reg: usize) -> Operand {
        Operand {
            kind: OperandKind::Register(Register::VirtualRegister(reg)),
            width_in_bits,
        }
    }

    pub fn mem_base(width_in_bits: u8, base: Register) -> Operand {
        Self::mem_base_displ(width_in_bits, base, 0)
    }

    pub fn mem_base_displ(width_in_bits: u8, base: Register, displacement: i32) -> Operand {
        Operand {
            kind: OperandKind::Memory {
                base: Some(base),
                index: None,
                scale: MemoryScale::S1,
                displacement,
                segment_override: None,
            },
            width_in_bits,
        }
    }

    pub fn mem_base_idx_scale(
        width_in_bits: u8,
        base: Register,
        idx: Register,
        scale: MemoryScale,
    ) -> Operand {
        Self::mem_base_idx_scale_displ(width_in_bits, base, idx, scale, 0)
    }

    pub fn mem_base_idx_scale_displ(
        width_in_bits: u8,
        base: Register,
        idx: Register,
        scale: MemoryScale,
        displacement: i32,
    ) -> Operand {
        Operand {
            kind: OperandKind::Memory {
                base: Some(base),
                index: Some(idx),
                scale,
                displacement,
                segment_override: None,
            },
            width_in_bits,
        }
    }

    pub fn mem_seg_displ(
        width_in_bits: u8,
        segment: SegmentRegister,
        displacement: i32,
    ) -> Operand {
        Operand {
            kind: OperandKind::Memory {
                base: None,
                index: None,
                scale: MemoryScale::S1,
                displacement,
                segment_override: Some(segment),
            },
            width_in_bits,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
    operands: Vec<(OperandDirection, Operand)>,
}

macro_rules! alu_op {
    ($gen_name: ident, $opcode: ident) => {
        pub fn $gen_name(src: Operand, dst: Operand) -> Self {
            Instruction {
                opcode: Opcode::$opcode,
                operands: alloc::vec![(OperandDirection::In, src), (OperandDirection::InOut, dst)],
            }
        }
    };
}

impl Instruction {
    pub fn mov(src: Operand, dst: Operand) -> Self {
        Instruction {
            opcode: Opcode::MOV,
            operands: alloc::vec![(OperandDirection::In, src), (OperandDirection::Out, dst)],
        }
    }

    alu_op!(add, ADD);
    alu_op!(sub, SUB);

    fn operand_tuple2(&self) -> (&Operand, &Operand) {
        (&self.operands[0].1, &self.operands[1].1)
    }

    pub fn encode(&self) -> iced_x86::Instruction {
        match self.opcode {
            Opcode::MOV => {
                match self.operand_tuple2() {
                    (
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(src)),
                            width_in_bits: w,
                        },
                        Operand {
                            kind: OperandKind::Register(Register::PhysicalRegister(dst)),
                            width_in_bits: w2,
                        },
                    ) => {
                        assert!(w == w2);
                        //iced_x86::Instruction::with2(iced_x86::Code::Mov_r64_rm64, op0, op1)
                        todo!()
                    }
                    _ => todo!("operands not supported for mov"),
                }
            }
            _ => todo!(),
        }
    }
}
