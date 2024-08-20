use {
    crate::{
        arch::{
            x86::memory::{AlignedAllocator, ExecutableAllocator},
            PAGE_SIZE,
        },
        dbt::{
            emitter::Type,
            x86::{
                encoder::{Instruction, Operand, PhysicalRegister, Register},
                register_allocator::RegisterAllocator,
                Emitter,
            },
        },
    },
    alloc::{rc::Rc, vec::Vec},
    core::{
        cell::RefCell,
        fmt::{Debug, LowerHex},
        panic,
    },
    iced_x86::code_asm::CodeAssembler,
};

pub struct X86Emitter {
    current_block: X86BlockRef,
    next_vreg: usize,
}

impl X86Emitter {
    pub fn new(initial_block: X86BlockRef) -> Self {
        Self {
            current_block: initial_block,
            next_vreg: 0,
        }
    }

    pub fn next_vreg(&mut self) -> usize {
        let vreg = self.next_vreg;
        self.next_vreg += 1;
        vreg
    }
}

impl Emitter for X86Emitter {
    type NodeRef = X86NodeRef;
    type BlockRef = X86BlockRef;

    fn set_current_block(&mut self, block: Self::BlockRef) {
        self.current_block = block;
    }

    fn constant(&mut self, value: u64, typ: Type) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::Constant {
                value,
                width: typ.width,
            },
        })
    }

    fn read_register(&mut self, offset: Self::NodeRef, typ: Type) -> Self::NodeRef {
        match offset.kind() {
            NodeKind::Constant { value, .. } => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::GuestRegister { offset: *value },
            }),
            _ => panic!("can't read non constant offset"),
        }
    }

    fn add(&mut self, lhs: Self::NodeRef, rhs: Self::NodeRef) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: lhs.typ().clone(),
            kind: NodeKind::BinaryOperation {
                kind: BinaryOperationKind::Add(lhs, rhs),
            },
        })
    }

    fn write_register(&mut self, offset: Self::NodeRef, value: Self::NodeRef) {
        let offset = match offset.kind() {
            NodeKind::Constant { value, .. } => (*value).try_into().unwrap(),
            _ => panic!("not supported"),
        };
        let value = value.to_operand(self);

        self.current_block.append(Instruction::mov(
            value,
            Operand::mem_base_displ(
                64,
                Register::PhysicalRegister(PhysicalRegister::RBP),
                offset,
            ),
        ));
    }

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    ) {
        let condition = condition.to_operand(self);
        self.current_block
            .append(Instruction::test(condition.clone(), condition));

        self.current_block
            .append(Instruction::jne(true_target.clone()));
        self.current_block.set_next_0(true_target);

        self.current_block
            .append(Instruction::jmp(false_target.clone()));
        self.current_block.set_next_1(false_target);
    }

    fn jump(&mut self, target: Self::BlockRef) {
        self.current_block.append(Instruction::jmp(target.clone()));
        self.current_block.set_next_0(target);
    }

    fn leave(&mut self) {
        self.current_block.append(Instruction::ret());
    }
}

#[derive(Clone)]
pub struct X86NodeRef(Rc<X86Node>);

impl X86NodeRef {
    pub fn kind(&self) -> &NodeKind {
        &self.0.kind
    }

    pub fn typ(&self) -> &Type {
        &self.0.typ
    }

    pub fn to_operand(&self, emitter: &mut X86Emitter) -> Operand {
        match self.kind() {
            NodeKind::Constant { value, width } => {
                Operand::imm((*width).try_into().unwrap(), *value)
            }
            NodeKind::GuestRegister { offset } => {
                let dst = Operand::vreg(64, emitter.next_vreg());

                emitter.current_block.append(Instruction::mov(
                    Operand::mem_base_displ(
                        64,
                        super::encoder::Register::PhysicalRegister(
                            super::encoder::PhysicalRegister::RBP,
                        ),
                        (*offset).try_into().unwrap(),
                    ),
                    dst.clone(),
                ));

                dst
            }
            NodeKind::BinaryOperation { kind } => {
                let dst = Operand::vreg(64, emitter.next_vreg());

                match kind {
                    BinaryOperationKind::Add(lhs, rhs) => {
                        let lhs = lhs.to_operand(emitter);
                        let rhs = rhs.to_operand(emitter);
                        emitter
                            .current_block
                            .append(Instruction::mov(lhs, dst.clone()));
                        emitter
                            .current_block
                            .append(Instruction::add(rhs, dst.clone()));
                    }
                    _ => todo!(),
                }

                dst
            }
        }
    }
}

