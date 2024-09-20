use {
    crate::{
        rudder::{
            constant_value::ConstantValue, Block, PrimitiveType, PrimitiveTypeClass, Symbol, Type,
        },
        util::arena::{Arena, Ref},
    },
    common::{intern::InternedString, HashMap},
    itertools::Itertools,
    proc_macro2::TokenStream,
    quote::{format_ident, ToTokens, TokenStreamExt},
    std::{cmp::Ordering, fmt::Debug},
};

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperationKind {
    Not,
    Negate,
    Complement,
    Power2,
    Absolute,
    Ceil,
    Floor,
    SquareRoot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperationKind {
    Add,
    Sub,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    PowI,
    CompareEqual,
    CompareNotEqual,
    CompareLessThan,
    CompareLessThanOrEqual,
    CompareGreaterThan,
    CompareGreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TernaryOperationKind {
    AddWithCarry,
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

#[derive(Debug, Clone)]
pub enum StatementKind {
    Constant {
        typ: Type,
        value: ConstantValue,
    },

    ReadVariable {
        symbol: Symbol,
    },

    WriteVariable {
        symbol: Symbol,
        value: Ref<Statement>,
    },

    ReadRegister {
        typ: Type,
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Ref<Statement>,
    },

    WriteRegister {
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Ref<Statement>,
        value: Ref<Statement>,
    },

    ReadMemory {
        offset: Ref<Statement>,
        size: Ref<Statement>,
    },
    WriteMemory {
        offset: Ref<Statement>,
        value: Ref<Statement>,
    },

    ReadPc,
    WritePc {
        value: Ref<Statement>,
    },

    GetFlag {
        flag: Flag,
        operation: Ref<Statement>,
    },

    UnaryOperation {
        kind: UnaryOperationKind,
        value: Ref<Statement>,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Ref<Statement>,
        rhs: Ref<Statement>,
    },
    ShiftOperation {
        kind: ShiftOperationKind,
        value: Ref<Statement>,
        amount: Ref<Statement>,
    },
    Call {
        target: InternedString, // todo: ref<function>
        args: Vec<Ref<Statement>>,
        return_type: Type, /* todo: this is really bad. necessary to avoid needing to pass a
                            * rudder model into every .typ() call, and hopefully a function
                            * return type is unlikely to change after boom, but this should
                            * really be a function lookup */
    },
    Cast {
        kind: CastOperationKind,
        typ: Type,
        value: Ref<Statement>,
    },
    BitsCast {
        kind: CastOperationKind,
        typ: Type,
        value: Ref<Statement>,
        length: Ref<Statement>,
    },
    Jump {
        target: Ref<Block>,
    },
    Branch {
        condition: Ref<Statement>,
        true_target: Ref<Block>,
        false_target: Ref<Block>,
    },
    PhiNode {
        members: Vec<(Ref<Block>, Ref<Statement>)>,
    },
    Return {
        value: Ref<Statement>,
    },
    Select {
        condition: Ref<Statement>,
        true_value: Ref<Statement>,
        false_value: Ref<Statement>,
    },
    BitExtract {
        value: Ref<Statement>,
        start: Ref<Statement>,
        length: Ref<Statement>,
    },
    BitInsert {
        /// Target data that `length` bits of `source` will be inserted into at
        /// position `start`
        target: Ref<Statement>,
        /// Source bits that will be inserted into target
        source: Ref<Statement>,
        /// Offset into `target` that `source` will be inserted
        start: Ref<Statement>,
        /// Length of `source` that will be inserted
        length: Ref<Statement>,
    },
    ReadElement {
        vector: Ref<Statement>,
        index: Ref<Statement>,
    },
    /// Returns the vector with the mutated element
    AssignElement {
        vector: Ref<Statement>,
        value: Ref<Statement>,
        index: Ref<Statement>,
    },

    /// Fatal error, printing value of supplied Ref<StatementInner> for
    /// debugging purposes
    Panic(Ref<Statement>),

    /// `Default::default()`, or uninitialized, or ???
    Undefined,

    Assert {
        condition: Ref<Statement>,
    },

    CreateBits {
        value: Ref<Statement>,
        length: Ref<Statement>,
    },

    // creating bits and getting the value done through casting
    // gets the length when applied to bits
    SizeOf {
        value: Ref<Statement>,
    },

    /// Tests whether an instance of a union is of a given variant
    MatchesUnion {
        value: Ref<Statement>,
        variant: InternedString,
    },

    /// Extracts the contents of a variant of a union
    UnwrapUnion {
        value: Ref<Statement>,
        variant: InternedString,
    },

    CreateTuple(Vec<Ref<Statement>>),
    TupleAccess {
        index: usize,
        source: Ref<Statement>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    N,
    Z,
    C,
    V,
}

impl StatementKind {
    pub fn to_string(&self, arena: &Arena<Statement>) -> InternedString {
        match &self {
            StatementKind::Constant { typ, value } => format!("const #{} : {}", value, typ),
            StatementKind::ReadVariable { symbol } => {
                format!("read-var {}:{}", symbol.name(), symbol.typ())
            }
            StatementKind::WriteVariable { symbol, value } => {
                format!(
                    "write-var {}:{} <= {}:{}",
                    symbol.name(),
                    symbol.typ(),
                    value.to_string(arena),
                    value.get(arena).typ(arena)
                )
            }
            StatementKind::ReadRegister { typ, offset } => {
                format!("read-reg {}:{}", offset.to_string(arena), typ)
            }
            StatementKind::WriteRegister { offset, value } => {
                format!(
                    "write-reg {} <= {}",
                    offset.to_string(arena),
                    value.to_string(arena)
                )
            }
            StatementKind::ReadMemory { offset, size } => {
                format!(
                    "read-mem {}:{}",
                    offset.to_string(arena),
                    size.to_string(arena)
                )
            }
            StatementKind::WriteMemory { offset, value } => {
                format!(
                    "write-mem {} <= {}",
                    offset.to_string(arena),
                    value.to_string(arena)
                )
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

                format!("{} {} {}", op, lhs.to_string(arena), rhs.to_string(arena))
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

                format!("{} {}", op, value.to_string(arena))
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

                format!(
                    "{} {} {}",
                    op,
                    value.to_string(arena),
                    amount.to_string(arena)
                )
            }
            StatementKind::Call { target, args, .. } => {
                format!(
                    "call {}({})",
                    target,
                    args.iter().map(|s| s.to_string(arena)).join(", ")
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

                format!("cast {} {} -> {}", op, value.to_string(arena), typ)
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

                format!(
                    "bits-cast {} {} -> {} length {}",
                    op,
                    value.to_string(arena),
                    typ,
                    length.to_string(arena)
                )
            }
            StatementKind::Jump { target } => format!("jump block{:?}", target), // removed .index
            StatementKind::Branch {
                condition,
                true_target,
                false_target,
            } => {
                format!(
                    "branch {} block{:?} block{:?}", // removed .index
                    condition.to_string(arena),
                    true_target,
                    false_target,
                )
            }
            StatementKind::PhiNode { members } => {
                // format!( "phi ")?;

                // for member in members {
                //     format!( "(BLOCK, {}) ", member.1)?;
                // }

                // Ok(())
                todo!()
            }

            StatementKind::Return { value } => {
                format!("return {}", value.to_string(arena))
            }
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                format!(
                    "select {} {} {}",
                    condition.to_string(arena),
                    true_value.to_string(arena),
                    false_value.to_string(arena)
                )
            }
            StatementKind::Panic(statement) => {
                format!("panic {}", statement.to_string(arena))
            }
            StatementKind::Undefined => format!("undefined",),

            StatementKind::ReadPc => format!("read-pc"),
            StatementKind::WritePc { value } => format!("write-pc {}", value.to_string(arena)),
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => format!(
                "bit-extract {} {} {}",
                value.to_string(arena),
                start.to_string(arena),
                length.to_string(arena)
            ),
            StatementKind::BitInsert {
                target: original_value,
                source: insert_value,
                start,
                length,
            } => format!(
                "bit-insert {} {} {} {}",
                original_value.to_string(arena),
                insert_value.to_string(arena),
                start.to_string(arena),
                length.to_string(arena)
            ),
            StatementKind::ReadElement { vector, index } => {
                format!(
                    "read-element {}[{}]",
                    vector.to_string(arena),
                    index.to_string(arena)
                )
            }
            StatementKind::AssignElement {
                vector,
                value,
                index,
            } => format!(
                "mutate-element {}[{}] <= {}",
                vector.to_string(arena),
                index.to_string(arena),
                value.to_string(arena)
            ),

            StatementKind::SizeOf { value } => {
                format!("size-of {}", value.to_string(arena))
            }
            StatementKind::Assert { condition } => {
                format!("assert {}", condition.to_string(arena))
            }

            StatementKind::CreateBits { value, length } => {
                format!(
                    "create-bits {} {}",
                    value.to_string(arena),
                    length.to_string(arena)
                )
            }
            StatementKind::MatchesUnion { value, variant } => {
                format!("matches-union {} {variant}", value.to_string(arena))
            }
            StatementKind::UnwrapUnion { value, variant } => {
                format!("unwrap-union {} {variant}", value.to_string(arena))
            }
            StatementKind::TupleAccess { index, source } => {
                format!("tuple-access {}.{index}", source.to_string(arena))
            }
            StatementKind::GetFlag { flag, operation } => {
                format!("get-flag {flag:?} {}", operation.to_string(arena))
            }
            StatementKind::CreateTuple(values) => {
                format!(
                    "create-tuple {:?}",
                    values
                        .iter()
                        .map(|v| v.to_string(arena))
                        .collect::<Vec<_>>()
                )
            }
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    name: InternedString,
    kind: StatementKind,
}
impl ToTokens for Ref<Statement> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("s{}", self.index()))
    }
}

impl Ref<Statement> {
    pub fn to_string(&self, arena: &Arena<Statement>) -> InternedString {
        format!(
            "s{}: {}",
            self.index(),
            self.get(arena).kind().to_string(arena)
        )
        .into()
    }
}

impl Statement {
    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut StatementKind {
        &mut self.kind
    }

    pub fn has_side_effects(&self) -> bool {
        matches!(
            self.kind(),
            StatementKind::WriteVariable { .. }
                | StatementKind::WriteRegister { .. }
                | StatementKind::WriteMemory { .. }
                | StatementKind::WritePc { .. }
                | StatementKind::Call { .. }
                | StatementKind::Jump { .. }
                | StatementKind::Branch { .. }
                | StatementKind::Return { .. }
                | StatementKind::Panic(_)
                | StatementKind::Assert { .. }
        )
    }

    pub fn typ(&self, arena: &Arena<Statement>) -> Type {
        match &self.kind {
            StatementKind::Constant { typ, .. } => typ.clone(),
            StatementKind::ReadVariable { symbol } => symbol.typ(),
            StatementKind::WriteVariable { .. } => Type::void(),
            StatementKind::ReadRegister { typ, .. } => typ.clone(),
            StatementKind::WriteRegister { .. } => Type::unit(),
            StatementKind::ReadMemory { .. } => Type::Bits,
            StatementKind::WriteMemory { .. } => Type::unit(),
            StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareEqual,
                ..
            }
            | StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareNotEqual,
                ..
            }
            | StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareGreaterThanOrEqual,
                ..
            }
            | StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareGreaterThan,
                ..
            }
            | StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareLessThanOrEqual,
                ..
            }
            | StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareLessThan,
                ..
            } => Type::u1(),
            StatementKind::BinaryOperation { lhs, .. } => lhs.get(arena).typ(arena),
            StatementKind::UnaryOperation { value, .. } => value.get(arena).typ(arena),
            StatementKind::ShiftOperation { value, .. } => value.get(arena).typ(arena),
            StatementKind::Call { return_type, .. } => return_type.clone(),
            StatementKind::Cast { typ, .. } | StatementKind::BitsCast { typ, .. } => typ.clone(),
            StatementKind::Jump { .. } => Type::void(),
            StatementKind::Branch { .. } => Type::void(),
            StatementKind::PhiNode { members } => members
                .first()
                .map(|(_, stmt)| stmt.get(arena).typ(arena))
                .unwrap_or_else(|| (Type::void())),
            StatementKind::Return { .. } => Type::void(),
            StatementKind::Select { true_value, .. } => true_value.get(arena).typ(arena),
            StatementKind::Panic(_) => Type::void(),

            StatementKind::ReadPc => Type::u64(),
            StatementKind::WritePc { .. } => Type::void(),
            // todo: this is a simplification, be more precise about lengths?
            StatementKind::BitExtract { value, .. } => value.get(arena).typ(arena),
            StatementKind::BitInsert {
                target: original_value,
                ..
            } => original_value.get(arena).typ(arena),
            StatementKind::ReadElement { vector, .. } => {
                let Type::Vector { element_type, .. } = &vector.get(arena).typ(arena) else {
                    panic!("cannot read field of non-composite type")
                };

                (**element_type).clone()
            }
            StatementKind::AssignElement { vector, .. } => {
                // get type of the vector and return it
                vector.get(arena).typ(arena)
            }

            StatementKind::SizeOf { .. } => Type::u16(),
            StatementKind::Assert { .. } => Type::unit(),
            StatementKind::CreateBits { .. } => Type::Bits,
            StatementKind::MatchesUnion { .. } => Type::u1(),
            StatementKind::UnwrapUnion { value, variant } => {
                // let Type::Enum(variants) = &*value.get(arena).typ(arena) else {
                //     panic!("cannot unwrap non sum type");
                // };

                // variants
                //     .iter()
                //     .find(|(name, _)| *name == variant)
                //     .unwrap()
                //     .1
                //     .clone()
                todo!()
            }

            StatementKind::Undefined => Type::Any,
            StatementKind::TupleAccess { index, source } => {
                let Type::Tuple(ts) = &source.get(arena).typ(arena) else {
                    panic!();
                };

                ts[*index].clone()
            }

            StatementKind::GetFlag { .. } => Type::u1(),
            StatementKind::CreateTuple(values) => {
                Type::Tuple(values.iter().map(|v| v.get(arena).typ(arena)).collect())
            }
        }
    }

    pub fn update_names(&mut self, name: InternedString) {
        self.name = name;
    }

    pub fn replace_kind(&mut self, kind: StatementKind) {
        self.kind = kind;
    }

    pub fn replace_use(&mut self, use_of: Ref<Statement>, with: Ref<Statement>) {
        match self.kind.clone() {
            StatementKind::Return { value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::Return { value };
            }
            StatementKind::Branch {
                true_target,
                false_target,
                condition,
            } => {
                let condition = if condition == use_of {
                    with.clone()
                } else {
                    condition.clone()
                };

                self.kind = StatementKind::Branch {
                    condition,
                    true_target,
                    false_target,
                };
            }
            StatementKind::WriteVariable { symbol, value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::WriteVariable { symbol, value };
            }
            StatementKind::BinaryOperation { kind, lhs, rhs } => {
                if lhs == use_of {
                    self.kind = StatementKind::BinaryOperation {
                        kind,
                        lhs: with.clone(),
                        rhs,
                    };
                } else if rhs == use_of {
                    self.kind = StatementKind::BinaryOperation {
                        kind,
                        lhs,
                        rhs: with.clone(),
                    };
                } else {
                    panic!("should not get here");
                }
            }
            StatementKind::UnaryOperation { kind, value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::UnaryOperation { kind, value };
            }

            StatementKind::Cast { kind, typ, value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::Cast { kind, typ, value };
            }
            StatementKind::BitsCast {
                kind,
                typ,
                value,
                length,
            } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                let length = if length == use_of {
                    with.clone()
                } else {
                    length.clone()
                };

                self.kind = StatementKind::BitsCast {
                    kind,
                    typ,
                    value,
                    length,
                };
            }
            StatementKind::Call {
                target,
                args,
                return_type,
            } => {
                let args = args
                    .iter()
                    .map(|arg| {
                        if *arg == use_of {
                            with.clone()
                        } else {
                            arg.clone()
                        }
                    })
                    .collect();

                self.kind = StatementKind::Call {
                    target,
                    args,
                    return_type,
                };
            }
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                let start = if start == use_of {
                    with.clone()
                } else {
                    start.clone()
                };

                let length = if length == use_of {
                    with.clone()
                } else {
                    length.clone()
                };

                self.kind = StatementKind::BitExtract {
                    value,
                    start,
                    length,
                };
            }

            StatementKind::Assert { condition } => {
                let condition = if condition == use_of {
                    with.clone()
                } else {
                    condition.clone()
                };

                self.kind = StatementKind::Assert { condition };
            }
            StatementKind::ShiftOperation {
                kind,
                value,
                amount,
            } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                let amount = if amount == use_of {
                    with.clone()
                } else {
                    amount.clone()
                };

                self.kind = StatementKind::ShiftOperation {
                    kind,
                    value,
                    amount,
                };
            }
            StatementKind::WriteRegister { offset, value } => {
                let offset = if offset == use_of {
                    with.clone()
                } else {
                    offset.clone()
                };

                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::WriteRegister { offset, value };
            }
            StatementKind::WriteMemory { offset, value } => {
                let offset = if offset == use_of {
                    with.clone()
                } else {
                    offset.clone()
                };

                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::WriteMemory { offset, value }
            }
            StatementKind::ReadMemory { offset, size } => {
                let offset = if offset == use_of {
                    with.clone()
                } else {
                    offset.clone()
                };

                let size = if size == use_of {
                    with.clone()
                } else {
                    size.clone()
                };

                self.kind = StatementKind::ReadMemory { offset, size }
            }

            StatementKind::ReadElement { vector, index } => {
                let vector = if vector == use_of {
                    with.clone()
                } else {
                    vector.clone()
                };

                let index = if index == use_of {
                    with.clone()
                } else {
                    index.clone()
                };

                self.kind = StatementKind::ReadElement { vector, index };
            }

            StatementKind::BitInsert {
                target: original_value,
                source: insert_value,
                start,
                length,
            } => {
                let stmts = [original_value, insert_value, start, length]
                    .into_iter()
                    .map(|s| if s == use_of { with.clone() } else { s })
                    .collect::<Vec<_>>();

                self.kind = StatementKind::BitInsert {
                    target: stmts[0].clone(),
                    source: stmts[1].clone(),
                    start: stmts[2].clone(),
                    length: stmts[3].clone(),
                }
            }

            StatementKind::SizeOf { value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };
                self.kind = StatementKind::SizeOf { value };
            }

            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                let condition = if condition == use_of {
                    with.clone()
                } else {
                    condition.clone()
                };

                let true_value = if true_value == use_of {
                    with.clone()
                } else {
                    true_value.clone()
                };

                let false_value = if false_value == use_of {
                    with.clone()
                } else {
                    false_value.clone()
                };

                self.kind = StatementKind::Select {
                    condition,
                    true_value,
                    false_value,
                };
            }

            StatementKind::AssignElement {
                vector,
                value,
                index,
            } => {
                let vector = if vector == use_of {
                    with.clone()
                } else {
                    vector.clone()
                };

                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                let index = if index == use_of {
                    with.clone()
                } else {
                    index.clone()
                };

                self.kind = StatementKind::AssignElement {
                    vector,
                    value,
                    index,
                };
            }
            StatementKind::WritePc { value } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::WritePc { value };
            }
            StatementKind::Panic(value) => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::Panic(value)
            }

            StatementKind::CreateBits { value, length } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                let length = if length == use_of {
                    with.clone()
                } else {
                    length.clone()
                };

                self.kind = StatementKind::CreateBits { value, length };
            }
            StatementKind::MatchesUnion { value, variant } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::MatchesUnion { value, variant };
            }
            StatementKind::UnwrapUnion { value, variant } => {
                let value = if value == use_of {
                    with.clone()
                } else {
                    value.clone()
                };

                self.kind = StatementKind::UnwrapUnion { value, variant };
            }

            StatementKind::Constant { .. } => todo!(),
            StatementKind::ReadVariable { .. } => todo!(),
            StatementKind::ReadRegister { .. } => todo!(),
            StatementKind::ReadPc => todo!(),
            StatementKind::Jump { .. } => todo!(),
            StatementKind::PhiNode { .. } => todo!(),
            StatementKind::Undefined => todo!(),
            StatementKind::TupleAccess { index, source } => {
                let source = if source == use_of {
                    with.clone()
                } else {
                    source.clone()
                };

                self.kind = StatementKind::TupleAccess { index, source };
            }

            StatementKind::GetFlag { flag, operation } => {
                let operation = if operation == use_of {
                    with.clone()
                } else {
                    operation.clone()
                };
                self.kind = StatementKind::GetFlag { flag, operation };
            }
            StatementKind::CreateTuple(values) => {
                self.kind = StatementKind::CreateTuple(
                    values
                        .iter()
                        .map(|v| {
                            if *v == use_of {
                                with.clone()
                            } else {
                                v.clone()
                            }
                        })
                        .collect(),
                )
            }
        }
    }
}

