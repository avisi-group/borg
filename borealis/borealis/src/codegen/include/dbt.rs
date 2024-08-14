extern crate alloc;

use alloc::boxed::Box;

pub struct DynamicTranslator;

impl DynamicTranslator {
    pub fn build(&self, node_inner: NodeInner) -> Node {
        match node_inner {
            NodeInner::WriteRegister { .. } => {
                // do codegen
                todo!()
            }
            _ => {
                // todo: validate types here
                Node(Box::new(node_inner))
            }
        }
    }
}

pub struct Node(Box<NodeInner>);

pub enum NodeInner {
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    ShiftOperation(ShiftOperation),

    Constant { value: ConstantValue, typ: Type },

    ReadRegister { offset: Node, typ: Type },
    WriteRegister { offset: Node, value: Node },
}

pub struct Type {
    pub kind: TypeKind,
    pub width: u16,
}

pub enum TypeKind {
    Unsigned,
    Signed,
    Floating,
}

pub enum UnaryOperation {
    Not(Node),
    Negate(Node),
    Complement(Node),
    Power2(Node),
    Absolute(Node),
    Ceil(Node),
    Floor(Node),
    SquareRoot(Node),
}

pub enum BinaryOperation {
    Add(Node, Node),
    Sub(Node, Node),
    Multiply(Node, Node),
    Divide(Node, Node),
    Modulo(Node, Node),
    And(Node, Node),
    Or(Node, Node),
    Xor(Node, Node),
    PowI(Node, Node),
    CompareEqual(Node, Node),
    CompareNotEqual(Node, Node),
    CompareLessThan(Node, Node),
    CompareLessThanOrEqual(Node, Node),
    CompareGreaterThan(Node, Node),
    CompareGreaterThanOrEqual(Node, Node),
}

pub enum ShiftOperation {
    LogicalShiftLeft(Node, Node),
    LogicalShiftRight(Node, Node),
    ArithmeticShiftRight(Node, Node),
    RotateRight(Node, Node),
    RotateLeft(Node, Node),
}

pub struct Constant {
    pub value: ConstantValue,
    pub typ: Type,
}

pub enum ConstantValue {
    Unsigned(u128),
    Signed(i128),
    Floating(f64),
}
