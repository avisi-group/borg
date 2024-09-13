use {
    crate::rudder::{
        constant_value::ConstantValue, Block, Function, PrimitiveType, PrimitiveTypeClass, Symbol,
        Type, WeakBlock,
    },
    common::{intern::InternedString, shared::Shared},
    proc_macro2::TokenStream,
    quote::{format_ident, ToTokens, TokenStreamExt},
    std::{
        cmp::Ordering,
        fmt::Debug,
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

#[derive(Debug, Clone, PartialEq)]
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
        value: Statement,
    },

    ReadRegister {
        typ: Type,
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Statement,
    },

    WriteRegister {
        /// offset into register state
        ///
        /// During building, this should just be the `next_register_offset`
        /// value, not accessing any elements or fields
        offset: Statement,
        value: Statement,
    },

    ReadMemory {
        offset: Statement,
        size: Statement,
    },
    WriteMemory {
        offset: Statement,
        value: Statement,
    },

    ReadPc,
    WritePc {
        value: Statement,
    },

    GetFlag {
        flag: Flag,
        operation: Statement,
    },

    UnaryOperation {
        kind: UnaryOperationKind,
        value: Statement,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Statement,
        rhs: Statement,
    },
    ShiftOperation {
        kind: ShiftOperationKind,
        value: Statement,
        amount: Statement,
    },
    Call {
        target: Function,
        args: Vec<Statement>,
        tail: bool,
    },
    Cast {
        kind: CastOperationKind,
        typ: Type,
        value: Statement,
    },
    BitsCast {
        kind: CastOperationKind,
        typ: Type,
        value: Statement,
        length: Statement,
    },
    Jump {
        target: Block,
    },
    Branch {
        condition: Statement,
        true_target: Block,
        false_target: Block,
    },
    PhiNode {
        members: Vec<(Block, Statement)>,
    },
    Return {
        value: Statement,
    },
    Select {
        condition: Statement,
        true_value: Statement,
        false_value: Statement,
    },
    BitExtract {
        value: Statement,
        start: Statement,
        length: Statement,
    },
    BitInsert {
        /// Target data that `length` bits of `source` will be inserted into at
        /// position `start`
        target: Statement,
        /// Source bits that will be inserted into target
        source: Statement,
        /// Offset into `target` that `source` will be inserted
        start: Statement,
        /// Length of `source` that will be inserted
        length: Statement,
    },
    ReadElement {
        vector: Statement,
        index: Statement,
    },
    /// Returns the vector with the mutated element
    AssignElement {
        vector: Statement,
        value: Statement,
        index: Statement,
    },

    /// Fatal error, printing value of supplied statement for debugging
    /// purposes
    Panic(Statement),

    /// `Default::default()`, or uninitialized, or ???
    Undefined,

    Assert {
        condition: Statement,
    },

    CreateBits {
        value: Statement,
        length: Statement,
    },

    // creating bits and getting the value done through casting
    // gets the length when applied to bits
    SizeOf {
        value: Statement,
    },

    /// Tests whether an instance of a union is of a given variant
    MatchesUnion {
        value: Statement,
        variant: InternedString,
    },

    /// Extracts the contents of a variant of a union
    UnwrapUnion {
        value: Statement,
        variant: InternedString,
    },

    CreateTuple(Vec<Statement>),
    TupleAccess {
        index: usize,
        source: Statement,
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
    fn children(&self) -> Vec<Statement> {
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
            | StatementKind::Undefined => vec![],

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
}

#[derive(Debug, Clone)]
pub struct Statement {
    inner: Shared<StatementInner>,
}

impl Hash for Statement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::ptr::hash(self.inner.as_ptr(), state)
    }
}

impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        Shared::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Statement {}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(format_ident!("{}", self.name().to_string())) //todo: fix this?
    }
}

#[derive(Debug)]
pub struct StatementInner {
    name: InternedString,
    kind: StatementKind,
    parent: WeakBlock,
}

impl Statement {
    pub fn kind(&self) -> StatementKind {
        self.inner.get().kind.clone()
    }

    pub fn replace_kind(&self, kind: StatementKind) {
        let mut inner = self.inner.get_mut();
        inner.replace_kind(kind);
    }

    pub fn replace_use(&self, use_of: Statement, with: Statement) {
        let mut inner = self.inner.get_mut();
        inner.replace_use(use_of, with);
    }

    pub fn name(&self) -> InternedString {
        self.inner.get().name
    }

    pub fn parent_block(&self) -> WeakBlock {
        self.inner.get().parent.clone()
    }

    pub fn update_names(&self, name: InternedString) {
        self.inner.get_mut().update_names(name);
    }

