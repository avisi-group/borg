use {
    crate::guest::dbt::Translation,
    alloc::{
        boxed::Box,
        collections::{BTreeSet, LinkedList},
        rc::{Rc, Weak},
    },
    core::cell::RefCell,
};

pub struct Context {
    next_id: u64,
    blocks: LinkedList<Block>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            blocks: LinkedList::new(),
        }
    }

    pub fn create_block(&mut self) -> Block {
        let block = Rc::new(RefCell::new(BlockData::new(self.next_id)));
        self.next_id += 1;

        self.blocks.push_back(block.clone());

        block
    }

    pub fn lower(self, mut lowering_ctx: Box<dyn LoweringContext>) -> Translation {
        let mut work_list = LinkedList::new();
        let mut seen_list = BTreeSet::new();

        work_list.push_back(self.blocks.front().unwrap().clone());

        while !work_list.is_empty() {
            let current = work_list.pop_front().unwrap();
            seen_list.insert(current.borrow().id);

            lowering_ctx.lower_block(current);
        }

        lowering_ctx.complete()
    }
}

pub struct BlockData {
    id: u64,
    actions: LinkedList<Action>,
}

pub type Block = Rc<RefCell<BlockData>>;
pub type WeakBlock = Weak<RefCell<BlockData>>;

#[derive(Clone, Copy)]
pub enum TypeClass {
    Void,
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
}

#[derive(Clone, Copy)]
pub struct Type {
    cls: TypeClass,
    width_in_bits: u8,
}

macro_rules! def_type_helper {
    ($name: ident, $cls: ident, $width: expr) => {
        pub fn $name() -> Type {
            Type {
                cls: TypeClass::$cls,
                width_in_bits: $width,
            }
        }
    };
}

impl Type {
    pub fn void() -> Self {
        Self {
            cls: TypeClass::Void,
            width_in_bits: 0,
        }
    }

    def_type_helper!(u8, UnsignedInteger, 8);
    def_type_helper!(u16, UnsignedInteger, 16);
    def_type_helper!(u32, UnsignedInteger, 32);
    def_type_helper!(u64, UnsignedInteger, 64);
    def_type_helper!(s8, SignedInteger, 8);
    def_type_helper!(s16, SignedInteger, 16);
    def_type_helper!(s32, SignedInteger, 32);
    def_type_helper!(s64, SignedInteger, 64);
    def_type_helper!(f32, FloatingPoint, 32);
    def_type_helper!(f64, FloatingPoint, 64);
}

pub enum UnaryOperationKind {
    BitNot,
    Negative,
}

pub enum BinaryOperationKind {
    Add,
    Adc,
    Sub,
    Sbc,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    CmpEq,
    CmpNe,
    CmpLt,
    CmpLe,
    CmpGt,
    CmpGe,
}

pub enum TernaryOperationKind {
    Select,
}

pub enum ShiftOpKind {
    ShiftLeft,
    ShiftRight,
    ArithmeticShiftRight,
    RotateLeft,
    RotateRight,
}

pub enum CastOpKind {
    Truncate,
    ZeroExtend,
    SignExtend,
    Reinterpret,
    Convert,
}

pub enum ConstantKind {
    Unsigned(u64),
    Signed(i64),
    Floating(f64),
}

pub enum ValueKind {
    Constant(ConstantKind),
    ReadRegister(Value),
    UnaryOperation {
        kind: UnaryOperationKind,
        value: Value,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Value,
        rhs: Value,
    },
    TernaryOperation {
        kind: TernaryOperationKind,
        o1: Value,
        o2: Value,
        o3: Value,
    },
    ShiftOperation {
        kind: ShiftOpKind,
        value: Value,
        amount: Value,
    },
    CastOpKind {
        kind: CastOpKind,
        value: Value,
    },
}

pub struct ValueData {
    typ: Type,
    kind: ValueKind,
}

pub enum Action {
    WriteRegister {
        index: Value,
        value: Value,
    },
    Jump {
        target: WeakBlock,
    },
    Branch {
        condition: Value,
        true_target: WeakBlock,
        false_target: WeakBlock,
    },
    Leave,
}

pub type Value = Rc<RefCell<ValueData>>;

impl ValueData {
    fn new_value(typ: Type, kind: ValueKind) -> Value {
        Rc::new(RefCell::new(ValueData { typ, kind }))
    }

    pub fn typ(&self) -> &Type {
        &self.typ
    }

    pub fn kind(&self) -> &ValueKind {
        &self.kind
    }
}

macro_rules! binop {
    ($opname: ident, $kindname:ident) => {
        pub fn $opname(&self, lhs: Value, rhs: Value) -> Value {
            let typ = lhs.borrow().typ;
            ValueData::new_value(
                typ,
                ValueKind::BinaryOperation {
                    kind: BinaryOperationKind::$kindname,
                    lhs,
                    rhs,
                },
            )
        }
    };
}

impl BlockData {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            actions: LinkedList::new(),
        }
    }

    pub fn actions(&self) -> &LinkedList<Action> {
        &self.actions
    }
}

pub struct Builder(Block);

impl Builder {
    pub fn new(block: Block) -> Self {
        Self(block)
    }

    pub fn set_insert_point(&mut self, block: Block) {
        self.0 = block;
    }

    // --- Constants --- //

    pub fn const_u(&self, typ: Type, value: u64) -> Value {
        ValueData::new_value(typ, ValueKind::Constant(ConstantKind::Unsigned(value)))
    }

    pub fn const_s(&self, typ: Type, value: i64) -> Value {
        ValueData::new_value(typ, ValueKind::Constant(ConstantKind::Signed(value)))
    }

    pub fn const_f(&self, typ: Type, value: f64) -> Value {
        ValueData::new_value(typ, ValueKind::Constant(ConstantKind::Floating(value)))
    }

    pub fn const_u8(&self, value: u8) -> Value {
        self.const_u(Type::u8(), value as u64)
    }

    pub fn const_u16(&self, value: u16) -> Value {
        self.const_u(Type::u16(), value as u64)
    }

    pub fn const_u32(&self, value: u32) -> Value {
        self.const_u(Type::u32(), value as u64)
    }

    pub fn const_u64(&self, value: u64) -> Value {
        self.const_u(Type::u64(), value)
    }

    // --- Arithmetic --- //

    binop!(add, Add);
    binop!(sub, Sub);
    binop!(mul, Mul);
    binop!(div, Div);
    binop!(modulo, Mod);

    // --- Registers --- //

    pub fn read_register(&mut self, index: Value, typ: Type) -> Value {
        ValueData::new_value(typ, ValueKind::ReadRegister(index))
    }

    pub fn write_register(&mut self, index: Value, value: Value) {
        self.0
            .borrow_mut()
            .actions
            .push_back(Action::WriteRegister { index, value })
    }

    // --- Control Flow --- //

    pub fn jump(&mut self, target: WeakBlock) {
        self.0
            .borrow_mut()
            .actions
            .push_back(Action::Jump { target });
    }

    pub fn branch(&mut self, condition: Value, true_target: WeakBlock, false_target: WeakBlock) {
        self.0.borrow_mut().actions.push_back(Action::Branch {
            condition,
            true_target,
            false_target,
        });
    }

    pub fn leave(&mut self) {
        self.0.borrow_mut().actions.push_back(Action::Leave);
    }
}

pub trait LoweringContext {
    fn lower_block(&mut self, block: Block);

    fn complete(&mut self) -> Translation;
}
