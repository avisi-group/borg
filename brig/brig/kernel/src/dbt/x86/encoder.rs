use {
    crate::dbt::x86::emitter::X86BlockRef,
    alloc::vec::Vec,
    core::fmt::Debug,
    iced_x86::code_asm::{rax, rbx, rcx, rdx, AsmRegister64, CodeAssembler},
};

#[derive(Debug, Clone)]
pub enum Opcode {
    MOV,
    ADD,
    SUB,
    LABEL,
    JMP,
    RET,
}

#[derive(Debug, Clone)]
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

impl PhysicalRegister {
    pub fn index(&self) -> usize {
        match self {
            PhysicalRegister::RAX => 0,
            PhysicalRegister::RCX => 1,
            PhysicalRegister::RDX => 2,
            PhysicalRegister::RBX => 3,
            PhysicalRegister::RSI => 4,
            PhysicalRegister::RDI => 5,
            PhysicalRegister::RSP => 6,
            PhysicalRegister::RBP => 7,
            PhysicalRegister::R8 => 8,
            PhysicalRegister::R9 => 9,
            PhysicalRegister::R10 => 10,
            PhysicalRegister::R11 => 11,
            PhysicalRegister::R12 => 12,
            PhysicalRegister::R13 => 13,
            PhysicalRegister::R14 => 14,
            PhysicalRegister::R15 => 15,
        }
    }

    pub fn from_index(index: usize) -> PhysicalRegister {
        match index {
            0 => PhysicalRegister::RAX,
            1 => PhysicalRegister::RCX,
            2 => PhysicalRegister::RDX,
            3 => PhysicalRegister::RBX,
            4 => PhysicalRegister::RSI,
            5 => PhysicalRegister::RDI,
            6 => PhysicalRegister::RSP,
            7 => PhysicalRegister::RBP,
            8 => PhysicalRegister::R8,
            9 => PhysicalRegister::R9,
            10 => PhysicalRegister::R10,
            11 => PhysicalRegister::R11,
            12 => PhysicalRegister::R12,
            13 => PhysicalRegister::R13,
            14 => PhysicalRegister::R14,
            15 => PhysicalRegister::R15,
            _ => unreachable!(),
        }
    }
}

