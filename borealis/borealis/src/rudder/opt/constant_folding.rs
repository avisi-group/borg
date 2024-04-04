use {
    crate::rudder::{
        BinaryOperationKind, Block, CastOperationKind, ConstantValue, Function, Statement,
        StatementKind, ValueClass,
    },
    log::trace,
};

pub fn run(f: Function) -> bool {
    let mut changed = false;

    //trace!("constant folding {}", f.name());
    for block in f.entry_block().iter() {
        changed |= run_on_block(block);
    }

    changed
}

fn run_on_block(b: Block) -> bool {
    let mut changed = false;

    for stmt in b.statements() {
        changed |= run_on_stmt(stmt);
    }

    changed
}

fn run_on_stmt(stmt: Statement) -> bool {
    if matches!(stmt.kind(), StatementKind::Constant { .. }) {
        return false;
    }

    match stmt.kind() {
        StatementKind::BinaryOperation { kind, lhs, rhs } => match (lhs.kind(), rhs.kind()) {
            (
                StatementKind::Constant { value: lhs, .. },
                StatementKind::Constant { value: rhs, .. },
            ) => {
                let cv = match kind {
                    BinaryOperationKind::Add => lhs + rhs,
                    BinaryOperationKind::Sub => lhs - rhs,
                    BinaryOperationKind::Multiply => todo!(),
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

                stmt.replace_kind(StatementKind::Constant {
                    typ: stmt.typ(),
                    value: cv,
                });

                true
            }
            /*(
                StatementKind::Bundle {
                    value: lv,
                    length: ll,
                },
                StatementKind::Bundle {
                    value: rv,
                    length: rl,
                },
            ) => {
                let (
                    StatementKind::Constant {
                        typ: lvt,
                        value: lvv,
                    },
                    StatementKind::Constant {
                        typ: llt,
                        value: llv,
                    },
                    StatementKind::Constant {
                        typ: rvt,
                        value: rvv,
                    },
                    StatementKind::Constant {
                        typ: rlt,
                        value: rlv,
                    },
                ) = (lv.kind(), ll.kind(), rv.kind(), rl.kind())
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

                stmt.replace_kind(StatementKind::Constant {
                    typ: lhs.typ().clone(),
                    value: cv,
                });

                true
            }*/
            _ => false,
        },
        StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value,
        } => {
            if let StatementKind::Constant { value, .. } = value.kind() {
                stmt.replace_kind(StatementKind::Constant { typ, value });
                true
            } else {
                false
            }
        }
        StatementKind::Cast {
            kind: CastOperationKind::Truncate,
            typ,
            value,
        } => {
            if let StatementKind::Constant { value, .. } = value.kind() {
                if typ.is_u1() {
                    if let ConstantValue::SignedInteger(signed_value) = value {
                        stmt.replace_kind(StatementKind::Constant {
                            typ,
                            value: ConstantValue::UnsignedInteger(signed_value.try_into().unwrap()),
                        });
                    } else {
                        stmt.replace_kind(StatementKind::Constant { typ, value });
                    }
                } else {
                    stmt.replace_kind(StatementKind::Constant { typ, value });
                }

                true
            } else {
                false
            }
        }
        StatementKind::Cast {
            kind: CastOperationKind::Reinterpret,
            typ,
            value,
        } => {
            if let StatementKind::Constant { value, .. } = value.kind() {
                stmt.replace_kind(StatementKind::Constant { typ, value });
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
