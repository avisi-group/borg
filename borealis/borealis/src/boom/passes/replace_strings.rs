//! Replace strings with integers

use {
    crate::boom::{
        passes::{any::AnyExt, Pass},
        visitor::Visitor,
        Ast, Literal, Size, Type,
    },
    std::{cell::RefCell, rc::Rc},
};

/// Replace strings with integers
pub struct ReplaceStrings {
    did_change: bool,
}

impl ReplaceStrings {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self { did_change: false })
    }
}

impl Pass for ReplaceStrings {
    fn name(&self) -> &'static str {
        "ReplaceStrings"
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

impl Visitor for ReplaceStrings {
    fn visit_literal(&mut self, node: Rc<RefCell<Literal>>) {
        let mut node = node.borrow_mut();

        if let Literal::String(s) = *node {
            *node = Literal::Int(num_bigint::BigInt::from(s.key()));
            self.did_change = true;
        }
    }

    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        let mut node = node.borrow_mut();

        if let Type::String = *node {
            *node = Type::Int {
                signed: false,
                size: Size::Static(32),
            };
            self.did_change = true;
        }
    }
}
