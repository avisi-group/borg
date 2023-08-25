//! Remove all exception handling logic
//!
//! Remove:
//!
//! * type declarations with the type of union exception (or just union as there
//!   is only one union)
//! * assignments to any of the previous type declarations
//! * any assignments to exception that arent boolean(?)
//! * assignments to `throw`
//! * assignments where the value is a union
//! * whole of the if exception branch, replace with a `trap()` function call

use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            visitor::{Visitor, Walkable},
            Ast, Expression, Literal, Statement, Type, Value,
        },
        passes::{any::AnyExt, Pass},
    },
    common::{intern::InternedString, HashSet},
    std::{cell::RefCell, rc::Rc},
};

pub struct RemoveExceptions {
    did_change: bool,
}

impl RemoveExceptions {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self { did_change: false })
    }
}

impl Pass for RemoveExceptions {
    fn name(&self) -> &'static str {
        "RemoveExceptions"
    }

    fn reset(&mut self) {
        self.did_change = false;
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                {
                    let mut statements = def.entry_block.statements();
                    statements.insert(
                        0,
                        Statement::TypeDeclaration {
                            name: "exception".into(),
                            typ: Rc::new(RefCell::new(Type::FixedInt(8))),
                        }
                        .into(),
                    );
                    statements.insert(
                        1,
                        Statement::Copy {
                            expression: Expression::Identifier("exception".into()),
                            value: Literal::Int(0.into()).into(),
                        }
                        .into(),
                    );
                    def.entry_block.set_statements(statements);
                }

                self.visit_function_definition(def);
                self.did_change
            })
            .any()
    }
}

impl Visitor for RemoveExceptions {
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        let mut deleted_exception_vars = HashSet::default();

        let statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| statement_filter(&mut deleted_exception_vars, statement))
            .collect();

        block.set_statements(statements);

        if let Terminator::Conditional {
            condition: Value::Identifier(ident),
            target,
            ..
        } = block.terminator()
        {
            if ident.as_ref() == "exception" {
                target.set_statements(vec![Statement::FunctionCall {
                    expression: None,
                    name: "trap".into(),
                    arguments: vec![],
                }
                .into()]);
                target.set_terminator(Terminator::Return(None));
            }
        }

        block.walk(self);
    }
}

fn statement_filter(
    deleted_exception_vars: &mut HashSet<InternedString>,
    statement: Rc<RefCell<Statement>>,
) -> Option<Rc<RefCell<Statement>>> {
    let statement_cloned = statement.clone();

    match &mut *statement.borrow_mut() {
        Statement::TypeDeclaration {
            typ,
            name: local_var_name,
        } => {
            if let Type::Union { name, .. } = *typ.borrow() {
                if name.as_ref() == "exception" {
                    deleted_exception_vars.insert(*local_var_name);

                    None
                } else {
                    panic!("found non-exception union type decl")
                }
            } else {
                Some(statement_cloned)
            }
        }

        Statement::FunctionCall {
            expression: Some(expression),
            ..
        } => {
            let Expression::Identifier(ident) = expression else {
                return Some(statement_cloned);
            };

            if deleted_exception_vars.contains(ident) {
                None
            } else {
                Some(statement_cloned)
            }
        }

        Statement::Copy { expression, value } => {
            let Expression::Identifier(ident) = expression else {
                return Some(statement_cloned);
            };

            if deleted_exception_vars.contains(ident) {
                return None;
            }

            match ident.as_ref() {
                "exception" => {
                    if let Value::Identifier(ident) = &*value.borrow() {
                        if deleted_exception_vars.contains(ident) {
                            None
                        } else {
                            Some(statement_cloned)
                        }
                    } else {
                        Some(statement_cloned)
                    }
                }
                "throw" => None,
                "unconditional" => None,
                _ => Some(statement_cloned),
            }
        }

        _ => Some(statement_cloned),
    }
}
