use {
    crate::{brig::bits::BitsValue, rudder::constant_value::ConstantValue},
    common::{
        intern::InternedString,
        shared::{Shared, Weak},
        HashMap, HashSet,
    },
    log::warn,
    proc_macro2::TokenStream,
    quote::ToTokens,
    std::{
        cmp::Ordering,
        fmt::Debug,
        hash::{Hash, Hasher},
        sync::Arc,
    },
};

pub mod analysis;
pub mod build;
pub mod constant_value;
mod internal_fns;
pub mod opt;
mod pretty_print;
pub mod validator;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum PrimitiveTypeClass {
    Void,
    Unit,
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct PrimitiveType {
    pub tc: PrimitiveTypeClass,
    pub element_width_in_bits: usize,
}

impl PrimitiveType {
    pub fn type_class(&self) -> PrimitiveTypeClass {
        self.tc
    }

    pub fn width(&self) -> usize {
        self.element_width_in_bits
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Composite(Vec<Arc<Type>>),
    Vector {
        element_count: usize,
        element_type: Arc<Type>,
    },

    Bits,
    ArbitraryLengthInteger,
}

macro_rules! type_def_helper {
    ($name: ident, $cls: ident, $width: expr) => {
        pub fn $name() -> Self {
            Self::new_primitive(PrimitiveTypeClass::$cls, $width)
        }
    };
}

impl Type {
    pub fn new_primitive(tc: PrimitiveTypeClass, element_width: usize) -> Self {
        Self::Primitive(PrimitiveType {
            tc,
            element_width_in_bits: element_width,
        })
    }

    pub fn new_composite(fields: Vec<Arc<Type>>) -> Self {
        Self::Composite(fields)
    }

    pub fn void() -> Self {
        Self::Primitive(PrimitiveType {
            tc: PrimitiveTypeClass::Void,
            element_width_in_bits: 0,
        })
    }

    pub fn unit() -> Self {
        Self::Primitive(PrimitiveType {
            tc: PrimitiveTypeClass::Unit,
            element_width_in_bits: 0,
        })
    }

    /// Gets the offset in bytes of a field of a composite or an element of a
    /// vector
    pub fn byte_offset(&self, element_field: usize) -> Option<usize> {
        match self {
            Type::Composite(fields) => Some(
                fields
                    .iter()
                    .take(element_field)
                    .fold(0, |acc, typ| acc + typ.width_bytes()),
            ),
            Type::Vector { element_type, .. } => Some(element_field * element_type.width_bytes()),
            _ => None,
        }
    }

    pub fn width_bits(&self) -> usize {
        match self {
            Self::Composite(xs) => xs.iter().map(|x| x.width_bits()).sum(),
            // smallest with is 8 bits
            Self::Primitive(p) => p.element_width_in_bits.max(8),
            Self::Vector {
                element_count,
                element_type,
            } => element_type.width_bits() * element_count,
            Self::Bits | Self::ArbitraryLengthInteger => usize::try_from(BitsValue::BITS).unwrap(),
        }
    }

    pub fn width_bytes(&self) -> usize {
        self.width_bits().div_ceil(8)
    }

    type_def_helper!(u1, UnsignedInteger, 1);
    type_def_helper!(u8, UnsignedInteger, 8);
    type_def_helper!(u16, UnsignedInteger, 16);
    type_def_helper!(u32, UnsignedInteger, 32);
    type_def_helper!(u64, UnsignedInteger, 64);
    type_def_helper!(u128, UnsignedInteger, 128);
    type_def_helper!(s8, SignedInteger, 8);
    type_def_helper!(s16, SignedInteger, 16);
    type_def_helper!(s32, SignedInteger, 32);
    type_def_helper!(s64, SignedInteger, 64);
    type_def_helper!(s128, SignedInteger, 128);
    type_def_helper!(f32, FloatingPoint, 32);
    type_def_helper!(f64, FloatingPoint, 64);

    pub fn vectorize(self, element_count: usize) -> Self {
        Self::Vector {
            element_count,
            element_type: Arc::new(self),
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType { tc, .. }) => {
                matches!(tc, PrimitiveTypeClass::Void)
            }
            _ => false,
        }
    }

