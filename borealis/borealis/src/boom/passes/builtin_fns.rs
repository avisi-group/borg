// Although some builtin fns are handled in rudder, we handle some basic ones
// here to make the destruct_composites pass

use {
    crate::boom::{passes::Pass, Ast, Expression, Literal, Operation, Statement, Value},
    once_cell::sync::Lazy,
    regex::Regex,
    sailrs::shared::Shared,
};

const EQ_ANY_GENERIC: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^eq_any<([0-9a-zA-Z_%<>]+)>$").unwrap());

const VECTOR_ACCESS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^plain_vector_access<([0-9a-zA-Z_%<>]+)>$").unwrap());

const VECTOR_UPDATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^plain_vector_update<([0-9a-zA-Z_%<>]+)>$").unwrap());

#[derive(Debug, Default)]
pub struct HandleBuiltinFunctions;

impl HandleBuiltinFunctions {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for HandleBuiltinFunctions {
    fn name(&self) -> &'static str {
        "HandleBuiltinFunctions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get()
            .functions
            .iter()
            .flat_map(|(_, def)| def.entry_block.iter())
            .for_each(|b| {
                b.set_statements(
                    b.statements()
                        .into_iter()
                        .map(|s| {
                            if let Statement::FunctionCall {
                                name,
                                expression: Some(expression),
                                arguments,
                            } = &*(s.get())
                            {
                                // replace all eq functions with an equal operation
                                if EQ_ANY_GENERIC.is_match(name.as_ref())
                                    || name.as_ref() == "eq_int"
                                    || name.as_ref() == "eq_bits"
                                {
                                    assert_eq!(2, arguments.len());
                                    op(
                                        expression,
                                        Operation::Equal(
                                            arguments[0].clone(),
                                            arguments[1].clone(),
                                        ),
                                    )
                                } else if name.as_ref() == "IsZero" {
                                    op(
                                        expression,
                                        Operation::Equal(
                                            arguments[0].clone(),
                                            Shared::new(Value::Literal(Shared::new(Literal::Int(
                                                0.into(),
                                            )))),
                                        ),
                                    )
                                } else if VECTOR_ACCESS.is_match(name.as_ref()) {
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::VectorAccess {
                                            value: arguments[0].clone(),
                                            index: arguments[1].clone(),
                                        }),
                                    })
                                } else if VECTOR_UPDATE.is_match(name.as_ref()) {
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::VectorMutate {
                                            vector: arguments[0].clone(),
                                            element: arguments[2].clone(),
                                            index: arguments[1].clone(),
                                        }),
                                    })
                                } else {
                                    s.clone()
                                }
                            } else {
                                s.clone()
                            }
                        })
                        .collect(),
                );
            });

        false
    }
}

fn op(expression: &Expression, op: Operation) -> Shared<Statement> {
    Shared::new(Statement::Copy {
        expression: expression.clone(),
        value: Shared::new(Value::Operation(op)),
    })
}
