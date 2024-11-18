use {
    crate::boom::{
        control_flow::Terminator, passes::Pass, visitor::Visitor, Ast, Expression,
        FunctionDefinition, NamedType, NamedValue, Parameter, Statement, Type, Value,
    },
    common::{intern::InternedString, HashMap},
    itertools::Itertools,
    sailrs::shared::Shared,
};

#[derive(Debug, Default)]
pub struct DestructComposites;

impl DestructComposites {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for DestructComposites {
    fn name(&self) -> &'static str {
        "DestructComposites"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        // split struct registers into a register per field, returning the names and
        // types of the *removed* registers
        let mut removed_registers = HashMap::default();
        handle_registers(&mut ast.get_mut().registers, &mut removed_registers);

        // replace all field expressions and values with identifiers of the future
        // register/local var (1 layer only, do not handle nested field exprs yet)
        ast.get_mut()
            .functions
            .values_mut()
            .for_each(|def| remove_field_exprs(def));
        ast.get_mut()
            .functions
            .values_mut()
            .for_each(|def| remove_field_values(def));

        // replace struct return values with tuples
        ast.get_mut()
            .functions
            .iter_mut()
            .for_each(|(_, def)| split_return(def));

        {
            let functions = &ast.get().functions;

            functions.iter().for_each(|(_, def)| {
                // replace struct parameters in function signatures with individual fields,
                // returning the identifies and types of the removed parameters
                let mut removed_items = split_parameters(def.signature.parameters.clone());
                removed_items.extend(removed_registers.clone());
                destruct_local_structs(functions, removed_items, def);
            });
        }

        false
    }
}

fn handle_registers(
    registers: &mut HashMap<InternedString, Shared<Type>>,
    removed: &mut HashMap<InternedString, Shared<Type>>,
) {
    let mut to_add = vec![];

    registers.iter().for_each(|(name, typ)| match &*typ.get() {
        Type::Struct { fields, .. } => {
            removed.insert(*name, typ.clone());
            to_add.extend(fields.iter().map(
                |NamedType {
                     name: field_name,
                     typ,
                 }| (destructed_ident(*name, *field_name), typ.clone()),
            ));
        }
        Type::FixedVector {
            element_type,
            length,
        } => {
            if let Type::Struct { fields, .. } = &*element_type.get() {
                removed.insert(*name, typ.clone());
                to_add.extend(fields.iter().map(
                    |NamedType {
                         name: field_name,
                         typ,
                     }| {
                        (
                            destructed_ident(*name, *field_name),
                            Shared::new(Type::FixedVector {
                                length: *length,
                                element_type: typ.clone(),
                            }),
                        )
                    },
                ));
            }
        }
        _ => (),
    });

    for name in removed.keys() {
        registers.remove(name).unwrap();
    }
    registers.extend(to_add);
}

