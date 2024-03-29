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
//! * whole of the if exception branch, replace with a panic statement
//!
//! NOTE 2024-03-24 all non-rudder passes were deleted, this one was kept in case `make_exception_bool` turns out to be insufficient

use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        visitor::{Visitor, Walkable},
        Ast, Expression, FunctionDefinition, Literal, Size, Statement, Type, Value,
    },
    common::{intern::InternedString, HashSet},
    std::{cell::RefCell, rc::Rc},
};

/// Remove all exception handling logic
pub struct RemoveExceptions;

impl RemoveExceptions {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for RemoveExceptions {
    fn name(&self) -> &'static str {
        "RemoveExceptions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        // first pass to remove exceptions
        ast.borrow().functions.values().for_each(|def| {
            {
                let mut statements = def.entry_block.statements();

                if let Some(stmt) = statements.first() {
                    if let Statement::TypeDeclaration { name, .. } = &*stmt.borrow() {
                        if name.as_ref() == "exception" {
                            return;
                        }
                    }
                }

                statements.insert(
                    0,
                    Statement::TypeDeclaration {
                        name: "exception".into(),
                        typ: Rc::new(RefCell::new(Type::Int {
                            signed: false,
                            size: Size::Static(1),
                        })),
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
        });

        // second pass identifying exception blocks and raising back into if statements
        // to simplify control flow graph
        ast.borrow().functions.values().for_each(raise_exceptions);

        // TODO: write comment proving this only ever needs one pass
        false
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
                target.set_statements(vec![Statement::Panic(vec![]).into()]);
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
                "u__unconditional" => None,
                _ => Some(statement_cloned),
            }
        }

        _ => Some(statement_cloned),
    }
}

fn raise_exceptions(fn_def: &FunctionDefinition) {
    // first, find exception blocks (likely candidates are the target of "if
    // exception"s and contain a single panic instruction)
    let exception_blocks = fn_def
        .entry_block
        .iter()
        .filter(|block| {
            block.statements().len() == 1
                && (if let Statement::Panic(_) = &*block.statements()[0].borrow() {
                    true
                } else {
                    false
                } || if let Statement::Copy {
                    expression: Expression::Identifier(ident),
                    ..
                } = &*block.statements()[0].borrow()
                {
                    ident.as_ref() == "exception"
                } else {
                    false
                })
        })
        .collect::<HashSet<_>>();

    fn_def.entry_block.iter().for_each(|block| {
        if let Terminator::Conditional {
            condition,
            target,
            fallthrough,
        } = block.terminator()
        {
            if exception_blocks.contains(&target) {
                let mut statements = block.statements();
                statements.push(
                    Statement::If {
                        condition: Rc::new(RefCell::new(condition)),
                        if_body: vec![Statement::Panic(vec![]).into()],
                        else_body: vec![],
                    }
                    .into(),
                );
                block.set_statements(statements);
                block.set_terminator(Terminator::Unconditional {
                    target: fallthrough,
                });
            } else if exception_blocks.contains(&fallthrough) {
                let mut statements = block.statements();
                statements.push(
                    Statement::If {
                        condition: Rc::new(RefCell::new(condition)),
                        if_body: vec![],
                        else_body: vec![Statement::Panic(vec![]).into()],
                    }
                    .into(),
                );
                block.set_statements(statements);
                block.set_terminator(Terminator::Unconditional { target });
            }
        }
    });
}
