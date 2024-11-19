use {
    crate::boom::{
        control_flow::ControlFlowBlock, passes::Pass, Ast, Expression, FunctionDefinition, Literal,
        NamedType, Parameter, Size, Statement, Type, Value,
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
        let unions = ast.get().unions.clone();

        let removed = handle_registers(ast.clone());

        ast.get()
            .functions
            .iter()
            .for_each(|(_, def)| handle_function(def, removed.clone(), &unions));

        false
    }
}

fn handle_registers(ast: Shared<Ast>) -> HashMap<InternedString, Vec<NamedType>> {
    let union_regs = ast
        .get_mut()
        .registers
        .iter()
        .filter_map(|(name, typ)| {
            if let Type::Union { fields, .. } = &*typ.get() {
                Some((*name, fields.clone()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    for (register_name, fields) in &union_regs {
        ast.get_mut().registers.remove(register_name);

        ast.get_mut()
            .registers
            .extend(destruct(*register_name, fields));
    }

    union_regs
}

fn handle_function(
    def: &FunctionDefinition,
    mut removed: HashMap<InternedString, Vec<NamedType>>,
    unions: &HashMap<InternedString, Vec<NamedType>>,
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
            if let Type::Union { fields, .. } = &*param.typ.get() {
                removed.insert(param.name, fields.clone());

                destruct(param.name, fields)
                    .into_iter()
                    .map(|(name, typ)| Parameter { name, typ })
                    .collect()
            } else {
                vec![param.clone()]
            }
        })
        .collect();
    *def.signature.parameters.get_mut() = parameters;

    destruct_locals(removed, unions, def.entry_block.clone());
}

/// split locally declared unions into a tag and a local variable the size of
/// the largest value?
fn destruct_locals(
    mut removed: HashMap<InternedString, Vec<NamedType>>,
    unions: &HashMap<InternedString, Vec<NamedType>>,
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
                        if let Type::Union { fields, .. } = &*typ.get() {
                            removed.insert(*variable_name, fields.clone());
                            destruct(*variable_name, fields)
                                .into_iter()
                                .map(|(name, typ)| Statement::VariableDeclaration { name, typ })
                                .map(Shared::new)
                                .collect()
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

                        match (removed.get(src), removed.get(dst)) {
                            (Some(src_fields), Some(dst_fields)) => destruct(*src, src_fields)
                                .into_iter()
                                .zip(destruct(*dst, dst_fields).into_iter())
                                .map(|((src, _), (dst, _))| Statement::Copy {
                                    expression: Expression::Identifier(dst),
                                    value: Shared::new(Value::Identifier(src)),
                                })
                                .map(Shared::new)
                                .collect(),
                            (None, None) => vec![clone],
                            _ => panic!(),
                        }
                    }

                    Statement::FunctionCall {
                        expression,
                        name,
                        arguments,
                    } => {
                        // function name is a union tag constructor

                        // todo: tidy this up
                        if let Some((fields, tag, _)) = unions
                            .values()
                            .flat_map(|nts| {
                                nts.into_iter()
                                    .enumerate()
                                    .map(|(i, nt)| (nts.clone(), i, nt))
                            })
                            .find(|(_, _, nt)| nt.name == *name)
                        {
                            if let Some(Expression::Identifier(ident)) = expression {
                                assert_eq!(1, arguments.len());

                                fields
                                    .iter()
                                    .enumerate()
                                    .map(|(i, nt)| Statement::Copy {
                                        expression: Expression::Identifier(value_ident(
                                            *ident, nt.name, i,
                                        )),
                                        value: if i == tag {
                                            arguments[0].clone()
                                        } else {
                                            default_value(nt.typ.clone())
                                        },
                                    })
                                    .chain([Statement::Copy {
                                        expression: Expression::Identifier(tag_ident(*ident)),
                                        value: Shared::new(Value::Literal(Shared::new(
                                            Literal::Int(tag.into()),
                                        ))),
                                    }])
                                    .map(Shared::new)
                                    .collect()
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
                                                if let Some(fields) = removed.get(ident) {
                                                    destruct(*ident, fields)
                                                        .into_iter()
                                                        .map(|(name, _)| {
                                                            Expression::Identifier(name)
                                                        })
                                                        .collect()
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
                                    if let Some(fields) = removed.get(ident) {
                                        Some(Expression::Tuple(
                                            destruct(*ident, fields)
                                                .into_iter()
                                                .map(|(name, _)| Expression::Identifier(name))
                                                .collect(),
                                        ))
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
                                        if let Some(fields) = removed.get(ident) {
                                            destruct(*ident, fields)
                                                .into_iter()
                                                .map(|(name, _)| Value::Identifier(name))
                                                .map(Shared::new)
                                                .collect()
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

fn destruct(
    root_name: InternedString,
    fields: &[NamedType],
) -> Vec<(InternedString, Shared<Type>)> {
    [(
        tag_ident(root_name),
        Shared::new(Type::Integer {
            size: Size::Static(32),
        }),
    )]
    .into_iter()
    .chain(
        fields
            .iter()
            .enumerate()
            .map(|(tag, NamedType { name, typ })| {
                (value_ident(root_name, *name, tag), typ.clone())
            }),
    )
    .collect()
}

fn tag_ident(ident: InternedString) -> InternedString {
    format!("{ident}_tag").into()
}

fn value_ident(root: InternedString, variant: InternedString, tag: usize) -> InternedString {
    format!("{root}_{variant}_{tag}").into()
}

fn default_value(typ: Shared<Type>) -> Shared<Value> {
    match &*typ.get() {
        Type::Unit => Shared::new(Value::Literal(Shared::new(Literal::Unit))),
        Type::String => Shared::new(Value::Literal(Shared::new(Literal::String(
            InternedString::from_static("default value"),
        )))),
        Type::Bool => Shared::new(Value::Literal(Shared::new(Literal::Bool(false)))),
        // Type::Struct { name, fields } => Shared::new(Value::Struct {
        //     name: *name,
        //     fields: fields
        //         .iter()
        //         .cloned()
        //         .map(|NamedType { name, typ }| NamedValue {
        //             name,
        //             value: default_value(typ),
        //         })
        //         .collect(),
        // }),
        Type::Integer { .. } => Shared::new(Value::Literal(Shared::new(Literal::Int(0.into())))),
        Type::Bits { .. } => {
            Shared::new(Value::Literal(Shared::new(Literal::Bits(vec![0.into()]))))
        }
        Type::Union { .. } | Type::Struct { .. } => {
            Shared::new(Value::Literal(Shared::new(Literal::Undefined)))
        }
        t => todo!("{t:?}"),
    }
}
