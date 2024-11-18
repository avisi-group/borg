use {
    crate::boom::{
        control_flow::ControlFlowBlock, passes::Pass, Ast, Expression, FunctionDefinition,
        Parameter, Size, Statement, Type, Value,
    },
    common::{intern::InternedString, HashMap},
    sailrs::shared::Shared,
};

#[derive(Debug, Default)]
pub struct DestructUnions;

impl DestructUnions {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for DestructUnions {
    fn name(&self) -> &'static str {
        "DestructUnions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let removed = handle_registers(ast.clone());

        let union_tag_type = Shared::new(Type::Integer {
            size: Size::Static(32),
        });

        let union_tags = ast
            .get()
            .unions
            .iter()
            .flat_map(|(_, (_, tags))| tags)
            .map(|(&i, &v)| (i, v))
            .collect::<HashMap<_, _>>();

        ast.get().functions.iter().for_each(|(_, def)| {
            handle_function(def, removed.clone(), &union_tags, union_tag_type.clone())
        });

        false
    }
}

fn handle_registers(ast: Shared<Ast>) -> HashMap<InternedString, Shared<Type>> {
    let union_regs = ast
        .get_mut()
        .registers
        .iter()
        .filter(|(_, typ)| matches!(&*typ.get(), Type::Union { .. }))
        .map(|(name, typ)| (*name, typ.clone()))
        .collect::<HashMap<_, _>>();

    for (register_name, typ) in &union_regs {
        ast.get_mut().registers.remove(register_name);

        ast.get_mut().registers.insert(
            tag_ident(*register_name),
            Shared::new(Type::Integer {
                size: Size::Static(32),
            }),
        );
        ast.get_mut()
            .registers
            .insert(value_ident(*register_name), typ.clone());
    }

    union_regs
}

fn handle_function(
    def: &FunctionDefinition,
    mut removed: HashMap<InternedString, Shared<Type>>,
    union_tags: &HashMap<InternedString, usize>,
    union_tag_type: Shared<Type>,
) {
    // todo
    // if let Type::Union { width } = &*def.signature.return_type.get() {
    //     if let Some(_) = union_tags.get(&def.signature.name) {
    //         // union constructor, will be handled later
    //     } else {
    //         *def.signature.return_type.get_mut() = Type::Tuple(vec![
    //             union_tag_type.clone(),
    //             Shared::new(Type::Union { width: *width }),
    //         ])
    //     }
    // };

    let parameters = def
        .signature
        .parameters
        .get()
        .iter()
        .flat_map(|param| {
            if let Type::Union { .. } = &*param.typ.get() {
                removed.insert(param.name, param.typ.clone());
                vec![
                    Parameter {
                        name: tag_ident(param.name),
                        typ: union_tag_type.clone(),
                    },
                    Parameter {
                        name: value_ident(param.name),
                        typ: param.typ.clone(),
                    },
                ]
            } else {
                vec![param.clone()]
            }
        })
        .collect();
    *def.signature.parameters.get_mut() = parameters;

    destruct_locals(
        removed,
        &union_tags,
        union_tag_type.clone(),
        def.entry_block.clone(),
    );
}

