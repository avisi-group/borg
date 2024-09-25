// if Regex::new(r"^eq_any<([0-9a-zA-Z_%<>]+)>$")
// .unwrap()
// .is_match(name.as_ref())
// {
// Some(build(
//     self.block,
//     self.block_arena_mut(),
//     Statement::BinaryOperation {
//         kind: BinaryOperationKind::CompareEqual,
//         lhs: args[0].clone(),
//         rhs: args[1].clone(),
//     },
// ))
// } else if Regex::new(r"^plain_vector_update<([0-9a-zA-Z_%<>]+)>$")
// .unwrap()
// .is_match(name.as_ref())
// {
// Some(build(
//     self.block,
//     self.block_arena_mut(),
//     Statement::AssignElement {
//         vector: args[0].clone(),
//         value: args[2].clone(),
//         index: args[1].clone(),
//     },
// ))
// }

use {
    crate::boom::{passes::Pass, Ast, Bit, Literal, Operation, Statement, Value},
    common::shared::Shared,
    regex::Regex,
};

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
                                if Regex::new(r"^eq_any<([0-9a-zA-Z_%<>]+)>$")
                                    .unwrap()
                                    .is_match(name.as_ref())
                                {
                                    assert_eq!(2, arguments.len());
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::Operation(Operation::Equal(
                                            arguments[0].clone(),
                                            arguments[1].clone(),
                                        ))),
                                    })
                                } else if name.as_ref() == "undefined_bitvector"
                                    || name.as_ref() == "undefined_vector<b>"
                                {
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::Literal(Shared::new(
                                            Literal::Bits(vec![Bit::Zero]),
                                        ))),
                                    })
                                } else if Regex::new(r"^plain_vector_access<([0-9a-zA-Z_%<>]+)>$")
                                    .unwrap()
                                    .is_match(name.as_ref())
                                {
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::VectorAccess {
                                            value: arguments[0].clone(),
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
