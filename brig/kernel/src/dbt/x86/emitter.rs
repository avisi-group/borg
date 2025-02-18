use {
    crate::dbt::{
        bit_extract, bit_insert,
        emitter::{BlockResult, Type},
        x86::{
            encoder::{
                width::Width, Instruction, Opcode, Operand, OperandKind, PhysicalRegister, Register,
            },
            register_allocator::RegisterAllocator,
            Emitter, X86TranslationContext,
        },
    },
    alloc::{rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        mask::mask,
        HashMap,
    },
    core::{
        cell::RefCell,
        cmp::Ordering,
        fmt::Debug,
        hash::{Hash, Hasher},
        panic,
    },
    proc_macro_lib::ktest,
};

const INVALID_OFFSET: i32 = 0xDEAD00F;

/// X86 emitter error
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum X86Error {
    ///  Left and right types do not match in binary operation: {op:?}
    BinaryOperationTypeMismatch { op: BinaryOperationKind },
    /// Register allocation failed
    RegisterAllocation,
}

pub struct X86Emitter<'ctx> {
    current_block: Ref<X86Block>,
    current_block_operands: HashMap<X86NodeRef, Operand>,
    panic_block: Ref<X86Block>,
    next_vreg: usize,
    arena: Arena<X86Node>,
    ctx: &'ctx mut X86TranslationContext,
}

impl<'ctx> X86Emitter<'ctx> {
    pub fn new(ctx: &'ctx mut X86TranslationContext) -> Self {
        Self {
            current_block: ctx.initial_block(),
            current_block_operands: HashMap::default(),
            panic_block: ctx.panic_block(),
            next_vreg: 0,
            arena: Arena::new(),
            ctx,
        }
    }

