use {
    crate::dbt::x86::{emitter::X86Block, encoder::width::Width},
    common::{arena::Ref, HashMap},
    core::fmt::{Debug, Display, Formatter},
    displaydoc::Display,
    iced_x86::code_asm::{
        qword_ptr, AsmMemoryOperand, AsmRegister16, AsmRegister32, AsmRegister64, AsmRegister8,
        CodeAssembler, CodeLabel,
    },
};

mod mov;
mod setne;
mod shl;
mod shr;
pub mod width;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Opcode {
    /// mov {0}, {1}
    MOV(Operand, Operand),
    /// movzx {0}, {1}
    MOVZX(Operand, Operand),
    /// movsx {0}, {1}
    MOVSX(Operand, Operand),
    /// cmove {0}, {1}
    CMOVE(Operand, Operand),
    /// cmovne {0}, {1}
    CMOVNE(Operand, Operand),
    /// lea {0}, {1}
    LEA(Operand, Operand),
    /// shl {0}, {1}
    SHL(Operand, Operand),
    /// shr {0}, {1}
    SHR(Operand, Operand),
    /// sar {0}, {1}
    SAR(Operand, Operand),
    /// add {0}, {1}
    ADD(Operand, Operand),
    /// adc {0}, {1}, {2}
    ADC(Operand, Operand, Operand),
    /// sub {0}, {1}
    SUB(Operand, Operand),
    /// or {0}, {1},
    OR(Operand, Operand),
    /// xor {0}, {1},
    XOR(Operand, Operand),
    /// and {0}, {1},
    AND(Operand, Operand),
    /// imul {0}, {1},
    IMUL(Operand, Operand),
    /// idiv {0}, {1}, {2}
    IDIV(Operand, Operand, Operand),
    /// not {0}
    NOT(Operand),
    /// neg {0}
    NEG(Operand),
    /// bextr {0}, {1}, {2}
    BEXTR(Operand, Operand, Operand),
    /// jmp {0}
    JMP(Operand),
    /// push {0}
    PUSH(Operand),
    /// pop {0}
    POP(Operand),
    /// ret
    RET,
    /// test {0}, {1}
    TEST(Operand, Operand),
    /// cmp {0}, {1}
    CMP(Operand, Operand),

    /// sets {0}
    SETS(Operand), //n
    /// sete {0}
    SETE(Operand), //z
    /// setc {0}
    SETC(Operand), //c
    /// seto {0}
    SETO(Operand), //v