/// split locally declared unions into a tag and a local variable the size of
/// the largest value?
fn destruct_locals(
    mut removed: HashMap<InternedString, Shared<Type>>,
    union_tags: &HashMap<InternedString, usize>,
    union_tag_type: Shared<Type>,
    entry_block: ControlFlowBlock,
) {
    entry_block.iter().for_each(|block| {
        let destructed = block
            .statements()
            .into_iter()
            .flat_map(|statement| {
                let clone = statement.clone();

                match &*statement.get() {
                    Statement::VariableDeclaration {
                        name: variable_name,
                        typ,
                    } => {
                        // if we have a type declaration for a union, emit value and tag variables
                        // instead
                        if let Type::Union { .. } = &*typ.get() {
                            removed.insert(*variable_name, typ.clone());
                            vec![
                                Shared::new(Statement::VariableDeclaration {
                                    name: value_ident(*variable_name),
                                    typ: typ.clone(),
                                }),
                                Shared::new(Statement::VariableDeclaration {
                                    name: tag_ident(*variable_name),
                                    typ: union_tag_type.clone(),
                                }),
                            ]
                        } else {
                            return vec![clone];
                        }
                    }
                    Statement::Copy {
                        expression: Expression::Identifier(dst),
                        value,
                    } => {
                        let Value::Identifier(src) = &*value.get() else {
                            return vec![clone];
                        };

                        if removed.contains_key(dst) || removed.contains_key(src) {
                            vec![
                                Shared::new(Statement::Copy {
                                    expression: Expression::Identifier(tag_ident(*dst)),
                                    value: Shared::new(Value::Identifier(tag_ident(*src))),
                                }),
                                Shared::new(Statement::Copy {
                                    expression: Expression::Identifier(value_ident(*dst)),
                                    value: Shared::new(Value::Identifier(value_ident(*src))),
                                }),
                            ]

                            // assign value
                            // assign tag
                        } else {
                            vec![clone]
                        }
                    }

                    Statement::FunctionCall {
                        expression,
                        name,
                        arguments,
                    } => {
                        // function name is a union tag constructor
                        if let Some(tag) = union_tags.get(name) {
                            if let Some(Expression::Identifier(ident)) = expression {
                                assert_eq!(1, arguments.len());

                                vec![
                                    Shared::new(Statement::Copy {
                                        expression: Expression::Identifier(tag_ident(*ident)),
                                        value: Shared::new(Value::Literal(Shared::new(
                                            crate::boom::Literal::Int((*tag).into()),
                                        ))),
                                    }),
                                    Shared::new(Statement::Copy {
                                        expression: Expression::Identifier(value_ident(*ident)),
                                        value: arguments[0].clone(),
                                    }),
                                ]
                            } else {
                                panic!();
                            }
                        } else {
                            // fix union destination or argument
                            let expression = match expression {
                                Some(Expression::Tuple(exprs)) => Some(Expression::Tuple(
                                    exprs
                                        .iter()
                                        .flat_map(|e| {
                                            if let Expression::Identifier(ident) = e {
                                                if removed.contains_key(ident) {
                                                    vec![
                                                        Expression::Identifier(tag_ident(*ident)),
                                                        Expression::Identifier(value_ident(*ident)),
                                                    ]
                                                } else {
                                                    vec![Expression::Identifier(*ident)]
                                                }
                                            } else {
                                                vec![e.clone()]
                                            }
                                        })
                                        .collect(),
                                )),
                                Some(Expression::Identifier(ident)) => {
                                    if removed.contains_key(ident) {
                                        Some(Expression::Tuple(vec![
                                            Expression::Identifier(tag_ident(*ident)),
                                            Expression::Identifier(value_ident(*ident)),
                                        ]))
                                    } else {
                                        Some(Expression::Identifier(*ident))
                                    }
                                }
                                Some(expr) => Some(expr.clone()),
                                None => None,
                            };

                            let arguments = arguments
                                .iter()
                                .flat_map(|arg| {
                                    if let Value::Identifier(ident) = &*arg.get() {
                                        if removed.contains_key(ident) {
                                            vec![
                                                Shared::new(Value::Identifier(tag_ident(*ident))),
                                                Shared::new(Value::Identifier(value_ident(*ident))),
                                            ]
                                        } else {
                                            vec![arg.clone()]
                                        }
                                    } else {
                                        vec![arg.clone()]
                                    }
                                })
                                .collect();

                            vec![Shared::new(Statement::FunctionCall {
                                expression,
                                name: *name,
                                arguments,
                            })]
                        }
                    }
                    _ => vec![clone],
                }
            })
            .collect();

        block.set_statements(destructed);
    });
}

fn value_ident(local_union_ident: InternedString) -> InternedString {
    format!("{local_union_ident}_value").into()
}

fn tag_ident(local_union_ident: InternedString) -> InternedString {
    format!("{local_union_ident}_tag").into()
}