pub enum Location {
    End,
    Before(Ref<Statement>),
}

/// Creates a new statement in the block's arena, and pushes it to the end of
/// the block's statements
pub fn build_at(
    block: Ref<Block>,
    arena: &mut Arena<Block>,
    kind: StatementKind,
    location: Location,
) -> Ref<Statement> {
    let r = block.get_mut(arena).arena_mut().insert(Statement {
        name: "???".into(),
        kind,
    });
    match location {
        Location::Before(before) => block.get_mut(arena).insert_statement_before(before, r),
        Location::End => block.get_mut(arena).statements.push(r),
    }

    r
}

/// Creates a new statement in the block's arena, and pushes it to the end of
/// the block's statements
pub fn build(block: Ref<Block>, arena: &mut Arena<Block>, kind: StatementKind) -> Ref<Statement> {
    build_at(block, arena, kind, Location::End)
}

pub fn cast(
    block: Ref<Block>,
    arena: &mut Arena<Block>,
    source: Ref<Statement>,
    destination_type: Type,
) -> Ref<Statement> {
    cast_at(block, arena, source, destination_type, Location::End)
}

// No-op if same type
pub fn cast_at(
    block: Ref<Block>,
    arena: &mut Arena<Block>,
    source: Ref<Statement>,
    destination_type: Type,
    location: Location,
) -> Ref<Statement> {
    let s_arena = block.get(arena).arena();

    if source.get(s_arena).typ(s_arena) == destination_type {
        return source;
    }

    match (&source.get(s_arena).typ(s_arena), &destination_type) {
        // both primitives, do a cast
        (Type::Primitive(source_primitive), Type::Primitive(dest_primitive)) => {
            // compare widths
            match source_primitive.width().cmp(&dest_primitive.width()) {
                // source is larger than destination
                Ordering::Greater => build_at(
                    block,
                    arena,
                    StatementKind::Cast {
                        kind: CastOperationKind::Truncate,
                        typ: destination_type,
                        value: source,
                    },
                    location,
                ),

                // destination is larger than source
                Ordering::Less => {
                    let kind = match source_primitive.type_class() {
                        PrimitiveTypeClass::Void => panic!("cannot cast void"),
                        PrimitiveTypeClass::Unit => panic!("cannot cast unit"),
                        PrimitiveTypeClass::UnsignedInteger => CastOperationKind::ZeroExtend,
                        PrimitiveTypeClass::SignedInteger => CastOperationKind::SignExtend,
                        PrimitiveTypeClass::FloatingPoint => CastOperationKind::SignExtend,
                    };

                    build_at(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind,
                            typ: destination_type,
                            value: source,
                        },
                        location,
                    )
                }

                // equal width
                Ordering::Equal => build_at(
                    block,
                    arena,
                    StatementKind::Cast {
                        kind: CastOperationKind::Reinterpret,
                        typ: destination_type,
                        value: source,
                    },
                    location,
                ),
            }
        }

        (
            Type::Vector {
                element_count: src_count,
                element_type: src_type,
            },
            Type::Vector {
                element_count: dst_count,
                element_type: dst_type,
            },
        ) => {
            if src_type != dst_type {
                todo!();
            }

            match (src_count, dst_count) {
                (0, 0) => panic!("no cast needed, both unknown"),
                (_, 0) => {
                    // casting fixed to unknown
                    build_at(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        },
                        location,
                    )
                }
                (0, _) => {
                    // casting fixed to unknown
                    build_at(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        },
                        location,
                    )
                }
                (_, _) => panic!("casting from fixed to fixed"),
            }
        }

        (
            Type::Primitive(PrimitiveType {
                element_width_in_bits,
                ..
            }),
            Type::ArbitraryLengthInteger,
        ) => {
            assert!(*element_width_in_bits < 128);

            build_at(
                block,
                arena,
                StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                },
                location,
            )
        }

        (
            Type::Primitive(PrimitiveType {
                element_width_in_bits,
                ..
            }),
            Type::Bits,
        ) => {
            if *element_width_in_bits > 128 {
                log::warn!(
                    "source type in cast {} -> {} exceeds 128 bits",
                    source.get(s_arena).typ(s_arena),
                    destination_type
                );
            }

            build_at(
                block,
                arena,
                StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                },
                location,
            )
        }

        (Type::ArbitraryLengthInteger, Type::Primitive(_)) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        (Type::Bits, Type::Primitive(_)) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        (Type::ArbitraryLengthInteger, Type::Bits) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        (Type::ArbitraryLengthInteger, Type::Rational) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
            location,
        ),
        (Type::Rational, Type::ArbitraryLengthInteger) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        // allow casting any to anything
        (Type::Any, _) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        // unions can go from and to anything
        // todo: verify width here
        (Type::Union { .. }, _) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
            location,
        ),
        (_, Type::Union { .. }) => build_at(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
            location,
        ),

        (src, dst) => {
            println!("current block: {:?}", block.get(arena));
            panic!(
                "cannot cast {:?} from {src:?} to {dst:?}",
                source.get(s_arena)
            );
        }
    }
}

