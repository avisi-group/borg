//! Resolves assignments to `return` variables.
//!
//! JIB contains assignments to a `return` variable which is returned implicity
//! by the `return` statement, this must be transformed into a return of the
//! assigned value. Must return the value last assigned to the return variable.

use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::{any::AnyExt, Pass},
        visitor::{Visitor, Walkable},
        Ast, Expression, Statement, Type, Value,
    },
    std::{cell::RefCell, rc::Rc},
};

/// Resolves assignments to `return` variables.
pub struct ResolveReturns {
    did_change: bool,
    return_type: Option<Rc<RefCell<Type>>>,
}

impl ResolveReturns {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            return_type: None,
        })
    }
}

impl Pass for ResolveReturns {
    fn name(&self) -> &'static str {
        "ResolveReturns"
    }

    fn reset(&mut self) {
        self.did_change = false;
        self.return_type = None;
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.reset();

                // get return type of function:
                // if void, there should be no return assigments
                self.return_type = if let Type::Unit = &*def.signature.return_type.borrow() {
                    None
                } else {
                    Some(def.signature.return_type.clone())
                };

                // if not void, create a new local variable called "return_value", and replace
                // all assignments to "return" to "return_value" then return
                // "return_value";
                if let Some(typ) = &self.return_type {
                    let mut statements = def.entry_block.statements();

                    let return_value_exists = statements.iter().any(|statement| {
                        if let Statement::TypeDeclaration { name, .. } = &*statement.borrow() {
                            if name.as_ref() == "return_value" {
                                return true;
                            }
                        }

                        false
                    });

                    if !return_value_exists {
                        statements.insert(
                            0,
                            Statement::TypeDeclaration {
                                name: "return_value".into(),
                                typ: typ.clone(),
                            }
                            .into(),
                        );
                    }

                    def.entry_block.set_statements(statements);
                }

                // visit every block: if not void, replace returns with return return_value
                // visit every statement: if not void, replace assignments to return with
                // return_value, if void, no such assignments should exist
                self.visit_function_definition(def);

                self.did_change
            })
            .any()
    }
}

impl Visitor for ResolveReturns {
    // visit every block: if not void, replace returns with return return_value
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        // only modify returns if the block terminates with a return
        match block.terminator() {
            Terminator::Return(None) => {
                if self.return_type.is_some() {
                    block.set_terminator(Terminator::Return(Some(Value::Identifier(
                        "return_value".into(),
                    ))));
                    self.did_change = true;
                }
            }
            Terminator::Return(Some(Value::Identifier(ident))) => {
                match self.return_type {
                    Some(_) => {
                        if ident.as_ref() != "return_value" {
                            panic!("return type of function is not void but return value is {ident:?} not \"return_value\"");
                        }
                    }
                    None => {
                        panic!("return type of function is void but return value is {ident:?} not None");
                    }
                }
            }
            Terminator::Return(Some(_)) => {
                panic!("return already set to non-identifier, should never occur");
            }
            // do nothing
            Terminator::Conditional { .. } | Terminator::Unconditional { .. } => (),
        }

        block.walk(self);
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        match *node.borrow_mut() {
            Statement::Copy {
                ref mut expression, ..
            }
            | Statement::FunctionCall {
                expression: Some(ref mut expression),
                ..
            } => {
                let Expression::Identifier(ident) = expression else {
                    return;
                };

                if ident.as_ref() == "return" {
                    *expression = Expression::Identifier("return_value".into());
                    self.did_change = true;
                }
            }
            _ => (),
        }
    }
}
