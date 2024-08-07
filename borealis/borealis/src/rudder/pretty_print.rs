use {
    crate::rudder::{
        analysis::cfg::ControlFlowGraphAnalysis,
        statement::{
            value_class::ValueClass, BinaryOperationKind, CastOperationKind, ShiftOperationKind,
            Statement, StatementKind, UnaryOperationKind,
        },
        Block, ConstantValue, Context, Function, FunctionKind, PrimitiveTypeClass, Symbol, Type,
    },
    itertools::Itertools,
    std::fmt::{Display, Formatter, Result},
};

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Type::Primitive(p) => match &p.tc {
                PrimitiveTypeClass::Void => write!(f, "void"),
                PrimitiveTypeClass::Unit => write!(f, "()"),
                PrimitiveTypeClass::UnsignedInteger => write!(f, "u{}", self.width_bits()),
                PrimitiveTypeClass::SignedInteger => write!(f, "i{}", self.width_bits()),
                PrimitiveTypeClass::FloatingPoint => write!(f, "f{}", self.width_bits()),
            },
            Type::Product(_) => write!(f, "struct"),
            Type::Sum(_) => write!(f, "enum"),
            Type::Vector {
                element_count,
                element_type,
            } => write!(f, "[{element_type}; {element_count:?}]"),
            Type::Bits => write!(f, "bv"),
            Type::ArbitraryLengthInteger => write!(f, "i"),
            Type::String => write!(f, "str"),
            Type::Rational => write!(f, "rational"),
            Type::Any => write!(f, "any"),
        }
    }
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ConstantValue::UnsignedInteger(v) => write!(f, "{v}u"),
            ConstantValue::SignedInteger(v) => write!(f, "{v}s"),
            ConstantValue::FloatingPoint(v) => write!(f, "{v}f"),
            ConstantValue::Unit => write!(f, "()"),
            ConstantValue::String(str) => write!(f, "{str:?}"),
            ConstantValue::Rational(r) => write!(f, "{r:?}"),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name())
    }
}

