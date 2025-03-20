use {
    crate::dbt::{
        Alloc, bit_extract, bit_insert,
        emitter::Type,
        x86::{
            Emitter, X86TranslationContext,
            encoder::{
                Instruction, Opcode, Operand, OperandKind, PhysicalRegister, Register, width::Width,
            },
            register_allocator::RegisterAllocator,
        },
    },
    alloc::{rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        hashmap::HashMap,
        mask::mask,
    },
    core::{
        alloc::Allocator,
        cell::RefCell,
        cmp::Ordering,
        fmt::Debug,
        hash::{Hash, Hasher},
        panic,
    },
    derive_where::derive_where,
    proc_macro_lib::ktest,
};

const INVALID_OFFSET: i32 = 0xDEAD00F;

/// X86 emitter error
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum X86Error {
    /// Left and right types do not match in binary operation
    BinaryOperationTypeMismatch,
    /// Register allocation failed
    RegisterAllocation,
}

pub struct X86Emitter<'ctx, A: Alloc> {
    current_block: Ref<X86Block<A>>,
    current_block_operands: HashMap<X86NodeRef<A>, Operand<A>>,
    panic_block: Ref<X86Block<A>>,
    next_vreg: usize,
    ctx: &'ctx mut X86TranslationContext<A>,
}