    pub fn typ(&self) -> Type {
        match self.kind() {
            StatementKind::Constant { typ, .. } => typ,
            StatementKind::ReadVariable { symbol } => symbol.typ(),
            StatementKind::WriteVariable { .. } => Type::void(),
            StatementKind::ReadRegister { typ, .. } => typ,
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
            StatementKind::BinaryOperation { lhs, .. } => lhs.typ(),
            StatementKind::UnaryOperation { value, .. } => value.typ(),
            StatementKind::ShiftOperation { value, .. } => value.typ(),
            StatementKind::Call { target, .. } => target.return_type(),
            StatementKind::Cast { typ, .. } | StatementKind::BitsCast { typ, .. } => typ,
            StatementKind::Jump { .. } => Type::void(),
            StatementKind::Branch { .. } => Type::void(),
            StatementKind::PhiNode { members } => members
                .first()
                .map(|(_, stmt)| stmt.typ())
                .unwrap_or_else(|| (Type::void())),
            StatementKind::Return { .. } => Type::void(),
            StatementKind::Select { true_value, .. } => true_value.typ(),
            StatementKind::Panic(_) => Type::void(),

            StatementKind::ReadPc => Type::u64(),
            StatementKind::WritePc { .. } => Type::void(),
            // todo: this is a simplification, be more precise about lengths?
            StatementKind::BitExtract { value, .. } => value.typ(),
            StatementKind::BitInsert {
                target: original_value,
                ..
            } => original_value.typ(),
            StatementKind::ReadElement { vector, .. } => {
                let Type::Vector { element_type, .. } = &vector.typ() else {
                    panic!("cannot read field of non-composite type")
                };

                (**element_type).clone()
            }
            StatementKind::AssignElement { vector, .. } => {
                // get type of the vector and return it
                vector.typ()
            }

            StatementKind::SizeOf { .. } => Type::u16(),
            StatementKind::Assert { .. } => Type::unit(),
            StatementKind::CreateBits { .. } => Type::Bits,
            StatementKind::MatchesUnion { .. } => Type::u1(),
            StatementKind::UnwrapUnion { value, variant } => {
                // let Type::Enum(variants) = &*value.typ() else {
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
                let Type::Tuple(ts) = &source.typ() else {
                    panic!();
                };

                ts[index].clone()
            }

            StatementKind::GetFlag { .. } => Type::u1(),
            StatementKind::CreateTuple(values) => {
                Type::Tuple(values.iter().map(|v| v.typ()).collect())
            }
        }
    }

    pub fn has_value(&self) -> bool {
        !self.typ().is_void()
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
}

impl StatementInner {
    pub fn update_names(&mut self, name: InternedString) {
        self.name = name;
    }

    pub fn replace_kind(&mut self, kind: StatementKind) {
        self.kind = kind;
    }

    pub fn replace_use(&mut self, use_of: Statement, with: Statement) {
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

            StatementKind::Call { target, args, tail } => {
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

                self.kind = StatementKind::Call { target, args, tail };
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

pub struct StatementBuilder {
    statements: Vec<Statement>,
    parent: WeakBlock,
}

impl StatementBuilder {
    /// Creates a new `StatementBuilder`
    pub fn new(parent: WeakBlock) -> Self {
        Self {
            statements: vec![],
            parent,
        }
    }

    /// Builds a new `Statement` from a `StatementKind`, adds it to the builder,
    /// and returns it
    pub fn build(&mut self, kind: StatementKind) -> Statement {
        let statement = Statement {
            inner: Shared::new(StatementInner {
                name: "???".into(),
                kind,
                parent: self.parent.clone(),
            }),
        };

        self.statements.push(statement.clone());

        statement
    }

    /// Consumes a `StatementBuilder` and returns it's statements
    pub fn finish(self) -> Vec<Statement> {
        self.statements
    }

    // No-op if same type
    pub fn generate_cast(&mut self, source: Statement, destination_type: Type) -> Statement {
        if source.typ() == destination_type {
            return source;
        }

        match (&source.typ(), &destination_type) {
            // both primitives, do a cast
            (Type::Primitive(source_primitive), Type::Primitive(dest_primitive)) => {
                // compare widths
                match source_primitive.width().cmp(&dest_primitive.width()) {
                    // source is larger than destination
                    Ordering::Greater => self.build(StatementKind::Cast {
                        kind: CastOperationKind::Truncate,
                        typ: destination_type,
                        value: source,
                    }),

                    // destination is larger than source
                    Ordering::Less => {
                        let kind = match source_primitive.type_class() {
                            PrimitiveTypeClass::Void => panic!("cannot cast void"),
                            PrimitiveTypeClass::Unit => panic!("cannot cast unit"),
                            PrimitiveTypeClass::UnsignedInteger => CastOperationKind::ZeroExtend,
                            PrimitiveTypeClass::SignedInteger => CastOperationKind::SignExtend,
                            PrimitiveTypeClass::FloatingPoint => CastOperationKind::SignExtend,
                        };

                        self.build(StatementKind::Cast {
                            kind,
                            typ: destination_type,
                            value: source,
                        })
                    }

                    // equal width
                    Ordering::Equal => self.build(StatementKind::Cast {
                        kind: CastOperationKind::Reinterpret,
                        typ: destination_type,
                        value: source,
                    }),
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
                        self.build(StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        })
                    }
                    (0, _) => {
                        // casting fixed to unknown
                        self.build(StatementKind::Cast {
                            kind: CastOperationKind::Convert,
                            typ: destination_type,
                            value: source,
                        })
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

                self.build(StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                })
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
                        source.typ(),
                        destination_type
                    );
                }

                self.build(StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: destination_type,
                    value: source,
                })
            }

            (Type::ArbitraryLengthInteger, Type::Primitive(_)) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            }),

            (Type::Bits, Type::Primitive(_)) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            }),

            (Type::ArbitraryLengthInteger, Type::Bits) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            }),

            (Type::ArbitraryLengthInteger, Type::Rational) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            }),
            (Type::Rational, Type::ArbitraryLengthInteger) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            }),

            // allow casting any to anything
            (Type::Any, _) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Convert,
                typ: destination_type,
                value: source,
            }),

            // unions can go from and to anything
            // todo: verify width here
            (Type::Union { .. }, _) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            }),
            (_, Type::Union { .. }) => self.build(StatementKind::Cast {
                kind: CastOperationKind::Reinterpret,
                typ: destination_type,
                value: source,
            }),

            (src, dst) => {
                println!("current statements: {:?}", self.statements);
                panic!(
                    "cannot cast {:?} from {src:?} to {dst:?}",
                    *source.inner.get()
                );
            }
        }
    }
}
