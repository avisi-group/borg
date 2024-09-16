use {
    crate::{
        rudder::{
            constant_value::ConstantValue, Block, Function, Model, PrimitiveType,
            PrimitiveTypeClass, Symbol, Type,
        },
        util::arena::{Arena, Ref},
    },
    common::{intern::InternedString, shared::Shared},
    itertools::Itertools,
    proc_macro2::TokenStream,
    quote::{format_ident, ToTokens, TokenStreamExt},
    std::{
        cmp::Ordering,
        fmt::{Debug, Formatter, Write},
        hash::{Hash, Hasher},
    },
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
        value: Ref<StatementInner>,
    },

    ReadRegister {
        typ: Type,
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Ref<StatementInner>,
    },

    WriteRegister {
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Ref<StatementInner>,
        value: Ref<StatementInner>,
    },

    ReadMemory {
        offset: Ref<StatementInner>,
        size: Ref<StatementInner>,
    },
    WriteMemory {
        offset: Ref<StatementInner>,
        value: Ref<StatementInner>,
    },

    ReadPc,
    WritePc {
        value: Ref<StatementInner>,
    },

    GetFlag {
        flag: Flag,
        operation: Ref<StatementInner>,
    },

    UnaryOperation {
        kind: UnaryOperationKind,
        value: Ref<StatementInner>,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Ref<StatementInner>,
        rhs: Ref<StatementInner>,
    },
    ShiftOperation {
        kind: ShiftOperationKind,
        value: Ref<StatementInner>,
        amount: Ref<StatementInner>,
    },
    Call {
        target: InternedString, // todo: ref<function>
        args: Vec<Ref<StatementInner>>,
    },
    Cast {
        kind: CastOperationKind,
        typ: Type,
        value: Ref<StatementInner>,
    },
    BitsCast {
        kind: CastOperationKind,
        typ: Type,
        value: Ref<StatementInner>,
        length: Ref<StatementInner>,
    },
    Jump {
        target: Ref<Block>,
    },
    Branch {
        condition: Ref<StatementInner>,
        true_target: Ref<Block>,
        false_target: Ref<Block>,
    },
    PhiNode {
        members: Vec<(Ref<Block>, Ref<StatementInner>)>,
    },
    Return {
        value: Ref<StatementInner>,
    },
    Select {
        condition: Ref<StatementInner>,
        true_value: Ref<StatementInner>,
        false_value: Ref<StatementInner>,
    },
    BitExtract {
        value: Ref<StatementInner>,
        start: Ref<StatementInner>,
        length: Ref<StatementInner>,
    },
    BitInsert {
        /// Target data that `length` bits of `source` will be inserted into at
        /// position `start`
        target: Ref<StatementInner>,
        /// Source bits that will be inserted into target
        source: Ref<StatementInner>,
        /// Offset into `target` that `source` will be inserted
        start: Ref<StatementInner>,
        /// Length of `source` that will be inserted
        length: Ref<StatementInner>,
    },
    ReadElement {
        vector: Ref<StatementInner>,
        index: Ref<StatementInner>,
    },
    /// Returns the vector with the mutated element
    AssignElement {
        vector: Ref<StatementInner>,
        value: Ref<StatementInner>,
        index: Ref<StatementInner>,
    },

    /// Fatal error, printing value of supplied Ref<StatementInner> for
    /// debugging purposes
    Panic(Ref<StatementInner>),

    /// `Default::default()`, or uninitialized, or ???
    Undefined,

    Assert {
        condition: Ref<StatementInner>,
    },

    CreateBits {
        value: Ref<StatementInner>,
        length: Ref<StatementInner>,
    },

    // creating bits and getting the value done through casting
    // gets the length when applied to bits
    SizeOf {
        value: Ref<StatementInner>,
    },

    /// Tests whether an instance of a union is of a given variant
    MatchesUnion {
        value: Ref<StatementInner>,
        variant: InternedString,
    },

    /// Extracts the contents of a variant of a union
    UnwrapUnion {
        value: Ref<StatementInner>,
        variant: InternedString,
    },

    CreateTuple(Vec<Ref<StatementInner>>),
    TupleAccess {
        index: usize,
        source: Ref<StatementInner>,
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
    fn children(&self) -> Vec<Ref<StatementInner>> {
        match self {
            StatementKind::Constant { .. }
            | StatementKind::Jump { .. }
            | StatementKind::ReadRegister { .. }
            | StatementKind::WriteRegister { .. }
            | StatementKind::ReadMemory { .. }
            | StatementKind::WriteMemory { .. }
            | StatementKind::Branch { .. }
            | StatementKind::PhiNode { .. }
            | StatementKind::Return { .. }
            | StatementKind::Panic(_)
            | StatementKind::ReadPc
            | StatementKind::WritePc { .. }
            | StatementKind::ReadVariable { .. }
            | StatementKind::WriteVariable { .. }
            | StatementKind::Undefined => {
                vec![]
            }

            StatementKind::BinaryOperation { lhs, rhs, .. } => {
                [lhs, rhs].into_iter().cloned().collect()
            }
            StatementKind::UnaryOperation { value, .. } => vec![value.clone()],
            StatementKind::ShiftOperation { value, amount, .. } => {
                [value, amount].into_iter().cloned().collect()
            }
            StatementKind::Call { args, .. } => args.clone(),
            StatementKind::Cast { value, .. } => vec![value.clone()],
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => [condition, true_value, false_value]
                .into_iter()
                .cloned()
                .collect(),

            StatementKind::BitExtract {
                value,
                start,
                length,
            } => [value, start, length].into_iter().cloned().collect(),

            StatementKind::BitInsert {
                target: original_value,
                source: insert_value,
                start,
                length,
            } => [original_value, insert_value, start, length]
                .into_iter()
                .cloned()
                .collect(),

            // complicated! todo: be more precise here
            StatementKind::ReadElement { vector, index } => {
                [vector, index].into_iter().cloned().collect()
            }
            StatementKind::AssignElement {
                vector,
                value,
                index,
            } => [value, vector, index].into_iter().cloned().collect(),

            StatementKind::Assert { condition } => vec![condition.clone()],
            StatementKind::BitsCast { value, length, .. } => {
                [value, length].into_iter().cloned().collect()
            }
            StatementKind::CreateBits { value, length } => {
                [value, length].into_iter().cloned().collect()
            }

            StatementKind::SizeOf { value }
            | StatementKind::MatchesUnion { value, .. }
            | StatementKind::UnwrapUnion { value, .. } => vec![value.clone()],

            StatementKind::TupleAccess { source, .. } => vec![source.clone()],

            StatementKind::GetFlag { operation, .. } => vec![operation.clone()],
            StatementKind::CreateTuple(values) => values.clone(),
        }
    }

    pub fn to_string(&self, arena: &Arena<StatementInner>) -> InternedString {
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
                    value.get(arena).name(),
                    value.get(arena).typ(arena)
                )
            }
            StatementKind::ReadRegister { typ, offset } => {
                format!("read-reg {}:{}", offset.get(arena).name(), typ)
            }
            StatementKind::WriteRegister { offset, value } => {
                format!(
                    "write-reg {} <= {}",
                    offset.get(arena).name(),
                    value.get(arena).name()
                )
            }
            StatementKind::ReadMemory { offset, size } => {
                format!(
                    "read-mem {}:{}",
                    offset.get(arena).name(),
                    size.get(arena).name()
                )
            }
            StatementKind::WriteMemory { offset, value } => {
                format!(
                    "write-mem {} <= {}",
                    offset.get(arena).name(),
                    value.get(arena).name()
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

                format!("{} {} {}", op, lhs.get(arena).name(), rhs.get(arena).name())
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

                format!("{} {}", op, value.get(arena).name())
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
                    value.get(arena).name(),
                    amount.get(arena).name()
                )
            }
            StatementKind::Call { target, args } => {
                format!(
                    "call {}({})",
                    target,
                    args.iter().map(|s| s.get(arena).name()).join(", ")
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

                format!("cast {} {} -> {}", op, value.get(arena).name(), typ)
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
                    value.get(arena).name(),
                    typ,
                    length.get(arena).name()
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
                    condition.get(arena).name(),
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
                format!("return {}", value.get(arena).name())
            }
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                format!(
                    "select {} {} {}",
                    condition.get(arena).name(),
                    true_value.get(arena).name(),
                    false_value.get(arena).name()
                )
            }
            StatementKind::Panic(statement) => {
                format!("panic {}", statement.get(arena).name())
            }
            StatementKind::Undefined => format!("undefined",),

            StatementKind::ReadPc => format!("read-pc"),
            StatementKind::WritePc { value } => format!("write-pc {}", value.get(arena).name()),
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => format!(
                "bit-extract {} {} {}",
                value.get(arena).name(),
                start.get(arena).name(),
                length.get(arena).name()
            ),
            StatementKind::BitInsert {
                target: original_value,
                source: insert_value,
                start,
                length,
            } => format!(
                "bit-insert {} {} {} {}",
                original_value.get(arena).name(),
                insert_value.get(arena).name(),
                start.get(arena).name(),
                length.get(arena).name()
            ),
            StatementKind::ReadElement { vector, index } => {
                format!(
                    "read-element {}[{}]",
                    vector.get(arena).name(),
                    index.get(arena).name()
                )
            }
            StatementKind::AssignElement {
                vector,
                value,
                index,
            } => format!(
                "mutate-element {}[{}] <= {}",
                vector.get(arena).name(),
                index.get(arena).name(),
                value.get(arena).name()
            ),

            StatementKind::SizeOf { value } => {
                format!("size-of {}", value.get(arena).name())
            }
            StatementKind::Assert { condition } => {
                format!("assert {}", condition.get(arena).name())
            }

            StatementKind::CreateBits { value, length } => {
                format!(
                    "create-bits {} {}",
                    value.get(arena).name(),
                    length.get(arena).name()
                )
            }
            StatementKind::MatchesUnion { value, variant } => {
                format!("matches-union {} {variant}", value.get(arena).name())
            }
            StatementKind::UnwrapUnion { value, variant } => {
                format!("unwrap-union {} {variant}", value.get(arena).name())
            }
            StatementKind::TupleAccess { index, source } => {
                format!("tuple-access {}.{index}", source.get(arena).name())
            }
            StatementKind::GetFlag { flag, operation } => {
                format!("get-flag {flag:?} {}", operation.get(arena).name())
            }
            StatementKind::CreateTuple(values) => {
                format!(
                    "create-tuple {:?}",
                    values
                        .iter()
                        .map(|v| v.get(arena).name())
                        .collect::<Vec<_>>()
                )
            }
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct StatementInner {
    name: InternedString,
    kind: StatementKind,
    parent: Ref<Block>,
}
impl ToTokens for StatementInner {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("{}", self.name().to_string())) //todo: fix
                                                                    // this?
    }
}

