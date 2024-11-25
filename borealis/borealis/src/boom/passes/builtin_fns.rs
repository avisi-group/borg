// Although some builtin fns are handled in rudder, we handle some basic ones
// here to make the destruct_composites pass

use {
    crate::boom::{passes::Pass, Ast, Bit, Expression, Literal, Operation, Statement, Value},
    common::intern::InternedString,
    once_cell::sync::Lazy,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    regex::Regex,
    sailrs::shared::Shared,
};

const EQ_ANY_GENERIC: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^eq_any<([0-9a-zA-Z_%<>]+)>$").unwrap());

const VECTOR_ACCESS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^plain_vector_access<([0-9a-zA-Z_%<>]+)>$").unwrap());

const VECTOR_UPDATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^plain_vector_update<([0-9a-zA-Z_%<>]+)>|internal_vector_update$").unwrap()
});

const UNDEFINED: Lazy<Regex> = Lazy::new(|| Regex::new(r"^undefined_([0-9a-zA-Z_%<>]+)$").unwrap());

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
        ast.get().functions.par_iter().for_each(|(_, def)| {
            def.entry_block.iter().for_each(|b| {
                b.set_statements(
                    b.statements()
                        .into_iter()
                        .filter_map(|s| {
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
                                    Some(op(
                                        expression,
                                        Operation::Equal(
                                            arguments[0].clone(),
                                            arguments[1].clone(),
                                        ),
                                    ))
                                } else if name.as_ref() == "IsZero" {
                                    Some(op(
                                        expression,
                                        Operation::Equal(
                                            arguments[0].clone(),
                                            Shared::new(Value::Literal(Shared::new(Literal::Int(
                                                0.into(),
                                            )))),
                                        ),
                                    ))
                                } else if VECTOR_ACCESS.is_match(name.as_ref()) {
                                    Some(Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::VectorAccess {
                                            value: arguments[0].clone(),
                                            index: arguments[1].clone(),
                                        }),
                                    }))
                                } else if VECTOR_UPDATE.is_match(name.as_ref()) {
                                    Some(Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::VectorMutate {
                                            vector: arguments[0].clone(),
                                            element: arguments[2].clone(),
                                            index: arguments[1].clone(),
                                        }),
                                    }))
                                } else if let Some(captures) = UNDEFINED.captures(name.as_ref()) {
                                    let typ_name = captures.get(1).unwrap();
                                    if ast
                                        .get()
                                        .enums
                                        .get(&InternedString::from(typ_name.as_str()))
                                        .is_some()
                                    {
                                        Some(Shared::new(Statement::Copy {
                                            expression: expression.clone(),
                                            value: Shared::new(Value::Literal(Shared::new(
                                                Literal::Bits(vec![Bit::Zero; 32]),
                                            ))),
                                        }))
                                    } else {
                                        Some(s.clone())
                                    }
                                } else if name.as_ref() == "sail_cons" // drop these for now
                                // todo: don't
                                    || name.as_ref() == "internal_vector_init"
                                {
                                    None
                                } else {
                                    Some(s.clone())
                                }
                            } else {
                                Some(s.clone())
                            }
                        })
                        .collect(),
                );
            });
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
