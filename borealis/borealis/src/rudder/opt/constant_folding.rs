use common::{
    arena::{Arena, Ref},
    rudder::{
        block::Block,
        constant_value::ConstantValue,
        function::Function,
        statement::{BinaryOperationKind, CastOperationKind, Statement, UnaryOperationKind},
        types::{PrimitiveTypeClass, Type},
    },
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    //trace!("constant folding {}", f.name());
    for block in f.block_iter().collect::<Vec<_>>() {
        changed |= run_on_block(block, f.arena_mut());
    }

    changed
}

fn run_on_block(b: Ref<Block>, arena: &mut Arena<Block>) -> bool {
    let mut changed = false;

    for stmt in b
        .get(arena)
        .statements()
        .iter()
        .copied()
        .collect::<Vec<_>>()
    {
        changed |= run_on_stmt(stmt, b.get_mut(arena).arena_mut());
    }

    changed
}

fn run_on_stmt(stmt: Ref<Statement>, arena: &mut Arena<Statement>) -> bool {
    if matches!(stmt.get(arena), Statement::Constant { .. }) {
        return false;
    }

    match stmt.get(arena).clone() {
        Statement::UnaryOperation {
            kind: unary_op_kind,
            value,
        } => match value.get(arena).clone() {
            Statement::Constant {
                value: constant_value,
                ..
            } => match unary_op_kind {
                UnaryOperationKind::Not => {
                    let constant = Statement::Constant {
                        typ: stmt.get(arena).typ(arena),
                        value: !constant_value,
                    };

                    stmt.get_mut(arena).replace_kind(constant);

                    true
                }
                _ => false,
            },
            _ => false,
        },
        Statement::BinaryOperation { kind, lhs, rhs } => {
            match (lhs.get(arena).clone(), rhs.get(arena).clone()) {
                (
                    Statement::Constant { value: lhs, .. },
                    Statement::Constant { value: rhs, .. },
                ) => {
                    let cv = match kind {
                        BinaryOperationKind::Add => lhs + rhs,
                        BinaryOperationKind::Sub => lhs - rhs,
                        BinaryOperationKind::Multiply => lhs * rhs,
                        BinaryOperationKind::Divide => lhs / rhs,
                        BinaryOperationKind::Modulo => todo!(),
                        BinaryOperationKind::And => todo!(),
                        BinaryOperationKind::Or => todo!(),
                        BinaryOperationKind::Xor => todo!(),
                        BinaryOperationKind::PowI => lhs.powi(rhs),
                        BinaryOperationKind::CompareEqual => {
                            ConstantValue::UnsignedInteger((lhs == rhs) as u64)
                        }
                        BinaryOperationKind::CompareNotEqual => {
                            ConstantValue::UnsignedInteger((lhs != rhs) as u64)
                        }
                        BinaryOperationKind::CompareLessThan => {
                            ConstantValue::UnsignedInteger((lhs < rhs) as u64)
                        }
                        BinaryOperationKind::CompareLessThanOrEqual => {
                            ConstantValue::UnsignedInteger((lhs <= rhs) as u64)
                        }
                        BinaryOperationKind::CompareGreaterThan => {
                            ConstantValue::UnsignedInteger((lhs > rhs) as u64)
                        }
                        BinaryOperationKind::CompareGreaterThanOrEqual => {
                            ConstantValue::UnsignedInteger((lhs >= rhs) as u64)
                        }
                    };

                    let constant = Statement::Constant {
                        typ: stmt.get(arena).typ(arena),
                        value: cv,
                    };
                    stmt.get_mut(arena).replace_kind(constant);

                    true
                }
                /*(
                    Statement::Bundle {
                        value: lv,
                        length: ll,
                    },
                    Statement::Bundle {
                        value: rv,
                        length: rl,
                    },
                ) => {
                    let (
                        Statement::Constant {
                            typ: lvt,
                            value: lvv,
                        },
                        Statement::Constant {
                            typ: llt,
                            value: llv,
                        },
                        Statement::Constant {
                            typ: rvt,
                            value: rvv,
                        },
                        Statement::Constant {
                            typ: rlt,
                            value: rlv,
                        },
                    ) = (lv, ll, rv, rl)
                    else {
                        return false;
                    };

                    if llv != rlv {
                        return false;
                    }

                    trace!("maybe foldable with two bundles");

                    // replace this statement with a constant bundle
                    // _get_HFGRTR_EL2_Type_SCTLR_EL1

                    let cv = match kind {
                        BinaryOperationKind::Add => lvv + rvv,
                        BinaryOperationKind::Sub => lvv - rvv,
                        BinaryOperationKind::Multiply => {
                            return false;
                        }
                        BinaryOperationKind::Divide => todo!(),
                        BinaryOperationKind::Modulo => todo!(),
                        BinaryOperationKind::And => todo!(),
                        BinaryOperationKind::Or => todo!(),
                        BinaryOperationKind::Xor => todo!(),
                        BinaryOperationKind::CompareEqual => todo!(),
                        BinaryOperationKind::CompareNotEqual => todo!(),
                        BinaryOperationKind::CompareLessThan => todo!(),
                        BinaryOperationKind::CompareLessThanOrEqual => todo!(),
                        BinaryOperationKind::CompareGreaterThan => todo!(),
                        BinaryOperationKind::CompareGreaterThanOrEqual => todo!(),
                    };

                    stmt.replace_kind(Statement::Constant {
                        typ: lhs.typ().clone(),
                        value: cv,
                    });

                    true
                }*/
                _ => false,
            }
        }
        Statement::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value,
        } => {
            // watch out! if you cast a constant primitive to an arbitrary bits you lose
            // length information
            if let Type::Primitive(_) = &typ {
                if let Statement::Constant { value, .. } = value.get(arena).clone() {
                    let value = cast_integer(value, typ.clone());
                    stmt.get_mut(arena)
                        .replace_kind(Statement::Constant { typ, value });
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        Statement::Cast {
            kind: CastOperationKind::Truncate,
            typ,
            value,
        } => {
            if let Statement::Constant { value, .. } = value.get(arena).clone() {
                if typ.is_u1() {
                    if let ConstantValue::SignedInteger(signed_value) = value {
                        stmt.get_mut(arena).replace_kind(Statement::Constant {
                            typ,
                            value: ConstantValue::UnsignedInteger(signed_value.try_into().unwrap()),
                        });
                    } else {
                        stmt.get_mut(arena)
                            .replace_kind(Statement::Constant { typ, value });
                    }
                } else {
                    stmt.get_mut(arena)
                        .replace_kind(Statement::Constant { typ, value });
                }

                true
            } else {
                false
            }
        }
        Statement::Cast {
            kind: CastOperationKind::Reinterpret,
            typ,
            value,
        } => {
            if let Statement::Constant { value, .. } = value.get(arena) {
                let value = cast_integer(value.clone(), typ.clone());

                stmt.get_mut(arena)
                    .replace_kind(Statement::Constant { typ, value });
                true
            } else {
                false
            }
        }

        _ => {
            //trace!("candidate for folding not implemented: {}", stmt);
            false
        }
    }
}

fn cast_integer(value: ConstantValue, typ: Type) -> ConstantValue {
    match &typ {
        Type::Primitive(primitive) => match (primitive.tc, value) {
            (PrimitiveTypeClass::SignedInteger, ConstantValue::UnsignedInteger(i)) => {
                ConstantValue::SignedInteger(i64::try_from(i).unwrap())
            }
            (PrimitiveTypeClass::SignedInteger, ConstantValue::SignedInteger(i)) => {
                ConstantValue::SignedInteger(i)
            }
            (PrimitiveTypeClass::UnsignedInteger, ConstantValue::SignedInteger(i)) => {
                ConstantValue::UnsignedInteger(u64::try_from(i).unwrap())
            }
            (PrimitiveTypeClass::UnsignedInteger, ConstantValue::UnsignedInteger(i)) => {
                ConstantValue::UnsignedInteger(i)
            }
            (PrimitiveTypeClass::FloatingPoint, ConstantValue::SignedInteger(s)) => {
                ConstantValue::FloatingPoint(f64::from(i16::try_from(s).unwrap()))
            }
            (tc, cv) => {
                panic!("incompatible type class {tc:?} and constant value {cv:?}")
            }
        },
        // do nothing, todo: check width
        Type::Union { .. } => value,
        _ => panic!("failed to cast {value:x?} to type {typ:?}"),
    }
}
