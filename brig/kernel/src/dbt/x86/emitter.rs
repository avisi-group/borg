use {
    crate::dbt::{
        emitter::{BlockResult, Type},
        x86::{
            encoder::{Instruction, Operand, OperandKind, PhysicalRegister, Register},
            register_allocator::RegisterAllocator,
            Emitter,
        },
    },
    alloc::{rc::Rc, vec::Vec},
    common::mask::mask,
    core::{
        cell::RefCell,
        fmt::{Debug, LowerHex},
        hash::{Hash, Hasher},
        panic,
    },
    proc_macro_lib::ktest,
};
pub struct X86Emitter {
    current_block: X86BlockRef,
    panic_block: X86BlockRef,
    next_vreg: usize,
}

impl X86Emitter {
    pub fn new(initial_block: X86BlockRef, panic_block: X86BlockRef) -> Self {
        Self {
            current_block: initial_block,
            panic_block,
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
        let width = typ.width();
        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::Constant { value, width },
        })
    }

    // may not return a bits if `length` is a constant?
    fn create_bits(&mut self, value: Self::NodeRef, length: Self::NodeRef) -> Self::NodeRef {
        // evil bits that's really a fixed unsigned pretending to be a bitvector
        if let NodeKind::Constant { value: length, .. } = length.kind() {
            let length = u16::try_from(*length).unwrap();
            let target_type = match value.typ() {
                Type::Unsigned(_) => Type::Unsigned(length),
                Type::Signed(_) => Type::Signed(length),
                _ => todo!(),
            };

            self.cast(value, target_type, CastOperationKind::Truncate)
        } else {
            // todo: attach length information
            value
        }
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
                        value: (*constant_value == 0) as u64,
                        width: *width,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: value.typ().clone(),
                    kind: NodeKind::UnaryOperation(op),
                }),
            },
            Complement(value) => {
                match value.kind() {
                    NodeKind::Constant {
                        value: constant_value,
                        width,
                    } => Self::NodeRef::from(X86Node {
                        typ: value.typ().clone(),
                        kind: NodeKind::Constant {
                            value: (!constant_value) & mask(*width), /* only invert the bits that
                                                                      * are
                                                                      * part of the size of the
                                                                      * datatype */
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
                todo!("{op:?}")
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
            Multiply(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value * rhs_value,
                        width: *width,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            Or(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value | rhs_value,
                        width: *width,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            And(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value & rhs_value,
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
                    typ: Type::Unsigned(1),
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
                    typ: Type::Unsigned(1),
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
                    typ: Type::Unsigned(1),
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
                    typ: Type::Unsigned(1),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            CompareGreaterThan(left, right) => match (left.kind(), right.kind()) {
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
                        value: if left_value > right_value { 1 } else { 0 },
                        width: 1,
                    },
                }),
                _ => Self::NodeRef::from(X86Node {
                    typ: Type::Unsigned(1),
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
                    typ: Type::Unsigned(1),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            op => {
                todo!("{op:?}")
            }
        }
    }

    fn ternary_operation(&mut self, op: TernaryOperationKind) -> Self::NodeRef {
        use TernaryOperationKind::*;
        match &op {
            AddWithCarry(src, _dst, _carry) => Self::NodeRef::from(X86Node {
                typ: src.typ().clone(),
                kind: NodeKind::TernaryOperation(op),
            }),
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
                let original_width = value.typ().width();
                let target_width = target_type.width();

                let casted_value = match cast_kind {
                    CastOperationKind::ZeroExtend => {
                        if original_width == 64 {
                            *constant_value
                        } else {
                            // extending from the incoming value type - so can clear
                            // all upper bits.
                            let mask = mask(original_width);
                            *constant_value & mask
                        }
                    }
                    CastOperationKind::SignExtend => {
                        sign_extend(*constant_value, original_width, target_width)
                    }
                    CastOperationKind::Truncate => {
                        // truncating to the target width - just clear all irrelevant bits
                        let mask = mask(target_width);
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
                        width: target_type.width(),
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
        let typ = value.typ().clone();
        match (value.kind(), amount.kind()) {
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: amount_value,
                    ..
                },
            ) => {
                // mask to width of value
                self.constant((value_value << amount_value) & mask(*value_width), typ)
            }
            (_, _) => Self::NodeRef::from(X86Node {
                typ,
                kind: NodeKind::Shift {
                    value,
                    amount,
                    kind,
                },
            }),
        }
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
            // todo: constant start and length with non-constant value can still be specialized?
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
            _ => Self::NodeRef::from(X86Node {
                typ: true_value.typ().clone(),
                kind: NodeKind::Select {
                    condition,
                    true_value,
                    false_value,
                },
            }),
        }
    }

    fn write_register(&mut self, offset: Self::NodeRef, value: Self::NodeRef) {
        let offset = match offset.kind() {
            NodeKind::Constant { value, .. } => (*value).try_into().unwrap(),
            _ => panic!("register's have constant offsets"),
        };

        // todo: validate offset + width is within register file

        let value = value.to_operand(self);

        let width = value.width_in_bits;

        self.current_block.append(Instruction::mov(
            value,
            Operand::mem_base_displ(
                width,
                Register::PhysicalRegister(PhysicalRegister::R15),
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
        self.current_block.append(Instruction::jmp(target.clone()));
        self.current_block.set_next_0(target.clone());
        BlockResult::Static(target)
    }

    fn leave(&mut self) {
        self.current_block.append(Instruction::ret());
    }

    fn read_variable(&mut self, offset: usize, typ: Type) -> Self::NodeRef {
        let width = u8::try_from(typ.width()).unwrap();

        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::ReadStackVariable { offset, width },
        })
    }

    fn write_variable(&mut self, offset: usize, value: Self::NodeRef) {
        let value = value.to_operand(self);

        let width = value.width_in_bits;

        let mem = Operand::mem_base_displ(
            width,
            Register::PhysicalRegister(PhysicalRegister::RBP),
            -(i32::try_from(offset).unwrap()),
        );

        self.current_block.append(Instruction::mov(value, mem));
    }

    fn assert(&mut self, condition: Self::NodeRef) {
        match condition.kind() {
            NodeKind::Constant { value, .. } => {
                if *value == 0 {
                    self.panic("constant assert failed");
                }
            }
            _ => {
                let not_condition = self.unary_operation(UnaryOperationKind::Not(condition));
                let op = not_condition.to_operand(self);

                self.current_block.append(Instruction::test(op.clone(), op));
                self.current_block
                    .append(Instruction::jne(self.panic_block.clone()));
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

    // returns a tuple of (operation_result, flags)
    fn get_flags(&mut self) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: Type::Unsigned(4),
            kind: NodeKind::GetFlags,
        })
    }

    fn panic(&mut self, msg: &str) {
        let n = Self::NodeRef::from(X86Node {
            typ: Type::Unsigned(8),
            kind: NodeKind::Constant {
                value: match msg {
                    "undefined terminator" => 0x50,
                    "default terminator" => 0x51,
                    "constant assert failed" => 0x52,
                    "panic block" => 0x53,
                    _ => todo!("{msg}"),
                },
                width: 8,
            },
        })
        .to_operand(self);

        self.current_block.append(Instruction::int(n));
    }

    fn create_tuple(&mut self, values: Vec<Self::NodeRef>) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: Type::Tuple,
            kind: NodeKind::Tuple(values),
        })
    }

    fn access_tuple(&mut self, tuple: Self::NodeRef, index: usize) -> Self::NodeRef {
        let NodeKind::Tuple(values) = tuple.kind() else {
            unreachable!()
        };

        values[index].clone()
    }

    fn size_of(&mut self, value: Self::NodeRef) -> Self::NodeRef {
        match value.typ() {
            Type::Unsigned(w) | Type::Signed(w) | Type::Floating(w) => {
                self.constant(u64::from(*w), Type::Unsigned(16))
            }

            Type::Bits => {
                if let NodeKind::Constant { width, .. } = value.kind() {
                    self.constant(u64::from(*width), Type::Unsigned(16))
                } else {
                    todo!("size of {value:#?}")
                }
            }
            Type::Tuple => todo!(),
        }
    }

    fn bits_cast(
        &mut self,
        value: Self::NodeRef,
        length: Self::NodeRef,
        typ: Type,
        kind: CastOperationKind,
    ) -> Self::NodeRef {
        match (value.kind(), length.kind()) {
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: length_value,
                    width: length_width,
                },
            ) => {
                let length = u16::try_from(*length_value).unwrap();

                let typ = match value.typ() {
                    Type::Unsigned(_) | Type::Bits => Type::Unsigned(length),
                    Type::Signed(_) => Type::Signed(length),
                    _ => todo!(),
                };

                self.constant(*value_value, typ)
            }
            (
                _,
                NodeKind::Constant {
                    value: length_value,
                    width: length_width,
                },
            ) => {
                let length = u16::try_from(*length_value).unwrap();

                let typ = match value.typ() {
                    Type::Unsigned(_) => Type::Unsigned(length),
                    Type::Signed(_) => Type::Signed(length),
                    _ => todo!(),
                };

                self.cast(value, typ, kind)
            }
            (_, _) => {
                // todo: attach length information
                value
            }
        }
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
                        Register::PhysicalRegister(PhysicalRegister::R15),
                        (*offset).try_into().unwrap(),
                    ),
                    dst.clone(),
                ));

                dst
            }
            NodeKind::ReadStackVariable { offset, width } => {
                let dst = Operand::vreg(*width, emitter.next_vreg());

                emitter.current_block.append(Instruction::mov(
                    Operand::mem_base_displ(
                        *width,
                        Register::PhysicalRegister(PhysicalRegister::RBP),
                        -(i32::try_from(*offset).unwrap()),
                    ),
                    dst.clone(),
                ));

                dst
            }
            NodeKind::BinaryOperation(kind) => match kind {
                BinaryOperationKind::Add(left, right) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let left = left.to_operand(emitter);
                    let right = right.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(left, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::add(right, dst.clone()));

                    dst
                }
                BinaryOperationKind::Sub(left, right) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let left = left.to_operand(emitter);
                    let right = right.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(left, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::sub(right, dst.clone()));

                    dst
                }
                BinaryOperationKind::Or(left, right) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let left = left.to_operand(emitter);
                    let right = right.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(left, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::or(right, dst.clone()));

                    dst
                }
                BinaryOperationKind::And(left, right) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let left = left.to_operand(emitter);
                    let right = right.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(left, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::and(right, dst.clone()));

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
            NodeKind::TernaryOperation(kind) => match kind {
                TernaryOperationKind::AddWithCarry(a, b, carry) => {
                    let dst = Operand::vreg(64, emitter.next_vreg());

                    let a = a.to_operand(emitter);
                    let b = b.to_operand(emitter);
                    let carry = carry.to_operand(emitter);
                    emitter
                        .current_block
                        .append(Instruction::mov(b.clone(), dst.clone()));

                    emitter
                        .current_block
                        .append(Instruction::adc(a, dst.clone(), carry));

                    dst
                }
            },
            NodeKind::ReadVariable { symbol } => symbol
                .0
                .borrow()
                .as_ref()
                .unwrap()
                .clone()
                .to_operand(emitter),
            NodeKind::UnaryOperation(kind) => match &kind {
                // todo: not might be wrong here (output 0 or 1?)
                UnaryOperationKind::Complement(value) | UnaryOperationKind::Not(value) => {
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
                let value = value.to_operand(emitter);
                let start = start.to_operand(emitter);
                let length = length.to_operand(emitter);

                //  start[0..8] ++ length[0..8];
                let control_byte = {
                    let mask = Operand::imm(64, 0xff);

                    let start = {
                        let dst = Operand::vreg(64, emitter.next_vreg());
                        emitter
                            .current_block
                            .append(Instruction::mov(start, dst.clone()));
                        emitter
                            .current_block
                            .append(Instruction::and(mask.clone(), dst.clone()));
                        dst
                    };

                    let length = {
                        let dst = Operand::vreg(64, emitter.next_vreg());
                        emitter
                            .current_block
                            .append(Instruction::mov(length, dst.clone()));
                        emitter
                            .current_block
                            .append(Instruction::and(mask.clone(), dst.clone()));
                        emitter
                            .current_block
                            .append(Instruction::shl(Operand::imm(8, 8), dst.clone()));
                        dst
                    };

                    let dst = Operand::vreg(64, emitter.next_vreg());

                    emitter
                        .current_block
                        .append(Instruction::mov(start, dst.clone()));
                    emitter
                        .current_block
                        .append(Instruction::or(length, dst.clone()));

                    dst
                };

                let dst = Operand::vreg(64, emitter.next_vreg());

                emitter
                    .current_block
                    .append(Instruction::bextr(control_byte, value, dst.clone()));

                dst
            }
            NodeKind::Cast { value, kind } => {
                let target_width = canonicalize_width(self.typ().width());
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
                        CastOperationKind::SignExtend => {
                            if src.width_in_bits == dst.width_in_bits {
                                emitter
                                    .current_block
                                    .append(Instruction::mov(src, dst.clone()));
                            } else {
                                emitter
                                    .current_block
                                    .append(Instruction::movsx(src, dst.clone()));
                            }
                        }
                        CastOperationKind::Convert => {
                            panic!("{:?}\n{:#?}", self.typ(), value);
                        }
                        CastOperationKind::Truncate => {
                            assert!(src.width_in_bits > dst.width_in_bits);
                            emitter
                                .current_block
                                .append(Instruction::mov(src, dst.clone()));
                        }

                        CastOperationKind::Reinterpret => {
                            // todo: actually reinterpret and fix the following:
                            // if src.width_in_bits != dst.width_in_bits {
                            //     panic!("failed to reinterpret\n{value:#?}\n as {:?}",
                            // self.typ()); }
                            emitter
                                .current_block
                                .append(Instruction::mov(src, dst.clone()));
                        }
                        _ => todo!("{kind:?} to {:?}\n{value:#?}", self.typ()),
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

                    ShiftOperationKind::LogicalShiftRight => {
                        emitter
                            .current_block
                            .append(Instruction::shr(amount, op0.clone()));
                    }

                    ShiftOperationKind::ArithmeticShiftRight => {
                        emitter
                            .current_block
                            .append(Instruction::sar(amount, op0.clone()));
                    }
                    _ => todo!("{kind:?}"),
                }

                op0
            }
            NodeKind::BitInsert {
                target,
                source,
                start,
                length,
            } => {
                // todo: test this
                let target = target.to_operand(emitter);
                let source = source.to_operand(emitter);
                let start = start.to_operand(emitter);
                let length = length.to_operand(emitter);

                let mask = Operand::vreg(64, emitter.next_vreg());
                emitter
                    .current_block
                    .append(Instruction::mov(Operand::imm(64, 1), mask.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(length.clone(), mask.clone()));
                emitter
                    .current_block
                    .append(Instruction::sub(Operand::imm(64, 1), mask.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(start, mask.clone()));

                let masked_target = Operand::vreg(64, emitter.next_vreg());
                emitter
                    .current_block
                    .append(Instruction::mov(target, masked_target.clone()));
                emitter
                    .current_block
                    .append(Instruction::and(mask.clone(), masked_target.clone()));

                let shifted_source = Operand::vreg(64, emitter.next_vreg());
                emitter
                    .current_block
                    .append(Instruction::mov(source, shifted_source.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(length.clone(), shifted_source.clone()));

                {
                    // not strictly necessary but may avoid issues if there is junk data
                    let invert_mask = Operand::vreg(64, emitter.next_vreg());
                    emitter
                        .current_block
                        .append(Instruction::mov(mask.clone(), invert_mask.clone()));
                    emitter
                        .current_block
                        .append(Instruction::not(invert_mask.clone()));
                    emitter.current_block.append(Instruction::and(
                        invert_mask.clone(),
                        shifted_source.clone(),
                    ));
                }

                // mask off target bits
                // shift source by start
                // apply ~mask to source
                // OR source and target
                emitter.current_block.append(Instruction::or(
                    shifted_source.clone(),
                    masked_target.clone(),
                ));

                masked_target
            }
            NodeKind::GetFlags => {
                //  todo: assert that the operation comes right before this emitted instruction

                let n = Operand::vreg(8, emitter.next_vreg());
                let z = Operand::vreg(8, emitter.next_vreg());
                let c = Operand::vreg(8, emitter.next_vreg());
                let v = Operand::vreg(8, emitter.next_vreg());
                let dest = Operand::vreg(8, emitter.next_vreg());

                emitter.current_block.append(Instruction::sets(n.clone()));
                emitter.current_block.append(Instruction::sete(z.clone()));
                emitter.current_block.append(Instruction::setc(c.clone()));
                emitter.current_block.append(Instruction::seto(v.clone()));

                emitter
                    .current_block
                    .append(Instruction::xor(dest.clone(), dest.clone()));

                emitter
                    .current_block
                    .append(Instruction::or(n.clone(), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(Operand::imm(1, 1), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::or(z.clone(), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(Operand::imm(1, 1), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::or(c.clone(), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::shl(Operand::imm(1, 1), dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::or(v.clone(), dest.clone()));
                // nzcv
                dest
            }
            NodeKind::Tuple(vec) => unreachable!(),
            NodeKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                let dest = Operand::vreg(
                    u8::try_from(true_value.typ().width()).unwrap(),
                    emitter.next_vreg(),
                );

                let condition = condition.to_operand(emitter);
                let true_value = true_value.to_operand(emitter);
                let false_value = false_value.to_operand(emitter);

                emitter
                    .current_block
                    .append(Instruction::test(condition.clone(), condition.clone()));
                emitter
                    .current_block
                    .append(Instruction::cmove(false_value, dest.clone()));
                emitter
                    .current_block
                    .append(Instruction::cmovne(true_value, dest.clone()));

                dest
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
    TernaryOperation(TernaryOperationKind),
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
    ReadStackVariable {
        // positive offset here (will be subtracted from RBP)
        offset: usize,
        width: u8,
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
    GetFlags,
    Tuple(Vec<X86NodeRef>),
    Select {
        condition: X86NodeRef,
        true_value: X86NodeRef,
        false_value: X86NodeRef,
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
            writeln!(f, "\t{instr}")?;
        }

        Ok(())
    }
}

impl Hash for X86BlockRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0.as_ptr() as usize).hash(state)
    }
}

impl X86BlockRef {
    pub fn id(&self) -> i32 {
        self.0.borrow().id
    }

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
    id: i32,
    instructions: Vec<Instruction>,
    next_0: Option<X86BlockRef>,
    next_1: Option<X86BlockRef>,
}

impl X86Block {
    pub fn new(id: i32) -> Self {
        Self {
            id,
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

    let dst = match (&left.kind, &right.kind) {
        (Register(_), Register(_))
        | (Register(_), Memory { .. })
        | (Memory { .. }, Register(_))
        | (Immediate(_), Register(_))
        | (Immediate(_), Memory { .. })
        | (Memory { .. }, Memory { .. }) => {
            let left = if let (Memory { .. }, Memory { .. }) = (&left.kind, &right.kind) {
                let dst = Operand::vreg(64, emitter.next_vreg());
                emitter
                    .current_block
                    .append(Instruction::mov(left, dst.clone()));
                dst
            } else {
                left
            };

            emitter.current_block.append(Instruction::cmp(left, right));

            // setCC only sets the lowest bit

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
    };

    // setCC instructions only set the least significant byte to 0x00 or 0x01, we
    // need to clear or set the other 63 bits
    emitter.current_block.append(Instruction::and(
        Operand {
            kind: OperandKind::Immediate(0b1),
            width_in_bits: 64,
        },
        dst.clone(),
    ));
    emitter.current_block.append(Instruction::neg(dst.clone()));

    dst

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
