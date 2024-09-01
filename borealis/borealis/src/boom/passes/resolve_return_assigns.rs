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
    common::shared::Shared,
};

/// Resolves assignments to `return` variables.
pub struct ResolveReturns {}

impl ResolveReturns {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {})
    }
}

impl Pass for ResolveReturns {
    fn name(&self) -> &'static str {
        "ResolveReturns"
    }

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get().functions.values().for_each(|def| {
            // run before struct/union splitting so should only have one return type
            assert_eq!(1, def.signature.return_types.len());

            {
                let mut statements = def.entry_block.statements();
                statements.insert(
                    0,
                    Statement::VariableDeclaration {
                        name: "return_value".into(),
                        typ: def.signature.return_types[0].clone(),
                    }
                    .into(),
                );
                def.entry_block.set_statements(statements);
            }

            def.entry_block
                .iter()
                .flat_map(|b| b.statements())
                .for_each(|s| {
                    if let Statement::Copy {
                        ref mut expression, ..
                    }
                    | Statement::FunctionCall {
                        expression: Some(ref mut expression),
                        ..
                    } = &mut *s.get_mut()
                    {
                        let Expression::Identifier(ident) = expression else {
                            return;
                        };

                        if ident.as_ref() == "return" {
                            *expression = Expression::Identifier("return_value".into());
                        }
                    }
                });
        });

        false
    }

    fn reset(&mut self) {
        todo!()
    }
}
