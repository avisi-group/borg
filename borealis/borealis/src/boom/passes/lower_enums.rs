//! Replace enums with integers

use {
    crate::boom::{
        passes::{any::AnyExt, Pass},
        visitor::Visitor,
        Ast, Size, Type,
    },
    std::{cell::RefCell, rc::Rc},
};

/// Replace enums with integers
pub struct LowerEnums {
    did_change: bool,
}

impl LowerEnums {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self { did_change: false })
    }
}

impl Pass for LowerEnums {
    fn name(&self) -> &'static str {
        "LowerEnums"
    }

    fn reset(&mut self) {
        self.did_change = false;
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        let regs_did_change = ast
            .borrow()
            .registers
            .values()
            .map(|typ| {
                self.reset();
                self.visit_type(typ.clone());
                self.did_change
            })
            .any();

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

        regs_did_change || defs_did_change || fns_did_change
    }
}

impl Visitor for LowerEnums {
    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        let mut node = node.borrow_mut();

        if let Type::Enum { .. } = *node {
            *node = Type::Int {
                signed: false,
                size: Size::Static(32),
            }
        }
    }
}