impl Into<AsmRegister64> for &PhysicalRegister {
    fn into(self) -> AsmRegister64 {
        match self {
            PhysicalRegister::RAX => rax,
            PhysicalRegister::RCX => rcx,
            PhysicalRegister::RDX => rdx,
            PhysicalRegister::RBX => rbx,
            PhysicalRegister::RSI => todo!(),
            PhysicalRegister::RDI => todo!(),
            PhysicalRegister::RSP => todo!(),
            PhysicalRegister::RBP => todo!(),
            PhysicalRegister::R8 => todo!(),
            PhysicalRegister::R9 => todo!(),
            PhysicalRegister::R10 => todo!(),
            PhysicalRegister::R11 => todo!(),
            PhysicalRegister::R12 => todo!(),
            PhysicalRegister::R13 => todo!(),
            PhysicalRegister::R14 => todo!(),
            PhysicalRegister::R15 => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SegmentRegister {
    FS,
    GS,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum MemoryScale {
    S1,
    S2,
    S4,
    S8,
}

#[derive(Clone)]
pub enum OperandKind {
    Immediate(u64),
    Memory { base: Option<Register>, index: Option<Register>, scale: MemoryScale, displacement: i32, segment_override: Option<SegmentRegister> },
    Register(Register),
    Target(X86BlockRef),
}

impl Debug for OperandKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Immediate(arg0) => f.debug_tuple("Immediate").field(arg0).finish(),
            Self::Memory { base, index, scale, displacement, segment_override } => f.debug_struct("Memory").field("base", base).field("index", index).field("scale", scale).field("displacement", displacement).field("segment_override", segment_override).finish(),
            Self::Register(arg0) => f.debug_tuple("Register").field(arg0).finish(),
            Self::Target(arg0) => write!(f, "{arg0:x}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperandDirection {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone)]
pub struct Operand {
    kind: OperandKind,
    width_in_bits: u8,
}

impl Operand {
    pub fn imm(width_in_bits: u8, value: u64) -> Operand {
        Operand { kind: OperandKind::Immediate(value), width_in_bits }
    }

    pub fn preg(width_in_bits: u8, reg: PhysicalRegister) -> Operand {
        Operand { kind: OperandKind::Register(Register::PhysicalRegister(reg)), width_in_bits }
    }

    pub fn vreg(width_in_bits: u8, reg: usize) -> Operand {
        Operand { kind: OperandKind::Register(Register::VirtualRegister(reg)), width_in_bits }
    }

    pub fn mem_base(width_in_bits: u8, base: Register) -> Operand {
        Self::mem_base_displ(width_in_bits, base, 0)
    }

    pub fn mem_base_displ(width_in_bits: u8, base: Register, displacement: i32) -> Operand {
        Operand { kind: OperandKind::Memory { base: Some(base), index: None, scale: MemoryScale::S1, displacement, segment_override: None }, width_in_bits }
    }

    pub fn mem_base_idx_scale(width_in_bits: u8, base: Register, idx: Register, scale: MemoryScale) -> Operand {
        Self::mem_base_idx_scale_displ(width_in_bits, base, idx, scale, 0)
    }

    pub fn mem_base_idx_scale_displ(width_in_bits: u8, base: Register, idx: Register, scale: MemoryScale, displacement: i32) -> Operand {
        Operand { kind: OperandKind::Memory { base: Some(base), index: Some(idx), scale, displacement, segment_override: None }, width_in_bits }
    }

    pub fn mem_seg_displ(width_in_bits: u8, segment: SegmentRegister, displacement: i32) -> Operand {
        Operand { kind: OperandKind::Memory { base: None, index: None, scale: MemoryScale::S1, displacement, segment_override: Some(segment) }, width_in_bits }
    }

    pub fn target(target: X86BlockRef) -> Self {
        Self { kind: OperandKind::Target(target), width_in_bits: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    operands: Vec<(OperandDirection, Operand)>,
}

macro_rules! alu_op {
    ($gen_name: ident, $opcode: ident) => {
        pub fn $gen_name(src: Operand, dst: Operand) -> Self {
            Instruction { opcode: Opcode::$opcode, operands: alloc::vec![(OperandDirection::In, src), (OperandDirection::InOut, dst)] }
        }
    };
}

impl Instruction {
    pub fn mov(src: Operand, dst: Operand) -> Self {
        Self { opcode: Opcode::MOV, operands: alloc::vec![(OperandDirection::In, src), (OperandDirection::Out, dst)] }
    }

    pub fn label() -> Self {
        Self { opcode: Opcode::LABEL, operands: alloc::vec![] }
    }

    pub fn jmp(block: X86BlockRef) -> Self {
        Self { opcode: Opcode::JMP, operands: alloc::vec![(OperandDirection::In, Operand::target(block))] }
    }

    pub fn ret() -> Self {
        Self { opcode: Opcode::RET, operands: alloc::vec![] }
    }

    alu_op!(add, ADD);
    alu_op!(sub, SUB);

    fn operand_tuple2(&self) -> (&Operand, &Operand) {
        (&self.operands[0].1, &self.operands[1].1)
    }

    pub fn encode(&self, assembler: &mut CodeAssembler) {
        match &self.opcode {
            Opcode::MOV => match self.operand_tuple2() {
                (Operand { kind: OperandKind::Register(Register::PhysicalRegister(src)), width_in_bits: w }, Operand { kind: OperandKind::Register(Register::PhysicalRegister(dst)), width_in_bits: w2 }) => {
                    assert!(w == w2);

                    assembler.mov::<AsmRegister64, AsmRegister64>((src).into(), (dst).into()).unwrap();
                }
                ops => todo!("{ops:?} operands not supported for mov"),
            },
            Opcode::LABEL => {}
            o => unimplemented!("opcode {o:?}"),
        };
    }

    pub fn get_use_defs(&mut self) -> impl Iterator<Item = (OperandDirection, &mut Register)> + '_ {
        self.operands
            .iter_mut()
            .filter_map(|(direction, operand)| match &mut operand.kind {
                OperandKind::Immediate(_) => None,
                // todo: avoid allocation here
                OperandKind::Memory { base, index, .. } => Some([base, index].into_iter().filter_map(|reg| reg.as_mut().map(|reg| (OperandDirection::In, reg))).collect::<Vec<_>>()),
                OperandKind::Register(reg) => Some([(direction.clone(), reg)].into_iter().collect::<Vec<_>>()),
                OperandKind::Target(_) => None,
            })
            .flatten()
    }
}