impl Display for StatementKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            StatementKind::Constant { typ, value } => write!(f, "const #{} : {}", value, typ),
            StatementKind::ReadVariable { symbol } => {
                write!(f, "read-var {}:{}", symbol.name(), symbol.typ())
            }
            StatementKind::WriteVariable { symbol, value } => {
                write!(
                    f,
                    "write-var {}:{} <= {}:{}",
                    symbol.name(),
                    symbol.typ(),
                    value.name(),
                    value.typ()
                )
            }
            StatementKind::ReadRegister { typ, offset } => {
                write!(f, "read-reg {}:{}", offset.name(), typ)
            }
            StatementKind::WriteRegister { offset, value } => {
                write!(f, "write-reg {} <= {}", offset.name(), value.name())
            }
            StatementKind::ReadMemory { offset, size } => {
                write!(f, "read-mem {}:{}", offset.name(), size.name())
            }
            StatementKind::WriteMemory { offset, value } => {
                write!(f, "write-mem {} <= {}", offset.name(), value.name())
            }
            StatementKind::BinaryOperation { kind, lhs, rhs } => {
                let op = match kind {
                    BinaryOperationKind::Add => "add",
                    BinaryOperationKind::Sub => "sub",
                    BinaryOperationKind::Multiply => "mul",
                    BinaryOperationKind::Divide => "div",
                    BinaryOperationKind::Modulo => "mod",
                    BinaryOperationKind::CompareEqual => "cmp-eq",
                    BinaryOperationKind::CompareNotEqual => "cmp-ne",
                    BinaryOperationKind::CompareLessThan => "cmp-lt",
                    BinaryOperationKind::CompareLessThanOrEqual => "cmp-le",
                    BinaryOperationKind::CompareGreaterThan => "cmp-gt",
                    BinaryOperationKind::CompareGreaterThanOrEqual => "cmp-ge",
                    BinaryOperationKind::And => "and",
                    BinaryOperationKind::Or => "or",
                    BinaryOperationKind::Xor => "xor",
                    BinaryOperationKind::PowI => "powi",
                };

                write!(f, "{} {} {}", op, lhs.name(), rhs.name())
            }
            StatementKind::UnaryOperation { kind, value } => {
                let op = match kind {
                    UnaryOperationKind::Complement => "cmpl",
                    UnaryOperationKind::Not => "not",
                    UnaryOperationKind::Negate => "neg",
                    UnaryOperationKind::Power2 => "pow2",
                    UnaryOperationKind::Absolute => "abs",
                    UnaryOperationKind::Ceil => "ceil",
                    UnaryOperationKind::Floor => "floor",
                    UnaryOperationKind::SquareRoot => "sqrt",
                };

                write!(f, "{} {}", op, value.name())
            }
            StatementKind::ShiftOperation {
                kind,
                value,
                amount,
            } => {
                let op = match kind {
                    ShiftOperationKind::LogicalShiftLeft => "lsl",
                    ShiftOperationKind::LogicalShiftRight => "lsr",
                    ShiftOperationKind::ArithmeticShiftRight => "asr",
                    ShiftOperationKind::RotateRight => "ror",
                    ShiftOperationKind::RotateLeft => "rol",
                };

                write!(f, "{} {} {}", op, value.name(), amount.name())
            }
            StatementKind::Call { target, args, tail } => {
                write!(
                    f,
                    "{}call {}({})",
                    if *tail { "tail-" } else { "" },
                    target.name(),
                    args.iter().map(Statement::name).join(", ")
                )
            }
            StatementKind::Cast { kind, typ, value } => {
                let op = match kind {
                    CastOperationKind::ZeroExtend => "zx",
                    CastOperationKind::SignExtend => "sx",
                    CastOperationKind::Truncate => "trunc",
                    CastOperationKind::Reinterpret => "reint",
                    CastOperationKind::Convert => "cvt",
                    CastOperationKind::Broadcast => "bcast",
                };

                write!(f, "cast {} {} -> {}", op, value.name(), typ)
            }
            StatementKind::BitsCast {
                kind,
                typ,
                value,
                length,
            } => {
                let op = match kind {
                    CastOperationKind::ZeroExtend => "zx",
                    CastOperationKind::SignExtend => "sx",
                    CastOperationKind::Truncate => "trunc",
                    CastOperationKind::Reinterpret => "reint",
                    CastOperationKind::Convert => "cvt",
                    CastOperationKind::Broadcast => "bcast",
                };

                write!(
                    f,
                    "bits-cast {} {} -> {} length {}",
                    op,
                    value.name(),
                    typ,
                    length.name()
                )
            }
            StatementKind::Jump { target } => write!(f, "jump b{}", target.name()),
            StatementKind::Branch {
                condition,
                true_target,
                false_target,
            } => {
                write!(
                    f,
                    "branch {} b{} b{}",
                    condition.name(),
                    true_target.name(),
                    false_target.name()
                )
            }
            StatementKind::PhiNode { members } => {
                write!(f, "phi ")?;

                for member in members {
                    write!(f, "(BLOCK, {}) ", member.1)?;
                }

                Ok(())
            }
            StatementKind::Return { value: None } => {
                write!(f, "return")
            }
            StatementKind::Return { value: Some(value) } => {
                write!(f, "return {}", value.name())
            }
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                write!(
                    f,
                    "select {} {} {}",
                    condition.name(),
                    true_value.name(),
                    false_value.name()
                )
            }
            StatementKind::Panic(statements) => {
                write!(
                    f,
                    "panic {}",
                    statements.iter().map(Statement::name).join(" ")
                )
            }
            StatementKind::Undefined => write!(f, "undefined",),
            StatementKind::PrintChar(c) => {
                write!(f, "put-char {}", c.name(),)
            }
            StatementKind::ReadPc => write!(f, "read-pc"),
            StatementKind::WritePc { value } => write!(f, "write-pc {}", value.name()),
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => write!(
                f,
                "bit-extract {} {} {}",
                value.name(),
                start.name(),
                length.name()
            ),
            StatementKind::BitInsert {
                target: original_value,
                source: insert_value,
                start,
                length,
            } => write!(
                f,
                "bit-insert {} {} {} {}",
                original_value.name(),
                insert_value.name(),
                start.name(),
                length.name()
            ),
            StatementKind::ReadElement { vector, index } => {
                write!(f, "read-element {}[{}]", vector.name(), index.name())
            }
            StatementKind::MutateElement {
                vector,
                value,
                index,
            } => write!(
                f,
                "mutate-element {}[{}] <= {}",
                vector.name(),
                index.name(),
                value.name()
            ),
            StatementKind::CreateProduct { typ, fields } => {
                write!(
                    f,
                    "create-product {} = {:?}",
                    typ,
                    fields.iter().map(Statement::name).collect::<Vec<_>>()
                )
            }
            StatementKind::CreateSum {
                typ,
                variant,
                value,
            } => {
                write!(f, "create-sum {} = {}:{:?}", typ, variant, value.name(),)
            }
            StatementKind::SizeOf { value } => {
                write!(f, "size-of {}", value.name())
            }
            StatementKind::Assert { condition } => {
                write!(f, "assert {}", condition.name())
            }

            StatementKind::CreateBits { value, length } => {
                write!(f, "create-bits {} {}", value.name(), length.name())
            }
            StatementKind::MatchesSum { value, variant } => {
                write!(f, "matches-sum {} {variant}", value.name())
            }
            StatementKind::UnwrapSum { value, variant } => {
                write!(f, "unwrap-sum {} {variant}", value.name())
            }
            StatementKind::ExtractField { value, field } => {
                write!(f, "extract-field {}.{field}", value.name())
            }
            StatementKind::UpdateField {
                original_value,
                field,
                field_value,
            } => {
                write!(
                    f,
                    "update-field {}.{field} <- {}",
                    original_value.name(),
                    field_value.name()
                )
            }
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let vc = match self.class() {
            ValueClass::None => "-",
            ValueClass::Constant => "C",
            ValueClass::Static => "S",
            ValueClass::Dynamic => "D",
        };

        write!(f, "[{}] {}: {}", vc, self.name(), self.kind())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for stmt in &self.inner.get().statements {
            writeln!(f, "    {}", stmt)?;
        }

        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let cfg = ControlFlowGraphAnalysis::new(self);

        self.inner.get().entry_block.iter().try_for_each(|block| {
            let preds = cfg
                .predecessors_for(&block)
                .unwrap()
                .iter()
                .map(|b| b.name())
                .join(", ");

            let succs = cfg
                .successors_for(&block)
                .unwrap()
                .iter()
                .map(|b| b.name())
                .join(", ");

            writeln!(
                f,
                "  block {}: preds={{{preds}}}, succs={{{succs}}}",
                block.name()
            )?;
            write!(f, "{}", block)
        })
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.update_names();

        writeln!(f, "rudder context:")?;

        for (name, (kind, func)) in self.fns.iter() {
            writeln!(
                f,
                "function {} ({}):",
                name,
                if matches!(kind, FunctionKind::Execute) {
                    "execute"
                } else {
                    "other"
                }
            )?;

            write!(f, "{}", func)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for ValueClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ValueClass::None => write!(f, "N"),
            ValueClass::Constant => write!(f, "C"),
            ValueClass::Static => write!(f, "S"),
            ValueClass::Dynamic => write!(f, "D"),
        }
    }
}