fn destruct_local_structs(
    functions: &HashMap<InternedString, FunctionDefinition>,
    mut removed_items: HashMap<InternedString, Shared<Type>>,
    fn_def: &FunctionDefinition,
) {
    // go through each statement in the function
    // if the statement is a struct type declaration, remove it and replace with
    // type decls for each field of the struct
    fn_def.entry_block.iter().for_each(|block| {
        let statements = block.statements();

        let destructed = statements
            .into_iter()
            .flat_map(|statement| {
                let clone = statement.clone();

                match &*statement.get() {
                    // if a struct local var is declared, replace it with declarations for all its
                    // fields
                    Statement::VariableDeclaration {name,typ } => {
                      split_declarations(*name, typ.clone(),&mut removed_items)
                    }
                    // if a struct is copied into a local variable, replace with several copies into
                    // each field
                    Statement::Copy { expression, value } => {
                        // if we are assigning to a field, replace with a copy where expression is
                        // destructed
                        if let Expression::Field { expression, field } = expression {
                            let Expression::Identifier(struc) = **expression else {
                                panic!();
                            };

                            return vec![Shared::new(Statement::Copy { expression: Expression::Identifier(destructed_ident(struc, *field)), value: value.clone() })];
                        }

                        // if we are copying *from* a field
                        {
                            let value_mut = &mut *value.get_mut();

                            if let Value::Field { value: struc_val, field_name } = value_mut {
                                let Value::Identifier(struc) = *(struc_val.clone().get()) else {
                                    panic!();
                                };

                                *value_mut = Value::Identifier(destructed_ident(struc, *field_name));
                            }
                        }

                        // otherwise assigning to whole struct
                        let Expression::Identifier(dest) = expression else {
                            return vec![clone];
                        };

                        let Some(dest_typ) = removed_items.get(dest) else {
                            return vec![clone];
                        };

                        if let Type::Struct { fields: dest_fields, .. } = &*dest_typ.get() {
                            // names of the fields to be copied into
                            let local_fields = fields_to_expressions(*dest, &dest_fields);// dest_fields.iter().map(|NamedType { name, .. }| Expression::Identifier(destructed_ident(*dest, *name))).collect::<Vec<_>>();

                            let values = match &*value.get() {
                                // if the value is an identifier, look up fields in structs map, and get
                                // list of values from that
                                Value::Identifier(_) => {
                                    split_arguments(&[value.clone()], &removed_items)
                                    // let typ = removed_items.get(ident).unwrap_or_else(|| panic!("attempting to assign non struct value identifier {ident:?} in {}", fn_def.signature.name));

                                    // let Type::Struct { fields, .. } = &*typ.get() else {
                                    //     panic!("not a struct?");
                                    // };

                                    // fields.iter().map(|NamedType { name, .. }| Value::Identifier(destructed_ident(*ident, *name))).map(Shared::new).collect::<Vec<_>>()
                                }
                                // if the value is a struct, use those fields
                                // todo: recursive structs here too
                                Value::Struct { fields, .. } => fields.iter().map(|NamedValue { value, .. }| value).cloned().collect::<Vec<_>>(),

                                Value::VectorAccess { value, index } => {
                                    let Value::Identifier(ident) = &*value.get() else { todo!() };
                                    let typ = removed_items.get(ident).unwrap_or_else(|| panic!("attempting to assign non struct value identifier {ident:?} in {}", fn_def.signature.name));

                                    let Type::FixedVector { element_type, .. } = &*typ.get() else {
                                        panic!("not a fixed vector?");
                                    };

                                    let Type::Struct { fields, .. } = &*element_type.get() else {
                                        panic!("not a struct?");
                                    };

                                    fields.iter().map(|NamedType { name, .. }| Value::Identifier(destructed_ident(*ident, *name))).map(Shared::new).map(|value| Value::VectorAccess { value, index: index.clone() }).map(Shared::new).collect::<Vec<_>>()
                                }
                                _ => panic!("value is a struct in {clone:?} in {}", fn_def.signature.name),
                            };

                            local_fields.into_iter().zip(values).map(|(expression, value)| Statement::Copy { expression, value }).map(Shared::new).collect()
                        } else {
                            let Value::VectorMutate { vector, element, index } = &*value.get() else { todo!() };

                            let Value::Identifier(element) = &*element.get() else {
                                todo!();
                            };

                            let Value::Identifier(source) = &*vector.get() else {
                                todo!();
                            };

                            assert_eq!(*source, *dest);

                            let Type::FixedVector { element_type, .. } = &*dest_typ.get() else {
                                todo!();
                            };

                            let Type::Struct { fields: dest_fields, .. } = &*element_type.get() else {
                                todo!();
                            };

                            // names of the fields to be copied into
                            dest_fields
                                .iter()
                                .map(|NamedType { name, .. }| *name)
                                .map(|field_name| {
                                    let field_vec = destructed_ident(*dest, field_name);
                                    let element = destructed_ident(*element, field_name);

                                    Shared::new(Statement::Copy {
                                        expression: Expression::Identifier(field_vec),
                                        value: Shared::new(Value::VectorMutate { vector: Shared::new(Value::Identifier(field_vec)), element: Shared::new(Value::Identifier(element)), index: index.clone() }),
                                    })
                                })
                                .collect()
                        }
                    }

                    // if we return a struct from a function call, assign it to the individual field variables
                    Statement::FunctionCall { expression: Some(expression), name, arguments } => {

                       let expression = if let Expression::Field { expression, field } = expression {
                            let mut idents = vec![*field];
                            let mut current_expression:Box<Expression> = expression.clone();
                            loop {
                                match *current_expression {
                                    Expression::Field { expression, field } => {idents.push(field); current_expression = expression},
                                    Expression::Identifier(ident) => {idents.push(ident); break;}
                                    _ => panic!(),
                                }
                            }
                            Expression::Identifier(InternedString::from(idents.iter().rev().join("_")))
                        } else {
                            (*expression).clone()
                        };

                        let expression = {
                            let Some(def) = functions.get(name) else {
                                // should properly handle built-ins here (but none return structs so this should
                                // be fine)
                                //
                                // 2024-09-25: this was a lie :(
                                return vec![clone];
                            };


                            if let Type::Tuple(_return_types) = &*def.signature.return_type.get() {
                                let Expression::Identifier(dest) = expression else {
                                    // only visiting each statement once so this should be true (for now, unions
                                    // break this)
                                    panic!();
                                };

                                let Some(typ) = removed_items.get(&dest) else {
                                    // tuple return type already removed
                                    return vec![clone];
                                };

                                let Type::Struct { fields, .. } = &*typ.get() else {
                                    panic!("not a struct?");
                                };

                                Some(fields_to_tuple(dest, &fields))
                            } else {
                                Some(expression.clone())
                            }
                        };

                        let arguments = split_arguments(arguments, &removed_items);

                        vec![Shared::new(Statement::FunctionCall { expression, name: *name, arguments })]
                    }
                    _ => vec![clone],
                }
            })
            .collect();

        block.set_statements(destructed);

        if let Terminator::Return(Value::Identifier(return_value_ident)) = block.terminator() {
            // done to distinguish structs of one element (converted to unary tuple) from a single return value
            if removed_items.contains_key(&return_value_ident) {
                   block.set_terminator(Terminator::Return(Value::Tuple(split_arguments(&[Shared::new(Value::Identifier(return_value_ident))], &removed_items))));
            }
        }
    });
}

