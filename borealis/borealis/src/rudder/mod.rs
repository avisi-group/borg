use {
    common::intern::InternedString,
    log::warn,
    proc_macro2::TokenStream,
    quote::ToTokens,
    std::{
        cell::RefCell,
        collections::{HashMap, HashSet, LinkedList},
        hash::{Hash, Hasher},
        rc::Rc,
    },
};

pub mod build;
mod pretty_print;

#[derive(Hash, Clone, Copy)]
pub enum PrimitiveTypeClass {
    Void,
    Unit,
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
}

#[derive(Hash, Clone)]
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

#[derive(Hash, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Composite(Vec<Rc<Type>>),
    Vector {
        element_count: usize,
        element_type: Rc<Type>,
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

    pub fn width(&self) -> usize {
        match self {
            Self::Composite(xs) => xs.iter().map(|x| x.width()).sum(),
            Self::Primitive(p) => p.element_width_in_bits,
            Type::Vector {
                element_count,
                element_type,
            } => element_type.width() * element_count,
        }
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
    CmpEq,
    CmpNe,
    CmpLt,
    CmpLe,
    CmpGt,
    CmpGe,
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
        offset: Statement,
    },
    WriteRegister {
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
    Trap,
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
    pub fn from_kind(kind: StatementKind) -> Self {
        Self {
            inner: Rc::new(RefCell::new(StatementInner {
                name: "".into(),
                kind,
            })),
        }
    }

    pub fn kind(&self) -> StatementKind {
        (*self.inner).borrow().kind.clone()
    }

    pub fn name(&self) -> InternedString {
        (*self.inner).borrow().name.clone()
    }

    pub fn update_names(&self, name: InternedString) {
        (*self.inner).borrow_mut().update_names(name);
    }

    pub fn classify(&self) -> ValueClass {
        match self.kind() {
            StatementKind::Constant { typ, value } => ValueClass::Fixed,
            StatementKind::ReadVariable { symbol } => todo!(),
            StatementKind::WriteVariable { symbol, value } => todo!(),
            StatementKind::ReadRegister { typ, offset } => todo!(),
            StatementKind::WriteRegister { offset, value } => todo!(),
            StatementKind::ReadMemory { typ, offset } => todo!(),
            StatementKind::WriteMemory { offset, value } => todo!(),
            StatementKind::BinaryOperation { kind, lhs, rhs } => todo!(),
            StatementKind::UnaryOperation { kind, value } => todo!(),
            StatementKind::ShiftOperation {
                kind,
                value,
                amount,
            } => todo!(),
            StatementKind::Call { target, args } => todo!(),
            StatementKind::Cast { kind, typ, value } => todo!(),
            StatementKind::Jump { target } => todo!(),
            StatementKind::Branch {
                condition,
                true_target,
                false_target,
            } => todo!(),
            StatementKind::PhiNode { members } => todo!(),
            StatementKind::Return { value } => todo!(),
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => todo!(),
            StatementKind::Trap => todo!(),
            StatementKind::ReadPc => ValueClass::Dynamic,
            StatementKind::WritePc { value } => todo!(),
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => todo!(),
            StatementKind::BitInsert {
                original_value,
                insert_value,
                start,
                length,
            } => todo!(),
        }
    }

    pub fn get_type(&self) -> Rc<Type> {
        match self.kind() {
            StatementKind::Constant { typ, value } => typ,
            StatementKind::ReadVariable { symbol } => symbol.typ,
            StatementKind::WriteVariable { symbol, value } => Rc::new(Type::void()),
            StatementKind::ReadRegister { typ, offset } => typ,
            StatementKind::WriteRegister { offset, value } => Rc::new(Type::void()),
            StatementKind::ReadMemory { typ, offset } => typ,
            StatementKind::WriteMemory { offset, value } => Rc::new(Type::void()),
            StatementKind::BinaryOperation { kind, lhs, rhs } => lhs.get_type(),
            StatementKind::UnaryOperation { kind, value } => value.get_type(),
            StatementKind::ShiftOperation {
                kind,
                value,
                amount,
            } => value.get_type(),
            StatementKind::Call { target, args } => target.return_type(),
            StatementKind::Cast { kind, typ, value } => typ,
            StatementKind::Jump { target } => Rc::new(Type::void()),
            StatementKind::Branch {
                condition,
                true_target,
                false_target,
            } => Rc::new(Type::void()),
            StatementKind::PhiNode { members } => members
                .first()
                .map(|(_, stmt)| stmt.get_type())
                .unwrap_or_else(|| Rc::new(Type::void())),
            StatementKind::Return { value } => Rc::new(Type::void()),
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => true_value.get_type(),
            StatementKind::Trap => Rc::new(Type::void()),
            StatementKind::ReadPc => Rc::new(Type::u64()),
            StatementKind::WritePc { value } => Rc::new(Type::void()),
            StatementKind::BitExtract {
                value,
                start,
                length,
            } => value.get_type(),
            StatementKind::BitInsert {
                original_value,
                insert_value,
                start,
                length,
            } => original_value.get_type(),
        }
    }

    pub fn has_value(&self) -> bool {
        !self.get_type().is_void()
    }
}

impl StatementInner {
    pub fn update_names(&mut self, name: InternedString) {
        self.name = name;
    }
}

#[derive(Clone)]
pub struct Block {
    inner: Rc<RefCell<BlockInner>>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(BlockInner {
                name: "???".into(),
                statements: LinkedList::new(),
            })),
        }
    }

    pub fn name(&self) -> InternedString {
        (*self.inner).borrow().name.clone()
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
        self.name = name.clone();

        let mut idx = 0;
        for stmt in &self.statements {
            stmt.update_names(format!("s_{}_{}", name.clone(), idx).into());
            idx += 1;
        }
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
                local_variables: HashMap::new(),
                entry_block: Block::new(),
            })),
        };

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
        let mut idx = 0;
        for b in self.inner.borrow().entry_block.iter() {
            b.update_names(format!("b_{}", idx).into());
            idx += 1;
        }
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

pub struct Context {
    fns: HashMap<InternedString, (FunctionKind, Function)>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            fns: HashMap::default(),
        }
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

    pub fn get_functions(&self) -> Vec<(InternedString, Function)> {
        self.fns
            .iter()
            .map(|(name, (_, function))| (*name, function.clone()))
            .collect()
    }
}
