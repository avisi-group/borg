//! Replace any access or assignment to PSTATE with a call to write/read
//! register

use {
    crate::{
        boom::{visitor::Visitor, Ast, Expression, Statement, Value},
        passes::Pass,
    },
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug, Default)]
pub struct RegisterHandler;

impl RegisterHandler {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RegisterHandler {
    fn name(&self) -> &'static str {
        "RegisterHandler"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .iter()
            .for_each(|(_, def)| self.visit_function_definition(def));

        false
    }
}

impl Visitor for RegisterHandler {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        let (field, value) = {
            // if statement is a copy into PSTATE, repalce with call to write_register
            let Statement::Copy { expression, value } = &*node.borrow() else {
                return;
            };

            let Expression::Field { expression, field } = expression else {
                return;
            };

            let Expression::Identifier(ident) = **expression else {
                return;
            };

            if ident.as_ref() != "PSTATE" {
                return;
            }

            (*field, value.clone())
        };

        *node.borrow_mut() = Statement::FunctionCall {
            expression: None,
            name: "write_register".into(),
            arguments: vec![
                Rc::new(RefCell::new(Value::Identifier(
                    ("reg_".to_owned() + field.as_ref()).into(),
                ))),
                value,
            ],
        }
    }
}