    pub fn ctx(&self) -> &X86TranslationContext {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut X86TranslationContext {
        &mut self.ctx
    }

    pub fn next_vreg(&mut self) -> usize {
        let vreg = self.next_vreg;
        self.next_vreg += 1;
        vreg
    }

    pub fn push_instruction(&mut self, instr: Instruction) {
        self.current_block
            .get_mut(self.ctx.arena_mut())
            .append(instr);
    }

    pub fn push_target(&mut self, target: Ref<X86Block>) {
        log::debug!("adding target {target:?} to {:?}", self.current_block);
        self.current_block
            .get_mut(self.ctx.arena_mut())
            .push_next(target);
    }

    pub fn new_node(&mut self, node: X86Node) -> Ref<X86Node> {
        self.arena.insert(node)
    }

    /// Same as `to_operand` but if the value is a constant, move it to a
    /// register
    fn to_operand_reg_promote(&mut self, node: &X86NodeRef) -> Operand {
        if let NodeKind::Constant { .. } = node.kind() {
            let width = Width::from_uncanonicalized(node.typ().width()).unwrap();
            let value_reg = Operand::vreg(width, self.next_vreg());
            let value_imm = self.to_operand(node);
            self.push_instruction(Instruction::mov(value_imm, value_reg).unwrap());
            value_reg
        } else {
            self.to_operand(node)
        }
    }

    /// Same as `to_operand` but if the value is a constant and larger than 32
    /// bits, move it to a register
    fn to_operand_oversize_reg_promote(&mut self, node: &X86NodeRef) -> Operand {
        let op = self.to_operand(node);

        if let OperandKind::Immediate(value) = op.kind() {
            if *value > (u32::MAX as u64) {
                let tmp = Operand::vreg(op.width(), self.next_vreg());
                self.push_instruction(Instruction::mov(op, tmp).unwrap());
                return tmp;
            }
        }

        op
    }

    fn to_operand(&mut self, node: &X86NodeRef) -> Operand {
        if let Some(operand) = self.current_block_operands.get(node) {
            return *operand;
        }

        let op = match node.kind() {
            NodeKind::Constant { value, width } => Operand::imm(
                Width::from_uncanonicalized(*width)
                    .unwrap_or_else(|e| panic!("failed to canonicalize width of {node:?}: {e}")),
                *value,
            ),
            NodeKind::GuestRegister { offset } => {
                let width = Width::from_uncanonicalized(node.typ().width()).unwrap_or_else(|e| {
                    panic!("invalid width register at offset {offset:?}: {e:?}")
                });
                let dst = Operand::vreg(width, self.next_vreg());

                self.push_instruction(
                    Instruction::mov(
                        Operand::mem_base_displ(
                            width,
                            Register::PhysicalRegister(PhysicalRegister::RBP),
                            (*offset).try_into().unwrap(),
                        ),
                        dst,
                    )
                    .unwrap(),
                );

                dst
            }
            NodeKind::ReadStackVariable { offset, width } => {
                let width = Width::from_uncanonicalized(*width).unwrap();
                let dst = Operand::vreg(width, self.next_vreg());

                self.push_instruction(
                    Instruction::mov(
                        Operand::mem_base_displ(
                            width,
                            Register::PhysicalRegister(PhysicalRegister::R14),
                            -(i32::try_from(*offset).unwrap()),
                        ),
                        dst,
                    )
                    .unwrap(),
                );

                dst
            }
            NodeKind::BinaryOperation(kind) => self.binary_operation_to_operand(kind),
            NodeKind::TernaryOperation(kind) => match kind {
                TernaryOperationKind::AddWithCarry(a, b, carry) => {
                    let a_width = Width::from_uncanonicalized(a.typ().width()).unwrap();
                    let b_width = Width::from_uncanonicalized(b.typ().width()).unwrap();

                    assert_eq!(a_width, b_width);
                    assert_eq!(carry.typ().width(), 1);

                    let dst = Operand::vreg(a_width, self.next_vreg());

                    let a = self.to_operand(a);
                    let b = self.to_operand(b);
                    let carry = self.to_operand(carry);
                    self.push_instruction(Instruction::mov(b, dst).unwrap());
                    self.push_instruction(Instruction::adc(a, dst, carry));

                    dst
                }
            },
            NodeKind::UnaryOperation(kind) => match &kind {
                UnaryOperationKind::Complement(value) => {
                    let width = Width::from_uncanonicalized(value.typ().width()).unwrap();
                    let dst = Operand::vreg(width, self.next_vreg());
                    let value = self.to_operand(value);
                    self.push_instruction(Instruction::mov(value, dst).unwrap());
                    self.push_instruction(Instruction::not(dst));
                    dst
                }
                UnaryOperationKind::Not(value) => {
                    let width = Width::from_uncanonicalized(value.typ().width()).unwrap();
                    let value = self.to_operand(value);
                    let dst = Operand::vreg(width, self.next_vreg());

                    self.push_instruction(Instruction::cmp(Operand::imm(width, 0), value));
                    self.push_instruction(Instruction::sete(dst));
                    self.push_instruction(Instruction::and(Operand::imm(width, 1), dst));

                    dst
                }
                UnaryOperationKind::Ceil(value) => {
                    let NodeKind::Tuple(real) = value.kind() else {
                        panic!();
                    };

                    let [num, den] = real.as_slice() else {
                        panic!();
                    };

                    assert_eq!(num.typ().width(), den.typ().width());

                    let width = Width::from_uncanonicalized(num.typ().width()).unwrap();
                    let num = self.to_operand(num);
                    let den = self.to_operand(den);
                    let divisor = Operand::vreg(width, self.next_vreg());

                    let rax = Operand::preg(width, PhysicalRegister::RAX);
                    let rdx = Operand::preg(width, PhysicalRegister::RDX);

                    self.push_instruction(Instruction::xor(rdx, rdx));
                    self.push_instruction(Instruction::mov(num, rax).unwrap());
                    self.push_instruction(Instruction::mov(den, divisor).unwrap());
                    self.push_instruction(Instruction::idiv(rdx, rax, divisor));

                    let quotient = Operand::vreg(width, self.next_vreg());
                    let remainder = Operand::vreg(width, self.next_vreg());
                    self.push_instruction(Instruction::mov(rax, quotient).unwrap());
                    self.push_instruction(Instruction::mov(rdx, remainder).unwrap());

                    let nz = Operand::vreg(Width::_8, self.next_vreg());
                    let g = Operand::vreg(Width::_8, self.next_vreg());

                    self.push_instruction(Instruction::test(remainder, remainder));
                    self.push_instruction(Instruction::setnz(nz));
                    self.push_instruction(Instruction::test(num, num));
                    self.push_instruction(Instruction::setg(g));
                    self.push_instruction(Instruction::and(g, nz));
                    let mask = Operand::vreg(width, self.next_vreg());
                    self.push_instruction(Instruction::movzx(nz, mask));

                    self.push_instruction(Instruction::add(mask, quotient));

                    quotient
                }
                UnaryOperationKind::Floor(value) => {
                    let NodeKind::Tuple(real) = value.kind() else {
                        panic!();
                    };

                    let [num, den] = real.as_slice() else {
                        panic!();
                    };

                    assert_eq!(num.typ().width(), den.typ().width());

                    let width = Width::from_uncanonicalized(num.typ().width()).unwrap();
                    let num = self.to_operand(num);
                    let den = self.to_operand(den);
                    let divisor = Operand::vreg(width, self.next_vreg());

                    let rax = Operand::preg(width, PhysicalRegister::RAX);
                    let rdx = Operand::preg(width, PhysicalRegister::RDX);

                    self.push_instruction(Instruction::xor(rdx, rdx));
                    self.push_instruction(Instruction::mov(num, rax).unwrap());
                    self.push_instruction(Instruction::mov(den, divisor).unwrap());
                    self.push_instruction(Instruction::idiv(rdx, rax, divisor));

                    let quotient = Operand::vreg(width, self.next_vreg());
                    let remainder = Operand::vreg(width, self.next_vreg());
                    self.push_instruction(Instruction::mov(rax, quotient).unwrap());
                    self.push_instruction(Instruction::mov(rdx, remainder).unwrap());

                    let nz = Operand::vreg(Width::_8, self.next_vreg());
                    let s = Operand::vreg(Width::_8, self.next_vreg());

                    self.push_instruction(Instruction::test(remainder, remainder));
                    self.push_instruction(Instruction::setnz(nz));
                    self.push_instruction(Instruction::test(num, num));
                    self.push_instruction(Instruction::sets(s));
                    self.push_instruction(Instruction::and(s, nz));
                    let mask = Operand::vreg(width, self.next_vreg());
                    self.push_instruction(Instruction::movzx(nz, mask));

                    self.push_instruction(Instruction::sub(mask, quotient));

                    quotient
                }
                kind => todo!("{kind:?}"),
            },
            NodeKind::BitExtract {
                value,
                start,
                length,
            } => {
                let mut value = if let NodeKind::Constant { .. } = value.kind() {
                    let width = Width::from_uncanonicalized(value.typ().width()).unwrap();
                    let value_reg = Operand::vreg(width, self.next_vreg());
                    let value_imm = self.to_operand(value);
                    self.push_instruction(Instruction::mov(value_imm, value_reg).unwrap());
                    value_reg
                } else {
                    self.to_operand(value)
                };

                if value.width() < Width::_64 {
                    let tmp = Operand::vreg(Width::_64, self.next_vreg());
                    self.push_instruction(Instruction::movzx(value, tmp));
                    value = tmp;
                }

                let start = self.to_operand(start);
                let length = self.to_operand(length);

                //  start[0..8] ++ length[0..8];
                let control_byte = {
                    let mask = Operand::imm(Width::_64, 0xff);

                    let start = {
                        let dst = Operand::vreg(Width::_64, self.next_vreg());
                        self.push_instruction(Instruction::mov(start, dst).unwrap());
                        self.push_instruction(Instruction::and(mask, dst));
                        dst
                    };

                    let length = {
                        let dst = Operand::vreg(Width::_64, self.next_vreg());
                        self.push_instruction(Instruction::mov(length, dst).unwrap());
                        self.push_instruction(Instruction::and(mask, dst));
                        self.push_instruction(Instruction::shl(Operand::imm(Width::_8, 8), dst));
                        dst
                    };

                    let dst = Operand::vreg(Width::_64, self.next_vreg());

                    self.push_instruction(Instruction::mov(start, dst).unwrap());
                    self.push_instruction(Instruction::or(length, dst));

                    dst
                };

                // todo: this 64 should be the value of `length`
                let dst = Operand::vreg(Width::_64, self.next_vreg());

                self.push_instruction(Instruction::bextr(control_byte, value, dst));

                dst
            }
            NodeKind::Cast { value, kind } => {
                let target_width = Width::from_uncanonicalized(node.typ().width()).unwrap();
                let dst = Operand::vreg(target_width, self.next_vreg());
                let mut src = self.to_operand(value);

                if node.typ() == value.typ() {
                    self.push_instruction(Instruction::mov(src, dst).unwrap());
                } else {
                    match kind {
                        CastOperationKind::ZeroExtend => {
                            if src.width() == dst.width() {
                                self.push_instruction(Instruction::mov(src, dst).unwrap());
                            } else {
                                self.push_instruction(Instruction::movzx(src, dst));
                            }
                        }
                        CastOperationKind::SignExtend => {
                            if src.width() == dst.width() {
                                self.push_instruction(Instruction::mov(src, dst).unwrap());
                            } else {
                                self.push_instruction(Instruction::movsx(src, dst));
                            }
                        }
                        CastOperationKind::Convert => {
                            panic!("{:?}\n{:#?}", node.typ(), value);
                        }
                        CastOperationKind::Truncate => {
                            let src_width = src.width();
                            let dst_width = dst.width();
                            if src_width < dst_width {
                                panic!("src ({src_width} bits) must be larger than dst ({dst_width} bits)");
                            }

                            src.width_in_bits = dst.width_in_bits;

                            self.push_instruction(Instruction::mov(src, dst).unwrap());
                        }

                        CastOperationKind::Reinterpret => match src.width().cmp(&dst.width()) {
                            Ordering::Equal => {
                                self.push_instruction(Instruction::mov(src, dst).unwrap())
                            }
                            Ordering::Less => self.push_instruction(Instruction::movzx(src, dst)),
                            Ordering::Greater => {
                                src.width_in_bits = dst.width_in_bits;
                                self.push_instruction(Instruction::mov(src, dst).unwrap())
                            }
                        },
                        _ => todo!("{kind:?} to {:?}\n{value:#?}", node.typ()),
                    }
                }

                dst
            }
            NodeKind::Shift {
                value,
                amount,
                kind,
            } => {
                let mut amount = self.to_operand(amount);
                let value = self.to_operand(value);

                let dst = Operand::vreg(value.width(), self.next_vreg());
                self.push_instruction(Instruction::mov(value, dst).unwrap());

                if let OperandKind::Register(_) = amount.kind() {
                    // truncate (high bits don't matter anyway)
                    amount.width_in_bits = Width::_8;
                    let amount_dst = Operand::preg(Width::_8, PhysicalRegister::RCX);
                    self.push_instruction(Instruction::mov(amount, amount_dst).unwrap());
                    amount = amount_dst;
                }

                match kind {
                    ShiftOperationKind::LogicalShiftLeft => {
                        self.push_instruction(Instruction::shl(amount, dst));
                    }

                    ShiftOperationKind::LogicalShiftRight => {
                        self.push_instruction(Instruction::shr(amount, dst));
                    }

                    ShiftOperationKind::ArithmeticShiftRight => {
                        self.push_instruction(Instruction::sar(amount, dst));
                    }
                    _ => todo!("{kind:?}"),
                }

                dst
            }
            NodeKind::BitInsert {
                target,
                source,
                start,
                length,
            } => {
                // todo: test this
                let target = self.to_operand(target);
                let source = self.to_operand(source);
                let mut start = self.to_operand(start);
                let mut length = self.to_operand(length);

                let source = match source.width().cmp(&target.width()) {
                    Ordering::Equal => source,
                    Ordering::Greater => {
                        panic!("source width exceeds target")
                    }
                    Ordering::Less => {
                        let new_source = Operand::vreg(target.width(), self.next_vreg());
                        self.push_instruction(Instruction::movzx(source, new_source));
                        new_source
                    }
                };
                assert_eq!(start.width(), length.width());

                // fix length for shl
                if let OperandKind::Register(_) = length.kind() {
                    // truncate (high bits don't matter anyway)
                    length.width_in_bits = Width::_8;
                    let tmp = Operand::preg(Width::_8, PhysicalRegister::RCX);
                    self.push_instruction(Instruction::mov(length, tmp).unwrap());
                    length = tmp;
                }

                // fix start for shl
                if let OperandKind::Register(_) = start.kind() {
                    // truncate (high bits don't matter anyway)
                    start.width_in_bits = Width::_8;
                    let tmp = Operand::preg(Width::_8, PhysicalRegister::RCX);
                    self.push_instruction(Instruction::mov(start, tmp).unwrap());
                    start = tmp;
                }

                let width = target.width();

                // mask off target bits
                let mask = Operand::vreg(width, self.next_vreg());
                self.push_instruction(Instruction::mov(Operand::imm(width, 1), mask).unwrap());
                self.push_instruction(Instruction::shl(length, mask)); // cast length here
                self.push_instruction(Instruction::sub(Operand::imm(width, 1), mask));
                self.push_instruction(Instruction::shl(start, mask));
                self.push_instruction(Instruction::not(mask));

                let masked_target = Operand::vreg(width, self.next_vreg());
                self.push_instruction(Instruction::mov(target, masked_target).unwrap());
                self.push_instruction(Instruction::and(mask, masked_target));

                // shift source by start
                let shifted_source = Operand::vreg(width, self.next_vreg());
                self.push_instruction(Instruction::mov(source, shifted_source).unwrap());
                self.push_instruction(Instruction::shl(start, shifted_source));

                // apply ~mask to source
                {
                    // not strictly necessary but may avoid issues if there is junk data
                    let invert_mask = Operand::vreg(width, self.next_vreg());
                    self.push_instruction(Instruction::mov(mask, invert_mask).unwrap());
                    self.push_instruction(Instruction::not(invert_mask));
                    self.push_instruction(Instruction::and(invert_mask, shifted_source));
                }

                // OR source and target
                self.push_instruction(Instruction::or(shifted_source, masked_target));

                masked_target
            }
            NodeKind::GetFlags { operation } => {
                let n = Operand::vreg(Width::_8, self.next_vreg());
                let z = Operand::vreg(Width::_8, self.next_vreg());
                let c = Operand::vreg(Width::_8, self.next_vreg());
                let v = Operand::vreg(Width::_8, self.next_vreg());
                let dest = Operand::vreg(Width::_8, self.next_vreg());

                let instrs = [
                    Instruction::sets(n),
                    Instruction::sete(z),
                    Instruction::setc(c),
                    Instruction::seto(v),
                    Instruction::xor(dest, dest),
                    Instruction::or(n, dest),
                    Instruction::shl(Operand::imm(Width::_8, 1), dest),
                    Instruction::or(z, dest),
                    Instruction::shl(Operand::imm(Width::_8, 1), dest),
                    Instruction::or(c, dest),
                    Instruction::shl(Operand::imm(Width::_8, 1), dest),
                    Instruction::or(v, dest),
                ];

                match self.current_block_operands.get(operation).copied() {
                    Some(operation_operand) => {
                        let block_instructions = &mut self
                            .current_block
                            .clone()
                            .get_mut(self.ctx_mut().arena_mut())
                            .instructions;

                        let (index, adc) = block_instructions
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, i)| matches!(i.0, Opcode::ADC(_, _, _)))
                            .unwrap();

                        if let Opcode::ADC(_, dst, _) = &adc.0 {
                            assert_eq!(*dst, operation_operand)
                        } else {
                            panic!()
                        };

                        for instr in instrs.into_iter().rev() {
                            block_instructions.insert(index + 1, instr);
                        }
                    }
                    None => {
                        let _target = self.to_operand(operation);

                        self.current_block
                            .clone()
                            .get_mut(self.ctx_mut().arena_mut())
                            .instructions
                            .extend_from_slice(&instrs);
                    }
                }
                // if the last instruction wasn't an ADC, emit one? todo:
                if !matches!(
                    self.current_block
                        .get(self.ctx.arena())
                        .instructions()
                        .last()
                        .map(|i| &i.0),
                    Some(Opcode::ADC(_, _, _))
                ) {
                    let _op = self.to_operand(operation);
                }

                // nzcv
                dest
            }
            NodeKind::Tuple(vec) => panic!("cannot convert to operand: {vec:#?}"),
            NodeKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                let width = Width::from_uncanonicalized(true_value.typ().width()).unwrap();
                let dest = Operand::vreg(width, self.next_vreg());

                let condition = self.to_operand(condition);
                let true_value = self.to_operand_reg_promote(true_value);
                let false_value = self.to_operand(false_value);

                // if this sequence is modified, the register allocator must be fixed
                self.push_instruction(Instruction::test(condition, condition));
                self.push_instruction(Instruction::mov(false_value, dest).unwrap());
                self.push_instruction(Instruction::cmovne(true_value, dest)); // this write to dest does not result in deallocation

                dest
            }
            NodeKind::ReadMemory { address } => {
                let width = Width::from_uncanonicalized(node.typ().width()).unwrap();

                let address = self.to_operand(address);
                let OperandKind::Register(address_reg) = address.kind() else {
                    panic!()
                };

                let dest = Operand::vreg(width, self.next_vreg());

                self.push_instruction(
                    Instruction::mov(Operand::mem_base_displ(width, *address_reg, 0), dest)
                        .unwrap(),
                );

                dest
            }
        };

        self.current_block_operands.insert(node.clone(), op);
        op
    }

    fn binary_operation_to_operand(&mut self, kind: &BinaryOperationKind) -> Operand {
        use BinaryOperationKind::*;

        let (Add(left, right)
        | Sub(left, right)
        | Or(left, right)
        | Modulo(left, right)
        | Divide(left, right)
        | Multiply(left, right)
        | And(left, right)
        | Xor(left, right)
        | PowI(left, right)
        | CompareEqual(left, right)
        | CompareNotEqual(left, right)
        | CompareLessThan(left, right)
        | CompareLessThanOrEqual(left, right)
        | CompareGreaterThan(left, right)
        | CompareGreaterThanOrEqual(left, right)) = kind;

        // do this first to avoid tuple issues
        if let BinaryOperationKind::CompareEqual(left, right)
        | BinaryOperationKind::CompareNotEqual(left, right)
        | BinaryOperationKind::CompareGreaterThan(left, right)
        | BinaryOperationKind::CompareGreaterThanOrEqual(left, right)
        | BinaryOperationKind::CompareLessThan(left, right)
        | BinaryOperationKind::CompareLessThanOrEqual(left, right) = kind
        {
            return encode_compare(kind, self, left.clone(), right.clone());
        }

        // pull out widths but also validate types are compatible
        let (left, mut right) = match (left.typ(), right.typ()) {
            (Type::Unsigned(l), Type::Unsigned(r)) => match l.cmp(r) {
                Ordering::Less => {
                    todo!("zero extend {l} to {r}")
                }
                Ordering::Equal => (
                    self.to_operand_oversize_reg_promote(left),
                    self.to_operand_oversize_reg_promote(right),
                ),
                Ordering::Greater => {
                    let left = self.to_operand_oversize_reg_promote(left);
                    let right = self.to_operand_oversize_reg_promote(right);

                    let tmp = Operand::vreg(left.width(), self.next_vreg());
                    self.push_instruction(Instruction::movzx(right, tmp));

                    (left, tmp)
                }
            },

            (Type::Bits, Type::Unsigned(_)) => todo!(), // (64, r)
            (Type::Unsigned(_), Type::Bits) => {
                let left = self.to_operand_oversize_reg_promote(left);
                let right = self.to_operand_oversize_reg_promote(right);

                if left.width() == right.width() {
                    (left, right)
                } else {
                    todo!()
                }
            }
            (Type::Signed(l), Type::Signed(r)) => match l.cmp(r) {
                Ordering::Less => {
                    let left = self.to_operand_oversize_reg_promote(left);
                    let right = self.to_operand_oversize_reg_promote(right);
                    let tmp = Operand::vreg(right.width(), self.next_vreg());
                    self.push_instruction(Instruction::movsx(left, tmp));
                    (tmp, right)
                }
                Ordering::Equal => (
                    self.to_operand_oversize_reg_promote(left),
                    self.to_operand_oversize_reg_promote(right),
                ),
                Ordering::Greater => {
                    todo!("sign extend {r} to {l}")
                }
            },

            (Type::Floating(l), Type::Floating(r)) => todo!(),

            (Type::Tuple, Type::Tuple) => {
                todo!()
            }

            (left, right) => todo!("{left:?} {right:?}"),
        };

        let width = left.width();
        assert_eq!(width, right.width());

        let dst = Operand::vreg(width, self.next_vreg());

        match kind {
            BinaryOperationKind::Add(_, _) => {
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::add(right, dst));
                dst
            }
            BinaryOperationKind::Sub(_, _) => {
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::sub(right, dst));
                dst
            }
            BinaryOperationKind::Or(_, _) => {
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::or(right, dst));
                dst
            }
            BinaryOperationKind::Multiply(_, _) => {
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::imul(right, dst));
                dst
            }
            BinaryOperationKind::And(_, _) => {
                if let OperandKind::Immediate(i) = right.kind() {
                    if *i > u32::MAX as u64 {
                        let new = Operand::vreg(width, self.next_vreg());
                        self.push_instruction(Instruction::mov(right, new).unwrap());
                        right = new;
                    }
                }
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::and(right, dst));

                dst
            }

            BinaryOperationKind::Divide(dividend, divisor) => {
                assert_eq!(dividend.typ().width(), 64);
                assert_eq!(divisor.typ().width(), 64);

                let dividend = self.to_operand(dividend);
                let divisor = self.to_operand_reg_promote(divisor);

                let _0 = Operand::imm(Width::_64, 0);

                let hi = Operand::preg(width, PhysicalRegister::RDX);
                let lo = Operand::preg(width, PhysicalRegister::RAX);

                self.push_instruction(Instruction::mov(_0, hi).unwrap());
                self.push_instruction(Instruction::mov(dividend, lo).unwrap());
                self.push_instruction(Instruction::idiv(hi, lo, divisor));

                lo
            }

            BinaryOperationKind::Modulo(dividend, divisor) => {
                assert_eq!(dividend.typ().width(), 64);
                assert_eq!(divisor.typ().width(), 64);

                let dividend = self.to_operand(dividend);
                let divisor = self.to_operand_reg_promote(divisor);

                let _0 = Operand::imm(Width::_64, 0);

                let hi = Operand::preg(width, PhysicalRegister::RDX);
                let lo = Operand::preg(width, PhysicalRegister::RAX);

                self.push_instruction(Instruction::mov(_0, hi).unwrap());
                self.push_instruction(Instruction::mov(dividend, lo).unwrap());
                self.push_instruction(Instruction::idiv(hi, lo, divisor));

                hi
            }

            op => todo!("{op:#?}"),
        }
    }
}

