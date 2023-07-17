//! Resolves assignments to `return` variables.
//!
//! JIB contains assignments to a `return` variable which is returned implicity
//! by the `return` statement, this must be transformed into a return of the
//! assigned value. Must return the value last assigned to the return variable.

use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            visitor::{Visitor, Walkable},
            Ast, Expression, Statement, Type, Value,
        },
        passes::{any::AnyExt, Pass},
    },
    common::HashSet,
    std::{cell::RefCell, rc::Rc},
};

pub struct ResolveReturns {
    did_change: bool,
    visited_blocks: HashSet<ControlFlowBlock>,
    return_type: Option<Rc<RefCell<Type>>>,
}

impl ResolveReturns {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            visited_blocks: HashSet::default(),
            return_type: None,
        })
    }
}

impl Pass for ResolveReturns {
    fn name(&self) -> &'static str {
        "ResolveReturns"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.did_change = false;

                // get return type of function:
                // if void, there should be no return assigments
                //  and replace all assignments to return to that
                // then return return_value;
                self.return_type = if let Type::Unit = &*def.signature.return_type.borrow() {
                    None
                } else {
                    Some(def.signature.return_type.clone())
                };

                // if not void, create a new local variable called "return_value",
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
                            Rc::new(RefCell::new(Statement::TypeDeclaration {
                                name: "return_value".into(),
                                typ: typ.clone(),
                            })),
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
        if self.is_block_visited(block) {
            return;
        }
        self.set_block_visited(block);

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
            Terminator::Return(Some(_)) => {
                panic!("return already set");
            }
            _ => {}
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

    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool {
        self.visited_blocks.contains(block)
    }

    fn set_block_visited(&mut self, block: &ControlFlowBlock) {
        self.visited_blocks.insert(block.clone());
    }
}