    pub fn is_unit(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType { tc, .. }) => {
                matches!(tc, PrimitiveTypeClass::Unit)
            }
            _ => false,
        }
    }

    pub fn is_bits(&self) -> bool {
        matches!(self, Self::Bits)
    }

    pub fn is_u1(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::UnsignedInteger,
                element_width_in_bits,
            }) => *element_width_in_bits == 1,
            _ => false,
        }
    }

    pub fn is_unknown_length_vector(&self) -> bool {
        matches!(
            self,
            Self::Vector {
                element_count: 0,
                ..
            }
        )
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SymbolKind {
    Parameter,
    LocalVariable,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: InternedString,
    kind: SymbolKind,
    typ: Arc<Type>,
}

impl Symbol {
    pub fn name(&self) -> InternedString {
        self.name
    }

    pub fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub fn typ(&self) -> Arc<Type> {
        self.typ.clone()
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperationKind {
    Add,
    Sub,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    CompareEqual,
    CompareNotEqual,
    CompareLessThan,
    CompareLessThanOrEqual,
    CompareGreaterThan,
    CompareGreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum UnaryOperationKind {
    Not,
    Negate,
    Complement,
    Power2,
    Absolute,
}

#[derive(Debug, Clone)]
pub enum CastOperationKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    Reinterpret,
    Convert,
    Broadcast,
}

#[derive(Debug, Clone)]
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
        typ: Arc<Type>,
        value: ConstantValue,
    },

    ReadVariable {
        symbol: Symbol,
        /// Indices, when not empty, indicate access to fields/elements
        ///
        /// [1,4,2] => 1st field of the 4th element of the 2nd field for a
        /// struct of a vec of structs
        indices: Vec<usize>,
    },

    WriteVariable {
        symbol: Symbol,
        /// Indices, when not empty, indicate access to fields/elements
        ///
        /// [1,4,2] => 1st field of the 4th element of the 2nd field for a
        /// struct of a vec of structs
        indices: Vec<usize>,
        value: Statement,
    },

    ReadRegister {
        typ: Arc<Type>,
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
        typ: Arc<Type>,
        offset: Statement,
    },
    WriteMemory {
        offset: Statement,
        value: Statement,
    },

    ReadPc,

    WritePc {
        value: Statement,
    },

    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Statement,
        rhs: Statement,
    },
    UnaryOperation {
        kind: UnaryOperationKind,
        value: Statement,
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
        typ: Arc<Type>,
        value: Statement,
    },
    BitsCast {
        kind: CastOperationKind,
        typ: Arc<Type>,
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
        value: Option<Statement>,
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
        original_value: Statement,
        insert_value: Statement,
        start: Statement,
        length: Statement,
    },
    ReadElement {
        vector: Statement,
        index: Statement,
    },
    /// Returns the vector with the mutated element
    MutateElement {
        vector: Statement,
        value: Statement,
        index: Statement,
    },

    /// Fatal error, printing values of supplied statements for debugging
    /// purposes
    Panic(Vec<Statement>),

    Assert {
        condition: Statement,
    },

    CreateComposite {
        typ: Arc<Type>,
        /// Index of fields should match type
        fields: Vec<Statement>,
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
}

#[derive(Eq, PartialEq)]
pub enum ValueClass {
    None,
    Constant,
    Static,
    Dynamic,
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
        self.inner.get_mut().replace_kind(kind);
    }

    pub fn replace_use(&self, use_of: Statement, with: Statement) {
        self.inner.get_mut().replace_use(use_of, with);
    }

    pub fn name(&self) -> InternedString {
        self.inner.get().name
    }

    pub fn parent(&self) -> WeakBlock {
        self.inner.get().parent.clone()
    }

    pub fn update_names(&self, name: InternedString) {
        self.inner.get_mut().update_names(name);
    }

    pub fn classify(&self) -> ValueClass {
        match self.kind() {
            StatementKind::Constant { .. } => ValueClass::Constant,
            StatementKind::ReadRegister { .. } => ValueClass::Dynamic,
            StatementKind::WriteRegister { .. } => ValueClass::None,
            StatementKind::ReadMemory { .. } => ValueClass::Dynamic,
            StatementKind::WriteMemory { .. } => ValueClass::None,
            StatementKind::BinaryOperation { lhs, rhs, .. } => {
                match (lhs.classify(), rhs.classify()) {
                    (ValueClass::Constant, ValueClass::Constant) => ValueClass::Constant,
                    (ValueClass::Constant, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Constant, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Static, ValueClass::Constant) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Constant) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Static) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Dynamic) => ValueClass::Dynamic,
                    _ => panic!("cannot classify binary operation"),
                }
            }
            StatementKind::UnaryOperation { value, .. } => match value.classify() {
                ValueClass::Constant => ValueClass::Constant,
                ValueClass::Static => ValueClass::Static,
                ValueClass::Dynamic => ValueClass::Dynamic,
                _ => panic!("cannot classify unary operation"),
            },
            StatementKind::ShiftOperation { value, amount, .. } => {
                match (value.classify(), amount.classify()) {
                    (ValueClass::Constant, ValueClass::Constant) => ValueClass::Constant,
                    (ValueClass::Constant, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Constant) => ValueClass::Static,
                    (ValueClass::Dynamic, ValueClass::Constant) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Static) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Constant, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Static, ValueClass::Dynamic) => ValueClass::Dynamic,
                    _ => panic!("cannot classify shift operation"),
                }
            }
            StatementKind::Call { args, .. } => {
                if args.iter().any(|a| a.classify() == ValueClass::None) {
                    panic!("illegal arguments to function call");
                }

                if args.iter().any(|a| a.classify() == ValueClass::Dynamic) {
                    ValueClass::Dynamic
                } else {
                    ValueClass::Static
                }
            }
            StatementKind::Cast { value, .. } => match value.classify() {
                ValueClass::Constant => ValueClass::Constant,
                ValueClass::Static => ValueClass::Static,
                ValueClass::Dynamic => ValueClass::Dynamic,
                _ => panic!("cannot classify cast operation {} in {}", value, self),
            },
            StatementKind::Jump { .. } => ValueClass::None,
            StatementKind::Branch { .. } => ValueClass::None,
            StatementKind::PhiNode { .. } => todo!(),
            StatementKind::Return { .. } => ValueClass::None,
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                match (
                    condition.classify(),
                    true_value.classify(),
                    false_value.classify(),
                ) {
                    (ValueClass::Static, ValueClass::Static, ValueClass::Static) => {
                        ValueClass::Static
                    }
                    _ => ValueClass::Dynamic,
                }
            }
            StatementKind::Panic(_) => ValueClass::None,
            StatementKind::ReadPc => ValueClass::Dynamic,
            StatementKind::WritePc { .. } => ValueClass::None,
            StatementKind::BitExtract { .. } => ValueClass::Dynamic,
            StatementKind::BitInsert { .. } => ValueClass::Dynamic,
            StatementKind::ReadVariable { .. } => ValueClass::Dynamic,
            StatementKind::WriteVariable { .. } => ValueClass::Dynamic,
            StatementKind::ReadElement { .. } => ValueClass::Dynamic,
            StatementKind::MutateElement { .. } => ValueClass::Dynamic,
            StatementKind::CreateComposite { .. } => ValueClass::Dynamic,
            StatementKind::SizeOf { .. } => ValueClass::Dynamic,
            StatementKind::Assert { .. } => ValueClass::None,
            StatementKind::BitsCast { .. } => ValueClass::Dynamic,
            StatementKind::CreateBits { .. } => ValueClass::Dynamic,
        }
    }

    pub fn typ(&self) -> Arc<Type> {
        match self.kind() {
            StatementKind::Constant { typ, .. } => typ,
            StatementKind::ReadVariable { symbol, indices } => {
                let mut current_type = symbol.typ();

                for index in indices {
                    if let Type::Composite(fields) = &*current_type {
                        current_type = fields[index].clone();
                    } else {
                        panic!("cannot get field of non-composite type")
                    }
                }

                current_type
            }
            StatementKind::WriteVariable { .. } => Arc::new(Type::void()),
            StatementKind::ReadRegister { typ, .. } => typ,
            StatementKind::WriteRegister { .. } => Arc::new(Type::unit()),
            StatementKind::ReadMemory { typ, .. } => typ,
            StatementKind::WriteMemory { .. } => Arc::new(Type::unit()),
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
            } => Arc::new(Type::u1()),
            StatementKind::BinaryOperation { lhs, .. } => lhs.typ(),
            StatementKind::UnaryOperation { value, .. } => value.typ(),
            StatementKind::ShiftOperation { value, .. } => value.typ(),
            StatementKind::Call { target, tail, .. } => {
                if !tail {
                    target.return_type()
                } else {
                    Arc::new(Type::void())
                }
            }
            StatementKind::Cast { typ, .. } | StatementKind::BitsCast { typ, .. } => typ,
            StatementKind::Jump { .. } => Arc::new(Type::void()),
            StatementKind::Branch { .. } => Arc::new(Type::void()),
            StatementKind::PhiNode { members } => members
                .first()
                .map(|(_, stmt)| stmt.typ())
                .unwrap_or_else(|| Arc::new(Type::void())),
            StatementKind::Return { .. } => Arc::new(Type::void()),
            StatementKind::Select { true_value, .. } => true_value.typ(),
            StatementKind::Panic(_) => Arc::new(Type::void()),
            StatementKind::ReadPc => Arc::new(Type::u64()),
            StatementKind::WritePc { .. } => Arc::new(Type::void()),
            StatementKind::BitExtract { value, .. } => value.typ(),
            StatementKind::BitInsert { original_value, .. } => original_value.typ(),
            StatementKind::ReadElement { vector, .. } => {
                let Type::Vector { element_type, .. } = &*vector.typ() else {
                    panic!("cannot read field of non-composite type")
                };

                element_type.clone()
            }
            StatementKind::MutateElement { vector, .. } => {
                // get type of the vector and return it
                vector.typ()
            }
            StatementKind::CreateComposite { typ, .. } => typ,
            StatementKind::SizeOf { .. } => Arc::new(Type::u16()),
            StatementKind::Assert { .. } => Arc::new(Type::unit()),
            StatementKind::CreateBits { .. } => Arc::new(Type::Bits),
        }
    }

    pub fn has_value(&self) -> bool {
        !self.typ().is_void()
    }

    pub fn has_side_effects(&self) -> bool {
        match self.kind() {
            StatementKind::WriteVariable { .. } => true,
            StatementKind::WriteRegister { .. } => true,
            StatementKind::WriteMemory { .. } => true,
            StatementKind::WritePc { .. } => true,
            StatementKind::Call { .. } => true,
            StatementKind::Jump { .. } => true,
            StatementKind::Branch { .. } => true,
            StatementKind::Return { .. } => true,
            StatementKind::Panic(_) => true,
            StatementKind::Assert { .. } => true,
            _ => false,
        }
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
            StatementKind::Return { .. } => {
                self.kind = StatementKind::Return {
                    value: Some(with.clone()),
                };
            }
            StatementKind::Branch {
                true_target,
                false_target,
                ..
            } => {
                self.kind = StatementKind::Branch {
                    condition: with.clone(),
                    true_target: true_target,
                    false_target: false_target,
                };
            }
            StatementKind::WriteVariable {
                symbol, indices, ..
            } => {
                self.kind = StatementKind::WriteVariable {
                    symbol,
                    indices,
                    value: with.clone(),
                };
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
            StatementKind::UnaryOperation { kind, .. } => {
                self.kind = StatementKind::UnaryOperation {
                    kind,
                    value: with.clone(),
                };
            }

            StatementKind::Cast { kind, typ, .. } => {
                self.kind = StatementKind::Cast {
                    kind,
                    typ: typ,
                    value: with.clone(),
                };
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

            StatementKind::Assert { .. } => {
                self.kind = StatementKind::Assert {
                    condition: with.clone(),
                };
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

            StatementKind::CreateComposite { typ, fields } => {
                let fields = fields
                    .iter()
                    .map(|field| {
                        if *field == use_of {
                            with.clone()
                        } else {
                            field.clone()
                        }
                    })
                    .collect();

                self.kind = StatementKind::CreateComposite { typ, fields };
            }
            StatementKind::BitInsert {
                original_value,
                insert_value,
                start,
                length,
            } => {
                let stmts = [original_value, insert_value, start, length]
                    .into_iter()
                    .map(|s| if s == use_of { with.clone() } else { s })
                    .collect::<Vec<_>>();

                self.kind = StatementKind::BitInsert {
                    original_value: stmts[0].clone(),
                    insert_value: stmts[1].clone(),
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

            _ => {
                panic!("use replacement not implemented for {}", self.kind);
            }
        }
    }
}

struct StatementBuilder {
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
                name: "".into(),
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
    fn generate_cast(&mut self, source: Statement, destination_type: Arc<Type>) -> Statement {
        if source.typ() == destination_type {
            return source;
        }

        match (&*source.typ(), &*destination_type) {
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
                            PrimitiveTypeClass::FloatingPoint => {
                                panic!("cannot cast floating point")
                            }
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

                if *src_count > 0 && *dst_count == 0 {
                    // casting known to unknown
                    self.build(StatementKind::Cast {
                        kind: CastOperationKind::Convert,
                        typ: destination_type,
                        value: source,
                    })
                } else {
                    todo!()
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
                assert!(*element_width_in_bits < 128);

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

            (src, dst) => {
                panic!("cannot cast from {src} to {dst}");
            }
        }
    }
}

#[derive(Clone)]
pub struct Block {
    inner: Shared<BlockInner>,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.get().name)
    }
}

#[derive(Debug, Clone)]
pub struct WeakBlock {
    inner: Weak<BlockInner>,
}

impl WeakBlock {
    pub fn upgrade(&self) -> Block {
        Block {
            inner: self.inner.upgrade().unwrap(),
        }
    }
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn weak(&self) -> WeakBlock {
        WeakBlock {
            inner: self.inner.downgrade(),
        }
    }

    pub fn name(&self) -> InternedString {
        self.inner.get().name
    }

    pub fn update_names(&self, name: InternedString) {
        self.inner.get_mut().update_names(name);
    }

    pub fn statements(&self) -> Vec<Statement> {
        self.inner.get().statements.clone()
    }

    pub fn terminator_statement(&self) -> Option<Statement> {
        self.inner.get().statements.last().cloned()
    }

    pub fn set_statements<I: Iterator<Item = Statement>>(&self, statements: I) {
        self.inner.get_mut().statements = statements.collect();
    }

    pub fn extend_statements<I: Iterator<Item = Statement>>(&self, stmts: I) {
        self.inner.get_mut().statements.extend(stmts)
    }

    fn index_of_statement(&self, reference: &Statement) -> usize {
        self.inner
            .get()
            .statements
            .iter()
            .enumerate()
            .find(|(_, candidate)| *candidate == reference)
            .unwrap()
            .0
    }

    pub fn insert_statement_before(&self, reference: &Statement, new: Statement) {
        let index = self.index_of_statement(reference);
        self.inner.get_mut().statements.insert(index, new);
    }

    pub fn kill_statement(&self, stmt: &Statement) {
        //assert!(Rc::ptr_eq()

        let index = self.index_of_statement(stmt);

        self.inner.get_mut().statements.remove(index);
    }

    pub fn iter(&self) -> BlockIterator {
        BlockIterator::new(self.clone())
    }

    pub fn targets(&self) -> Vec<Block> {
        match self.terminator_statement().unwrap().kind() {
            StatementKind::Jump { target } => vec![target],
            StatementKind::Branch {
                true_target,
                false_target,
                ..
            } => vec![true_target, false_target],
            StatementKind::Return { .. }
            | StatementKind::Panic(_)
            | StatementKind::Call { tail: true, .. } => vec![],
            _ => panic!("invalid terminator for block"),
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            inner: Shared::new(BlockInner {
                name: "???".into(),
                statements: Vec::new(),
            }),
        }
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::ptr::hash(self.inner.as_ptr(), state)
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        Shared::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Block {}

#[derive(Debug)]
pub struct BlockInner {
    name: InternedString,
    statements: Vec<Statement>,
}

impl BlockInner {
    pub fn update_names(&mut self, name: InternedString) {
        self.name = name;

        self.statements.iter().enumerate().for_each(|(idx, stmt)| {
            stmt.update_names(format!("s_{}_{}", name.clone(), idx).into());
        });
    }
}

pub struct BlockIterator {
    visited: HashSet<Block>,
    remaining: Vec<Block>,
}

impl BlockIterator {
    fn new(start_block: Block) -> Self {
        Self {
            visited: HashSet::default(),
            remaining: vec![start_block],
        }
    }
}

impl Iterator for BlockIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            let Some(current) = self.remaining.pop() else {
                // if remaining is empty, all blocks have been visited
                return None;
            };

            // if we've already visited the node, get the next one
            if self.visited.contains(&current) {
                continue;
            } else {
                // otherwise return it
                break current;
            }
        };

        // mark current node as processed
        self.visited.insert(current.clone());

        // push children to visit
        if let Some(last) = current.statements().last() {
            self.remaining.extend(match &last.inner.get().kind {
                StatementKind::Jump { target } => vec![target.clone()],
                StatementKind::Branch {
                    true_target,
                    false_target,
                    ..
                } => vec![true_target.clone(), false_target.clone()],
                StatementKind::Return { .. }
                | StatementKind::Panic(_)
                | StatementKind::Call { tail: true, .. } => vec![],
                _ => {
                    warn!("block missing terminator");
                    vec![]
                }
            })
        }

        Some(current)
    }
}

#[derive(Clone)]
pub struct Function {
    inner: Shared<FunctionInner>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.get().name)
    }
}

