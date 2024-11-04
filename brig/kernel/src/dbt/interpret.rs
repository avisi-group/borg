use {
    crate::dbt::{bit_extract, bit_insert},
    alloc::vec::Vec,
    common::{
        arena::Ref,
        intern::InternedString,
        mask::mask,
        rudder::{
            block::Block,
            constant_value::ConstantValue,
            statement::{
                BinaryOperationKind, CastOperationKind, ShiftOperationKind, Statement,
                TernaryOperationKind, UnaryOperationKind,
            },
            types::{PrimitiveType, PrimitiveTypeClass, Type},
            Model,
        },
        HashMap,
    },
    core::{
        borrow::Borrow,
        cmp::{max, Ordering},
        ops::{Add, BitAnd, BitOr, Div, Mul, Sub},
        panic,
    },
};

pub fn interpret(
    model: &Model,
    function_name: &str,
    arguments: &[Value],
    register_file: *mut u8,
) -> Value {
    log::debug!("interpreting {function_name}");
    let function_name = InternedString::from(function_name);
    let function = model.functions().get(&function_name).unwrap();

    let mut interpreter = Interpreter::new(model, function_name, register_file);

    // insert arguments
    interpreter.locals.extend(
        function
            .parameters()
            .into_iter()
            .zip(arguments.iter())
            .map(|(symbol, value)| (symbol.name(), value.clone())),
    );

    let mut current_block = function.entry_block();
    loop {
        match interpreter.interpret_block(current_block) {
            BlockResult::NextBlock(next) => current_block = next,
            BlockResult::ReturnValue(value) => return value,
        }
    }
}

struct Interpreter<'f> {
    model: &'f Model,
    function_name: InternedString,
    // local variables
    locals: HashMap<InternedString, Value>,
    // value of previously evaluated statements
    statement_values: HashMap<Ref<Statement>, Value>,
    register_file: *mut u8,
    // nzcv
    flags: u8,
}

impl<'f> Interpreter<'f> {
    fn new(model: &'f Model, function_name: InternedString, register_file: *mut u8) -> Self {
        Self {
            model,
            function_name,
            locals: HashMap::default(),
            statement_values: HashMap::default(),
            register_file,
            flags: 0,
        }
    }

    fn resolve<R: Borrow<Ref<Statement>>>(&self, statement_ref: R) -> Value {
        self.statement_values
            .get(statement_ref.borrow())
            .unwrap_or_else(|| {
                panic!(
                    "failed to resolve {:?} in {:?}\nstatement_values: {:?}",
                    statement_ref.borrow(),
                    self.function_name,
                    self.statement_values,
                )
            })
            .clone()
    }

    fn resolve_u64<R: Borrow<Ref<Statement>>>(&self, statement_ref: R) -> u64 {
        match self.resolve(statement_ref) {
            Value::UnsignedInteger {
                value: u,
                length: _,
            } => u,
            Value::SignedInteger {
                value: i,
                length: _,
            } => i
                .try_into()
                .unwrap_or_else(|_| panic!("cannot resolve {i} as u64")),
            _ => panic!(),
        }
    }