impl<'ctx> Emitter for X86Emitter<'ctx> {
    type NodeRef = X86NodeRef;
    type BlockRef = Ref<X86Block>;
    type SymbolRef = X86SymbolRef;

    fn set_current_block(&mut self, block: Self::BlockRef) {
        self.current_block = block;
        self.current_block_operands = HashMap::default();
    }

    fn get_current_block(&self) -> Self::BlockRef {
        self.current_block
    }

    fn constant(&mut self, value: u64, typ: Type) -> Self::NodeRef {
        let width = typ.width();
        if width == 0 {
            panic!(
                "no zero width constants allowed! {typ:?} @ {:?}",
                self.current_block
            )
        }
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

    fn read_register(&mut self, offset: u64, typ: Type) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::GuestRegister { offset },
        })
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
            Ceil(_) | Floor(_) => Self::NodeRef::from(X86Node {
                typ: Type::Signed(64),
                kind: NodeKind::UnaryOperation(op),
            }),

            _ => {
                todo!("{op:?}")
            }
        }
    }

    fn binary_operation(&mut self, op: BinaryOperationKind) -> Self::NodeRef {
        use BinaryOperationKind::*;

        // todo: re-enable me
        // match &op {
        //     Add(lhs, rhs)
        //     | Sub(lhs, rhs)
        //     | Multiply(lhs, rhs)
        //     | Divide(lhs, rhs)
        //     | Modulo(lhs, rhs)
        //     | Or(lhs, rhs)
        //     | Xor(lhs, rhs)
        //     | And(lhs, rhs)
        //     | PowI(lhs, rhs)
        //     | CompareEqual(lhs, rhs)
        //     | CompareNotEqual(lhs, rhs)
        //     | CompareLessThan(lhs, rhs)
        //     | CompareLessThanOrEqual(lhs, rhs)
        //     | CompareGreaterThan(lhs, rhs)
        //     | CompareGreaterThanOrEqual(lhs, rhs) => {
        //         if lhs.typ() != rhs.typ() {
        //             return Err(X86Error::BinaryOperationTypeMismatch { op: op.clone()
        // });         }
        //     }
        // }

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
                        value: lhs_value.wrapping_add(*rhs_value),// todo: THIS WILL WRAP AT 64 NOT *width*!
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
                        value: lhs_value.wrapping_sub(*rhs_value),// todo: THIS WILL WRAP AT 64 NOT *width*!
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
            Divide(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value / rhs_value,
                        width: *width,
                    },
                }),
                (NodeKind::Tuple(left), NodeKind::Tuple(right)) => {
                    match (left.as_slice(), right.as_slice()) {
                        ([left_num, left_den], [right_num, right_den]) => {
                            let num = self.binary_operation(BinaryOperationKind::Multiply(
                                left_num.clone(),
                                right_den.clone(),
                            ));
                            let den = self.binary_operation(BinaryOperationKind::Multiply(
                                left_den.clone(),
                                right_num.clone(),
                            ));
                            self.create_tuple(alloc::vec![num, den])
                        }
                        _ => panic!(),
                    }
                }
                _ => Self::NodeRef::from(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            Modulo(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
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
                        value: lhs_value % rhs_value,
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

            CompareGreaterThan(_, _)
            | CompareGreaterThanOrEqual(_, _)
            | CompareLessThan(_, _)
            | CompareLessThanOrEqual(_, _) => emit_compare(op),

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
                if let Type::Bits = target_type {
                    panic!("don't cast to a bits:(")
                }

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

                self.constant(casted_value, target_type)
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
        match (value.kind(), amount.kind(), kind.clone()) {
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: amount_value,
                    ..
                },
                ShiftOperationKind::LogicalShiftLeft,
            ) => {
                let shifted = match (value_value, amount_value) {
                    (0, _) => 0,
                    (v, 0) => *v,
                    (v, a) => v
                        .checked_shl(u32::try_from(*a).unwrap())
                        .unwrap_or_else(|| panic!("failed to shift left {value:?} by {amount:?}")),
                };

                // shift and mask to width of value
                self.constant(shifted & mask(*value_width), typ)
            }
            (
                NodeKind::Constant {
                    value: value_value, ..
                },
                NodeKind::Constant {
                    value: amount_value,
                    ..
                },
                ShiftOperationKind::LogicalShiftRight,
            ) => {
                // mask to width of value
                self.constant(
                    value_value
                        .checked_shr(u32::try_from(*amount_value).unwrap())
                        .unwrap_or(0),
                    typ,
                )
            }
            (
                NodeKind::Constant {
                    value: value_value,
                    width: 64, // has to be 64 for the i64 shift to be valid
                },
                NodeKind::Constant {
                    value: amount_value,
                    ..
                },
                ShiftOperationKind::ArithmeticShiftRight,
            ) => {
                let signed_value = *value_value as i64;
                let shifted = signed_value
                    .checked_shr(u32::try_from(*amount_value).unwrap())
                    .unwrap() as u64;

                // mask to width of value
                self.constant(shifted, typ)
            }
            (NodeKind::Constant { .. }, NodeKind::Constant { .. }, k) => {
                todo!("{k:?}")
            }
            (_, _, _) => Self::NodeRef::from(X86Node {
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
            // total constant
            (
                NodeKind::Constant { value, .. },
                NodeKind::Constant { value: start, .. },
                NodeKind::Constant { value: length, .. },
            ) => self.constant(
                bit_extract(*value, *start, *length),
                Type::Unsigned(u16::try_from(*length).unwrap()),
            ),

            // known start and length
            (
                _,
                NodeKind::Constant {
                    value: _start_value,
                    ..
                },
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
            ) => {
                // value >> start && mask(length)
                let shifted = self.shift(
                    value.clone(),
                    start.clone(),
                    ShiftOperationKind::LogicalShiftRight,
                );

                let cast = self.cast(
                    shifted,
                    Type::Unsigned(u16::try_from(*length_value).unwrap()),
                    CastOperationKind::Truncate,
                );

                let mask = self.constant(
                    mask(u32::try_from(*length_value).unwrap()),
                    cast.typ().clone(),
                );

                self.binary_operation(BinaryOperationKind::And(cast, mask))
            }
            // // known value, unknown start and length
            // (NodeKind::Constant { .. }, _, _) => {
            //     let value =
            //     Self::NodeRef::from(X86Node {
            //         typ,
            //         kind: NodeKind::BitExtract {
            //             value,
            //             start,
            //             length,
            //         },
            //     })
            // }
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
            ) => self.constant(
                bit_insert(*target, *source, *start, *length),
                Type::Unsigned(u16::try_from(*length).unwrap()),
            ),
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

    fn write_register(&mut self, offset: u64, value: Self::NodeRef) {
        // todo: validate offset + width is within register file

        let value = self.to_operand(&value);

        let width = value.width();

        self.push_instruction(
            Instruction::mov(
                value,
                Operand::mem_base_displ(
                    width,
                    Register::PhysicalRegister(PhysicalRegister::RBP),
                    offset.try_into().unwrap(),
                ),
            )
            .unwrap(),
        );
    }

    fn read_memory(&mut self, address: Self::NodeRef, typ: Type) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::ReadMemory { address },
        })
    }

    fn write_memory(&mut self, address: Self::NodeRef, value: Self::NodeRef) {
        let address = self.to_operand(&address);
        let OperandKind::Register(address_reg) = address.kind() else {
            panic!()
        };

        let value = self.to_operand(&value);
        let width = value.width();

        self.push_instruction(
            Instruction::mov(value, Operand::mem_base_displ(width, *address_reg, 0)).unwrap(),
        );
    }

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    ) -> BlockResult {
        match condition.kind() {
            NodeKind::Constant { .. } => {
                todo!("this was handled in models.rs")
            }
            _ => {
                let condition = self.to_operand(&condition);

                self.push_instruction(Instruction::test(condition, condition));

                self.push_instruction(Instruction::jne(true_target.clone()));
                self.push_target(true_target.clone());

                self.push_instruction(Instruction::jmp(false_target.clone()));
                self.push_target(false_target.clone());

                // if condition is static, return BlockResult::Static
                // else
                BlockResult::Dynamic(true_target, false_target)
            }
        }
    }

    fn jump(&mut self, target: Self::BlockRef) -> BlockResult {
        self.push_instruction(Instruction::jmp(target.clone()));
        self.push_target(target.clone());
        BlockResult::Static(target)
    }

    fn leave(&mut self) {
        self.push_instruction(Instruction::ret());
    }

    fn read_virt_variable(&mut self, symbol: Self::SymbolRef) -> Self::NodeRef {
        symbol
            .0
            .borrow()
            .as_ref()
            .unwrap_or_else(|| panic!("tried to read from {symbol:?} but it was never written to"))
            .clone()
    }
    fn write_virt_variable(&mut self, symbol: Self::SymbolRef, value: Self::NodeRef) {
        *symbol.0.borrow_mut() = Some(value);
    }

    fn read_stack_variable(&mut self, offset: usize, typ: Type) -> Self::NodeRef {
        let width = typ.width();

        Self::NodeRef::from(X86Node {
            typ,
            kind: NodeKind::ReadStackVariable { offset, width },
        })
    }

    fn write_stack_variable(&mut self, offset: usize, value: Self::NodeRef) {
        let value = self.to_operand(&value);

        let mem = Operand::mem_base_displ(
            value.width(),
            Register::PhysicalRegister(PhysicalRegister::R14),
            -(i32::try_from(offset).unwrap()),
        );

        self.push_instruction(Instruction::mov(value, mem).unwrap());
    }

    fn assert(&mut self, condition: Self::NodeRef, meta: u64) {
        match condition.kind() {
            NodeKind::Constant { value, .. } => {
                if *value == 0 {
                    self.panic("constant assert failed");
                }
            }
            _ => {
                let not_condition = self.unary_operation(UnaryOperationKind::Not(condition));
                let op = self.to_operand(&not_condition);

                self.push_instruction(Instruction::test(op, op));
                self.push_instruction(
                    Instruction::mov(
                        Operand::imm(Width::_64, meta),
                        Operand::preg(Width::_64, PhysicalRegister::R15),
                    )
                    .unwrap(),
                );
                self.push_instruction(Instruction::jne(self.panic_block.clone()));
            }
        }
    }

    fn mutate_element(
        &mut self,
        _vector: Self::NodeRef,
        _index: Self::NodeRef,
        _value: Self::NodeRef,
    ) -> Self::NodeRef {
        todo!()
    }

    // returns a tuple of (operation_result, flags)
    fn get_flags(&mut self, operation: Self::NodeRef) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: Type::Unsigned(4),
            kind: NodeKind::GetFlags { operation },
        })
    }

    fn panic(&mut self, msg: &str) {
        let n = self.to_operand(&Self::NodeRef::from(X86Node {
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
        }));

        self.push_instruction(Instruction::int(n));
    }

    fn create_tuple(&mut self, values: Vec<Self::NodeRef>) -> Self::NodeRef {
        Self::NodeRef::from(X86Node {
            typ: Type::Tuple,
            kind: NodeKind::Tuple(values),
        })
    }

    fn access_tuple(&mut self, tuple: Self::NodeRef, index: usize) -> Self::NodeRef {
        let NodeKind::Tuple(values) = tuple.kind() else {
            panic!("accessing non tuple: {:?}", *tuple.0)
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
                    match value.kind() {
                        NodeKind::Cast {
                            value,
                            kind: CastOperationKind::ZeroExtend,
                        } => match value.typ() {
                            Type::Unsigned(w) => self.constant(u64::from(*w), Type::Unsigned(16)),
                            _ => todo!(),
                        },
                        _ => todo!("size of {value:#?}"),
                    }
                }
            }
            Type::Tuple => todo!(),
        }
    }

    fn bits_cast(
        &mut self,
        value: Self::NodeRef,
        length: Self::NodeRef,
        _typ: Type,
        kind: CastOperationKind,
    ) -> Self::NodeRef {
        match (value.kind(), length.kind(), kind) {
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
                CastOperationKind::Truncate,
            ) => {
                let target_length = u16::try_from(*length_value).unwrap();

                assert!(target_length <= *value_width);

                let typ = match value.typ() {
                    Type::Unsigned(_) | Type::Bits => Type::Unsigned(target_length),
                    Type::Signed(_) => Type::Signed(target_length),
                    _ => todo!(),
                };

                self.constant(*value_value & mask(target_length), typ)
            }
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
                CastOperationKind::SignExtend,
            ) => {
                let target_length = u16::try_from(*length_value).unwrap();

                assert!(target_length >= *value_width);

                let typ = match value.typ() {
                    Type::Unsigned(_) | Type::Bits => Type::Unsigned(target_length),
                    Type::Signed(_) => Type::Signed(target_length),
                    _ => todo!(),
                };

                let sign_extended =
                    ((*value_value as i64) << (64 - value_width)) >> (64 - value_width);

                self.constant(sign_extended as u64 & mask(target_length), typ)
            }
            (
                NodeKind::Constant {
                    value: value_value,
                    width: value_width,
                },
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
                CastOperationKind::ZeroExtend,
            ) => {
                let target_length = u16::try_from(*length_value).unwrap();

                assert!(target_length >= *value_width);

                let typ = match value.typ() {
                    Type::Unsigned(_) | Type::Bits => Type::Unsigned(target_length),
                    Type::Signed(_) => Type::Signed(target_length),
                    _ => todo!(),
                };

                self.constant(*value_value, typ)
            }
            (
                _,
                NodeKind::Constant {
                    value: length_value,
                    ..
                },
                CastOperationKind::SignExtend,
            ) => self.cast(
                value,
                Type::Signed(u16::try_from(*length_value).unwrap()),
                CastOperationKind::SignExtend,
            ),
            _ => {
                // todo: attach length information
                // todo: fix other cast operation kinds!
                value
            }
        }
    }
}

