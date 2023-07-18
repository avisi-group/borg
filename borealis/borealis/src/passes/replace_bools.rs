//! Replace booleans in statements with integers
//!
//! GenC doesn't have bools, so we need to replace all bools with uint8

use {
    crate::{
        boom::{visitor::Visitor, Ast, Literal, Type},
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

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.did_change = false;
                self.visit_function_definition(def);
                self.did_change
            })
            .any()
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

        if let Type::Bool = *node {
            *node = Type::Fbits(8, false);
            self.did_change = true;
        }
    }
}