impl<'a, 'ctx, A: Alloc> X86Emitter<'ctx, A> {
    pub fn new(ctx: &'ctx mut X86TranslationContext<A>) -> Self {
        Self {
            current_block: ctx.initial_block(),
            current_block_operands: HashMap::default(),
            panic_block: ctx.panic_block(),
            next_vreg: 0,
            ctx,
        }
    }

    pub fn ctx(&self) -> &X86TranslationContext<A> {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut X86TranslationContext<A> {
        &mut self.ctx
    }

    fn node(&self, node: X86Node<A>) -> X86NodeRef<A> {
        X86NodeRef(Rc::new_in(node, self.ctx().allocator.clone()))
    }

    pub fn next_vreg(&mut self) -> usize {
        let vreg = self.next_vreg;
        self.next_vreg += 1;
        vreg
    }

    pub fn push_instruction(&mut self, instr: Instruction<A>) {
        self.current_block
            .get_mut(self.ctx.arena_mut())
            .append(instr);
    }

    pub fn push_target(&mut self, target: Ref<X86Block<A>>) {
        log::debug!("adding target {target:?} to {:?}", self.current_block);
        self.current_block
            .get_mut(self.ctx.arena_mut())
            .push_next(target);
    }

    // pub fn new_node(&mut self, node: X86Node<A>) -> Ref<X86Node<A>> {
    //     self.arena.insert(node)
    // }

    /// Same as `to_operand` but if the value is a constant, move it to a
    /// register
    fn to_operand_reg_promote(&mut self, node: &X86NodeRef<A>) -> Operand<A> {
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
    fn to_operand_oversize_reg_promote(&mut self, node: &X86NodeRef<A>) -> Operand<A> {
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

    fn to_operand(&mut self, node: &X86NodeRef<A>) -> Operand<A> {
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
                                panic!(
                                    "src ({src_width} bits) must be larger than dst ({dst_width} bits)"
                                );
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
                // todo: test this and use x86 bit insert instructions
                let target = self.to_operand(target);
                let source = self.to_operand(source);

                let start = self.to_operand(start);
                let length = self.to_operand(length);

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

                let width = target.width();

                // mask off target bits
                let mask = Operand::vreg(width, self.next_vreg());

                if let OperandKind::Immediate(64) = length.kind() {
                    self.push_instruction(Instruction::mov(Operand::imm(width, 0), mask).unwrap());
                } else {
                    self.push_instruction(Instruction::mov(Operand::imm(width, 1), mask).unwrap());
                    self.push_instruction(Instruction::shl(length, mask));
                    self.push_instruction(Instruction::sub(Operand::imm(width, 1), mask));
                    self.push_instruction(Instruction::shl(start, mask));
                    self.push_instruction(Instruction::not(mask));
                }

                let masked_target = Operand::vreg(width, self.next_vreg());
                self.push_instruction(Instruction::mov(target, masked_target).unwrap());
                self.push_instruction(Instruction::and(mask, masked_target));

                // shift source by start
                let shifted_source = Operand::vreg(width, self.next_vreg());
                self.push_instruction(Instruction::mov(source, shifted_source).unwrap());
                self.push_instruction(Instruction::shl(start, shifted_source));

                // apply ~mask to source
                {
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

                if self.ctx().memory_mask {
                    let mask = Operand::vreg(Width::_64, self.next_vreg());
                    self.push_instruction(
                        Instruction::mov(Operand::imm(Width::_64, 0x0000_00FF_FFFF_FFFF), mask)
                            .unwrap(),
                    );
                    self.push_instruction(Instruction::and(mask, address));
                }

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

    fn binary_operation_to_operand(&mut self, kind: &BinaryOperationKind<A>) -> Operand<A> {
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
            (Type::Unsigned(l), Type::Unsigned(r)) => {
                let left = self.to_operand_oversize_reg_promote(left);
                let right = self.to_operand_oversize_reg_promote(right);

                match left.width().cmp(&right.width()) {
                    Ordering::Less => {
                        let tmp = Operand::vreg(right.width(), self.next_vreg());
                        self.push_instruction(Instruction::movzx(left, tmp));
                        (right, tmp)
                    }
                    Ordering::Equal => (left, right),
                    Ordering::Greater => {
                        let tmp = Operand::vreg(left.width(), self.next_vreg());
                        self.push_instruction(Instruction::movzx(right, tmp));

                        (left, tmp)
                    }
                }
            }

            (Type::Bits, Type::Unsigned(_)) => {
                let l = self.to_operand_oversize_reg_promote(left);
                let r = self.to_operand_oversize_reg_promote(right);

                if l.width() == r.width() {
                    (l, r)
                } else {
                    todo!("{left:?} {right:?} => {l:?} {r:?}")
                }
            }
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

            (Type::Floating(_), Type::Floating(_)) => todo!(),

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

                let right = if width >= Width::_32 {
                    if let OperandKind::Immediate(i) = *right.kind() {
                        if i > i32::MAX as u64 {
                            let tmp = Operand::vreg(width, self.next_vreg());
                            self.push_instruction(Instruction::mov(right, tmp).unwrap());
                            tmp
                        } else {
                            right
                        }
                    } else {
                        right
                    }
                } else {
                    right
                };

                self.push_instruction(Instruction::or(right, dst));
                dst
            }

            BinaryOperationKind::Xor(_, _) => {
                self.push_instruction(Instruction::mov(left, dst).unwrap());
                self.push_instruction(Instruction::xor(right, dst));
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

impl<'ctx, A: Alloc> Emitter<A> for X86Emitter<'ctx, A> {
    type NodeRef = X86NodeRef<A>;
    type BlockRef = Ref<X86Block<A>>;
    type SymbolRef = X86SymbolRef<A>;

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
        self.node(X86Node {
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
        self.node(X86Node {
            typ,
            kind: NodeKind::GuestRegister { offset },
        })
    }

    fn unary_operation(&mut self, op: UnaryOperationKind<A>) -> Self::NodeRef {
        use UnaryOperationKind::*;

        match &op {
            Not(value) => match value.kind() {
                NodeKind::Constant {
                    value: constant_value,
                    width,
                } => self.node(X86Node {
                    typ: value.typ().clone(),
                    kind: NodeKind::Constant {
                        value: (*constant_value == 0) as u64,
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
                    typ: value.typ().clone(),
                    kind: NodeKind::UnaryOperation(op),
                }),
            },
            Complement(value) => {
                match value.kind() {
                    NodeKind::Constant {
                        value: constant_value,
                        width,
                    } => self.node(X86Node {
                        typ: value.typ().clone(),
                        kind: NodeKind::Constant {
                            value: (!constant_value) & mask(*width), /* only invert the bits that
                                                                      * are
                                                                      * part of the size of the
                                                                      * datatype */
                            width: *width,
                        },
                    }),
                    _ => self.node(X86Node {
                        typ: value.typ().clone(),
                        kind: NodeKind::UnaryOperation(op),
                    }),
                }
            }
            Ceil(value) => {
                let NodeKind::Tuple(values) = value.kind() else {
                    panic!()
                };

                if values
                    .iter()
                    .all(|v| matches!(v.kind(), NodeKind::Constant { .. }))
                {
                    todo!()
                } else {
                    self.node(X86Node {
                        typ: Type::Signed(64),
                        kind: NodeKind::UnaryOperation(op),
                    })
                }
            }

            Floor(value) => {
                let NodeKind::Tuple(values) = value.kind() else {
                    panic!()
                };

                if values
                    .iter()
                    .all(|v| matches!(v.kind(), NodeKind::Constant { .. }))
                {
                    let [num, den] = values.as_slice() else {
                        panic!()
                    };

                    assert_eq!(*num.typ(), Type::Signed(64));
                    assert_eq!(*den.typ(), Type::Signed(64));

                    let (
                        NodeKind::Constant { value: num, .. },
                        NodeKind::Constant { value: den, .. },
                    ) = (num.kind(), den.kind())
                    else {
                        panic!()
                    };

                    let num = *num as i64;
                    let den = *den as i64;

                    let value = num.div_floor(den) as u64;

                    self.node(X86Node {
                        typ: Type::Signed(64),
                        kind: NodeKind::Constant { value, width: 64 },
                    })
                } else {
                    self.node(X86Node {
                        typ: Type::Signed(64),
                        kind: NodeKind::UnaryOperation(op),
                    })
                }
            }

            _ => {
                todo!("{op:?}")
            }
        }
    }

    fn binary_operation(&mut self, op: BinaryOperationKind<A>) -> Self::NodeRef {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value.wrapping_add(*rhs_value),// todo: THIS WILL WRAP AT 64 NOT *width*!
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value.wrapping_sub(*rhs_value),// todo: THIS WILL WRAP AT 64 NOT *width*!
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value * rhs_value,
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
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
                            let mut tuple = Vec::new_in(self.ctx().allocator());
                            tuple.push(num);
                            tuple.push(den);

                            self.create_tuple(tuple)
                        }
                        _ => panic!(),
                    }
                }
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value % rhs_value,
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value | rhs_value,
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },
            Xor(lhs, rhs) => match (lhs.kind(), rhs.kind()) {
                (
                    NodeKind::Constant {
                        value: lhs_value,
                        width,
                    },
                    NodeKind::Constant {
                        value: rhs_value, ..
                    },
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value ^ rhs_value,
                        width: *width,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: lhs_value & rhs_value,
                        width: *width,
                    },
                }),
                (
                    NodeKind::Constant {
                        value: lhs_value, ..
                    },
                    ..,
                ) => {
                    if *lhs_value == 0 {
                        self.constant(0, *rhs.typ())
                    } else {
                        self.node(X86Node {
                            typ: lhs.typ().clone(),
                            kind: NodeKind::BinaryOperation(op),
                        })
                    }
                }
                (
                    ..,
                    NodeKind::Constant {
                        value: rhs_value, ..
                    },
                ) => {
                    if *rhs_value == 0 {
                        self.constant(0, *lhs.typ())
                    } else {
                        self.node(X86Node {
                            typ: rhs.typ().clone(),
                            kind: NodeKind::BinaryOperation(op),
                        })
                    }
                }
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if lhs_value == rhs_value { 1 } else { 0 },
                        width: 1,
                    },
                }),
                _ => self.node(X86Node {
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
                ) => self.node(X86Node {
                    typ: lhs.typ().clone(),
                    kind: NodeKind::Constant {
                        value: if lhs_value != rhs_value { 1 } else { 0 },
                        width: 1,
                    },
                }),
                _ => self.node(X86Node {
                    typ: Type::Unsigned(1),
                    kind: NodeKind::BinaryOperation(op),
                }),
            },

            CompareGreaterThan(_, _)
            | CompareGreaterThanOrEqual(_, _)
            | CompareLessThan(_, _)
            | CompareLessThanOrEqual(_, _) => emit_compare(op, self),

            op => {
                todo!("{op:?}")
            }
        }
    }

    fn ternary_operation(&mut self, op: TernaryOperationKind<A>) -> Self::NodeRef {
        use TernaryOperationKind::*;
        match &op {
            AddWithCarry(src, _dst, _carry) => self.node(X86Node {
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
            _ => self.node(X86Node {
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
            (_, _, _) => self.node(X86Node {
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
            //     self.node(X86Node {
            //         typ,
            //         kind: NodeKind::BitExtract {
            //             value,
            //             start,
            //             length,
            //         },
            //     })
            // }
            // todo: constant start and length with non-constant value can still be specialized?
            _ => self.node(X86Node {
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
                NodeKind::Constant {
                    value: target,
                    width: target_width,
                },
                NodeKind::Constant { value: source, .. },
                NodeKind::Constant { value: start, .. },
                NodeKind::Constant { value: length, .. },
            ) => self.constant(
                bit_insert(*target, *source, *start, *length),
                Type::Unsigned(*target_width),
            ),
            _ => self.node(X86Node {
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
            _ => self.node(X86Node {
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

        if offset == self.ctx().sctlr_el1_offset {
            self.ctx_mut().set_mmu_config_flag(); // block contains an instr that modifies sctlr
        } else if offset == self.ctx().ttbr0_el1_offset || offset == self.ctx().ttbr1_el1_offset {
            self.ctx_mut().set_mmu_needs_invalidate_flag();
        }

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
        self.node(X86Node {
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

        // It occurs to me that the Arm distribution we're running is a 39-bit address
        // space

        //  Well we've got a fucking 48-bit address space

        //  So we can do high and low in one page table?

        //  So, we can just treat their canonical upper addresses, as access in our
        // canonical lower range

        //  Yes

        //   Just a simple bit of bit shifting and masking should do the trick

        // Amazing

        // wait so the high address I'm seeing in the store instruction is a bug then?

        // So, Arm's address space looks like this:
        // 0000 0000 0000 0000 .. 0000 007F FFFF FFFF
        // FFFF FF80 0000 0000 .. FFFF FFFF FFFF FFFF

        // x86_64 with 48-bit addressing looks like
        // 0000 0000 0000 0000 .. 0000 7FFF FFFF FFFF
        // FFFF 8000 0000 0000 .. FFFF FFFF FFFF FFFF

        // if we mask highest 6 nibbles we get a contiguous address space

        if self.ctx().memory_mask {
            let mask = Operand::vreg(Width::_64, self.next_vreg());
            self.push_instruction(
                Instruction::mov(Operand::imm(Width::_64, 0x0000_00FF_FFFF_FFFF), mask).unwrap(),
            );
            self.push_instruction(Instruction::and(mask, address));
        }

        self.push_instruction(
            Instruction::mov(value, Operand::mem_base_displ(width, *address_reg, 0)).unwrap(),
        );
    }

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    ) {
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
            }
        }
    }

    fn jump(&mut self, target: Self::BlockRef) {
        self.push_instruction(Instruction::jmp(target.clone()));
        self.push_target(target.clone());
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

        self.node(X86Node {
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
        self.node(X86Node {
            typ: Type::Unsigned(4),
            kind: NodeKind::GetFlags { operation },
        })
    }

    fn panic(&mut self, msg: &str) {
        let n = self.to_operand(&self.node(X86Node {
            typ: Type::Unsigned(8),
            kind: NodeKind::Constant {
                value: match msg {
                    "undefined terminator" => 0x50,
                    "default terminator" => 0x51,
                    "constant assert failed" => 0x52,
                    "panic block" => 0x53,
                    "match" => 0x54,
                    _ => todo!("{msg}"),
                },
                width: 8,
            },
        }));

        self.push_instruction(Instruction::int(n));
    }

    fn create_tuple(&mut self, values: Vec<Self::NodeRef, A>) -> Self::NodeRef {
        self.node(X86Node {
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
                        NodeKind::ReadStackVariable { .. } => self.constant(64, Type::Unsigned(16)),
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

#[derive_where(Debug)]
pub struct X86NodeRef<A: Alloc>(Rc<X86Node<A>, A>);

impl<A: Alloc> Clone for X86NodeRef<A> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<A: Alloc> Hash for X86NodeRef<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl<A: Alloc> Eq for X86NodeRef<A> {}

impl<A: Alloc> PartialEq for X86NodeRef<A> {
    fn eq(&self, other: &X86NodeRef<A>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<A: Alloc> X86NodeRef<A> {
    pub fn kind(&self) -> &NodeKind<A> {
        &self.0.kind
    }

    pub fn typ(&self) -> &Type {
        &self.0.typ
    }
}

#[derive_where(Debug)]
pub struct X86Node<A: Alloc> {
    pub typ: Type,
    pub kind: NodeKind<A>,
}

#[derive_where(Debug, PartialEq, Eq)]
pub enum NodeKind<A: Alloc> {
    Constant {
        value: u64,
        width: u16,
    },
    GuestRegister {
        offset: u64,
    },
    ReadMemory {
        address: X86NodeRef<A>,
    },
    UnaryOperation(UnaryOperationKind<A>),
    BinaryOperation(BinaryOperationKind<A>),
    TernaryOperation(TernaryOperationKind<A>),
    Cast {
        value: X86NodeRef<A>,
        kind: CastOperationKind,
    },
    Shift {
        value: X86NodeRef<A>,
        amount: X86NodeRef<A>,
        kind: ShiftOperationKind,
    },
    ReadStackVariable {
        // positive offset here (will be subtracted from RSP)
        offset: usize,
        width: u16,
    },
    BitExtract {
        value: X86NodeRef<A>,
        start: X86NodeRef<A>,
        length: X86NodeRef<A>,
    },
    BitInsert {
        target: X86NodeRef<A>,
        source: X86NodeRef<A>,
        start: X86NodeRef<A>,
        length: X86NodeRef<A>,
    },
    GetFlags {
        operation: X86NodeRef<A>,
    },
    Tuple(Vec<X86NodeRef<A>, A>),
    Select {
        condition: X86NodeRef<A>,
        true_value: X86NodeRef<A>,
        false_value: X86NodeRef<A>,
    },
}

#[derive(Clone)]
#[derive_where(Debug, PartialEq, Eq)]
pub enum BinaryOperationKind<A: Alloc> {
    Add(X86NodeRef<A>, X86NodeRef<A>),
    Sub(X86NodeRef<A>, X86NodeRef<A>),
    Multiply(X86NodeRef<A>, X86NodeRef<A>),
    Divide(X86NodeRef<A>, X86NodeRef<A>),
    Modulo(X86NodeRef<A>, X86NodeRef<A>),
    And(X86NodeRef<A>, X86NodeRef<A>),
    Or(X86NodeRef<A>, X86NodeRef<A>),
    Xor(X86NodeRef<A>, X86NodeRef<A>),
    PowI(X86NodeRef<A>, X86NodeRef<A>),
    CompareEqual(X86NodeRef<A>, X86NodeRef<A>),
    CompareNotEqual(X86NodeRef<A>, X86NodeRef<A>),
    CompareLessThan(X86NodeRef<A>, X86NodeRef<A>),
    CompareLessThanOrEqual(X86NodeRef<A>, X86NodeRef<A>),
    CompareGreaterThan(X86NodeRef<A>, X86NodeRef<A>),
    CompareGreaterThanOrEqual(X86NodeRef<A>, X86NodeRef<A>),
}

#[derive(Clone)]
#[derive_where(Debug, PartialEq, Eq)]
pub enum UnaryOperationKind<A: Alloc> {
    Not(X86NodeRef<A>),
    Negate(X86NodeRef<A>),
    Complement(X86NodeRef<A>),
    Power2(X86NodeRef<A>),
    Absolute(X86NodeRef<A>),
    Ceil(X86NodeRef<A>),
    Floor(X86NodeRef<A>),
    SquareRoot(X86NodeRef<A>),
}

#[derive(Clone)]
#[derive_where(Debug, PartialEq, Eq)]
pub enum TernaryOperationKind<A: Alloc> {
    AddWithCarry(X86NodeRef<A>, X86NodeRef<A>, X86NodeRef<A>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastOperationKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    Reinterpret,
    Convert,
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub struct X86Block<A: Alloc> {
    instructions: Vec<Instruction<A>, A>,
    next: Vec<Ref<X86Block<A>>, A>,
    linked: bool,
    mark: X86BlockMark,
}

impl<A: Alloc> X86Block<A> {
    pub fn new_in(allocator: A) -> Self {
        Self {
            instructions: Vec::new_in(allocator.clone()),
            next: Vec::new_in(allocator),
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

    pub fn append(&mut self, instruction: Instruction<A>) {
        self.instructions.push(instruction);
    }

    pub fn allocate_registers<R: RegisterAllocator>(&mut self, allocator: &mut R) {
        allocator.allocate(self.instructions_mut());
    }

    pub fn instructions(&self) -> &[Instruction<A>] {
        &self.instructions
    }

    pub fn instructions_mut(&mut self) -> &mut Vec<Instruction<A>, A> {
        &mut self.instructions
    }

    pub fn next_blocks(&self) -> &[Ref<X86Block<A>>] {
        &self.next
    }

    pub fn clear_next_blocks(&mut self) {
        self.next.clear();
    }

    pub fn push_next(&mut self, target: Ref<X86Block<A>>) {
        self.next.push(target);
        if self.next.len() > 2 {
            panic!(
                "bad, blocks should not have more than 2 real targets (asserts complicate things)"
            )
        }
    }
}

impl<A: Alloc> Debug for X86Block<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for instr in &self.instructions {
            writeln!(f, "\t{instr}")?;
        }

        Ok(())
    }
}

#[derive(Clone)]
#[derive_where(Debug)]
pub struct X86SymbolRef<A: Alloc>(pub Rc<RefCell<Option<X86NodeRef<A>>>, A>);

fn encode_compare<A: Alloc>(
    kind: &BinaryOperationKind<A>,
    emitter: &mut X86Emitter<A>,
    right: X86NodeRef<A>, /* TODO: this was flipped in order to make tests pass, unflip right
                           * and left and fix the body of the function */
    left: X86NodeRef<A>,
) -> Operand<A> {
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
        (Type::Unsigned(_) | Type::Bits, Type::Unsigned(_) | Type::Bits) => false,
        (Type::Signed(_), Type::Signed(_)) => true,
        _ => panic!("different types in comparison:\n{left:?}\nand\n{right:?}"),
    };

    let left_op = emitter.to_operand(&left);
    let right_op = emitter.to_operand(&right);

    // only valid compare instructions are (source-destination):
    // reg reg
    // reg mem
    // mem reg
    // imm reg
    // imm mem

    // anything else (imm on the right) must be reworked

    match (left_op.kind(), right_op.kind()) {
        (Register(_), Register(_))
        | (Register(_), Memory { .. })
        | (Memory { .. }, Register(_))
        | (Immediate(_), Register(_))
        | (Immediate(_), Memory { .. })
        | (Memory { .. }, Memory { .. }) => {
            let left = if let (Memory { .. }, Memory { .. }) = (left_op.kind(), right_op.kind()) {
                let new_left = Operand::vreg(left_op.width(), emitter.next_vreg());
                emitter.push_instruction(Instruction::mov(left_op, new_left).unwrap());
                new_left
            } else {
                left_op
            };

            emitter.push_instruction(Instruction::cmp(left, right_op));

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
                _ => todo!("panic!(\"{{kind:?}} is not a compare\")"),
            });

            dst
        }

        (Memory { .. }, Immediate(_)) | (Register(_), Immediate(_)) => {
            emitter.push_instruction(Instruction::cmp(right_op, left_op));
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
                _ => todo!(), //panic!("{kind:?} is not a compare"),
            });

            dst
        }

        (Immediate(_), Immediate(_)) => {
            panic!(
                "why was this not const evaluated? {:?} {:?} {:?}",
                left,
                right,
                todo!() // kind
            )
        }
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

fn emit_compare<A: Alloc>(
    op: BinaryOperationKind<A>,
    emitter: &mut X86Emitter<A>,
) -> X86NodeRef<A> {
    use BinaryOperationKind::*;

    let (CompareLessThan(left, right)
    | CompareLessThanOrEqual(left, right)
    | CompareGreaterThan(left, right)
    | CompareGreaterThanOrEqual(left, right)) = &op
    else {
        panic!("only greater/less than comparisons should be handled here");
    };

    match (left.kind(), right.kind()) {
        (
            NodeKind::Constant {
                value: left_value, ..
            },
            NodeKind::Constant {
                value: right_value, ..
            },
        ) => {
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

            emitter.node(X86Node {
                typ: left.typ().clone(),
                kind: NodeKind::Constant {
                    value: result as u64,
                    width: 1,
                },
            })
        }
        // attempt const eval of reals
        (NodeKind::Tuple(left_real), NodeKind::Tuple(right_real)) => {
            if left_real
                .iter()
                .all(|v| matches!(v.kind(), NodeKind::Constant { .. }))
                && right_real
                    .iter()
                    .all(|v| matches!(v.kind(), NodeKind::Constant { .. }))
            {
                match (left_real.clone().as_slice(), right_real.as_slice()) {
                    ([left_num, left_den], [right_num, right_den]) => {
                        let left = emitter.binary_operation(BinaryOperationKind::Multiply(
                            left_num.clone(),
                            right_den.clone(),
                        ));

                        let right = emitter.binary_operation(BinaryOperationKind::Multiply(
                            left_den.clone(),
                            right_num.clone(),
                        ));

                        assert!(matches!(left.kind(), NodeKind::Constant { .. }));
                        assert!(matches!(right.kind(), NodeKind::Constant { .. }));

                        let op = match op {
                            CompareLessThan(_, _) => CompareLessThan(left, right),
                            CompareLessThanOrEqual(_, _) => CompareLessThanOrEqual(left, right),
                            CompareGreaterThan(_, _) => CompareGreaterThan(left, right),
                            CompareGreaterThanOrEqual(_, _) => {
                                CompareGreaterThanOrEqual(left, right)
                            }
                            _ => panic!(),
                        };

                        emit_compare(op, emitter)
                    }
                    _ => panic!(),
                }
            } else {
                // else emit an X86 node
                emitter.node(X86Node {
                    typ: Type::Unsigned(1),
                    kind: NodeKind::BinaryOperation(op),
                })
            }
        }
        _ => {
            // else emit an X86 node
            emitter.node(X86Node {
                typ: Type::Unsigned(1),
                kind: NodeKind::BinaryOperation(op),
            })
        }
    }
}