impl ToTokens for Function {
    fn to_tokens(&self, _: &mut TokenStream) {
        todo!()
    }
}

#[derive(Clone)]
pub struct FunctionInner {
    name: InternedString,
    return_type: Arc<Type>,
    parameters: Vec<Symbol>,
    local_variables: HashMap<InternedString, Symbol>,
    entry_block: Block,
}

impl Function {
    pub fn new<I: Iterator<Item = (InternedString, Arc<Type>)>>(
        name: InternedString,
        return_type: Arc<Type>,
        parameters: I,
    ) -> Self {
        let mut celf = Self {
            inner: Shared::new(FunctionInner {
                name,
                return_type: return_type.clone(),
                parameters: parameters
                    .map(|(name, typ)| Symbol {
                        name,
                        kind: SymbolKind::Parameter,
                        typ,
                    })
                    .collect(),
                local_variables: HashMap::default(),
                entry_block: Block::new(),
            }),
        };

        if return_type.is_void() {
            panic!("functions must have a return type (unit not void)");
        }

        celf.add_local_variable("exception".into(), Arc::new(Type::u32()));
        celf.add_local_variable("return_value".into(), return_type);
        celf.add_local_variable("throw".into(), Arc::new(Type::u32()));

        celf
    }

    pub fn name(&self) -> InternedString {
        self.inner.get().name
    }