impl From<X86Node> for X86NodeRef {
    fn from(node: X86Node) -> Self {
        Self(Rc::new(node))
    }
}

pub struct X86Node {
    pub typ: Type,
    pub kind: NodeKind,
}

pub enum NodeKind {
    Constant { value: u64, width: u16 },
    GuestRegister { offset: u64 },
    BinaryOperation { kind: BinaryOperationKind },
}

pub enum BinaryOperationKind {
    Add(X86NodeRef, X86NodeRef),
    Sub(X86NodeRef, X86NodeRef),
    Multiply(X86NodeRef, X86NodeRef),
    Divide(X86NodeRef, X86NodeRef),
    Modulo(X86NodeRef, X86NodeRef),
    And(X86NodeRef, X86NodeRef),
    Or(X86NodeRef, X86NodeRef),
    Xor(X86NodeRef, X86NodeRef),
    PowI(X86NodeRef, X86NodeRef),
    CompareEqual(X86NodeRef, X86NodeRef),
    CompareNotEqual(X86NodeRef, X86NodeRef),
    CompareLessThan(X86NodeRef, X86NodeRef),
    CompareLessThanOrEqual(X86NodeRef, X86NodeRef),
    CompareGreaterThan(X86NodeRef, X86NodeRef),
    CompareGreaterThanOrEqual(X86NodeRef, X86NodeRef),
}

#[derive(Clone)]
pub struct X86BlockRef(Rc<RefCell<X86Block>>);

impl Eq for X86BlockRef {}

impl PartialEq for X86BlockRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr().eq(&other.0.as_ptr())
    }
}

impl Ord for X86BlockRef {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.as_ptr().cmp(&other.0.as_ptr())
    }
}

impl PartialOrd for X86BlockRef {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.as_ptr().partial_cmp(&other.0.as_ptr())
    }
}

impl LowerHex for X86BlockRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "blockref {:p}", self.0.as_ptr())
    }
}

impl Debug for X86BlockRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "block {:p}:", self.0.as_ptr())?;
        for instr in &self.0.borrow().instructions {
            writeln!(f, "\t{instr:x?}")?;
        }

        Ok(())
    }
}

impl X86BlockRef {
    pub fn append(&self, instruction: Instruction) {
        self.0.borrow_mut().instructions.push(instruction);
    }

    pub fn allocate_registers<R: RegisterAllocator>(&self, allocator: &mut R) {
        self.0
            .borrow_mut()
            .instructions
            .iter_mut()
            .rev()
            .for_each(|i| allocator.process(i));
    }

    pub fn instructions(&self) -> Vec<Instruction> {
        self.0.borrow().instructions.clone()
    }

    /// Host address of the translated machine code block
    pub fn host_address(&self) -> u64 {
        0xffff8000000a82b0
    }

    pub fn get_next_0(&self) -> Option<X86BlockRef> {
        self.0.borrow().next_0.clone()
    }

    pub fn get_next_1(&self) -> Option<X86BlockRef> {
        self.0.borrow().next_1.clone()
    }

    pub fn set_next_0(&self, target: X86BlockRef) {
        self.0.borrow_mut().next_0 = Some(target);
    }

    pub fn set_next_1(&self, target: X86BlockRef) {
        self.0.borrow_mut().next_1 = Some(target);
    }
}

impl From<X86Block> for X86BlockRef {
    fn from(block: X86Block) -> Self {
        Self(Rc::new(RefCell::new(block)))
    }
}

pub struct X86Block {
    instructions: Vec<Instruction>,
    next_0: Option<X86BlockRef>,
    next_1: Option<X86BlockRef>,
}

impl X86Block {
    pub fn new() -> Self {
        Self {
            instructions: alloc::vec![Instruction::label()],
            next_0: None,
            next_1: None,
        }
    }
}