fn split_return(fn_def: &mut FunctionDefinition) {
    fn_def.signature.return_type =
        nested_structs_to_flat_tuple(fn_def.signature.return_type.clone());
}

fn nested_structs_to_flat_tuple(typ: Shared<Type>) -> Shared<Type> {
    fn inner(typ: Shared<Type>) -> Vec<Shared<Type>> {
        if let Type::Struct { fields, .. } = &*typ.get() {
            fields
                .iter()
                .map(|nt| nt.typ.clone())
                .flat_map(inner)
                .collect()
        } else {
            vec![typ.clone()]
        }
    }

    match &*typ.get() {
        Type::Struct { .. } => Shared::new(Type::Tuple(inner(typ.clone()))),
        _ => typ.clone(),
    }
}

fn destructed_ident(
    local_variable_name: InternedString,
    field_name: InternedString,
) -> InternedString {
    format!("{local_variable_name}_{field_name}").into()
}

fn remove_field_exprs(def: &FunctionDefinition) {
    def.entry_block
        .iter()
        .flat_map(|b| b.statements())
        .for_each(|s| match &mut *s.get_mut() {
            Statement::Copy { expression, .. } => {
                if let Expression::Field {
                    expression: inner_expr,
                    field,
                } = expression
                {
                    if let Expression::Identifier(ident) = **inner_expr {
                        *expression = Expression::Identifier(destructed_ident(ident, *field));
                    }
                }
            }
            Statement::FunctionCall { expression, .. } => {
                if let Some(Expression::Field {
                    expression: inner_expr,
                    field,
                }) = expression
                {
                    if let Expression::Identifier(ident) = **inner_expr {
                        *expression = Some(Expression::Identifier(destructed_ident(ident, *field)));
                    }
                }
            }
            _ => (),
        })
}

fn remove_field_values(def: &FunctionDefinition) {
    struct FieldVisitor;

    impl Visitor for FieldVisitor {
        fn visit_value(&mut self, node: Shared<Value>) {
            // if value is a field...
            let (ident, field) = {
                let Value::Field { value, field_name } = &*node.get() else {
                    return;
                };
                let Value::Identifier(ident) = &*value.get() else {
                    panic!("field access to non identifier")
                };

                (*ident, *field_name)
            };

            //...replace it with the identifier of the corresponding local variable
            *node.get_mut() = Value::Identifier(destructed_ident(ident, field))
        }
    }

    FieldVisitor.visit_function_definition(def);
}