    pub fn weight(&self) -> u64 {
        0 //self.inner.borrow().entry_block().iter().
    }

    pub fn signature(&self) -> (Arc<Type>, Vec<Symbol>) {
        (
            self.inner.get().return_type.clone(),
            self.inner.get().parameters.clone(),
        )
    }

    pub fn update_names(&self) {
        self.inner
            .get()
            .entry_block
            .iter()
            .enumerate()
            .for_each(|(idx, b)| {
                b.update_names(format!("{idx}").into());
            });
    }

    pub fn add_local_variable(&mut self, name: InternedString, typ: Arc<Type>) {
        self.inner.get_mut().local_variables.insert(
            name,
            Symbol {
                name,
                kind: SymbolKind::LocalVariable,
                typ,
            },
        );
    }

    pub fn get_local_variable(&self, name: InternedString) -> Option<Symbol> {
        self.inner.get().local_variables.get(&name).cloned()
    }

    pub fn local_variables(&self) -> Vec<Symbol> {
        self.inner.get().local_variables.values().cloned().collect()
    }

    pub fn remove_local_variable(&self, symbol: &Symbol) {
        self.inner.get_mut().local_variables.remove(&symbol.name());
    }

    pub fn get_parameter(&self, name: InternedString) -> Option<Symbol> {
        self.inner
            .get()
            .parameters
            .iter()
            .find(|sym| sym.name() == name)
            .cloned()
    }