// impl Statement {

//     pub fn replace_kind(&self, kind: StatementKind) {
//         let mut inner = self.inner.get_mut();
//         inner.replace_kind(kind);
//     }

//     pub fn replace_use(&self, use_of: Statement, with: Statement) {
//         let mut inner = self.inner.get_mut();
//         inner.replace_use(use_of, with);
//     }

//     pub fn update_names(&self, name: InternedString) {
//         self.inner.get_mut().update_names(name);
//     }

//     pub fn typ(&self) -> Type {

//     }

// }

impl StatementInner {
    pub fn to_string(&self, arena: &Arena<StatementInner>) -> InternedString {
        format!("{}: {}", self.name(), self.kind().to_string(arena)).into()
    }

    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }
    pub fn name(&self) -> InternedString {
        self.name
    }

    pub fn parent_block(&self) -> Ref<Block> {
        self.parent
    }

    //     pub fn has_value(&self) -> bool {
    //     !self.typ().is_void()
    // }

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

    pub fn typ(&self, arena: &Arena<StatementInner>) -> Type {
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
            StatementKind::Call { target, .. } => Type::Any, // todo: need rudder model
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

    pub fn replace_use(&mut self, use_of: Ref<StatementInner>, with: Ref<StatementInner>) {
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
            StatementKind::Call { target, args } => {
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

                self.kind = StatementKind::Call { target, args };
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

/// Creates a new statement in the block's arena, and pushes it to the end of
/// the block's statements
pub fn build(
    block: Ref<Block>,
    arena: &mut Arena<Block>,
    kind: StatementKind,
) -> Ref<StatementInner> {
    let r = block.get_mut(arena).arena_mut().insert(StatementInner {
        name: "???".into(),
        kind,
        parent: block,
    });
    block.get_mut(arena).statements.push(r);
    r
}

// No-op if same type
pub fn cast(
    block: Ref<Block>,
    arena: &mut Arena<Block>,
    source: Ref<StatementInner>,
    destination_type: Type,
) -> Ref<StatementInner> {
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
                Ordering::Greater => build(
                    block,
                    arena,
                    StatementKind::Cast {
                        kind: CastOperationKind::Truncate,
                        typ: destination_type,
                        value: source,
                    },
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

                    build(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind,
                            typ: destination_type,
                            value: source,
                        },
                    )
                }

                // equal width
                Ordering::Equal => build(
                    block,
                    arena,
                    StatementKind::Cast {
                        kind: CastOperationKind::Reinterpret,
                        typ: destination_type,
                        value: source,
                    },
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
                    build(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        },
                    )
                }
                (0, _) => {
                    // casting fixed to unknown
                    build(
                        block,
                        arena,
                        StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        },
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

            build(
                block,
                arena,
                StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                },
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

            build(
                block,
                arena,
                StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                },
            )
        }

        (Type::ArbitraryLengthInteger, Type::Primitive(_)) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
        ),

        (Type::Bits, Type::Primitive(_)) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
        ),

        (Type::ArbitraryLengthInteger, Type::Bits) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
        ),

        (Type::ArbitraryLengthInteger, Type::Rational) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
        ),
        (Type::Rational, Type::ArbitraryLengthInteger) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
        ),

        // allow casting any to anything
        (Type::Any, _) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            },
        ),

        // unions can go from and to anything
        // todo: verify width here
        (Type::Union { .. }, _) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
        ),
        (_, Type::Union { .. }) => build(
            block,
            arena,
            StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            },
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
