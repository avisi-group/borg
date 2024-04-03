use log::trace;

use crate::rudder::{Block, CastOperationKind, Statement, StatementKind, ValueClass};

pub fn run(f: crate::rudder::Function) -> bool {
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
                crate::rudder::BinaryOperationKind::Add => lhs + rhs,
                crate::rudder::BinaryOperationKind::Sub => lhs - rhs,
                crate::rudder::BinaryOperationKind::Multiply => todo!(),
                crate::rudder::BinaryOperationKind::Divide => todo!(),
                crate::rudder::BinaryOperationKind::Modulo => todo!(),
                crate::rudder::BinaryOperationKind::And => todo!(),
                crate::rudder::BinaryOperationKind::Or => todo!(),
                crate::rudder::BinaryOperationKind::Xor => todo!(),
                crate::rudder::BinaryOperationKind::CompareEqual => todo!(),
                crate::rudder::BinaryOperationKind::CompareNotEqual => todo!(),
                crate::rudder::BinaryOperationKind::CompareLessThan => todo!(),
                crate::rudder::BinaryOperationKind::CompareLessThanOrEqual => todo!(),
                crate::rudder::BinaryOperationKind::CompareGreaterThan => todo!(),
                crate::rudder::BinaryOperationKind::CompareGreaterThanOrEqual => todo!(),
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
                panic!("operand to cast must be constant ({})", value)
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
