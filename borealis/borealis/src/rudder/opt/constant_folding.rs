use {
    crate::rudder::{
        BinaryOperationKind, Block, CastOperationKind, ConstantValue, Function, Statement,
        StatementKind, ValueClass,
    },
    log::trace,
};

pub fn run(f: Function) -> bool {
    let mut changed = false;

    trace!("constant folding {}", f.name());
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

    if stmt.classify() != ValueClass::Constant {
        return false;
    }

    match stmt.kind() {
        StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let StatementKind::Constant { value: lhs, .. } = lhs.kind() else {
                panic!("lhs must be constant ({})", lhs)
            };

            let StatementKind::Constant { value: rhs, .. } = rhs.kind() else {
                panic!("rhs must be constant ({})", rhs)
            };

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
        StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ,
            value,
        } => {
            let StatementKind::Constant { value, .. } = value.kind() else {
                panic!("operand to cast must be constant ({})", value)
            };

            stmt.replace_kind(StatementKind::Constant { typ, value });

            true
        }
        StatementKind::Cast {
            kind: CastOperationKind::Truncate,
            typ,
            value,
        } => {
            let StatementKind::Constant { value, .. } = value.kind() else {
                panic!(
                    "operand to cast must be constant stmt={stmt}; value={}",
                    value
                )
            };

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
        }
        StatementKind::Cast {
            kind: CastOperationKind::Reinterpret,
            typ,
            value,
        } => {
            let StatementKind::Constant { value, .. } = value.kind() else {
                panic!(
                    "operand to cast must be constant stmt={stmt}; value={}",
                    value
                )
            };

            stmt.replace_kind(StatementKind::Constant { typ, value });

            true
        }

        _ => {
            trace!("candidate for folding not implemented: {}", stmt);

            false
        }
    }
}