    /// setne {0}
    SETNE(Operand),
    /// setnz {0}
    SETNZ(Operand),
    /// setb {0}
    SETB(Operand),
    /// setbe {0}
    SETBE(Operand),
    /// seta {0}
    SETA(Operand),
    /// setg {0}
    SETG(Operand),
    /// setae {0}
    SETAE(Operand),
    /// jne {0}
    JNE(Operand),
    /// nop
    NOP,
    /// int {0}
    INT(Operand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
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

impl From<&PhysicalRegister> for AsmRegister8 {
    fn from(phys: &PhysicalRegister) -> Self {
        use iced_x86::code_asm::{
            al, bl, bpl, cl, dil, dl, r10b, r11b, r12b, r13b, r14b, r15b, r8b, r9b, sil, spl,
        };

        match phys {
            PhysicalRegister::RAX => al,
            PhysicalRegister::RCX => cl,
            PhysicalRegister::RDX => dl,
            PhysicalRegister::RBX => bl,
            PhysicalRegister::RSI => sil,
            PhysicalRegister::RDI => dil,
            PhysicalRegister::RSP => spl,
            PhysicalRegister::RBP => bpl,
            PhysicalRegister::R8 => r8b,
            PhysicalRegister::R9 => r9b,
            PhysicalRegister::R10 => r10b,
            PhysicalRegister::R11 => r11b,
            PhysicalRegister::R12 => r12b,
            PhysicalRegister::R13 => r13b,
            PhysicalRegister::R14 => r14b,
            PhysicalRegister::R15 => r15b,
        }
    }
}

impl From<PhysicalRegister> for AsmRegister8 {
    fn from(phys: PhysicalRegister) -> Self {
        Self::from(&phys)
    }
}

impl From<&PhysicalRegister> for AsmRegister16 {
    fn from(phys: &PhysicalRegister) -> Self {
        use iced_x86::code_asm::{
            ax, bp, bx, cx, di, dx, r10w, r11w, r12w, r13w, r14w, r15w, r8w, r9w, si, sp,
        };

        match phys {
            PhysicalRegister::RAX => ax,
            PhysicalRegister::RCX => cx,
            PhysicalRegister::RDX => dx,
            PhysicalRegister::RBX => bx,
            PhysicalRegister::RSI => si,
            PhysicalRegister::RDI => di,
            PhysicalRegister::RSP => sp,
            PhysicalRegister::RBP => bp,
            PhysicalRegister::R8 => r8w,
            PhysicalRegister::R9 => r9w,
            PhysicalRegister::R10 => r10w,
            PhysicalRegister::R11 => r11w,
            PhysicalRegister::R12 => r12w,
            PhysicalRegister::R13 => r13w,
            PhysicalRegister::R14 => r14w,
            PhysicalRegister::R15 => r15w,
        }
    }
}

impl From<&PhysicalRegister> for AsmRegister32 {
    fn from(phys: &PhysicalRegister) -> Self {
        use iced_x86::code_asm::{
            eax, ebp, ebx, ecx, edi, edx, esi, esp, r10d, r11d, r12d, r13d, r14d, r15d, r8d, r9d,
        };

        match phys {
            PhysicalRegister::RAX => eax,
            PhysicalRegister::RCX => ecx,
            PhysicalRegister::RDX => edx,
            PhysicalRegister::RBX => ebx,
            PhysicalRegister::RSI => esi,
            PhysicalRegister::RDI => edi,
            PhysicalRegister::RSP => esp,
            PhysicalRegister::RBP => ebp,
            PhysicalRegister::R8 => r8d,
            PhysicalRegister::R9 => r9d,
            PhysicalRegister::R10 => r10d,
            PhysicalRegister::R11 => r11d,
            PhysicalRegister::R12 => r12d,
            PhysicalRegister::R13 => r13d,
            PhysicalRegister::R14 => r14d,
            PhysicalRegister::R15 => r15d,
        }
    }
}

impl From<PhysicalRegister> for AsmRegister16 {
    fn from(phys: PhysicalRegister) -> Self {
        Self::from(&phys)
    }
}

impl From<PhysicalRegister> for AsmRegister32 {
    fn from(phys: PhysicalRegister) -> Self {
        Self::from(&phys)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum SegmentRegister {
    /// fs
    FS,
    /// gs
    GS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
    Target(Ref<X86Block>),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.kind, self.width_in_bits)
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
            OperandKind::Target(tgt) => write!(f, "{tgt:?}"),
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
            Self::Target(arg0) => write!(f, "{arg0:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operand {
    kind: OperandKind,
    width_in_bits: Width,
}

impl Operand {
    pub fn kind(&self) -> &OperandKind {
        &self.kind
    }

    pub fn width(&self) -> Width {
        self.width_in_bits
    }

    pub fn imm(width_in_bits: Width, value: u64) -> Operand {
        Operand {
            kind: OperandKind::Immediate(value),
            width_in_bits: (width_in_bits),
        }
    }

    pub fn preg(width_in_bits: Width, reg: PhysicalRegister) -> Operand {
        Operand {
            kind: OperandKind::Register(Register::PhysicalRegister(reg)),
            width_in_bits: (width_in_bits),
        }
    }

    pub fn vreg(width_in_bits: Width, reg: usize) -> Operand {
        Operand {
            kind: OperandKind::Register(Register::VirtualRegister(reg)),
            width_in_bits: (width_in_bits),
        }
    }

    pub fn mem_base(width_in_bits: Width, base: Register) -> Operand {
        Self::mem_base_displ(width_in_bits, base, 0)
    }

    pub fn mem_base_displ(width_in_bits: Width, base: Register, displacement: i32) -> Operand {
        Operand {
            kind: OperandKind::Memory {
                base: Some(base),
                index: None,
                scale: MemoryScale::S1,
                displacement,
                segment_override: None,
            },
            width_in_bits: (width_in_bits),
        }
    }

    pub fn mem_base_idx_scale(
        width_in_bits: u16,
        base: Register,
        idx: Register,
        scale: MemoryScale,
    ) -> Operand {
        Self::mem_base_idx_scale_displ(width_in_bits, base, idx, scale, 0)
    }

    pub fn mem_base_idx_scale_displ(
        width_in_bits: u16,
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
            width_in_bits: Width::from_uncanonicalized(width_in_bits).unwrap(),
        }
    }

    pub fn mem_seg_displ(
        width_in_bits: u16,
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
            width_in_bits: Width::from_uncanonicalized(width_in_bits).unwrap(),
        }
    }

    pub fn target(target: Ref<X86Block>) -> Self {
        Self {
            kind: OperandKind::Target(target),
            width_in_bits: Width::_64, // todo: not really true, fix this
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction(pub Opcode);

macro_rules! alu_op {
    ($gen_name: ident, $opcode: ident) => {
        pub fn $gen_name(src: Operand, dst: Operand) -> Self {
            // todo: re-enable me
            // if src.width() != dst.width() {
            //     panic!("different widths: {src} {dst}")
            // }
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
    base: PhysicalRegister,
    index: Option<Register>,
    scale: MemoryScale,
    displacement: i32,
) -> AsmMemoryOperand {
    let mut mem = AsmRegister64::from(base) + displacement;

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
    pub fn adc(a: Operand, b: Operand, c: Operand) -> Self {
        Self(Opcode::ADC(a, b, c))
    }

    pub fn mov(src: Operand, dst: Operand) -> Self {
        Self(Opcode::MOV(src, dst))
    }

    pub fn movzx(src: Operand, dst: Operand) -> Self {
        Self(Opcode::MOVZX(src, dst))
    }

    pub fn movsx(src: Operand, dst: Operand) -> Self {
        Self(Opcode::MOVSX(src, dst))
    }

    pub fn lea(src: Operand, dst: Operand) -> Self {
        Self(Opcode::LEA(src, dst))
    }

    pub fn and(src: Operand, dst: Operand) -> Self {
        Self(Opcode::AND(src, dst))
    }

    pub fn imul(src: Operand, dst: Operand) -> Self {
        Self(Opcode::IMUL(src, dst))
    }

    pub fn idiv(dividend_hi: Operand, dividend_lo: Operand, divisor: Operand) -> Self {
        Self(Opcode::IDIV(dividend_hi, dividend_lo, divisor))
    }

    pub fn shl(amount: Operand, op0: Operand) -> Self {
        Self(Opcode::SHL(amount, op0))
    }

    pub fn shr(amount: Operand, op0: Operand) -> Self {
        Self(Opcode::SHR(amount, op0))
    }

    pub fn sar(amount: Operand, op0: Operand) -> Self {
        Self(Opcode::SAR(amount, op0))
    }

    pub fn bextr(ctrl: Operand, src: Operand, dst: Operand) -> Self {
        Self(Opcode::BEXTR(ctrl, src, dst))
    }

    pub fn jmp(block: Ref<X86Block>) -> Self {
        Self(Opcode::JMP(Operand::target(block)))
    }

    pub fn push(src: Operand) -> Self {
        Self(Opcode::PUSH(src))
    }

    pub fn pop(dest: Operand) -> Self {
        Self(Opcode::POP(dest))
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
    pub fn seto(r: Operand) -> Self {
        Self(Opcode::SETO(r))
    }
    pub fn setc(r: Operand) -> Self {
        Self(Opcode::SETC(r))
    }
    pub fn sete(r: Operand) -> Self {
        Self(Opcode::SETE(r))
    }

    pub fn sets(r: Operand) -> Self {
        Self(Opcode::SETS(r))
    }

    pub fn setne(r: Operand) -> Self {
        Self(Opcode::SETNE(r))
    }
    pub fn setnz(r: Operand) -> Self {
        Self(Opcode::SETNZ(r))
    }

    pub fn setb(r: Operand) -> Self {
        Self(Opcode::SETB(r))
    }
    pub fn setg(r: Operand) -> Self {
        Self(Opcode::SETG(r))
    }
    pub fn setbe(r: Operand) -> Self {
        Self(Opcode::SETBE(r))
    }

    pub fn seta(r: Operand) -> Self {
        Self(Opcode::SETA(r))
    }
    pub fn setae(r: Operand) -> Self {
        Self(Opcode::SETAE(r))
    }

    pub fn jne(block: Ref<X86Block>) -> Self {
        Self(Opcode::JNE(Operand::target(block)))
    }

    pub fn not(r: Operand) -> Self {
        Self(Opcode::NOT(r))
    }

    pub fn neg(r: Operand) -> Self {
        Self(Opcode::NEG(r))
    }

    pub fn int(n: Operand) -> Self {
        Self(Opcode::INT(n))
    }

    pub fn cmove(src: Operand, dest: Operand) -> Self {
        Self(Opcode::CMOVE(src, dest))
    }

    pub fn cmovne(src: Operand, dest: Operand) -> Self {
        Self(Opcode::CMOVNE(src, dest))
    }

    alu_op!(add, ADD);
    alu_op!(sub, SUB);
    alu_op!(or, OR);
    alu_op!(xor, XOR);

    pub fn encode(
        &self,
        assembler: &mut CodeAssembler,
        label_map: &HashMap<Ref<X86Block>, CodeLabel>,
    ) {
        use {
            Opcode::*,
            OperandKind::{Immediate as I, Memory as M, Register as R, Target as T},
            Register::PhysicalRegister as PHYS,
        };

        match &self.0 {
            MOV(src, dst) => mov::encode(assembler, src, dst),
            MOVSX(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: src_width,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: dst_width,
                },
            ) => match (*src_width, *dst_width) {
                (Width::_32, Width::_64) => assembler
                    .movsxd::<AsmRegister64, AsmRegister32>(dst.into(), src.into())
                    .unwrap(),
                (src, dst) => todo!("{src} -> {dst} sign extend mov not implemented"),
            },
            // MOVZX R -> R
            MOVZX(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: src_width,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: dst_width,
                },
            ) => match (*src_width, *dst_width) {
                (Width::_16, Width::_32) => assembler
                    .movzx::<AsmRegister32, AsmRegister16>(dst.into(), src.into())
                    .unwrap(),
                (Width::_16, Width::_64) => assembler
                    .movzx::<AsmRegister64, AsmRegister16>(dst.into(), src.into())
                    .unwrap(),
                (Width::_8, Width::_64) => assembler
                    .movzx::<AsmRegister64, AsmRegister8>(dst.into(), src.into())
                    .unwrap(),
                (Width::_32, Width::_64) => {
                    assembler
                        .xor::<AsmRegister64, AsmRegister64>(dst.into(), dst.into())
                        .unwrap();
                    assembler
                        .mov::<AsmRegister32, AsmRegister32>(dst.into(), src.into())
                        .unwrap()
                }
                (Width::_32, Width::_32) => assembler
                    .mov::<AsmRegister32, AsmRegister32>(dst.into(), src.into())
                    .unwrap(),

                (src, dst) => todo!("{src} -> {dst} zero extend mov not implemented"),
            },

            LEA(
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
                    .lea::<AsmRegister64, AsmMemoryOperand>(
                        dst.into(),
                        qword_ptr(memory_operand_to_iced(*base, *index, *scale, *displacement)),
                    )
                    .unwrap();
            }

            // ADD R -> R
            ADD(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .add::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            // ADD IMM -> R
            ADD(
                Operand {
                    kind: I(src),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .add::<AsmRegister64, i32>(dst.into(), i32::try_from(*src as i64).unwrap())
                    .unwrap();
            }

            // SUB IMM -> R
            SUB(
                Operand {
                    kind: I(src),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .sub::<AsmRegister64, i32>(dst.into(), i32::try_from(*src).unwrap())
                    .unwrap();
            }
            // SUB IMM -> R
            SUB(
                Operand {
                    kind: I(src),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_32,
                },
            ) => {
                assembler
                    .sub::<AsmRegister32, i32>(dst.into(), i32::try_from(*src).unwrap())
                    .unwrap();
            }
            // SUB IMM -> R: todo remove me
            SUB(
                Operand {
                    kind: I(src),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_8,
                },
            ) => {
                assembler
                    .sub::<AsmRegister8, i32>(dst.into(), i32::try_from(*src).unwrap())
                    .unwrap();
            }
            // SUB R -> R
            SUB(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .sub::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            // SUB IMM -> R
            SUB(
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
                    .sub::<AsmRegister8, i32>(dst.into(), i32::try_from(*src).unwrap())
                    .unwrap();
            }

            // TEST R, R
            TEST(
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
                    .test::<AsmRegister64, AsmRegister64>(left.into(), right.into())
                    .unwrap();
            }

            // TEST R, R
            TEST(
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
                    .test::<AsmRegister8, AsmRegister8>(left.into(), right.into())
                    .unwrap();
            }
            JNE(Operand {
                kind: T(target), ..
            }) => {
                let label = label_map
                    .get(target)
                    .unwrap_or_else(|| panic!("no label for {target:?} found"))
                    .clone();
                assembler.jne(label).unwrap();
            }
            JMP(Operand {
                kind: T(target), ..
            }) => {
                let label = label_map
                    .get(target)
                    .unwrap_or_else(|| panic!("no label for {target:?} found"))
                    .clone();
                assembler.jmp(label.clone()).unwrap();
            }
            RET => {
                assembler.ret().unwrap();
            }
            CMP(
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
                    .cmp::<AsmRegister8, AsmRegister8>(right.into(), left.into())
                    .unwrap();
            }
            CMP(
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
                    .cmp::<AsmRegister32, AsmRegister32>(right.into(), left.into())
                    .unwrap();
            }
            CMP(
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
                    .cmp::<AsmRegister64, AsmRegister64>(right.into(), left.into())
                    .unwrap();
            }
            CMP(
                Operand {
                    kind: I(left),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(right)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .cmp::<AsmRegister64, i32>(right.into(), (*left).try_into().unwrap())
                    .unwrap();
            }
            CMP(
                Operand {
                    kind: I(left),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(right)),
                    width_in_bits: Width::_32,
                },
            ) => {
                assembler
                    .cmp::<AsmRegister32, i32>(right.into(), (*left).try_into().unwrap())
                    .unwrap();
            }
            CMP(
                Operand {
                    kind: I(left),
                    width_in_bits: _,
                },
                Operand {
                    kind: R(PHYS(right)),
                    width_in_bits: Width::_8,
                },
            ) => {
                assembler
                    .cmp::<AsmRegister8, i32>(right.into(), (*left).try_into().unwrap())
                    .unwrap();
            }

            SETA(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.seta::<AsmRegister8>(dst.into()).unwrap();
            }
            SETG(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.setg::<AsmRegister8>(dst.into()).unwrap();
            }
            SETAE(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.setae::<AsmRegister8>(dst.into()).unwrap();
            }
            SETE(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.sete::<AsmRegister8>(dst.into()).unwrap();
            }
            SETO(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.seto::<AsmRegister8>(dst.into()).unwrap();
            }
            SETC(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.setc::<AsmRegister8>(dst.into()).unwrap();
            }
            SETS(Operand {
                kind: R(PHYS(dst)), ..
            }) => {
                assembler.sets::<AsmRegister8>(dst.into()).unwrap();
            }
            NOT(Operand {
                kind: R(PHYS(value)),
                ..
            }) => assembler.not::<AsmRegister64>(value.into()).unwrap(),
            NEG(Operand {
                kind: R(PHYS(value)),
                ..
            }) => assembler.neg::<AsmRegister64>(value.into()).unwrap(),
            SHL(amount, value) => shl::encode(assembler, amount, value),
            SHR(amount, value) => shr::encode(assembler, amount, value),
            SAR(
                Operand {
                    kind: R(PHYS(amount)),
                    width_in_bits: Width::_8,
                },
                Operand {
                    kind: R(PHYS(value)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .sar::<AsmRegister64, AsmRegister8>(value.into(), amount.into())
                    .unwrap();
            }
            // OR I R
            OR(
                Operand {
                    kind: I(left),
                    width_in_bits: Width::_8,
                },
                Operand {
                    kind: R(PHYS(right)),
                    width_in_bits: Width::_8,
                },
            ) => {
                assembler
                    .or::<AsmRegister8, i32>(right.into(), i32::try_from(*left).unwrap())
                    .unwrap();
            }
            // OR I R
            OR(
                Operand {
                    kind: I(left),
                    width_in_bits: Width::_8,
                },
                Operand {
                    kind: R(PHYS(right)),
                    width_in_bits: Width::_32,
                },
            ) => {
                assembler
                    .or::<AsmRegister32, i32>(right.into(), i32::try_from(*left).unwrap())
                    .unwrap();
            }
            // OR R R
            OR(
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
                    .or::<AsmRegister8, AsmRegister8>(right.into(), left.into())
                    .unwrap();
            }
            OR(
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
                    .or::<AsmRegister32, AsmRegister32>(right.into(), left.into())
                    .unwrap();
            }
            OR(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                //assert_eq!(src_width, dst_width);
                assembler
                    .or::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            XOR(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                //assert_eq!(src_width, dst_width);
                assembler
                    .xor::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            XOR(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_8,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_8,
                },
            ) => {
                //assert_eq!(src_width, dst_width);
                assembler
                    .xor::<AsmRegister8, AsmRegister8>(dst.into(), src.into())
                    .unwrap();
            }

            AND(
                Operand { kind: I(left), .. },
                Operand {
                    kind: R(PHYS(right)),
                    ..
                },
            ) => {
                if *left == u64::MAX {
                    // no-op
                } else {
                    if *left > u32::MAX as u64 {
                        panic!("AND immediate too large: {left:x}");
                    }
                    assembler
                        .and::<AsmRegister64, i32>(right.into(), *left as i32)
                        .unwrap();
                }
            }
            AND(
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
            AND(
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
            AND(
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
            BEXTR(
                Operand {
                    kind: R(PHYS(ctrl)),
                    ..
                },
                Operand {
                    kind: R(PHYS(src)), ..
                },
                Operand {
                    kind: R(PHYS(dst)), ..
                },
            ) => {
                assembler
                    .bextr::<AsmRegister64, AsmRegister64, AsmRegister64>(
                        dst.into(),
                        src.into(),
                        ctrl.into(),
                    )
                    .unwrap();
            }

            INT(Operand { kind: I(n), .. }) => {
                assembler.int(i32::try_from(*n).unwrap()).unwrap();
            }

            ADC(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(carry)),
                    ..
                },
            ) => {
                // sets the carry flag
                assembler
                    .add::<AsmRegister8, _>(carry.into(), 0xffff_ffffu32 as i32)
                    .unwrap();

                assembler
                    .adc::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }

            ADC(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: I(carry_in),
                    width_in_bits: Width::_8,
                },
            ) => match carry_in {
                0 => {
                    assembler
                        .add::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                        .unwrap();
                }
                1 => {
                    assembler.stc().unwrap();
                    assembler
                        .adc::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                        .unwrap();
                }
                _ => panic!(),
            },
            PUSH(Operand {
                kind: R(PHYS(src)),
                width_in_bits: Width::_64,
            }) => {
                assembler.push::<AsmRegister64>(src.into()).unwrap();
            }
            POP(Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_64,
            }) => {
                assembler.pop::<AsmRegister64>(dst.into()).unwrap();
            }

            SETB(Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
            }) => {
                assembler.setne::<AsmRegister8>(dst.into()).unwrap();
            }
            SETNZ(Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
            }) => assembler.setnz::<AsmRegister8>(dst.into()).unwrap(),
            SETBE(Operand {
                kind: R(PHYS(dst)),
                width_in_bits: Width::_8,
            }) => {
                assembler.setbe::<AsmRegister8>(dst.into()).unwrap();
            }
            SETNE(dst) => setne::encode(assembler, dst),

            CMOVE(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .cmove::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            CMOVNE(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assembler
                    .cmovne::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                    .unwrap();
            }
            IMUL(
                Operand {
                    kind: I(left),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => assembler
                .imul_3::<AsmRegister64, AsmRegister64, i32>(
                    dst.into(),
                    dst.into(),
                    i32::try_from(*left).unwrap(),
                )
                .unwrap(),
            IMUL(
                Operand {
                    kind: R(PHYS(src)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(dst)),
                    width_in_bits: Width::_64,
                },
            ) => assembler
                .imul_2::<AsmRegister64, AsmRegister64>(dst.into(), src.into())
                .unwrap(),
            IDIV(
                Operand {
                    kind: R(PHYS(hi)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(lo)),
                    width_in_bits: Width::_64,
                },
                Operand {
                    kind: R(PHYS(div)),
                    width_in_bits: Width::_64,
                },
            ) => {
                assert_eq!(*hi, PhysicalRegister::RDX);
                assert_eq!(*lo, PhysicalRegister::RAX);
                assembler.idiv::<AsmRegister64>(div.into()).unwrap();
            }
            NOP => assembler.nop().unwrap(),

            _ => panic!("cannot encode this instruction {}", self),
        }
    }

    pub fn get_operands(
        &mut self,
    ) -> impl Iterator<Item = Option<(OperandDirection, &mut Operand)>> + '_ {
        match &mut self.0 {
            Opcode::MOV(src, dst)
            | Opcode::MOVZX(src, dst)
            | Opcode::MOVSX(src, dst)
            | Opcode::LEA(src, dst)
            | Opcode::CMOVE(src, dst)
            | Opcode::CMOVNE(src, dst) => [
                Some((OperandDirection::In, src)),
                Some((OperandDirection::Out, dst)),
                None,
            ]
            .into_iter(),
            Opcode::SHL(src, dst)
            | Opcode::SHR(src, dst)
            | Opcode::SAR(src, dst)
            | Opcode::OR(src, dst)
            | Opcode::XOR(src, dst)
            | Opcode::ADD(src, dst)
            | Opcode::SUB(src, dst)
            | Opcode::AND(src, dst)
            | Opcode::IMUL(src, dst) => [
                Some((OperandDirection::In, src)),
                Some((OperandDirection::InOut, dst)),
                None,
            ]
            .into_iter(),
            Opcode::IDIV(dividend_hi, dividend_lo, divisor) => [
                Some((OperandDirection::InOut, dividend_hi)),
                Some((OperandDirection::InOut, dividend_lo)),
                Some((OperandDirection::In, divisor)),
            ]
            .into_iter(),
            Opcode::JMP(tgt) => [Some((OperandDirection::None, tgt)), None, None].into_iter(),
            Opcode::RET | Opcode::NOP => [None, None, None].into_iter(),
            Opcode::TEST(op0, op1) | Opcode::CMP(op0, op1) => [
                Some((OperandDirection::In, op0)),
                Some((OperandDirection::In, op1)),
                None,
            ]
            .into_iter(),
            Opcode::JNE(tgt) => [Some((OperandDirection::None, tgt)), None, None].into_iter(),
            Opcode::SETE(r)
            | Opcode::SETNE(r)
            | Opcode::SETNZ(r)
            | Opcode::SETB(r)
            | Opcode::SETBE(r)
            | Opcode::SETA(r)
            | Opcode::SETG(r)
            | Opcode::SETAE(r)
            | Opcode::SETS(r)
            | Opcode::SETO(r)
            | Opcode::SETC(r) => [Some((OperandDirection::Out, r)), None, None].into_iter(),
            Opcode::NOT(r) | Opcode::NEG(r) => {
                [Some((OperandDirection::InOut, r)), None, None].into_iter()
            }

            Opcode::BEXTR(ctrl, src, dst) => [
                Some((OperandDirection::In, ctrl)),
                Some((OperandDirection::In, src)),
                Some((OperandDirection::Out, dst)),
            ]
            .into_iter(),
            Opcode::INT(n) => [Some((OperandDirection::In, n)), None, None].into_iter(),
            Opcode::ADC(a, b, c) => [
                Some((OperandDirection::In, a)),
                Some((OperandDirection::In, b)),
                Some((OperandDirection::InOut, c)),
            ]
            .into_iter(),
            Opcode::PUSH(src) => [Some((OperandDirection::In, src)), None, None].into_iter(),
            Opcode::POP(dest) => [Some((OperandDirection::Out, dest)), None, None].into_iter(),
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
