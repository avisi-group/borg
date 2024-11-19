use {
    crate::boom::{
        control_flow::Terminator, passes::Pass, Ast, Expression, Literal, Size, Statement, Type,
        Value,
    },
    common::{intern::InternedString, HashSet},
    sailrs::shared::Shared,
};

#[derive(Debug, Default)]
pub struct RemoveUnits;

impl RemoveUnits {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RemoveUnits {
    fn name(&self) -> &'static str {
        "RemoveUnits"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let mut ast = ast.get_mut();

        let mut removed_registers = HashSet::default();

        ast.registers = ast
            .registers
            .iter()
            .filter_map(|(name, typ)| {
                if matches!(&*typ.get(), Type::Unit) {
                    removed_registers.insert(*name);
                    None
                } else {
                    Some((*name, typ.clone()))
                }
            })
            .collect();

        ast.functions.iter().for_each(|(_, def)| {
            let mut removed = removed_registers.clone();

            def.entry_block.iter().for_each(|b| {
                b.set_statements(
                    b.statements()
                        .into_iter()
                        .filter_map(|s| match &*(s.get()) {
                            Statement::VariableDeclaration { typ, name } => {
                                if let Type::Unit = &*typ.get() {
                                    removed.insert(*name);
                                    None
                                } else {
                                    Some(s.clone())
                                }
                            }
                            Statement::Copy { expression, value } => {
                                let expression = filter_expression(expression, &removed);
                                let value = filter_value(value.clone(), &removed);

                                if let (Some(expression), Some(value)) = (expression, value) {
                                    Some(Shared::new(Statement::Copy { expression, value }))
                                } else {
                                    None
                                }
                            }
                            Statement::FunctionCall {
                                expression,
                                name,
                                arguments,
                            } => {
                                let expression = expression
                                    .as_ref()
                                    .map(|e| filter_expression(e, &removed))
                                    .flatten();
                                let arguments = arguments
                                    .iter()
                                    .filter_map(|v| filter_value(v.clone(), &removed))
                                    .collect();

                                Some(Shared::new(Statement::FunctionCall {
                                    expression,
                                    name: *name,
                                    arguments,
                                }))
                            }
                            _ => Some(s.clone()),
                        })
                        .collect(),
                );

                if let Terminator::Return(Value::Identifier(ident)) = b.terminator() {
                    if removed.contains(&ident) {
                        // todo: should be option<value>
                        b.set_terminator(Terminator::Return(Value::Literal(Shared::new(
                            Literal::Unit,
                        ))));
                    }
                }
            });
        });

        false
    }
}

fn filter_expression(
    expression: &Expression,
    removed: &HashSet<InternedString>,
) -> Option<Expression> {
    match expression {
        Expression::Identifier(ident) => {
            if removed.contains(ident) {
                None
            } else {
                Some(expression.clone())
            }
        }
        Expression::Tuple(vec) => Some(Expression::Tuple(
            vec.iter()
                .filter_map(|e| filter_expression(e, removed))
                .collect(),
        )),
        _ => unreachable!(),
    }
}

fn filter_value(value: Shared<Value>, removed: &HashSet<InternedString>) -> Option<Shared<Value>> {
    match &*value.get() {
        Value::Identifier(ident) => {
            if removed.contains(ident) {
                None
            } else {
                Some(value.clone())
            }
        }
        Value::Literal(literal) => {
            if let Literal::Unit = &*literal.get() {
                None
            } else {
                Some(value.clone())
            }
        }
        Value::Tuple(vec) => Some(Shared::new(Value::Tuple(
            vec.iter()
                .filter_map(|v| filter_value(v.clone(), removed))
                .collect(),
        ))),
        _ => Some(value.clone()),
    }
}
