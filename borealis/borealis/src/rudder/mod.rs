#![allow(unused)]

use {
    crate::boom::{self, control_flow::ControlFlowBlock, FunctionDefinition},
    std::{cell::RefCell, collections::LinkedList, rc::Rc},
};

pub enum ConstantValue {
    UnsignedInteger(usize),
    SignedInteger(isize),
    FloatingPoint(f64),
}

pub enum BinaryOperationKind {
    Add,
    Sub,
    Multiply,
    Divide,
    Modulo,
}

pub enum StatementKind {
    Constant(ConstantValue),
    BinaryOperation {
        kind: BinaryOperationKind,
        lhs: Value,
        rhs: Value,
    },
}

pub enum ValueKind {
    Statement(Statement),
}

pub struct Value {
    kind: ValueKind,
}

pub enum ValueClassification {
    Fixed,
    Dynamic,
}

impl Value {
    pub fn classify(&self) -> ValueClassification {
        match &self.kind {
            ValueKind::Statement(stmt) => {
                todo!()
            }
        }
    }
}

pub struct Statement {
    inner: Rc<RefCell<StatementInner>>,
}

pub struct StatementInner {
    kind: StatementKind,
}

impl Statement {
    pub fn kind(&self) -> StatementKind {
        todo!()
    }

    fn from_boom(boom_stmt: Rc<RefCell<crate::boom::Statement>>) -> Self {
        match &*((*boom_stmt).borrow()) {
            boom::Statement::TypeDeclaration { name, typ } => todo!(),
            boom::Statement::Copy { expression, value } => todo!(),
            boom::Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => todo!(),
            boom::Statement::Label(_) => todo!(),
            boom::Statement::Goto(_) => todo!(),
            boom::Statement::Jump { condition, target } => todo!(),
            boom::Statement::End(_) => todo!(),
            boom::Statement::Undefined => todo!(),
            boom::Statement::If {
                condition,
                if_body,
                else_body,
            } => todo!(),
            boom::Statement::Exit(_) => todo!(),
            boom::Statement::Comment(_) => todo!(),
        }

        todo!()
    }
}

pub struct Block {
    inner: Rc<RefCell<BlockInner>>,
}

impl Block {
    fn from_boom(boom_block: &ControlFlowBlock) -> Self {
        let mut statements = LinkedList::new();
        for stmt in boom_block.statements() {
            statements.push_back(Statement::from_boom(stmt));
        }

        Self {
            inner: Rc::new(RefCell::new(BlockInner { statements })),
        }
    }
}

pub struct BlockInner {
    statements: LinkedList<Statement>,
}

pub struct Function {
    inner: FunctionInner,
}

pub struct FunctionInner {
    entry_block: Option<Block>,
}

impl Function {
    pub fn from_boom(boom_function: &FunctionDefinition) -> Self {
        let entry = Block::from_boom(&boom_function.entry_block);

        todo!()
    }
}
