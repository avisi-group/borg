use {
    crate::dbt::{
        emitter::{BlockResult, Flag, Type, TypeKind},
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
    proc_macro_lib::ktest,
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
        let width = typ.width;
        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::Constant { value, width },
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
            Not(value) => {
                match value.kind() {
                    NodeKind::Constant {
                        value: constant_value,
                        width,
                    } => Self::NodeRef::from(X86Node {
                        typ: value.typ().clone(),
                        kind: NodeKind::Constant {
                            value: constant_value ^ ((1 << *width) - 1), /* only NOT the bits that are part of the size of the datatype */
                            width: *width,
                        },
                    }),
                    _ => Self::NodeRef::from(X86Node {
                        typ: value.typ().clone(),
                        kind: NodeKind::UnaryOperation(op),
                    }),
                }
            }
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
            Sub(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value - rhs_value,
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
            CompareNotEqual(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: if lhs_value != rhs_value { 1 } else { 0 },
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
            CompareLessThan(left, right) => match (left.kind(), right.kind()) {
                (
                    NodeKind::Constant {
                        value: left_value, ..
                    },
                    NodeKind::Constant {
                        value: right_value, ..
                    },
                ) => Self::NodeRef::from(X86Node {
                    typ: left.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if left_value < right_value { 1 } else { 0 },
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
            CompareLessThanOrEqual(left, right) => match (left.kind(), right.kind()) {
                (
                    NodeKind::Constant {
                        value: left_value, ..
                    },
                    NodeKind::Constant {
                        value: right_value, ..
                    },
                ) => Self::NodeRef::from(X86Node {
                    typ: left.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if left_value <= right_value { 1 } else { 0 },
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
            CompareGreaterThanOrEqual(left, right) => match (left.kind(), right.kind()) {
                (
                    NodeKind::Constant {
                        value: left_value, ..
                    },
                    NodeKind::Constant {
                        value: right_value, ..
                    },
                ) => Self::NodeRef::from(X86Node {
                    typ: left.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if left_value >= right_value { 1 } else { 0 },
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
            op => {
                todo!("{op:?}")
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
                    CastOperationKind::SignExtend => {
                        sign_extend(*constant_value, original_width, target_width)
                    }
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
        let typ = value.typ().clone();
        match (value.kind(), start.kind(), length.kind()) {
            (
                NodeKind::Constant { value, .. },
                NodeKind::Constant { value: start, .. },
                NodeKind::Constant { value: length, .. },
            ) => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::Constant {
                    value: bit_extract(*value, *start, *length),
                    width: u16::try_from(*length).unwrap(),
                },
            }),
            _ => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::BitExtract {
                    value,
                    start,
                    length,
                },
            }),
        }
    }

    fn bit_insert(
        &mut self,
        target: Self::NodeRef,
        source: Self::NodeRef,
        start: Self::NodeRef,
        length: Self::NodeRef,
    ) -> Self::NodeRef {
        let typ = target.typ().clone();
        match (target.kind(), source.kind(), start.kind(), length.kind()) {
            (
                NodeKind::Constant { value: target, .. },
                NodeKind::Constant { value: source, .. },
                NodeKind::Constant { value: start, .. },
                NodeKind::Constant { value: length, .. },
            ) => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::Constant {
                    value: bit_insert(*target, *source, *start, *length),
                    width: u16::try_from(*length).unwrap(),
                },
            }),
            _ => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::BitInsert {
                    target,
                    source,
                    start,
                    length,
                },
            }),
        }
    }

    fn select(
        &mut self,
        condition: Self::NodeRef,
        true_value: Self::NodeRef,
        false_value: Self::NodeRef,
    ) -> Self::NodeRef {
        match condition.kind() {
            NodeKind::Constant { value, .. } => {
                if *value == 0 {
                    false_value
                } else {
                    true_value
                }
            }
            _ => todo!(),
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

    fn assert(&mut self, condition: Self::NodeRef) {
        match condition.kind() {
            NodeKind::Constant { value, .. } => assert!(*value != 0),
            _ => {
                // current todo: NOP
                // future todo: emit INTO
            }
        }
    }

    fn mutate_element(
        &mut self,
        vector: Self::NodeRef,
        index: Self::NodeRef,
        value: Self::NodeRef,
    ) -> Self::NodeRef {
        todo!()
    }

    fn get_flag(&mut self, flag: Flag, operation: Self::NodeRef) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 1,
            },
            kind: NodeKind::GetFlag { flag, operation },
        })
    }

    fn panic(&mut self, msg: &'static str) {
        let n = Self::NodeRef::from(X86Node {
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 8,
            },
            kind: NodeKind::Constant {
                value: match msg {
                    "undefined terminator" => 0x50,
                    "default terminator" => 0x51,
                    _ => todo!("{msg}"),
                },
                width: 8,
            },
        })
        .to_operand(self);

        self.current_block.append(Instruction::int(n));
    }
}

fn sign_extend(value: u64, original_width: u16, target_width: u16) -> u64 {
    const CONTAINER_WIDTH: u32 = u64::BITS;

    let original_width = u32::from(original_width);

    let signed_value = value as i64;

    let shifted_left = signed_value
        .checked_shl(CONTAINER_WIDTH - original_width)
        .unwrap_or_else(|| panic!("failed to shift left {value} by 64 - {original_width}"));

    let shifted_right = shifted_left
        .checked_shr(CONTAINER_WIDTH - original_width)
        .unwrap_or_else(|| panic!("failed to shift right {value} by 64 - {target_width}"));

    shifted_right as u64
}

#[ktest]
fn signextend_64() {
    assert_eq!(64, sign_extend(64, 8, 64));
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
                BinaryOperationKind::CompareEqual(left, right)
                | BinaryOperationKind::CompareNotEqual(left, right)
                | BinaryOperationKind::CompareGreaterThan(left, right)
                | BinaryOperationKind::CompareGreaterThanOrEqual(left, right)
                | BinaryOperationKind::CompareLessThan(left, right)
                | BinaryOperationKind::CompareLessThanOrEqual(left, right) => {
                    emit_compare(kind, emitter, left.clone(), right.clone())
                }

                op => todo!("{op:?}"),
            },
            NodeKind::ReadVariable { symbol } => symbol
                .0
                .borrow()
                .as_ref()
                .unwrap()
                .clone()
                .to_operand(emitter),
            NodeKind::UnaryOperation(kind) => match &kind {
                UnaryOperationKind::Not(value) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());
                    let value = value.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(value, dst.clone()));
                    emitter.current_block.append(Instruction::not(dst.clone()));
                    dst
                }
                kind => todo!("{kind:?}"),
            },
            NodeKind::BitExtract {
                value,
                start,
                length,
            } => {
                let src = value.to_operand(emitter);
                let start = start.to_operand(emitter);

                // let length = start[0..8] + length[0..8];
                let control_byte = {
                    let d_vreg = emitter.next_vreg();

                    let l = length.to_operand(emitter);
                    let d = Operand::vreg(64, d_vreg);
                    emitter.current_block.append(Instruction::mov(l, d.clone()));

                    emitter
                        .current_block
                        .append(Instruction::shl(Operand::imm(8, 8), d.clone()));

                    emitter.current_block.append(Instruction::movzx(
                        Operand::vreg(16, d_vreg),
                        Operand::vreg(32, d_vreg),
                    ));

                    emitter
                        .current_block
                        .append(Instruction::or(start, Operand::vreg(32, d_vreg)));

                    d
                };

                let dst = Operand::vreg(64, emitter.next_vreg());

                emitter
                    .current_block
                    .append(Instruction::bextr(control_byte, src, dst.clone()));

                dst
            }
            NodeKind::Cast { value, kind } => {
                let target_width = canonicalize_width(self.typ().width);
                let dst = Operand::vreg(target_width, emitter.next_vreg());
                let src = value.to_operand(emitter);

                if self.typ() == value.typ() {
                    emitter
                        .current_block
                        .append(Instruction::mov(src, dst.clone()));
                } else {
                    match kind {
                        CastOperationKind::ZeroExtend => {
                            if src.width_in_bits == dst.width_in_bits {
                                emitter
                                    .current_block
                                    .append(Instruction::mov(src, dst.clone()));
                            } else {
                                emitter
                                    .current_block
                                    .append(Instruction::movzx(src, dst.clone()));
                            }
                        }
                        CastOperationKind::Convert => {
                            panic!("{:?}\n{:#?}", self.typ(), value);
                        }
                        _ => todo!("{kind:?}: {value:?}"),
                    }
                }

                dst
            }
            NodeKind::Shift {
                value,
                amount,
                kind,
            } => {
                let amount = amount.to_operand(emitter);
                let op0 = value.to_operand(emitter);

                match kind {
                    ShiftOperationKind::LogicalShiftLeft => {
                        emitter
                            .current_block
                            .append(Instruction::shl(amount, op0.clone()));
                    }
                    _ => todo!(),
                }

                op0
            }
            NodeKind::BitInsert {
                target,
                source,
                start,
                length,
            } => todo!(),
            NodeKind::GetFlag { flag, operation } => todo!(),
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
    BitInsert {
        target: X86NodeRef,
        source: X86NodeRef,
        start: X86NodeRef,
        length: X86NodeRef,
    },
    GetFlag {
        flag: Flag,
        operation: X86NodeRef,
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

#[derive(Debug)]
pub enum TernaryOperationKind {
    AddWithCarry(X86NodeRef, X86NodeRef, X86NodeRef),
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

fn emit_compare(
    kind: &BinaryOperationKind,
    emitter: &mut X86Emitter,
    left: X86NodeRef,
    right: X86NodeRef,
) -> Operand {
    use crate::dbt::x86::encoder::OperandKind::*;
    let left = left.to_operand(emitter);
    let right = right.to_operand(emitter);

    // only valid compare instructions are (source-destination):
    // reg reg
    // reg mem
    // mem reg
    // imm reg
    // imm mem

    // anything else (imm on the right) must be reworked

    match (&left.kind, &right.kind) {
        (Register(_), Register(_))
        | (Register(_), Memory { .. })
        | (Memory { .. }, Register(_))
        | (Immediate(_), Register(_))
        | (Immediate(_), Memory { .. })
        | (Memory { .. }, Memory { .. }) => {
            let left = if let (Memory { .. }, Memory { .. }) = (&left.kind, &right.kind) {
                let loaded_left = Operand::vreg(64, emitter.next_vreg());
                emitter
                    .current_block
                    .append(Instruction::mov(left, loaded_left.clone()));
                loaded_left
            } else {
                left
            };

            emitter.current_block.append(Instruction::cmp(left, right));
            let dst = Operand::vreg(64, emitter.next_vreg());

            emitter.current_block.append(match kind {
                BinaryOperationKind::CompareEqual(_, _) => Instruction::sete(dst.clone()),
                BinaryOperationKind::CompareLessThan(_, _) => Instruction::setb(dst.clone()),
                BinaryOperationKind::CompareNotEqual(_, _) => Instruction::setne(dst.clone()),
                BinaryOperationKind::CompareLessThanOrEqual(_, _) => {
                    Instruction::setbe(dst.clone())
                }
                BinaryOperationKind::CompareGreaterThan(_, _) => Instruction::seta(dst.clone()),
                BinaryOperationKind::CompareGreaterThanOrEqual(_, _) => {
                    Instruction::setae(dst.clone())
                }
                _ => panic!("{kind:?} is not a compare"),
            });

            dst
        }

        (Memory { .. }, Immediate(_)) | (Register(_), Immediate(_)) => {
            emitter.current_block.append(Instruction::cmp(right, left));
            let dst = Operand::vreg(64, emitter.next_vreg());

            emitter.current_block.append(match kind {
                BinaryOperationKind::CompareEqual(_, _) => Instruction::sete(dst.clone()),
                BinaryOperationKind::CompareNotEqual(_, _) => Instruction::setne(dst.clone()),
                BinaryOperationKind::CompareLessThan(_, _) => Instruction::setae(dst.clone()),
                BinaryOperationKind::CompareLessThanOrEqual(_, _) => Instruction::seta(dst.clone()),
                BinaryOperationKind::CompareGreaterThan(_, _) => Instruction::setbe(dst.clone()),
                BinaryOperationKind::CompareGreaterThanOrEqual(_, _) => {
                    Instruction::setb(dst.clone())
                }
                _ => panic!("{kind:?} is not a compare"),
            });

            dst
        }

        (Immediate(_), Immediate(_)) => panic!("why was this not const evaluated?"),
        (Target(_), _) | (_, Target(_)) => panic!("why"),
    }

    // BinaryOperationKind::CompareEqual(left, right) => {
    //     let left = left.to_operand(emitter);
    //     let right = right.to_operand(emitter);

    // }
    // BinaryOperationKind::CompareLessThan(left, right) => {
    //     let left = left.to_operand(emitter);
    //     let right = right.to_operand(emitter);

    // }
}

fn canonicalize_width(width: u16) -> u8 {
    match width {
        0..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        33..=64 => 64,
        w => panic!("width {w:?} is greater than 64"),
    }
}

// generate n ones
fn ones(n: u64) -> u64 {
    let (res, overflowed) = 1u64.overflowing_shl(n.try_into().unwrap());

    if overflowed {
        if n == u64::from(u64::BITS) {
            u64::MAX
        } else {
            panic!("overflowed while generating mask of {n} 1s")
        }
    } else {
        res - 1
    }
}

fn bit_insert(target: u64, source: u64, start: u64, length: u64) -> u64 {
    let cleared_target = {
        let mask = !(ones(length) << start);
        target & mask
    };

    let shifted_source = {
        let mask = ones(length);
        let masked_source = source & mask;
        masked_source << start
    };

    cleared_target | shifted_source
}

fn bit_extract(value: u64, start: u64, length: u64) -> u64 {
    (value >> start) & ones(length)
}

// #[cfg(test)]
// mod tests {
//     use {
//         super::{bit_extract, bit_insert, ones},
//         proptest::prelude::*,
//     };

//     #[test]
//     fn ones_smoke() {
//         assert_eq!(0, ones(0));
//         assert_eq!(1, ones(1));
//         assert_eq!(0b111, ones(3));
//         assert_eq!(u32::MAX as u64, ones(u32::BITS as u64));
//         assert_eq!(u64::MAX, ones(u64::BITS as u64));
//     }

//     proptest! {
//         #[test]
//         fn ones_extract(start in 0u64..64, length in 0u64..64) {
//             if start + length <= 64 {
//                 // put some ones somewhere
//                 let value = ones(length) << start;
//                 // extract them out
//                 let extracted = bit_extract(value, start, length);

//                 // check it is equal
//                 assert_eq!(extracted, ones(length))
//             }
//         }

//         #[test]
//         fn bit_insert_extract_prop( target: u64,source: u64, start in
// 0u64..64, length in 0u64..64) {             if start + length <= 64 {
//                 // insert source into target
//                 let inserted = bit_insert(target, source, start, length);
//                 // extract it back out
//                 let extracted = bit_extract(inserted, start, length);

//                 // check it is equal
//                 assert_eq!(extracted, source & ((1 << length) - 1))
//             }
//         }
//     }
// }
