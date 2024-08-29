use {
    crate::dbt::{
        emitter::{BlockResult, Type, TypeKind},
        x86::{
            encoder::{Instruction, Operand, PhysicalRegister, Register},
            register_allocator::RegisterAllocator,
            Emitter,
        },
    },
    alloc::{rc::Rc, vec::Vec},
    core::{
        cell::RefCell,
        fmt::{Debug, LowerHex},
        panic,
    },
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
    type SymbolRef = X86SymbolRef;

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

    fn unary_operation(&mut self, op: UnaryOperationKind) -> Self::NodeRef {
        use UnaryOperationKind::*;

        match &op {
            Not(value) => match value.kind() {
                NodeKind::Constant {
                    value: constant_value,
                    width,
                } => Self::NodeRef::from(X86Node {
                    typ: value.typ().clone(),
                    kind: NodeKind::Constant {
                        value: constant_value ^ ((1 << *width) - 1),        // only NOT the bits that are part of the size of the datatype
                        width: *width,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: value.typ().clone(),
                    kind: NodeKind::UnaryOperation(op),
                }),
            },
            _ => {
                todo!()
            }
        }
    }

    fn binary_operation(&mut self, op: BinaryOperationKind) -> Self::NodeRef {
        use BinaryOperationKind::*;

        match &op {
            Add(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
                (
                    NodeKind::Constant {
                        value: lhs_value,
                        width,
                    },
                    NodeKind::Constant {
                        value: rhs_value, ..
                    },
                ) => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value + rhs_value,
                        width: *width,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            CompareEqual(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
                (
                    NodeKind::Constant {
                        value: lhs_value, ..
                    },
                    NodeKind::Constant {
                        value: rhs_value, ..
                    },
                ) => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if lhs_value == rhs_value { 1 } else { 0 },
                        width: 1,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: Type {
                        kind: TypeKind::Unsigned,
                        width: 1,
                    },
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            _ => {
                todo!()
            }
        }
    }

    fn cast(
        &mut self,
        value: Self::NodeRef,
        target_type: Type,
        cast_kind: CastOperationKind,
    ) -> Self::NodeRef {
        match value.kind() {
            NodeKind::Constant {
                value: constant_value,
                ..
            } => {
                let original_width = value.typ().width;
                let target_width = target_type.width;

                let casted_value = match cast_kind {
                    CastOperationKind::ZeroExtend => {
                        if original_width == 64 {
                            *constant_value
                        } else {
                            // extending from the incoming value type - so can clear
                            // all upper bits.
                            let mask = (1 << original_width) - 1;
                            *constant_value & mask
                        }
                    }
                    CastOperationKind::SignExtend => *constant_value << (64 - original_width) >> 5,
                    CastOperationKind::Truncate => {
                        // truncating to the target width - just clear all irrelevant bits
                        let mask = (1 << target_width) - 1;
                        *constant_value & mask
                    }
                    CastOperationKind::Reinterpret => *constant_value,
                    CastOperationKind::Convert => *constant_value,
                    CastOperationKind::Broadcast => *constant_value,
                };

                Self::NodeRef::from(X86Node {
                    typ: target_type.clone(),
                    kind: NodeKind::Constant {
                        value: casted_value,
                        width: target_type.width,
                    },
                })
            }
            _ => Self::NodeRef::from(X86Node {
                typ: target_type,
                kind: NodeKind::Cast {
                    value,
                    kind: cast_kind,
                },
            }),
        }
    }

    fn shift(
        &mut self,
        value: Self::NodeRef,
        amount: Self::NodeRef,
        kind: ShiftOperationKind,
    ) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: value.typ().clone(),
            kind: NodeKind::Shift {
                value,
                amount,
                kind,
            },
        })
    }

    fn bit_extract(
        &mut self,
        value: Self::NodeRef,
        start: Self::NodeRef,
        length: Self::NodeRef,
    ) -> Self::NodeRef {
        match (value.kind(), start.kind(), length.kind()) {
            (
                NodeKind::Constant {
                    value: value_value, ..
                },
                NodeKind::Constant {
                    value: start_value, ..
                },
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
            ) => Self::NodeRef::from(X86Node {
                typ: value.typ().clone(),
                kind: NodeKind::Constant {
                    value: (value_value >> start_value) & ((1 << length_value) - 1),
                    width: *length_value as u16,
                },
            }),
            _ => Self::NodeRef::from(X86Node {
                typ: value.typ().clone(),
                kind: NodeKind::BitExtract {
                    value,
                    start,
                    length,
                },
            }),
        }
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
    ) -> BlockResult {
        match condition.kind() {
            NodeKind::Constant { value, .. } => {
                if *value == 0 {
                    self.current_block.set_next_0(false_target.clone());
                    BlockResult::Static(false_target)
                } else {
                    self.current_block.set_next_0(true_target.clone());
                    BlockResult::Static(true_target)
                }
            }
            _ => {
                let condition = condition.to_operand(self);

                self.current_block
                    .append(Instruction::test(condition.clone(), condition));

                self.current_block
                    .append(Instruction::jne(true_target.clone()));
                self.current_block.set_next_0(true_target.clone());

                self.current_block
                    .append(Instruction::jmp(false_target.clone()));
                self.current_block.set_next_1(false_target.clone());

                // if condition is static, return BlockResult::Static
                // else
                BlockResult::Dynamic(true_target, false_target)
            }
        }
    }

    fn jump(&mut self, target: Self::BlockRef) -> BlockResult {
        //self.current_block.append(Instruction::jmp(target.clone()));
        self.current_block.set_next_0(target.clone());
        BlockResult::Static(target)
    }

    fn leave(&mut self) {
        self.current_block.append(Instruction::ret());
    }

    fn read_variable(&mut self, symbol: Self::SymbolRef) -> Self::NodeRef {
        symbol.0.borrow().as_ref().unwrap().clone()
    }

    fn write_variable(&mut self, symbol: Self::SymbolRef, value: Self::NodeRef) {
        *symbol.0.borrow_mut() = Some(value);
    }
}

