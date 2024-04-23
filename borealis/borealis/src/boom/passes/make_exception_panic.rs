//! Convert exception logic into panics

use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        visitor::{Visitor, Walkable},
        Ast, Expression, Literal, Statement, Type, Value,
    },
    common::{intern::InternedString, shared::Shared},
};

/// Remove all exception handling logic
pub struct MakeExceptionPanic;

impl MakeExceptionPanic {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for MakeExceptionPanic {
    fn name(&self) -> &'static str {
        "RemoveExceptions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get().functions.values().for_each(|def| {
            self.visit_function_definition(def);
        });

        // TODO: write comment proving this only ever needs one pass
        false
    }
}

impl Visitor for MakeExceptionPanic {
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        let mut deleted_exception_vars = vec![];

        let statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| statement_filter(&mut deleted_exception_vars, statement))
            .collect();

        block.set_statements(statements);

        if let Terminator::Conditional {
            condition: Value::Identifier(ident),
            fallthrough,
            ..
        } = block.terminator()
        {
            if ident.as_ref() == "exception" {
                block.set_terminator(Terminator::Unconditional {
                    target: fallthrough,
                })
            }
        }

        block.walk(self);
    }
}

fn statement_filter(
    deleted_exception_vars: &mut Vec<InternedString>,
    statement: Shared<Statement>,
) -> Option<Shared<Statement>> {
    let statement_cloned = statement.clone();

    match &mut *statement.get_mut() {
        Statement::TypeDeclaration {
            typ,
            name: local_var_name,
        } => {
            if let Type::Union { name, .. } = *typ.get() {
                if name.as_ref() == "exception" {
                    deleted_exception_vars.push(*local_var_name);

                    None
                } else {
                    Some(statement_cloned)
                }
            } else {
                Some(statement_cloned)
            }
        }

        Statement::FunctionCall { expression, .. } => {
            if let Some(Expression::Identifier(ident)) = expression {
                if deleted_exception_vars.contains(ident) {
                    return None;
                }
            }

            Some(statement_cloned)
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
                    if let Value::Literal(lit) = &*value.get() {
                        if let Literal::Bool(true) = &*lit.get() {
                            Some(Shared::new(Statement::Panic(vec![])))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "throw" => None,
                _ => Some(statement_cloned),
            }
        }

        _ => Some(statement_cloned),
    }
}
