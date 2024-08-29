use {
    crate::dbt::x86::emitter::X86BlockRef,
    alloc::{collections::btree_map::BTreeMap, vec::Vec},
    core::fmt::{Debug, Display, Formatter},
    displaydoc::Display,
    elf::segment,
    iced_x86::code_asm::{
        byte_ptr, dword_ptr, qword_ptr, AsmMemoryOperand, AsmRegister64, CodeAssembler, CodeLabel,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Opcode {
    /// mov {0}, {1}
    MOV(Operand, Operand),
    /// add {0}, {1}
    ADD(Operand, Operand),
    /// sub {0}, {1}
    SUB(Operand, Operand),
    /// jmp {0}
    JMP(Operand),
    /// ret
    RET,
    /// test {0}, {1}
    TEST(Operand, Operand),
    /// cmp {0}, {1}
    CMP(Operand, Operand),
    /// sete {0}
    SETE(Operand),
    /// jne {0}
    JNE(Operand),
    /// nop
    NOP,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum PhysicalRegister {
    /// rax
    RAX,
    /// rcx
    RCX,
    /// rdx
    RDX,
    /// rbx
    RBX,
    /// rsi
    RSI,
    /// rdi
    RDI,
    /// rsp
    RSP,
    /// rbp
    RBP,
    /// r8
    R8,
    /// r9
    R9,
    /// r10
    R10,
    /// r11
    R11,
    /// r12
    R12,
    /// r13
    R13,
    /// r14
    R14,
    /// r15
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

impl From<&PhysicalRegister> for AsmRegister64 {
    fn from(phys: &PhysicalRegister) -> Self {
        use iced_x86::code_asm::{
            r10, r11, r12, r13, r14, r15, r8, r9, rax, rbp, rbx, rcx, rdi, rdx, rsi, rsp,
        };

        match phys {
            PhysicalRegister::RAX => rax,
            PhysicalRegister::RCX => rcx,
            PhysicalRegister::RDX => rdx,
            PhysicalRegister::RBX => rbx,
            PhysicalRegister::RSI => rsi,
            PhysicalRegister::RDI => rdi,
            PhysicalRegister::RSP => rsp,
            PhysicalRegister::RBP => rbp,
            PhysicalRegister::R8 => r8,
            PhysicalRegister::R9 => r9,
            PhysicalRegister::R10 => r10,
            PhysicalRegister::R11 => r11,
            PhysicalRegister::R12 => r12,
            PhysicalRegister::R13 => r13,
            PhysicalRegister::R14 => r14,
            PhysicalRegister::R15 => r15,
        }
    }
}

impl From<PhysicalRegister> for AsmRegister64 {
    fn from(phys: PhysicalRegister) -> Self {
        Self::from(&phys)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum SegmentRegister {
    /// fs
    FS,
    /// gs
    GS,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    PhysicalRegister(PhysicalRegister),
    VirtualRegister(usize),
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Register::PhysicalRegister(pr) => write!(f, "%{pr}"),
            Register::VirtualRegister(vr) => write!(f, "v{vr}"),
        }
    }
}

impl Into<iced_x86::Register> for PhysicalRegister {
    fn into(self) -> iced_x86::Register {
        match self {
            PhysicalRegister::RAX => iced_x86::Register::RAX,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum MemoryScale {
    /// * 1
    S1,
    /// * 2
    S2,
    /// * 4
    S4,
    /// * 8
    S8,
}

impl Into<i32> for MemoryScale {
    fn into(self) -> i32 {
        match self {
            MemoryScale::S1 => 1,
            MemoryScale::S2 => 2,
            MemoryScale::S4 => 4,
            MemoryScale::S8 => 8,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
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
    Target(X86BlockRef),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for OperandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            OperandKind::Immediate(immval) => write!(f, "${immval}"),
            OperandKind::Memory {
                base,
                index,
                scale,
                displacement,
                segment_override,
            } => {
                if let Some(segment_override) = segment_override {
                    write!(f, "{segment_override}")?;
                }

                write!(f, "{displacement}(")?;

                if let Some(base) = base {
                    write!(f, "{base}")?;
                } else {
                    write!(f, "%riz")?;
                }

                if let Some(index) = index {
                    write!(f, ", {index}, {scale}")?;
                }

                write!(f, ")")
            }
            OperandKind::Register(reg) => write!(f, "{reg}"),
            OperandKind::Target(tgt) => write!(f, ">LBL"),
        }
    }
}

impl Debug for OperandKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Immediate(arg0) => f.debug_tuple("Immediate").field(arg0).finish(),
            Self::Memory {
                base,
                index,
                scale,
                displacement,
                segment_override,
            } => f
                .debug_struct("Memory")
                .field("base", base)
                .field("index", index)
                .field("scale", scale)
                .field("displacement", displacement)
                .field("segment_override", segment_override)
                .finish(),
            Self::Register(arg0) => f.debug_tuple("Register").field(arg0).finish(),
            Self::Target(arg0) => write!(f, "{arg0:x}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operand {
    pub kind: OperandKind,
    pub width_in_bits: u8,
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

    pub fn target(target: X86BlockRef) -> Self {
        Self {
            kind: OperandKind::Target(target),
            width_in_bits: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction(pub Opcode);

macro_rules! alu_op {
    ($gen_name: ident, $opcode: ident) => {
        pub fn $gen_name(src: Operand, dst: Operand) -> Self {
            Instruction(Opcode::$opcode(src, dst))
        }
    };
}

pub enum OperandDirection {
    None,
    In,
    Out,
    InOut,
}

pub enum UseDef<'a> {
    Use(&'a mut Register),
    Def(&'a mut Register),
    UseDef(&'a mut Register),
}

impl<'a> UseDef<'a> {
    pub fn from_operand_direction(
        direction: OperandDirection,
        register: &'a mut Register,
    ) -> Option<Self> {
        match direction {
            OperandDirection::None => None,
            OperandDirection::In => Some(UseDef::Use(register)),
            OperandDirection::Out => Some(UseDef::Def(register)),
            OperandDirection::InOut => Some(UseDef::UseDef(register)),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn memory_operand_to_iced(
    base: &PhysicalRegister,
    index: &Option<Register>,
    scale: &MemoryScale,
    displacement: &i32,
) -> AsmMemoryOperand {
    let mut mem = AsmRegister64::from(base) + *displacement;

    if let Some(Register::PhysicalRegister(index)) = index {
        let scale: i32 = match scale {
            MemoryScale::S1 => 1,
            MemoryScale::S2 => 2,
            MemoryScale::S4 => 4,
            MemoryScale::S8 => 8,
        }
        .into();

        mem = mem + AsmRegister64::from(index) * scale;
    }

    mem
}

impl Instruction {
    pub fn mov(src: Operand, dst: Operand) -> Self {
        Self(Opcode::MOV(src, dst))
    }

    pub fn jmp(block: X86BlockRef) -> Self {
        Self(Opcode::JMP(Operand::target(block)))
    }

    pub fn ret() -> Self {
        Self(Opcode::RET)
    }

    pub fn nop() -> Self {
        Self(Opcode::NOP)
    }

    pub fn test(op0: Operand, op1: Operand) -> Self {
        Self(Opcode::TEST(op0, op1))
    }

    pub fn cmp(op0: Operand, op1: Operand) -> Self {
        Self(Opcode::CMP(op0, op1))
    }

    pub fn sete(r: Operand) -> Self {
        Self(Opcode::SETE(r))
    }

    pub fn jne(block: X86BlockRef) -> Self {
        Self(Opcode::JNE(Operand::target(block)))
    }

    alu_op!(add, ADD);
    alu_op!(sub, SUB);

    pub fn encode(
        &self,
        assembler: &mut CodeAssembler,
        label_map: &BTreeMap<X86BlockRef, CodeLabel>,
    ) {
        use {
            Opcode::*,
            OperandKind::{Immediate as I, Memory as M, Register as R},
            Register::PhysicalRegister as PHYS,
        };

        match &self.0 {
            // MOV R -> R
            MOV(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: src_width_in_bits,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: dst_width_in_bits,
                },
            ) => {
                assert!(src_width_in_bits == dst_width_in_bits);

                assembler
                    .mov::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            // MOV M -> R
            MOV(
                Operand {
                    kind:
                        M {
                            base: Some(PHYS(base)),
                            index,
                            scale,
                            displacement,
                            ..
                        },
                    width_in_bits: src_width_in_bits,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: dst_width_in_bits,
                },
            ) => {
                assert!(src_width_in_bits == dst_width_in_bits);

                assembler
                    .mov::<AsmRegister64, AsmMemoryOperand>(
                        dst.into(),
                        memory_operand_to_iced(base, index, scale, displacement),
                    )
                    .unwrap();
            }
            // MOV R -> M
            MOV(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: src_width_in_bits,
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
                    width_in_bits: dst_width_in_bits,
                },
            ) => {
                assert!(src_width_in_bits == dst_width_in_bits);

                assembler
                    .mov::<AsmMemoryOperand, AsmRegister64>(
                        memory_operand_to_iced(base, index, scale, displacement),
                        src.into(),
                    )
                    .unwrap();
            }
            // MOV I -> M
            MOV(
                Operand {
                    kind: I(src),
                    width_in_bits: src_width_in_bits,
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
                    width_in_bits: dst_width_in_bits,
                },
            ) => {
                assert!(src_width_in_bits == dst_width_in_bits);

                assembler
                    .mov::<AsmMemoryOperand, u32>(
                        dword_ptr(memory_operand_to_iced(base, index, scale, displacement)),
                        *src as u32,
                    )
                    .unwrap();
            }
            _ => panic!("cannot encode this instruction {}", self),
        }
    }

    //     Opcode::ADD => match self.operands.as_slice() {
    //         [(
    //             OperandDirection::In,
    //             Operand {
    //                 kind: OperandKind::Immediate(imm),
    //                 width_in_bits: 32,
    //             },
    //         ), (
    //             OperandDirection::InOut,
    //             Operand {
    //                 kind:
    // OperandKind::Register(Register::PhysicalRegister(dst)),
    //                 width_in_bits: 64,
    //             },
    //         )] => {
    //             assembler
    //                 .add::<AsmRegister64, _>(dst.into(),
    // i32::try_from(*imm).unwrap())                 .unwrap();
    //         }

    //         [(
    //             OperandDirection::In,
    //             Operand {
    //                 kind:
    // OperandKind::Register(Register::PhysicalRegister(src)),
    //                 width_in_bits: 64,
    //             },
    //         ), (
    //             OperandDirection::InOut,
    //             Operand {
    //                 kind:
    // OperandKind::Register(Register::PhysicalRegister(dst)),
    //                 width_in_bits: 64,
    //             },
    //         )] => {
    //             assembler
    //                 .add::<AsmRegister64, AsmRegister64>(dst.into(),
    // src.into())                 .unwrap();
    //         }

    //         ops => todo!("{ops:?} operands not supported for add"),
    //     },

    //     Opcode::JNE => {
    //         let [(
    //             OperandDirection::In,
    //             Operand {
    //                 kind: OperandKind::Target(target),
    //                 ..
    //             },
    //         )] = self.operands.as_slice()
    //         else {
    //             panic!("invalid operands: {:?}", self.operands);
    //         };

    //         let label = label_map.get(target).unwrap().clone();
    //         assembler.jne(label).unwrap();
    //     }

    //     Opcode::JMP => {
    //         let [(
    //             OperandDirection::In,
    //             Operand {
    //                 kind: OperandKind::Target(target),
    //                 ..
    //             },
    //         )] = self.operands.as_slice()
    //         else {
    //             panic!("invalid operands: {:?}", self.operands);
    //         };

    //         let label = label_map.get(target).unwrap().clone();
    //         assembler.jmp(label).unwrap();
    //     }

    //     Opcode::LABEL => {}

    //     Opcode::TEST => match self.operands.as_slice() {
    //         [(
    //             OperandDirection::In,
    //             Operand {
    //                 kind:
    // OperandKind::Register(Register::PhysicalRegister(left)),
    //                 width_in_bits: w1,
    //             },
    //         ), (
    //             OperandDirection::In,
    //             Operand {
    //                 kind:
    // OperandKind::Register(Register::PhysicalRegister(right)),
    //                 width_in_bits: w2,
    //             },
    //         )] => {
    //             assert_eq!(w1, w2);

    //             assembler
    //                 .test::<AsmRegister64, AsmRegister64>(left.into(),
    // right.into())                 .unwrap();
    //         }
    //         _ => todo!(),
    //     },

    //     Opcode::RET => assembler.ret().unwrap(),

    //     o => unimplemented!("opcode {o:?}"),
    // };

    pub fn get_operands(
        &mut self,
    ) -> impl Iterator<Item = Option<(OperandDirection, &mut Operand)>> + '_ {
        match &mut self.0 {
            Opcode::MOV(src, dst) => [
                Some((OperandDirection::In, src)),
                Some((OperandDirection::Out, dst)),
            ]
            .into_iter(),
            Opcode::ADD(src, dst) | Opcode::SUB(src, dst) => [
                Some((OperandDirection::In, src)),
                Some((OperandDirection::InOut, dst)),
            ]
            .into_iter(),
            Opcode::JMP(tgt) => [Some((OperandDirection::None, tgt)), None].into_iter(),
            Opcode::RET | Opcode::NOP => [None, None].into_iter(),
            Opcode::TEST(op0, op1) | Opcode::CMP(op0, op1) => [
                Some((OperandDirection::In, op0)),
                Some((OperandDirection::In, op1)),
            ]
            .into_iter(),
            Opcode::JNE(tgt) => [Some((OperandDirection::None, tgt)), None].into_iter(),
            Opcode::SETE(r) => [Some((OperandDirection::Out, r)), None].into_iter(),
        }
    }

    pub fn get_use_defs(&mut self) -> impl Iterator<Item = UseDef> + '_ {
        self.get_operands()
            .flatten()
            .filter_map(|operand| match &mut operand.1.kind {
                OperandKind::Memory {
                    base: Some(base), ..
                } => Some(UseDef::Use(base)), // TODO: index
                OperandKind::Register(register) => {
                    Some(UseDef::from_operand_direction(operand.0, register).unwrap())
                }
                _ => None,
            })

        // self.operands
        //     .iter_mut()
        //     .filter_map(|(direction, operand)| match &mut operand.kind {
        //         OperandKind::Immediate(_) => None,
        //         // todo: avoid allocation here
        //         OperandKind::Memory { base, index, .. } => Some(
        //             [base, index]
        //                 .into_iter()
        //                 .filter_map(|reg| reg.as_mut().map(|reg|
        // (OperandDirection::In, reg)))
        // .collect::<Vec<_>>(),         ),
        //         OperandKind::Register(reg) => {
        //             Some([(direction.clone(),
        // reg)].into_iter().collect::<Vec<_>>())         }
        //         OperandKind::Target(_) => None,
        //     })
        //     .flatten()
    }
}