fn sign_extend(value: u64, original_width: u16, target_width: u16) -> u64 {
    if value == 0 {
        return 0;
    }

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

impl Hash for X86NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl Eq for X86NodeRef {}

impl PartialEq for X86NodeRef {
    fn eq(&self, other: &X86NodeRef) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl X86NodeRef {
    pub fn kind(&self) -> &NodeKind {
        &self.0.kind
    }

    pub fn typ(&self) -> &Type {
        &self.0.typ
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

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Constant {
        value: u64,
        width: u16,
    },
    GuestRegister {
        offset: u64,
    },
    ReadMemory {
        address: X86NodeRef,
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
    ReadStackVariable {
        // positive offset here (will be subtracted from RSP)
        offset: usize,
        width: u16,
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
    GetFlags {
        operation: X86NodeRef,
    },
    Tuple(Vec<X86NodeRef>),
    Select {
        condition: X86NodeRef,
        true_value: X86NodeRef,
        false_value: X86NodeRef,
    },
}

// todo: make me copy
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TernaryOperationKind {
    AddWithCarry(X86NodeRef, X86NodeRef, X86NodeRef),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CastOperationKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    Reinterpret,
    Convert,
    Broadcast,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShiftOperationKind {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    RotateRight,
    RotateLeft,
}

#[derive(Debug, Clone, Copy)]
pub enum X86BlockMark {
    None,
    Temporary,
    Permanent,
}

pub struct X86Block {
    instructions: Vec<Instruction>,
    next: Vec<Ref<X86Block>>,
    linked: bool,
    mark: X86BlockMark,
}

impl X86Block {
    pub fn new() -> Self {
        Self {
            instructions: alloc::vec![],
            next: alloc::vec![],
            linked: false,
            mark: X86BlockMark::None,
        }
    }

    pub fn set_linked(&mut self) {
        self.linked = true;
    }

    pub fn is_linked(&self) -> bool {
        self.linked
    }

    pub fn set_mark(&mut self, mark: X86BlockMark) {
        self.mark = mark;
    }

    pub fn get_mark(&self) -> X86BlockMark {
        self.mark
    }

    pub fn append(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn allocate_registers<R: RegisterAllocator>(&mut self, allocator: &mut R) {
        allocator.allocate(self.instructions_mut());
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn instructions_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    pub fn next_blocks(&self) -> &[Ref<X86Block>] {
        &self.next
    }

    pub fn clear_next_blocks(&mut self) {
        self.next.clear();
    }

    pub fn push_next(&mut self, target: Ref<X86Block>) {
        self.next.push(target);
        if self.next.len() > 2 {
            panic!(
                "bad, blocks should not have more than 2 real targets (asserts complicate things)"
            )
        }
    }
}

impl Debug for X86Block {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for instr in &self.instructions {
            writeln!(f, "\t{instr}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct X86SymbolRef(pub Rc<RefCell<Option<X86NodeRef>>>);

fn encode_compare(
    kind: &BinaryOperationKind,
    emitter: &mut X86Emitter,
    right: X86NodeRef, /* TODO: this was flipped in order to make tests pass, unflip right and
                        * left and fix the body of the function */
    left: X86NodeRef,
) -> Operand {
    use crate::dbt::x86::encoder::OperandKind::*;

    let (left, right) = match (left.kind(), right.kind()) {
        (NodeKind::Constant { .. }, NodeKind::Constant { .. }) => {
            panic!("should've been fixed earlier")
        }
        (NodeKind::Tuple(left_real), NodeKind::Tuple(right_real)) => {
            match (left_real.clone().as_slice(), right_real.as_slice()) {
                ([left_num, left_den], [right_num, right_den]) => (
                    emitter.binary_operation(BinaryOperationKind::Multiply(
                        left_num.clone(),
                        right_den.clone(),
                    )),
                    emitter.binary_operation(BinaryOperationKind::Multiply(
                        left_den.clone(),
                        right_num.clone(),
                    )),
                ),
                _ => panic!(),
            }
        }
        _ => (left, right),
    };

    let is_signed = match (left.typ(), right.typ()) {
        (Type::Unsigned(_), Type::Unsigned(_)) => false,
        (Type::Signed(_), Type::Signed(_)) => true,
        _ => panic!(),
    };

    let left = emitter.to_operand(&left);
    let right = emitter.to_operand(&right);

    // only valid compare instructions are (source-destination):
    // reg reg
    // reg mem
    // mem reg
    // imm reg
    // imm mem

    // anything else (imm on the right) must be reworked

    match (left.kind(), right.kind()) {
        (Register(_), Register(_))
        | (Register(_), Memory { .. })
        | (Memory { .. }, Register(_))
        | (Immediate(_), Register(_))
        | (Immediate(_), Memory { .. })
        | (Memory { .. }, Memory { .. }) => {
            let left = if let (Memory { .. }, Memory { .. }) = (left.kind(), right.kind()) {
                let new_left = Operand::vreg(left.width(), emitter.next_vreg());
                emitter.push_instruction(Instruction::mov(left, new_left).unwrap());
                new_left
            } else {
                left
            };

            emitter.push_instruction(Instruction::cmp(left, right));

            // setCC only sets the lowest bit, smallest unit is a byte, so use an 8 bit
            // destination register
            let dst = Operand::vreg(Width::_8, emitter.next_vreg());

            emitter.push_instruction(match (kind, is_signed) {
                (BinaryOperationKind::CompareEqual(_, _), _) => Instruction::sete(dst),
                (BinaryOperationKind::CompareNotEqual(_, _), _) => Instruction::setne(dst),

                (BinaryOperationKind::CompareLessThan(_, _), false) => Instruction::setb(dst),
                (BinaryOperationKind::CompareLessThanOrEqual(_, _), false) => {
                    Instruction::setbe(dst)
                }
                (BinaryOperationKind::CompareGreaterThan(_, _), false) => Instruction::seta(dst),
                (BinaryOperationKind::CompareGreaterThanOrEqual(_, _), false) => {
                    Instruction::setae(dst)
                }

                (BinaryOperationKind::CompareLessThan(_, _), true) => Instruction::setl(dst),
                (BinaryOperationKind::CompareLessThanOrEqual(_, _), true) => {
                    Instruction::setle(dst)
                }
                (BinaryOperationKind::CompareGreaterThan(_, _), true) => Instruction::setg(dst),
                (BinaryOperationKind::CompareGreaterThanOrEqual(_, _), true) => {
                    Instruction::setge(dst)
                }
                _ => panic!("{kind:?} is not a compare"),
            });

            dst
        }

        (Memory { .. }, Immediate(_)) | (Register(_), Immediate(_)) => {
            emitter.push_instruction(Instruction::cmp(right, left));
            let dst = Operand::vreg(Width::_8, emitter.next_vreg());

            emitter.push_instruction(match (kind, is_signed) {
                (BinaryOperationKind::CompareEqual(_, _), _) => Instruction::sete(dst),
                (BinaryOperationKind::CompareNotEqual(_, _), _) => Instruction::setne(dst),

                (BinaryOperationKind::CompareLessThan(_, _), false) => Instruction::setae(dst),
                (BinaryOperationKind::CompareLessThanOrEqual(_, _), false) => {
                    Instruction::seta(dst)
                }
                (BinaryOperationKind::CompareGreaterThan(_, _), false) => Instruction::setbe(dst),
                (BinaryOperationKind::CompareGreaterThanOrEqual(_, _), false) => {
                    Instruction::setb(dst)
                }

                (BinaryOperationKind::CompareLessThan(_, _), true) => Instruction::setge(dst),
                (BinaryOperationKind::CompareLessThanOrEqual(_, _), true) => Instruction::setg(dst),
                (BinaryOperationKind::CompareGreaterThan(_, _), true) => Instruction::setle(dst),
                (BinaryOperationKind::CompareGreaterThanOrEqual(_, _), true) => {
                    Instruction::setl(dst)
                }
                _ => panic!("{kind:?} is not a compare"),
            });

            dst
        }

        (Immediate(_), Immediate(_)) => panic!("why was this not const evaluated?"),
        (Target(_), _) | (_, Target(_)) => panic!("why"),
    }
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

fn emit_compare(op: BinaryOperationKind) -> X86NodeRef {
    use BinaryOperationKind::*;

    let (CompareLessThan(left, right)
    | CompareLessThanOrEqual(left, right)
    | CompareGreaterThan(left, right)
    | CompareGreaterThanOrEqual(left, right)) = &op
    else {
        panic!("only greater/less than comparisons should be handled here");
    };

    // if constant, do constant evaluation
    if let (
        NodeKind::Constant {
            value: left_value, ..
        },
        NodeKind::Constant {
            value: right_value, ..
        },
    ) = (left.kind(), right.kind())
    {
        let (is_signed, width) = match (left.typ(), right.typ()) {
            (Type::Signed(lw), Type::Signed(rw)) => {
                assert_eq!(lw, rw);
                (true, lw)
            }
            (Type::Unsigned(lw), Type::Unsigned(rw)) => {
                assert_eq!(lw, rw);
                (true, lw)
            }
            types => todo!("compare {types:?}"),
        };

        let result = if is_signed {
            match width {
                64 => {
                    let left = *left_value as i64;
                    let right = *right_value as i64;

                    match &op {
                        CompareLessThan(_, _) => left < right,
                        CompareLessThanOrEqual(_, _) => left <= right,
                        CompareGreaterThan(_, _) => left > right,
                        CompareGreaterThanOrEqual(_, _) => left >= right,
                        _ => panic!(),
                    }
                }
                w => todo!("{w:?}"),
            }
        } else {
            match &op {
                CompareLessThan(_, _) => left_value < right_value,
                CompareLessThanOrEqual(_, _) => left_value <= right_value,
                CompareGreaterThan(_, _) => left_value > right_value,
                CompareGreaterThanOrEqual(_, _) => left_value >= right_value,
                _ => panic!(),
            }
        };

        X86NodeRef::from(X86Node {
            typ: left.typ().clone(),
            kind: NodeKind::Constant {
                value: result as u64,
                width: 1,
            },
        })
    } else {
        // else emit an X86 node
        X86NodeRef::from(X86Node {
            typ: Type::Unsigned(1),
            kind: NodeKind::BinaryOperation(op),
        })
    }

    // match &op {
    //     CompareLessThan(left, right)
    //     | CompareLessThanOrEqual(left, right)
    //     | CompareGreaterThan(left, right)
    //     | CompareGreaterThanOrEqual(left, right) => match (left.kind(),
    // right.kind()) {         (
    //             NodeKind::Constant {
    //                 value: left_value, ..
    //             },
    //             NodeKind::Constant {
    //                 value: right_value, ..
    //             },
    //         ) => X86NodeRef::from(X86Node {
    //             typ: left.typ().clone(),
    //             kind: NodeKind::Constant {
    //                 value: if let Type::Signed(_) = left.typ() {
    //                     // todo: this is broken if signed size != 64
    //                     if (*left_value as i64) < (*right_value as i64) {
    //                         1
    //                     } else {
    //                         0
    //                     }
    //                 } else {
    //                     if left_value < right_value {
    //                         1
    //                     } else {
    //                         0
    //                     }
    //                 },
    //                 width: 1,
    //             },
    //         }),
    //         _ => X86NodeRef::from(X86Node {
    //             typ: Type::Unsigned(1),
    //             kind: NodeKind::BinaryOperation(op),
    //         }),
    //     },
    //     CompareLessThanOrEqual(left, right) => match (left.kind(),
    // right.kind()) {         (
    //             NodeKind::Constant {
    //                 value: left_value, ..
    //             },
    //             NodeKind::Constant {
    //                 value: right_value, ..
    //             },
    //         ) => X86NodeRef::from(X86Node {
    //             typ: left.typ().clone(),
    //             kind: NodeKind::Constant {
    //                 value: if left_value <= right_value { 1 } else { 0 },
    //                 width: 1,
    //             },
    //         }),
    //         _ => X86NodeRef::from(X86Node {
    //             typ: Type::Unsigned(1),
    //             kind: NodeKind::BinaryOperation(op),
    //         }),
    //     },
    //     CompareGreaterThan(left, right) => match (left.kind(), right.kind())
    // {         (
    //             NodeKind::Constant {
    //                 value: left_value, ..
    //             },
    //             NodeKind::Constant {
    //                 value: right_value, ..
    //             },
    //         ) => X86NodeRef::from(X86Node {
    //             typ: left.typ().clone(),
    //             kind: NodeKind::Constant {
    //                 value: if left_value > right_value { 1 } else { 0 },
    //                 width: 1,
    //             },
    //         }),
    //         _ => X86NodeRef::from(X86Node {
    //             typ: Type::Unsigned(1),
    //             kind: NodeKind::BinaryOperation(op),
    //         }),
    //     },
    //     CompareGreaterThanOrEqual(left, right) => match (left.kind(),
    // right.kind()) {         (
    //             NodeKind::Constant {
    //                 value: left_value, ..
    //             },
    //             NodeKind::Constant {
    //                 value: right_value, ..
    //             },
    //         ) => X86NodeRef::from(X86Node {
    //             typ: left.typ().clone(),
    //             kind: NodeKind::Constant {
    //                 value: if left_value >= right_value { 1 } else { 0 },
    //                 width: 1,
    //             },
    //         }),
    //         _ => X86NodeRef::from(X86Node {
    //             typ: Type::Unsigned(1),
    //             kind: NodeKind::BinaryOperation(op),
    //         }),
    //     },
    //     _ => panic!("only greater/less than comparisons should be handled
    // here"), }
}
