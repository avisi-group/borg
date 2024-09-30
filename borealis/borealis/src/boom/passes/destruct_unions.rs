use {
    crate::boom::{
        control_flow::ControlFlowBlock, passes::Pass, Ast, Expression, Size, Statement, Type, Value,
    },
    common::{HashMap, HashSet},
    sailrs::{intern::InternedString, shared::Shared},
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
        handle_registers(ast.clone());

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

        ast.get()
            .functions
            .iter()
            .map(|(_, d)| d.entry_block.clone())
            .for_each(|entry_block| {
                destruct_locals(&union_tags, union_tag_type.clone(), entry_block)
            });

        false
    }
}

fn handle_registers(ast: Shared<Ast>) {
    let union_regs = ast
        .get_mut()
        .registers
        .iter()
        .filter(|(_, typ)| matches!(&*typ.get(), Type::Union { .. }))
        .map(|(name, typ)| (*name, typ.clone()))
        .collect::<Vec<_>>();

    for (register_name, typ) in union_regs {
        ast.get_mut().registers.remove(&register_name);

        ast.get_mut().registers.insert(
            tag_ident(register_name),
            Shared::new(Type::Integer {
                size: Size::Static(32),
            }),
        );
        ast.get_mut()
            .registers
            .insert(value_ident(register_name), typ);
    }
}

/// split locally declared unions into a tag and a local variable the size of
/// the largest value?
fn destruct_locals(
    union_tags: &HashMap<InternedString, usize>,
    union_tag_type: Shared<Type>,
    entry_block: ControlFlowBlock,
) {
    let mut union_local_idents = HashSet::default();

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
                            union_local_idents.insert(*variable_name);
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

                        if union_local_idents.contains(dst) || union_local_idents.contains(src) {
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
                            vec![clone]
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