    fn interpret_block(&mut self, block_ref: Ref<Block>) -> BlockResult {
        log::trace!("{}: block {block_ref:?}", self.function_name);
        self.statement_values.clear();
        let block = block_ref.get(
            self.model
                .functions()
                .get(&self.function_name)
                .unwrap()
                .arena(),
        );
        for statement_ref in block.statements() {
            let statement = statement_ref.get(block.arena());
            let value = match statement {
                Statement::Constant { typ, value } => Some(Value::from_constant(value, typ)),
                Statement::ReadVariable { symbol } => Some(
                    self.locals
                        .get(&symbol.name())
                        .unwrap_or_else(|| panic!("no local found for {symbol:?}"))
                        .clone(),
                ),
                Statement::ReadRegister { typ, offset } => {
                    let width = match typ {
                        Type::Primitive(typ) => typ.width(),
                        t => todo!("{t}"),
                    };

                    let offset = self.resolve_u64(offset);
                    let value = match width {
                        1..=8 => self.read_reg::<u8>(offset) as u64,
                        9..=16 => self.read_reg::<u16>(offset) as u64,
                        17..=32 => self.read_reg::<u32>(offset) as u64,
                        33..=64 => self.read_reg::<u64>(offset),
                        65..=128 => u64::try_from(self.read_reg::<u128>(offset)).unwrap(),

                        w => {
                            log::trace!(
                                "tried to read a {w} bit register offset {offset}, returning 0"
                            );
                            0
                        }
                    };

                    Some(match typ {
                        Type::Primitive(PrimitiveType {
                            tc: PrimitiveTypeClass::UnsignedInteger,
                            element_width_in_bits,
                        }) => Value::UnsignedInteger {
                            value: value & mask(u32::try_from(*element_width_in_bits).unwrap()),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        },
                        Type::Primitive(PrimitiveType {
                            tc: PrimitiveTypeClass::SignedInteger,
                            element_width_in_bits,
                        }) => Value::SignedInteger {
                            value: (value & mask(u32::try_from(*element_width_in_bits).unwrap()))
                                as i64,
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        },
                        t => todo!("{t}"),
                    })
                }
                Statement::ReadMemory { .. } => todo!(),
                Statement::ReadPc => todo!(),
                Statement::GetFlags { operation: _ } => {
                    // todo: technically should get the last statement or
                    // something
                    Some(Value::UnsignedInteger {
                        value: u64::from(self.flags),
                        length: 4,
                    })
                }
                Statement::UnaryOperation { kind, value } => {
                    let value = self.resolve(value);

                    match kind {
                        UnaryOperationKind::Not => {
                            let Value::UnsignedInteger { value, .. } = value else {
                                panic!()
                            };

                            Some(Value::UnsignedInteger {
                                value: (value == 0) as u64,
                                length: 1,
                            })
                        }
                        UnaryOperationKind::Complement => {
                            let Value::UnsignedInteger { value, length } = value else {
                                todo!()
                            };

                            Some(Value::UnsignedInteger {
                                value: !value,
                                length,
                            })
                        }
                        _ => todo!("{kind:?} {value:?}"),
                    }
                }
                Statement::BinaryOperation { kind, lhs, rhs } => {
                    let left = self.resolve(lhs);
                    let right = self.resolve(rhs);

                    Some(match kind {
                        BinaryOperationKind::CompareEqual => Value::UnsignedInteger {
                            value: (left == right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::CompareNotEqual => Value::UnsignedInteger {
                            value: (left != right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::CompareLessThan => Value::UnsignedInteger {
                            value: (left < right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::CompareLessThanOrEqual => Value::UnsignedInteger {
                            value: (left <= right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::CompareGreaterThan => Value::UnsignedInteger {
                            value: (left > right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::CompareGreaterThanOrEqual => Value::UnsignedInteger {
                            value: (left >= right) as u64,
                            length: 1,
                        },
                        BinaryOperationKind::Sub => left - right,
                        BinaryOperationKind::Add => left + right,
                        BinaryOperationKind::Multiply => left * right,
                        BinaryOperationKind::Or => left | right,
                        BinaryOperationKind::And => left & right,
                        BinaryOperationKind::Divide => left / right,
                        _ => todo!("{kind:?}"),
                    })
                }
                Statement::TernaryOperation { kind, a, b, c } => match kind {
                    TernaryOperationKind::AddWithCarry => {
                        let Value::UnsignedInteger { length, value: a } = self.resolve(a) else {
                            panic!()
                        };
                        assert_eq!(length, 64);
                        let Value::UnsignedInteger { length, value: b } = self.resolve(b) else {
                            panic!()
                        };
                        assert_eq!(length, 64);
                        let Value::UnsignedInteger { length, value: c } = self.resolve(c) else {
                            panic!()
                        };
                        assert_eq!(length, 1);

                        // function AddWithCarry (x, y, carry_in) = {
                        //     let 'unsigned_sum = UInt(x) + UInt(y) + UInt(carry_in);
                        //     let 'signed_sum = SInt(x) + SInt(y) + UInt(carry_in);
                        //     let result : bits('N) = unsigned_sum['N - 1 .. 0];
                        //     let n : bits(1) = [result['N - 1]];
                        //     let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
                        //     let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
                        //     let v : bits(1) = if SInt(result) == signed_sum then 0b0 else 0b1;
                        //     return((result, ((n @ z) @ c) @ v))
                        // }

                        // not correct for lengths other than 64
                        let unsigned_sum = u128::from(a) + u128::from(b) + u128::from(c);
                        let signed_sum =
                            i128::from(a as i64) + i128::from(b as i64) + i128::from(c as i64);
                        let u_result = unsigned_sum as u64;
                        let i_result = unsigned_sum as i64;

                        let n = u_result >> 63;
                        let z = (u_result == 0) as u64;
                        let c = (u128::from(u_result) != unsigned_sum) as u64;
                        let v = (i128::from(i_result) != signed_sum) as u64;

                        self.flags = u8::try_from(n << 3 | z << 2 | c << 1 | v).unwrap();

                        Some(Value::UnsignedInteger {
                            value: u_result,
                            length: 64,
                        })
                    }
                },

                Statement::ShiftOperation {
                    kind,
                    value,
                    amount,
                } => {
                    let amount = self.resolve_u64(amount);

                    let value = self.resolve(value);

                    match (kind, &value) {
                        (
                            ShiftOperationKind::LogicalShiftLeft,
                            Value::UnsignedInteger { value, length },
                        ) => {
                            let (value, did_overflow) =
                                value.overflowing_shl(u32::try_from(amount).unwrap());

                            if did_overflow {
                                log::trace!("overflowed during lsl of {value} by {amount}");
                            }

                            Some(Value::UnsignedInteger {
                                value: value & mask(*length),
                                length: *length,
                            })
                        }
                        (
                            ShiftOperationKind::LogicalShiftRight,
                            Value::UnsignedInteger { value, length },
                        ) => {
                            let (value, did_overflow) =
                                value.overflowing_shr(u32::try_from(amount).unwrap());

                            if did_overflow {
                                log::trace!("overflowed during lsl of {value} by {amount}");
                            }

                            Some(Value::UnsignedInteger {
                                value: value & mask(*length),
                                length: *length,
                            })
                        }
                        (
                            ShiftOperationKind::LogicalShiftLeft,
                            Value::SignedInteger { value, length },
                        ) => Some(Value::SignedInteger {
                            value: value << amount,
                            length: *length,
                        }),
                        _ => todo!("{value:?} {kind:?} by {amount}"),
                    }
                }
                Statement::Call { target, args, .. } => {
                    log::trace!(
                        "{}: block {block_ref:?}: call {target:?}",
                        self.function_name
                    );

                    Some(interpret(
                        &self.model,
                        target.as_ref(),
                        &args.iter().map(|a| self.resolve(a)).collect::<Vec<_>>(),
                        self.register_file,
                    ))
                }
                Statement::Cast {
                    kind,
                    typ: dest_typ,
                    value,
                } => {
                    let source_typ = value.get(block.arena()).typ(block.arena());
                    let value = self.resolve(value);

                    match (&kind, &dest_typ, &value) {
                        (
                            CastOperationKind::SignExtend,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::SignedInteger,
                                element_width_in_bits,
                            }),
                            Value::SignedInteger { value, .. },
                        ) => Some(Value::SignedInteger {
                            value: sign_extend(
                                *value,
                                source_typ.width_bits(),
                                *element_width_in_bits,
                            ),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::Truncate,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::UnsignedInteger,
                                element_width_in_bits,
                            }),
                            Value::UnsignedInteger { value, .. },
                        ) => Some(Value::UnsignedInteger {
                            value: value & mask(u32::try_from(*element_width_in_bits).unwrap()),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::Reinterpret,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::SignedInteger,
                                element_width_in_bits,
                            }),
                            Value::UnsignedInteger { value, .. },
                        ) => Some(Value::SignedInteger {
                            value: i64::try_from(
                                *value & mask(u32::try_from(*element_width_in_bits).unwrap()),
                            )
                            .unwrap(),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::Reinterpret,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::SignedInteger,
                                element_width_in_bits,
                            }),
                            Value::SignedInteger { value, .. },
                        ) => Some(Value::SignedInteger {
                            value: i64::try_from(
                                *value as u64
                                    & mask(u32::try_from(*element_width_in_bits).unwrap()),
                            )
                            .unwrap(),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::ZeroExtend,
                            Type::Bits,
                            Value::UnsignedInteger { value, length },
                        ) => Some(Value::UnsignedInteger {
                            value: *value,
                            length: *length,
                        }),
                        (
                            CastOperationKind::ZeroExtend,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::UnsignedInteger,
                                element_width_in_bits,
                            }),
                            Value::UnsignedInteger { value, .. },
                        ) => Some(Value::UnsignedInteger {
                            value: *value,
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::Reinterpret,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::UnsignedInteger,
                                element_width_in_bits,
                            }),
                            Value::UnsignedInteger { value, length },
                        ) => {
                            if *element_width_in_bits == usize::from(*length) {
                                Some(Value::UnsignedInteger {
                                    value: *value,
                                    length: *length,
                                })
                            } else {
                                todo!()
                            }
                        }
                        (
                            CastOperationKind::ZeroExtend,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::SignedInteger,
                                element_width_in_bits,
                            }),
                            Value::UnsignedInteger { value, .. },
                        ) => Some(Value::SignedInteger {
                            value: i64::try_from(*value).unwrap(),
                            length: u16::try_from(*element_width_in_bits).unwrap(),
                        }),
                        (
                            CastOperationKind::Truncate,
                            Type::Primitive(PrimitiveType {
                                tc: PrimitiveTypeClass::UnsignedInteger,
                                element_width_in_bits,
                            }),
                            Value::SignedInteger { value, .. },
                        ) => {
                            let length = u16::try_from(*element_width_in_bits).unwrap();
                            Some(Value::UnsignedInteger {
                                value: u64::try_from(*value).unwrap() & mask(length),
                                length,
                            })
                        }
                        (CastOperationKind::Convert, Type::Bits, Value::UnsignedInteger { .. }) => {
                            Some(value)
                        }
                        (k, t, v) => todo!("{k:?} {t:?} {v:?}"),
                    }
                }
                Statement::BitsCast {
                    kind,
                    typ,
                    value,
                    length,
                } => {
                    let value = self.resolve(value);
                    let target_length = u16::try_from(self.resolve_u64(length)).unwrap();
                    match (kind, typ, &value) {
                        (
                            CastOperationKind::ZeroExtend,
                            Type::Bits,
                            Value::UnsignedInteger { value, length },
                        ) => {
                            if target_length > *length {
                                Some(Value::UnsignedInteger {
                                    value: value & mask(target_length),
                                    length: target_length,
                                })
                            } else {
                                panic!();
                            }
                        }
                        (
                            CastOperationKind::SignExtend,
                            Type::Bits,
                            Value::UnsignedInteger { value, length },
                        ) => {
                            if target_length > *length {
                                Some(Value::UnsignedInteger {
                                    value: value & mask(target_length),
                                    length: target_length,
                                })
                            } else {
                                panic!();
                            }
                        }
                        _ => todo!("{kind:?} {typ:?} {value:?} {length}"),
                    }
                }
                Statement::Select {
                    condition,
                    true_value,
                    false_value,
                } => {
                    let condition = self.resolve_u64(condition);

                    Some(self.resolve(if condition != 0 {
                        true_value
                    } else {
                        false_value
                    }))
                }
                Statement::BitExtract {
                    value,
                    start,
                    length,
                } => {
                    let value = self.resolve(value);
                    let start = self.resolve_u64(start);
                    let length = self.resolve_u64(length);

                    Some(match value {
                        Value::UnsignedInteger { value, .. } => Value::UnsignedInteger {
                            value: bit_extract(value, start, length),
                            length: u16::try_from(length).unwrap(),
                        },
                        // todo: test/verify this
                        Value::SignedInteger { value, .. } => Value::SignedInteger {
                            value: bit_extract(value as u64, start, length) as i64,
                            length: u16::try_from(length).unwrap(),
                        },
                        _ => todo!("{value:?}"),
                    })
                }
                Statement::BitInsert {
                    target,
                    source,
                    start,
                    length,
                } => {
                    let Value::UnsignedInteger {
                        value: target,
                        length: target_length,
                    } = self.resolve(target)
                    else {
                        panic!()
                    };

                    let source = self.resolve_u64(source);
                    let start = self.resolve_u64(start);
                    let length = self.resolve_u64(length);
                    Some(Value::UnsignedInteger {
                        value: bit_insert(target, source, start, length),
                        length: target_length,
                    })
                }
                Statement::ReadElement { .. } => todo!(),
                Statement::AssignElement { .. } => todo!(),
                Statement::CreateBits { value, length } => {
                    let value = self.resolve_u64(value);
                    let length = self.resolve_u64(length);

                    Some(Value::UnsignedInteger {
                        value,
                        length: u16::try_from(length).unwrap(),
                    })
                }
                Statement::SizeOf { value } => {
                    let value = self.resolve(value);
                    match value {
                        Value::UnsignedInteger { length, .. } => Some(Value::UnsignedInteger {
                            value: u64::from(length),
                            length: 16,
                        }),
                        _ => todo!("size-of {value:?}"),
                    }
                }
                Statement::MatchesUnion { .. } => todo!(),
                Statement::UnwrapUnion { .. } => todo!(),
                Statement::CreateTuple(vec) => {
                    Some(Value::Tuple(vec.iter().map(|s| self.resolve(s)).collect()))
                }
                Statement::TupleAccess { index, source } => {
                    let Value::Tuple(values) = self.resolve(source) else {
                        panic!()
                    };

                    Some(values[*index].clone())
                }

                Statement::WriteVariable { symbol, value } => {
                    self.locals.insert(symbol.name(), self.resolve(value));
                    None
                }
                Statement::WriteRegister { offset, value } => {
                    let (value, width) = match self.resolve(value) {
                        Value::UnsignedInteger { value, length } => (value, length),
                        Value::SignedInteger { value, length } => {
                            (value as u64 & mask(length), length)
                        }
                        Value::Unit => (0, 1), /* todo: investigate */
                        // why we are
                        // writing units to
                        // unions
                        t => todo!("{t:?}"),
                    };

                    let offset = self.resolve_u64(offset);

                    match width {
                        1..=8 => self.write_reg(offset, u16::try_from(value).unwrap()),
                        9..=16 => self.write_reg(offset, u16::try_from(value).unwrap()),
                        17..=32 => self.write_reg(offset, u32::try_from(value).unwrap()),
                        33..=64 => self.write_reg(offset, value),
                        65..=128 => {
                            self.write_reg(offset, value);
                            self.write_reg(offset + 8, 0u64); // todo: hack
                        }
                        w => {
                            log::trace!("tried to write {value} to a {w} bit register offset {offset}, did nothing");
                        }
                    }

                    None
                }
                Statement::WriteMemory { .. } => todo!(),
                Statement::WritePc { value } => {
                    self.write_reg(self.model.reg_offset("_PC") as u64, self.resolve_u64(value));
                    None
                }
                Statement::PhiNode { .. } => todo!(),

                Statement::Jump { target } => return BlockResult::NextBlock(*target),
                Statement::Branch {
                    condition,
                    true_target,
                    false_target,
                } => {
                    let condition = self.resolve_u64(condition);

                    return BlockResult::NextBlock(if condition != 0 {
                        *true_target
                    } else {
                        *false_target
                    });
                }

                Statement::Return { value } => {
                    return BlockResult::ReturnValue(self.resolve(value));
                }

                Statement::Panic(v) => {
                    let v = self.resolve(v);
                    panic!("panic! {v:?}")
                }
                Statement::Undefined => panic!("undefined"),
                Statement::Assert { condition } => {
                    let condition = self.resolve_u64(condition);
                    assert!(condition != 0);
                    None
                }
            };

            log::trace!(
                "{}: block {block_ref:?}: {statement_ref:?} = {value:?}",
                self.function_name
            );

            if let Some(value) = value {
                self.statement_values.insert(*statement_ref, value);
            }
        }

        unreachable!("block must end in a panic, jump, return, or branch")
    }

    fn read_reg<T>(&self, offset: u64) -> T {
        unsafe {
            (self.register_file.add(usize::try_from(offset).unwrap()) as *mut T).read_unaligned()
        }
    }

    fn write_reg<T>(&self, offset: u64, value: T) {
        unsafe {
            (self.register_file.add(usize::try_from(offset).unwrap()) as *mut T)
                .write_unaligned(value)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    UnsignedInteger { value: u64, length: u16 },
    SignedInteger { value: i64, length: u16 },
    FloatingPoint(f64),
    String(InternedString),
    Unit,
    Tuple(Vec<Value>),
}

impl Value {
    pub fn from_constant(value: &ConstantValue, typ: &Type) -> Self {
        match value {
            ConstantValue::UnsignedInteger(u) => Value::UnsignedInteger {
                value: *u,
                length: u16::try_from(typ.width_bits()).unwrap(),
            },
            ConstantValue::SignedInteger(i) => Value::SignedInteger {
                value: *i,
                length: u16::try_from(typ.width_bits()).unwrap(),
            },
            ConstantValue::FloatingPoint(f) => Value::FloatingPoint(*f),
            ConstantValue::Rational(_ratio) => todo!(),
            ConstantValue::String(interned_string) => Value::String(*interned_string),
            ConstantValue::Unit => Value::Unit,
            ConstantValue::Tuple(vec) => {
                let Type::Tuple(types) = typ else {
                    unreachable!()
                };
                Value::Tuple(
                    vec.iter()
                        .zip(types)
                        .map(|(v, t)| Value::from_constant(v, t))
                        .collect(),
                )
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (
                Value::UnsignedInteger {
                    value: left,
                    length: _left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: _right_length,
                },
            ) => {
                //assert_eq!(left_length, right_length);
                left.partial_cmp(right)
            }
            (
                Value::SignedInteger {
                    value: left,
                    length: _left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: _right_length,
                },
            ) => {
                //assert_eq!(left_length, right_length);
                left.partial_cmp(right)
            }
            (l, r) => todo!("{l:?} {r:?}"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left - right,
                length: max(left_length, right_length),
            },
            (
                Value::UnsignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::UnsignedInteger {
                value: left - right,
                length: max(left_length, right_length),
            },
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left - i64::try_from(right).unwrap(),
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left + right,
                length: max(left_length, right_length),
            },
            (
                Value::UnsignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: i64::try_from(left).unwrap() + right,
                length: max(left_length, right_length),
            },
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left + i64::try_from(right).unwrap(),
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left * right,
                length: max(left_length, right_length),
            },
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left * i64::try_from(right).unwrap(),
                length: max(left_length, right_length),
            },
            (
                Value::UnsignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::UnsignedInteger {
                value: left * right,
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::UnsignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::UnsignedInteger {
                value: left | right,
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::UnsignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::UnsignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::UnsignedInteger {
                value: left & right,
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}
impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (
                Value::SignedInteger {
                    value: left,
                    length: left_length,
                },
                Value::SignedInteger {
                    value: right,
                    length: right_length,
                },
            ) => Value::SignedInteger {
                value: left / right,
                length: max(left_length, right_length),
            },
            (left, right) => todo!("{left:?} {right:?}"),
        }
    }
}

enum BlockResult {
    NextBlock(Ref<Block>),
    ReturnValue(Value),
}

fn sign_extend(value: i64, source_width: usize, dest_width: usize) -> i64 {
    let shift_amount = usize::try_from(i64::BITS).unwrap() - source_width;

    (((value << shift_amount) >> shift_amount) as u64 & mask(u32::try_from(dest_width).unwrap()))
        as i64
}