#[derive(Debug, Clone)]
pub struct X86NodeRef(Rc<X86Node>);

impl X86NodeRef {
    pub fn kind(&self) -> &NodeKind {
        &self.0.kind
    }

    pub fn typ(&self) -> &Type {
        &self.0.typ
    }

    fn to_operand(&self, emitter: &mut X86Emitter) -> Operand {
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
            NodeKind::BinaryOperation(kind) => match kind {
                BinaryOperationKind::Add(lhs, rhs) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let lhs = lhs.to_operand(emitter);
                    let rhs = rhs.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(lhs, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::add(rhs, dst.clone()));

                    dst
                }
                BinaryOperationKind::CompareEqual(left, right) => {
                    let left = left.to_operand(emitter);
                    let right = right.to_operand(emitter);
                    emitter.current_block.append(Instruction::cmp(left, right));

                    let dst = Operand::vreg(64, emitter.next_vreg());
                    emitter.current_block.append(Instruction::sete(dst.clone()));

                    dst
                }
                _ => todo!(),
            },
            NodeKind::ReadVariable { symbol } => symbol
                .0
                .borrow()
                .as_ref()
                .unwrap()
                .clone()
                .to_operand(emitter),
            NodeKind::UnaryOperation(kind) => match &kind {
                _ => {
                    todo!()
                }
            },
            NodeKind::BitExtract {
                value: _value,
                start: _start,
                length: _length,
            } => {
                todo!()
            }
            NodeKind::Cast {
                value: _value,
                kind: _cast_kind,
            } => {
                /*let dst = Operand::vreg(64, emitter.next_vreg());

                let src = value.to_operand(emitter);
                emitter
                    .current_block
                    .append(Instruction::mov(src, dst.clone()));

                dst*/

                todo!()
            }
            NodeKind::Shift {
                value: _value,
                amount: _shift_amount,
                kind: _shift_kind,
            } => {
                todo!()
            }
        }
    }
}

impl From<X86Node> for X86NodeRef {
    fn from(node: X86Node) -> Self {
        Self(Rc::new(node))
    }
}

#[derive(Debug)]
pub struct X86Node {
    pub typ: Type,
    pub kind: NodeKind,
}

#[derive(Debug)]
pub enum NodeKind {
    Constant {
        value: u64,
        width: u16,
    },
    GuestRegister {
        offset: u64,
    },
    UnaryOperation(UnaryOperationKind),
    BinaryOperation(BinaryOperationKind),
    Cast {
        value: X86NodeRef,
        kind: CastOperationKind,
    },
    Shift {
        value: X86NodeRef,
        amount: X86NodeRef,
        kind: ShiftOperationKind,
    },
    ReadVariable {
        symbol: X86SymbolRef,
    },
    BitExtract {
        value: X86NodeRef,
        start: X86NodeRef,
        length: X86NodeRef,
    },
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOperationKind {
    Not(X86NodeRef),
    Negate(X86NodeRef),
    Complement(X86NodeRef),
    Power2(X86NodeRef),
    Absolute(X86NodeRef),
    Ceil(X86NodeRef),
    Floor(X86NodeRef),
    SquareRoot(X86NodeRef),
}

#[derive(Debug, Clone)]
pub enum CastOperationKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    Reinterpret,
    Convert,
    Broadcast,
}

#[derive(Debug, Clone)]
pub enum ShiftOperationKind {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    RotateRight,
    RotateLeft,
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
            instructions: alloc::vec![],
            next_0: None,
            next_1: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct X86SymbolRef(pub Rc<RefCell<Option<X86NodeRef>>>);
