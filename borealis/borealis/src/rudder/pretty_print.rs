use {
    super::{analysis, ValueClass},
    crate::rudder::{
        BinaryOperationKind, Block, CastOperationKind, ConstantValue, Context, Function,
        FunctionKind, PrimitiveTypeClass, Statement, StatementKind, Symbol, Type,
        UnaryOperationKind,
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
            Type::Composite(_) => write!(f, "struct"),
            Type::Vector {
                element_count,
                element_type,
            } => write!(f, "[{element_type}; {element_count:?}]"),
            Type::Bundled { value, len } => write!(f, "bundle({value}, {len})"),
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
            StatementKind::WriteVariable {
                symbol,
                indices,
                value,
            } => {
                if indices.is_empty() {
                    write!(f, "write-var {} <= {}", symbol.name(), value.name())
                } else {
                    write!(f, "write-var {}", symbol.name())?;
                    for index in indices {
                        write!(f, ".{}", index)?;
                    }
                    write!(f, " <= {}", value.name())
                }
            }
            StatementKind::ReadRegister { typ, offset } => {
                write!(f, "read-reg {}:{}", offset.name(), typ)
            }
            StatementKind::WriteRegister { offset, value } => {
                write!(f, "write-reg {} <= {}", offset.name(), value.name())
            }
            StatementKind::ReadMemory { typ, offset } => {
                write!(f, "read-mem {}:{}", offset.name(), typ)
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
                };

                write!(f, "{} {}", op, value.name())
            }
            StatementKind::ShiftOperation {
                kind,
                value,
                amount,
            } => {
                let op = match kind {
                    super::ShiftOperationKind::LogicalShiftLeft => "lsl",
                    super::ShiftOperationKind::LogicalShiftRight => "lsr",
                    super::ShiftOperationKind::ArithmeticShiftRight => "asr",
                    super::ShiftOperationKind::RotateRight => "ror",
                    super::ShiftOperationKind::RotateLeft => "rol",
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
                original_value,
                insert_value,
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
            StatementKind::ReadField { composite, field } => {
                write!(f, "read-field {}.{}", composite.name(), field)
            }
            StatementKind::MutateField {
                composite,
                field,
                value,
            } => write!(
                f,
                "mutate-field {}.{} <= {}",
                composite.name(),
                field,
                value.name()
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
            StatementKind::CreateComposite { typ, fields } => {
                write!(
                    f,
                    "create-composite {} = {:?}",
                    typ,
                    fields.iter().map(Statement::name).collect::<Vec<_>>()
                )
            }
            StatementKind::Bundle { value, length } => {
                write!(f, "bundle {}, {} ", value.name(), length.name())
            }
            StatementKind::UnbundleValue { bundle } => {
                write!(f, "unbundle-value {}", bundle.name())
            }
            StatementKind::UnbundleLength { bundle } => {
                write!(f, "unbundle-length {}", bundle.name())
            }
            StatementKind::Assert { condition } => {
                write!(f, "assert {}", condition.name())
            }
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {}", self.name(), self.kind())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for stmt in &(*self.inner).borrow().statements {
            writeln!(f, "    {}", stmt)?;
        }

        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let cfg = analysis::cfg::ControlFlowGraphAnalysis::new(self);

        self.inner
            .borrow()
            .entry_block
            .iter()
            .try_for_each(|block| {
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
