//! Replace booleans in statements with integers
//!
//! GenC doesn't have bools, so we need to replace all bools with uint8

use {
    crate::{
        boom::{visitor::Visitor, Ast, Literal, Size, Type},
        passes::{any::AnyExt, Pass},
    },
    std::{cell::RefCell, rc::Rc},
};

pub struct ReplaceBools {
    did_change: bool,
}

impl ReplaceBools {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self { did_change: false })
    }
}

impl Pass for ReplaceBools {
    fn name(&self) -> &'static str {
        "ReplaceBools"
    }

    fn reset(&mut self) {
        self.did_change = false;
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        let defs_did_change = ast
            .borrow()
            .definitions
            .iter()
            .map(|def| {
                self.reset();
                self.visit_definition(def);
                self.did_change
            })
            .any();

        let fns_did_change = ast
            .borrow()
            .functions
            .values()
            .map(|def| {
                self.reset();
                self.visit_function_definition(def);
                self.did_change
            })
            .any();

        defs_did_change || fns_did_change
    }
}

impl Visitor for ReplaceBools {
    fn visit_literal(&mut self, node: Rc<RefCell<Literal>>) {
        let mut node = node.borrow_mut();

        if let Literal::Bool(bool) = *node {
            *node = match bool {
                false => Literal::Int(num_bigint::BigInt::from(0)),
                true => Literal::Int(num_bigint::BigInt::from(1)),
            };
            self.did_change = true;
        }
    }

    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        let mut node = node.borrow_mut();

        match *node {
            Type::Bool | Type::Bit => {
                *node = Type::Int {
                    signed: false,
                    size: Size::Static(8),
                };
                self.did_change = true;
            }
            _ => {}
        }
    }
}
