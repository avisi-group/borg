use core::cmp::Ordering;

use crate::dbt::{
    Alloc,
    emitter::Type,
    x86::{
        Emitter,
        emitter::{
            BinaryOperationKind, CastOperationKind, NodeKind, ShiftOperationKind,
            TernaryOperationKind, UnaryOperationKind, X86Emitter, X86NodeRef,
        },
        encoder::{
            Instruction, Opcode, Operand, OperandKind, PhysicalRegister,
            Register::{self},
            width::Width,
        },
    },
};

impl<'a, 'ctx, A: Alloc> X86Emitter<'ctx, A> {
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
            if *value > (i32::MAX as u64) {
                let tmp = Operand::vreg(op.width(), self.next_vreg());
                self.push_instruction(Instruction::mov(op, tmp).unwrap());
                return tmp;
            }
        }

        op
    }

    pub(super) fn to_operand(&mut self, node: &X86NodeRef<A>) -> Operand<A> {
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
                self.push_instruction(Instruction::mov(false_value, dest).unwrap());
                self.push_instruction(Instruction::test(condition, condition));
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
