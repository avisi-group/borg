use core::borrow::Borrow;

use alloc::collections::LinkedList;

use self::encoder::{Instruction, Operand, PhysicalRegister, Register};

use super::emitter::{Action, Block, LoweringContext, Value};

mod encoder;

pub struct X86LoweringContext {
    instructions: LinkedList<Instruction>,
}

impl X86LoweringContext {
    pub fn new() -> Self {
        Self {
            instructions: LinkedList::new(),
        }
    }

    fn allocate(&mut self) {
        //
    }

    fn lower_action(&mut self, block: &Block, action: &Action) {
        match action {
            Action::WriteRegister { index, value } => {
                let src = self.value_to_operand(value);
                //let dst = match (*index).borrow().kind() {};

                self.instructions.push_back(Instruction::mov(
                    src,
                    Operand::mem_base_displ(
                        32,
                        Register::PhysicalRegister(PhysicalRegister::RBP),
                        0,
                    ),
                ));
            }
            Action::Jump { target } => todo!(),
            Action::Branch {
                condition,
                true_target,
                false_target,
            } => todo!(),
            Action::Leave => todo!(),
        }
    }

    fn value_to_operand(&mut self, value: &Value) -> Operand {
        todo!()
    }
}

impl LoweringContext for X86LoweringContext {
    fn lower_block(&mut self, block: super::emitter::Block) {
        for action in (*block).borrow().actions() {
            self.lower_action(block.borrow(), action);
        }
    }

    fn complete(mut self) -> super::Translation {
        self.allocate();

        todo!()
    }
}