fn split_arguments(
    arguments: &[Shared<Value>],
    removed_items: &HashMap<InternedString, Shared<Type>>,
) -> Vec<Shared<Value>> {
    fn inner(ident: InternedString, typ: Shared<Type>) -> Vec<Shared<Value>> {
        let Type::Struct { fields, .. } = &*typ.get() else {
            unreachable!("not a struct?");
        };
        fields
            .iter()
            .flat_map(|NamedType { name, typ }| {
                let name = destructed_ident(ident, *name);
                if let Type::Struct { .. } = &*typ.get() {
                    inner(name, typ.clone())
                } else {
                    vec![Shared::new(Value::Identifier(name))]
                }
            })
            .collect()
    }

    arguments
        .iter()
        .flat_map(|v| {
            if let Value::Identifier(ident) = &*v.get() {
                if let Some(typ) = removed_items.get(ident) {
                    inner(*ident, typ.clone())
                } else {
                    vec![v.clone()]
                }
            } else {
                vec![v.clone()]
            }
        })
        .collect()
}

fn split_parameters(parameters: Shared<Vec<Parameter>>) -> HashMap<InternedString, Shared<Type>> {
    let mut removed = HashMap::default();

    let mut parameters = parameters.get_mut();
    *parameters = parameters
        .iter()
        .flat_map(|parameter| {
            if let Type::Struct { .. } = &*parameter.typ.get() {
                removed.insert(parameter.name, parameter.typ.clone());

                split_parameter_type(parameter.name, parameter.typ.clone())
            } else {
                vec![parameter.clone()]
            }
        })
        .collect();

    removed
}

fn split_parameter_type(parameter_name: InternedString, typ: Shared<Type>) -> Vec<Parameter> {
    if let Type::Struct { fields, .. } = &*typ.get() {
        fields
            .iter()
            .flat_map(|NamedType { name, typ }| {
                let name = destructed_ident(parameter_name, *name);
                split_parameter_type(name, typ.clone())
            })
            .collect()
    } else {
        vec![Parameter {
            name: parameter_name,
            typ: typ.clone(),
        }]
    }
}

fn fields_to_expressions(
    local_variable_name: InternedString,
    fields: &[NamedType],
) -> Vec<Expression> {
    fields
        .iter()
        .flat_map(|NamedType { name, typ }| {
            let name = destructed_ident(local_variable_name, *name);
            if let Type::Struct { fields, .. } = &*typ.get() {
                fields_to_expressions(name, fields)
            } else {
                vec![Expression::Identifier(name)]
            }
        })
        .collect()
}

fn fields_to_tuple(local_variable_name: InternedString, fields: &[NamedType]) -> Expression {
    Expression::Tuple(fields_to_expressions(local_variable_name, fields))
}

fn split_declarations(
    root_name: InternedString,
    typ: Shared<Type>,
    removed_items: &mut HashMap<InternedString, Shared<Type>>,
) -> Vec<Shared<Statement>> {
    fn inner(
        named_type: NamedType,
        removed_items: &mut HashMap<InternedString, Shared<Type>>,
    ) -> Vec<NamedType> {
        if let Type::Struct { fields, .. } = &*named_type.typ.get() {
            removed_items.insert(named_type.name, named_type.typ.clone());
            fields
                .iter()
                .cloned()
                .flat_map(|NamedType { name, typ }| {
                    inner(
                        NamedType {
                            name: destructed_ident(named_type.name, name),
                            typ,
                        },
                        removed_items,
                    )
                })
                .collect()
        } else {
            vec![named_type.clone()]
        }
    }

    if let Type::Struct { .. } = &*typ.get() {
        removed_items.insert(root_name, typ.clone());

        inner(
            NamedType {
                name: root_name,
                typ: typ.clone(),
            },
            removed_items,
        )
        .into_iter()
        .map(|NamedType { name, typ }| Statement::VariableDeclaration { name, typ })
        .map(Shared::new)
        .collect()
    } else {
        vec![Shared::new(Statement::VariableDeclaration {
            name: root_name,
            typ: typ.clone(),
        })]
    }
}
