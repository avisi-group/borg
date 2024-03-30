use {
    common::{intern::InternedString, HashMap, HashSet},
    log::warn,
    proc_macro2::TokenStream,
    quote::ToTokens,
    std::{
        cell::RefCell,
        collections::LinkedList,
        hash::{Hash, Hasher},
        rc::Rc,
    },
};

pub mod build;
mod pretty_print;

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
pub enum PrimitiveTypeClass {
    Void,
    Unit,
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
}

#[derive(Hash, Clone, Eq, PartialEq)]
pub struct PrimitiveType {
    tc: PrimitiveTypeClass,
    element_width_in_bits: usize,
}

impl PrimitiveType {
    pub fn type_class(&self) -> PrimitiveTypeClass {
        self.tc
    }

    pub fn width(&self) -> usize {
        self.element_width_in_bits
    }
}

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Composite(Vec<Rc<Type>>),
    Vector {
        element_count: usize,
        element_type: Rc<Type>,
    },
    Bundled {
        value: Rc<Type>,
        len: Rc<Type>,
    },
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

    pub fn new_composite(fields: Vec<Rc<Type>>) -> Self {
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

    pub fn width_bits(&self) -> usize {
        match self {
            Self::Composite(xs) => xs.iter().map(|x| x.width_bits()).sum(),
            Self::Primitive(p) => p.element_width_in_bits,
            Self::Vector {
                element_count,
                element_type,
            } => element_type.width_bits() * element_count,
            Self::Bundled { value, .. } => value.width_bits(),
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
    type_def_helper!(s8, SignedInteger, 8);
    type_def_helper!(s16, SignedInteger, 16);
    type_def_helper!(s32, SignedInteger, 32);
    type_def_helper!(s64, SignedInteger, 64);
    type_def_helper!(f32, FloatingPoint, 32);
    type_def_helper!(f64, FloatingPoint, 64);

    pub fn vectorize(self, element_count: usize) -> Self {
        Self::Vector {
            element_count,
            element_type: Rc::new(self),
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

    /*pub fn is_integer(&self) -> bool {
        matches!(
            self.tc,
            TypeClass::UnsignedInteger | TypeClass::SignedInteger
        )
    }

    pub fn is_signed(&self) -> Option<bool> {
        match self.tc {
            TypeClass::SignedInteger => Some(true),
            TypeClass::UnsignedInteger => Some(false),
            _ => None,
        }
    }

    pub fn is_unsigned(&self) -> Option<bool> {
        match self.tc {
            TypeClass::SignedInteger => Some(false),
            TypeClass::UnsignedInteger => Some(true),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self.tc, TypeClass::FloatingPoint)
    }

    pub fn is_vector(&self) -> bool {
        self.element_count > 1
    }*/
}

#[derive(Clone)]
pub enum ConstantValue {
    UnsignedInteger(usize),
    SignedInteger(isize),
    FloatingPoint(f64),
    Unit,
}

#[derive(Clone, Copy)]
pub enum SymbolKind {
    Parameter,
    LocalVariable,
}

#[derive(Clone)]
pub struct Symbol {
    name: InternedString,
    kind: SymbolKind,
    typ: Rc<Type>,
}

impl Symbol {
    pub fn name(&self) -> InternedString {
        self.name
    }

    pub fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub fn typ(&self) -> Rc<Type> {
        self.typ.clone()
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum UnaryOperationKind {
    Not,
    Negate,
    Complement,
}

#[derive(Clone)]
pub enum CastOperationKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    Reinterpret,
    Convert,
    Broadcast,
}

#[derive(Clone)]
pub enum ShiftOperationKind {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    RotateRight,
    RotateLeft,
}

#[derive(Clone)]
pub enum StatementKind {
    Constant {
        typ: Rc<Type>,
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
        typ: Rc<Type>,
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
        typ: Rc<Type>,
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
    },
    Cast {
        kind: CastOperationKind,
        typ: Rc<Type>,
        value: Statement,
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
    ReadField {
        composite: Statement,
        field: usize,
    },
    /// Returns the composite with the mutated field
    MutateField {
        composite: Statement,
        field: usize,
        value: Statement,
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

    /// Fatal error, printing values of supplied statements for debugging purposes
    Panic(Vec<Statement>),

    Assert {
        condition: Statement,
    },

    CreateComposite {
        typ: Rc<Type>,
        /// Index of fields should match type
        fields: Vec<Statement>,
    },

    Bundle {
        value: Statement,
        length: Statement,
    },

    UnbundleValue {
        bundle: Statement,
    },

    UnbundleLength {
        bundle: Statement,
    },
}

pub enum ValueClass {
    Fixed,
    Dynamic,
}

#[derive(Clone)]
pub struct Statement {
    inner: Rc<RefCell<StatementInner>>,
}

pub struct StatementInner {
    name: InternedString,
    kind: StatementKind,
}

impl Statement {
    pub fn kind(&self) -> StatementKind {
        (*self.inner).borrow().kind.clone()
    }

    pub fn name(&self) -> InternedString {
        (*self.inner).borrow().name
    }

    pub fn update_names(&self, name: InternedString) {
        (*self.inner).borrow_mut().update_names(name);
    }

    pub fn classify(&self) -> ValueClass {
        match self.kind() {
            StatementKind::Constant { .. } => ValueClass::Fixed,
            StatementKind::ReadRegister { .. } => todo!(),
            StatementKind::WriteRegister { .. } => todo!(),
            StatementKind::ReadMemory { .. } => todo!(),
            StatementKind::WriteMemory { .. } => todo!(),
            StatementKind::BinaryOperation { .. } => todo!(),
            StatementKind::UnaryOperation { .. } => todo!(),
            StatementKind::ShiftOperation { .. } => todo!(),
            StatementKind::Call { .. } => todo!(),
            StatementKind::Cast { .. } => todo!(),
            StatementKind::Jump { .. } => todo!(),
            StatementKind::Branch { .. } => todo!(),
            StatementKind::PhiNode { .. } => todo!(),
            StatementKind::Return { .. } => todo!(),
            StatementKind::Select { .. } => todo!(),
            StatementKind::Panic(_) => todo!(),
            StatementKind::ReadPc => ValueClass::Dynamic,
            StatementKind::WritePc { .. } => todo!(),
            StatementKind::BitExtract { .. } => todo!(),
            StatementKind::BitInsert { .. } => todo!(),
            StatementKind::ReadVariable { .. } => todo!(),
            StatementKind::WriteVariable { .. } => todo!(),
            StatementKind::ReadField { .. } => todo!(),
            StatementKind::MutateField { .. } => todo!(),
            StatementKind::ReadElement { .. } => todo!(),
            StatementKind::MutateElement { .. } => todo!(),
            StatementKind::CreateComposite { .. } => todo!(),
            StatementKind::Bundle { .. } => todo!(),
            StatementKind::UnbundleValue { .. } => todo!(),
            StatementKind::UnbundleLength { .. } => todo!(),
            StatementKind::Assert { .. } => todo!(),
        }
    }

    pub fn typ(&self) -> Rc<Type> {
        match self.kind() {
            StatementKind::Constant { typ, .. } => typ,
            StatementKind::ReadVariable { symbol } => symbol.typ(),
            StatementKind::WriteVariable { .. } => Rc::new(Type::void()),
            StatementKind::ReadRegister { typ, .. } => typ,
            StatementKind::WriteRegister { .. } => Rc::new(Type::unit()),
            StatementKind::ReadMemory { typ, .. } => typ,
            StatementKind::WriteMemory { .. } => Rc::new(Type::void()),
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
            } => Rc::new(Type::u1()),
            StatementKind::BinaryOperation { lhs, .. } => lhs.typ(),
            StatementKind::UnaryOperation { value, .. } => value.typ(),
            StatementKind::ShiftOperation { value, .. } => value.typ(),
            StatementKind::Call { target, .. } => target.return_type(),
            StatementKind::Cast { typ, .. } => typ,
            StatementKind::Jump { .. } => Rc::new(Type::void()),
            StatementKind::Branch { .. } => Rc::new(Type::void()),
            StatementKind::PhiNode { members } => members
                .first()
                .map(|(_, stmt)| stmt.typ())
                .unwrap_or_else(|| Rc::new(Type::void())),
            StatementKind::Return { .. } => Rc::new(Type::void()),
            StatementKind::Select { true_value, .. } => true_value.typ(),
            StatementKind::Panic(_) => Rc::new(Type::void()),
            StatementKind::ReadPc => Rc::new(Type::u64()),
            StatementKind::WritePc { .. } => Rc::new(Type::void()),
            StatementKind::BitExtract { value, .. } => value.typ(),
            StatementKind::BitInsert { original_value, .. } => original_value.typ(),
            StatementKind::ReadField { composite, field } => {
                let Type::Composite(field_types) = &*composite.typ() else {
                    panic!("cannot read field of non-composite type")
                };

                field_types[field].clone()
            }
            StatementKind::MutateField { composite, .. } => {
                // get type of composite and return it
                composite.typ()
            }
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
            StatementKind::Bundle { value, length } => Rc::new(Type::Bundled {
                value: value.typ(),
                len: length.typ(),
            }),
            StatementKind::UnbundleValue { bundle } => {
                let Type::Bundled { value, .. } = &*bundle.typ() else {
                    panic!("cannot unbundle non-bundle");
                };
                value.clone()
            }
            StatementKind::UnbundleLength { bundle } => {
                let Type::Bundled { len, .. } = &*bundle.typ() else {
                    panic!("cannot unbundle non-bundle");
                };
                len.clone()
            }
            StatementKind::Assert { .. } => Rc::new(Type::unit()),
        }
    }

    pub fn has_value(&self) -> bool {
        !self.typ().is_void()
    }
}

impl StatementInner {
    pub fn update_names(&mut self, name: InternedString) {
        self.name = name;
    }
}

struct StatementBuilder {
    statements: Vec<Statement>,
}

impl StatementBuilder {
    /// Creates a new `StatementBuilder`
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    /// Builds a new `Statement` from a `StatementKind`, adds it to the builder,
    /// and returns it
    pub fn build(&mut self, kind: StatementKind) -> Statement {
        let statement = Statement {
            inner: Rc::new(RefCell::new(StatementInner {
                name: "".into(),
                kind,
            })),
        };

        self.statements.push(statement.clone());

        statement
    }

    /// Consumes a `StatementBuilder` and returns it's statements
    pub fn finish(self) -> Vec<Statement> {
        self.statements
    }
}

#[derive(Clone)]
pub struct Block {
    inner: Rc<RefCell<BlockInner>>,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> InternedString {
        (*self.inner).borrow().name
    }

    pub fn update_names(&self, name: InternedString) {
        (*self.inner).borrow_mut().update_names(name);
    }

    pub fn statements(&self) -> LinkedList<Statement> {
        self.inner.borrow().statements.clone()
    }

    pub fn set_statements<I: Iterator<Item = Statement>>(&self, statements: I) {
        self.inner.borrow_mut().statements = statements.collect();
    }

    pub fn extend_statements<I: Iterator<Item = Statement>>(&self, stmts: I) {
        self.inner.borrow_mut().statements.extend(stmts)
    }

    pub fn iter(&self) -> BlockIterator {
        BlockIterator::new(self.clone())
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(BlockInner {
                name: "???".into(),
                statements: LinkedList::new(),
            })),
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
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Block {}

pub struct BlockInner {
    name: InternedString,
    statements: LinkedList<Statement>,
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
        if let Some(last) = current.statements().back() {
            self.remaining.extend(match &last.inner.borrow().kind {
                StatementKind::Jump { target } => vec![target.clone()],
                StatementKind::Branch {
                    true_target,
                    false_target,
                    ..
                } => vec![true_target.clone(), false_target.clone()],
                StatementKind::Return { .. } => vec![],
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
    inner: Rc<RefCell<FunctionInner>>,
}

impl ToTokens for Function {
    fn to_tokens(&self, _: &mut TokenStream) {
        todo!()
    }
}

#[derive(Clone)]
pub struct FunctionInner {
    name: InternedString,
    return_type: Rc<Type>,
    parameters: Vec<Symbol>,
    local_variables: HashMap<InternedString, Symbol>,
    entry_block: Block,
}

impl Function {
    pub fn new<I: Iterator<Item = (InternedString, Rc<Type>)>>(
        name: InternedString,
        return_type: Rc<Type>,
        parameters: I,
    ) -> Self {
        let mut celf = Self {
            inner: Rc::new(RefCell::new(FunctionInner {
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
            })),
        };

        if return_type.is_void() {
            panic!("functions must have a return type (unit not void)");
        }

        celf.add_local_variable("exception".into(), Rc::new(Type::u32()));
        celf.add_local_variable("return_value".into(), return_type);
        celf.add_local_variable("throw".into(), Rc::new(Type::u32()));

        celf
    }

    pub fn name(&self) -> InternedString {
        self.inner.borrow().name
    }

    pub fn signature(&self) -> (Rc<Type>, Vec<Symbol>) {
        (
            self.inner.borrow().return_type.clone(),
            self.inner.borrow().parameters.clone(),
        )
    }

    pub fn update_names(&self) {
        self.inner
            .borrow()
            .entry_block
            .iter()
            .enumerate()
            .for_each(|(idx, b)| {
                b.update_names(format!("{idx}").into());
            });
    }

    pub fn add_local_variable(&mut self, name: InternedString, typ: Rc<Type>) {
        self.inner.borrow_mut().local_variables.insert(
            name,
            Symbol {
                name,
                kind: SymbolKind::LocalVariable,
                typ,
            },
        );
    }

    pub fn get_local_variable(&self, name: InternedString) -> Option<Symbol> {
        self.inner.borrow().local_variables.get(&name).cloned()
    }

    pub fn local_variables(&self) -> Vec<Symbol> {
        self.inner
            .borrow()
            .local_variables
            .values()
            .cloned()
            .collect()
    }

    pub fn get_parameter(&self, name: InternedString) -> Option<Symbol> {
        self.inner
            .borrow()
            .parameters
            .iter()
            .find(|sym| sym.name() == name)
            .cloned()
    }

    pub fn return_type(&self) -> Rc<Type> {
        self.inner.borrow().return_type.clone()
    }

    pub fn entry_block(&self) -> Block {
        self.inner.borrow().entry_block.clone()
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
    registers: Vec<(Rc<Type>, usize)>,
    structs: HashSet<Rc<Type>>,
    unions: HashSet<Rc<Type>>,
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

    pub fn optimise(&mut self) {
        todo!()
    }

    pub fn get_functions(&self) -> HashMap<InternedString, Function> {
        self.fns
            .iter()
            .map(|(name, (_, function))| (*name, function.clone()))
            .collect()
    }

    pub fn get_registers(&self) -> Vec<(Rc<Type>, usize)> {
        self.registers.clone()
    }

    pub fn get_structs(&self) -> HashSet<Rc<Type>> {
        self.structs.clone()
    }

    pub fn get_unions(&self) -> HashSet<Rc<Type>> {
        self.unions.clone()
    }
}
