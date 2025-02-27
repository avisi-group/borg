use {
    crate::boom::{
        Ast, Expression, Literal, Parameter, Statement, Type, Value, control_flow::Terminator,
        passes::Pass,
    },
    common::{HashSet, intern::InternedString},
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

        ast.functions.iter_mut().for_each(|(_, def)| {
            let mut removed = removed_registers.clone();

            if let Some(ret) = def.signature.return_type.clone() {
                def.signature.return_type = remove_unit_type(ret.clone());
            }

            let new_params = def
                .signature
                .parameters
                .get()
                .iter()
                .filter_map(|p| {
                    let o =
                        remove_unit_type(p.typ.clone()).map(|typ| Parameter { name: p.name, typ });
                    if o.is_none() {
                        removed.insert(p.name);
                    }
                    o
                })
                .collect();
            *def.signature.parameters.get_mut() = new_params;

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

                if let Terminator::Return(Some(value)) = b.terminator() {
                    b.set_terminator(Terminator::Return(
                        filter_value(Shared::new(value), &removed).map(|s| s.get().clone()),
                    ));
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
        e => unreachable!("{e:?}"),
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

fn remove_unit_type(typ: Shared<Type>) -> Option<Shared<Type>> {
    match &*typ.get() {
        Type::Unit => None,
        Type::Tuple(vec) => Some(Shared::new(Type::Tuple(
            vec.iter()
                .flat_map(|t| remove_unit_type(t.clone()))
                .collect(),
        ))),
        _ => Some(typ.clone()),
    }
}