pub fn import_statement(
    source_block: Ref<Block>,
    target_block: Ref<Block>,
    block_arena: &mut Arena<Block>,
    target_statement: Ref<Statement>,
    mapping: &HashMap<Ref<Statement>, Ref<Statement>>,
) -> Ref<Statement> {
    let mapped_kind = match target_statement
        .get(target_block.get(&block_arena).arena())
        .kind()
        .clone()
    {
        StatementKind::BinaryOperation { kind, lhs, rhs } => StatementKind::BinaryOperation {
            kind,
            lhs: mapping.get(&lhs).unwrap().clone(),
            rhs: mapping.get(&rhs).unwrap().clone(),
        },
        StatementKind::Constant { typ, value } => StatementKind::Constant { typ, value },
        StatementKind::ReadVariable { symbol } => StatementKind::ReadVariable { symbol },
        StatementKind::WriteVariable { symbol, value } => StatementKind::WriteVariable {
            symbol,
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadRegister { typ, offset } => StatementKind::ReadRegister {
            typ,
            offset: mapping.get(&offset).unwrap().clone(),
        },
        StatementKind::WriteRegister { offset, value } => StatementKind::WriteRegister {
            offset: mapping.get(&offset).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadMemory { offset, size } => StatementKind::ReadMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            size: mapping.get(&size).unwrap().clone(),
        },
        StatementKind::WriteMemory { offset, value } => StatementKind::WriteMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadPc => StatementKind::ReadPc,
        StatementKind::WritePc { value } => StatementKind::WritePc {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::UnaryOperation { kind, value } => StatementKind::UnaryOperation {
            kind,
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => StatementKind::ShiftOperation {
            kind,
            value: mapping.get(&value).unwrap().clone(),
            amount: mapping.get(&amount).unwrap().clone(),
        },
        StatementKind::Call {
            target,
            args,
            return_type,
        } => {
            let args = args
                .iter()
                .map(|stmt| mapping.get(stmt).unwrap().clone())
                .collect();

            StatementKind::Call {
                target,
                args,
                return_type,
            }
        }
        StatementKind::Cast { kind, typ, value } => StatementKind::Cast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => StatementKind::BitsCast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::Jump { target } => StatementKind::Jump { target },
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => StatementKind::Branch {
            condition: mapping.get(&condition).unwrap().clone(),
            true_target,
            false_target,
        },
        StatementKind::PhiNode { .. } => todo!(),
        StatementKind::Return { value } => StatementKind::Return {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => StatementKind::Select {
            condition: mapping.get(&condition).unwrap().clone(),
            true_value: mapping.get(&true_value).unwrap().clone(),
            false_value: mapping.get(&false_value).unwrap().clone(),
        },
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => StatementKind::BitExtract {
            value: mapping.get(&value).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => StatementKind::BitInsert {
            target: mapping.get(&target).unwrap().clone(),
            source: mapping.get(&source).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::ReadElement { vector, index } => StatementKind::ReadElement {
            vector: mapping.get(&vector).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        },
        StatementKind::AssignElement {
            vector,
            value,
            index,
        } => StatementKind::AssignElement {
            vector: mapping.get(&vector).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        },
        StatementKind::Panic(stmt) => StatementKind::Panic(mapping.get(&stmt).unwrap().clone()),

        StatementKind::Assert { condition } => StatementKind::Assert {
            condition: mapping.get(&condition).unwrap().clone(),
        },

        StatementKind::CreateBits { value, length } => StatementKind::CreateBits {
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::SizeOf { value } => StatementKind::SizeOf {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::MatchesUnion { value, variant } => StatementKind::MatchesUnion {
            value: mapping.get(&value).unwrap().clone(),
            variant,
        },
        StatementKind::UnwrapUnion { value, variant } => StatementKind::UnwrapUnion {
            value: mapping.get(&value).unwrap().clone(),
            variant,
        },

        StatementKind::Undefined => StatementKind::Undefined,
        StatementKind::TupleAccess { index, source } => StatementKind::TupleAccess {
            source: mapping.get(&source).unwrap().clone(),
            index,
        },
        StatementKind::GetFlag { flag, operation } => StatementKind::GetFlag {
            flag,
            operation: mapping.get(&operation).unwrap().clone(),
        },
        StatementKind::CreateTuple(values) => StatementKind::CreateTuple(
            values
                .iter()
                .map(|v| mapping.get(&v).unwrap())
                .cloned()
                .collect(),
        ),
    };

    build(source_block, block_arena, mapped_kind)
}