    pub fn return_type(&self) -> Arc<Type> {
        self.inner.get().return_type.clone()
    }

    pub fn entry_block(&self) -> Block {
        self.inner.get().entry_block.clone()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FunctionKind {
    Execute,
    Other,
}

#[derive(Default)]
pub struct Context {
    fns: HashMap<InternedString, (FunctionKind, Function)>,
    // offset-type pairs, offsets may not be unique? todo: ask tom
    registers: HashMap<InternedString, (Arc<Type>, usize)>,
    structs: HashSet<Arc<Type>>,
    unions: HashSet<Arc<Type>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_function(&mut self, name: InternedString, kind: FunctionKind, func: Function) {
        self.fns.insert(name, (kind, func));
    }

    pub fn update_names(&self) {
        for (_, func) in self.fns.values() {
            func.update_names();
        }
    }

    pub fn optimise(&mut self, level: opt::OptLevel) {
        opt::optimise(self, level);
    }

    pub fn validate(&mut self) -> Vec<validator::ValidationMessage> {
        validator::validate(self)
    }

    pub fn get_functions(&self) -> HashMap<InternedString, Function> {
        self.fns
            .iter()
            .map(|(name, (_, function))| (*name, function.clone()))
            .collect()
    }

    pub fn get_registers(&self) -> HashMap<InternedString, (Arc<Type>, usize)> {
        self.registers.clone()
    }

    pub fn get_structs(&self) -> HashSet<Arc<Type>> {
        self.structs.clone()
    }

    pub fn get_unions(&self) -> HashSet<Arc<Type>> {
        self.unions.clone()
    }
}
