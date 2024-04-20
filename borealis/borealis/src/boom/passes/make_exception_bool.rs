//! Remove non-bool assignments to `exception`
//!
//! Remove:
//!
//! * assignments where the value is a union

use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::Pass,
        visitor::{Visitor, Walkable},
        Ast, Expression, Literal, Size, Statement, Type, Value,
    },
    common::{intern::InternedString, shared::Shared, HashSet},
};

/// Remove all exception handling logic
pub struct MakeExceptionBool;

impl MakeExceptionBool {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self)
    }
}

impl Pass for MakeExceptionBool {
    fn name(&self) -> &'static str {
        "RemoveExceptions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get().functions.values().for_each(|def| {
            {
                let mut statements = def.entry_block.statements();

                if let Some(stmt) = statements.first() {
                    if let Statement::TypeDeclaration { name, .. } = &*stmt.get() {
                        if name.as_ref() == "exception" {
                            // skip if already done?
                            return;
                        }
                    }
                }

                statements.insert(
                    0,
                    Statement::TypeDeclaration {
                        name: "exception".into(),
                        typ: Shared::new(Type::Int {
                            signed: false,
                            size: Size::Static(1),
                        }),
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

        // TODO: write comment proving this only ever needs one pass
        false
    }
}

impl Visitor for MakeExceptionBool {
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
                    deleted_exception_vars.insert(*local_var_name);

                    None
                } else {
                    Some(statement_cloned)
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
                    if let Value::Identifier(ident) = &*value.get() {
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
