use {
    crate::boom::{
        control_flow::Terminator, passes::Pass, visitor::Visitor, Ast, Expression,
        FunctionDefinition, NamedType, NamedValue, Parameter, Statement, Type, Value,
    },
    sailrs::{intern::InternedString, shared::Shared, HashMap},
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
                destruct_local_structs(functions, removed_items, def)
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
                    Statement::VariableDeclaration {
                        name: variable_name,
                        typ,
                    } => {
                        let Type::Struct { fields, .. } = &*typ.get() else {
                            return vec![clone];
                        };

                        removed_items.insert(*variable_name, typ.clone());

                        fields
                            .iter()
                            .map(|NamedType { name: field_name, typ }| {
                                Statement::VariableDeclaration {
                                    name: destructed_ident(*variable_name, *field_name),
                                    typ: typ.clone(),
                                }
                                .into()
                            })
                            .collect()
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

                            return vec![Shared::new(Statement::Copy {
                                expression: Expression::Identifier(destructed_ident(struc, *field)),
                                value: value.clone(),
                            })];
                        }

                        // if we are copying *from* a field
                        {
                            let value_mut = &mut *value.get_mut();

                            if let Value::Field {
                                value: struc_val,
                                field_name,
                            } = value_mut
                            {
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

                        let Some(typ) = removed_items.get(dest) else {
                            return vec![clone];
                        };

                        let Type::Struct {  fields,.. } = &*typ.get() else {
                            panic!("not a struct?");
                        };

                        // names of the fields to be copied into
                        let local_fields = fields
                            .iter()
                            .map(|NamedType { name, .. }| Expression::Identifier(destructed_ident(*dest, *name)))
                            .collect::<Vec<_>>();

                        let values = match &*value.get() {
                            // if the value is an identifier, look up fields in structs map, and get
                            // list of values from that
                            Value::Identifier(ident) => {
                                let typ = removed_items.get(ident).unwrap_or_else(|| {
                                    panic!("attempting to assign non struct value identifier {ident:?} in {}", fn_def.signature.name)
                                });

                                let Type::Struct {  fields,.. } = &*typ.get() else {
                                    panic!("not a struct?");
                                };


                                fields
                                    .iter()
                                    .map(|NamedType { name, .. }| Value::Identifier(destructed_ident(*ident, *name)))
                                    .map(Shared::new)
                                    .collect::<Vec<_>>()
                            }
                            // if the value is a struct, use those fields
                            Value::Struct { fields, .. } => fields
                                .iter()
                                .map(|NamedValue { value, .. }| value)
                                .cloned()
                                .collect::<Vec<_>>(),

                            Value::VectorAccess { value, index } => {
                                let Value::Identifier(ident) = &*value.get() else {
                                    todo!()
                                };
                                let typ = removed_items.get(ident).unwrap_or_else(|| {
                                    panic!("attempting to assign non struct value identifier {ident:?} in {}", fn_def.signature.name)
                                });

                                let Type::FixedVector{  element_type,..   } = &*typ.get() else {
                                    panic!("not a fixed vector?");
                                };

                                let Type::Struct {  fields,.. } = &*element_type.get() else {
                                    panic!("not a struct?");
                                };

                                fields
                                    .iter()
                                    .map(|NamedType { name, .. }| Value::Identifier(destructed_ident(*ident, *name)))
                                    .map(Shared::new)
                                    .map(|value| Value::VectorAccess { value, index: index.clone() })
                                    .map(Shared::new)
                                    .collect::<Vec<_>>()
                            }
                            _ => panic!("value is a struct in {clone:?} in {}", fn_def.signature.name),
                        };

                        local_fields
                            .into_iter()
                            .zip(values)
                            .map(|(expression, value)| Statement::Copy { expression, value })
                            .map(Shared::new)
                            .collect()
                    }

                    // if we return a struct from a function call, assign it to the individual field variables
                    Statement::FunctionCall {
                        expression: Some(expression),
                        name,
                        arguments,
                    } => {
                        let expression = {
                            let Some(def) = functions.get(name) else {
                                // should properly handle built-ins here (but none return structs so this should
                                // be fine)
                                //
                                // 2024-09-25: this was a lie :(
                                return vec![clone];
                            };

                            if let Type::Tuple(return_types) = &*def.signature.return_type.get() {
                                let Expression::Identifier(dest) = expression else {
                                    // only visiting each statement once so this should be true (for now, unions
                                    // break this)
                                    panic!();
                                };

                                let typ = removed_items.get(dest).unwrap();
                                let Type::Struct {  fields,.. } = &*typ.get() else {
                                    panic!("not a struct?");
                                };

                                assert_eq!(fields.len(), return_types.len()); //todo: validate the types too

                                Some(Expression::Tuple(
                                    fields
                                        .iter()
                                        .map(|NamedType { name, .. }| {
                                            Expression::Identifier(destructed_ident(*dest, *name))
                                        })
                                        .collect(),
                                ))
                            } else {
                                Some(expression.clone())
                            }
                        };

                        let arguments = arguments
                            .iter()
                            .flat_map(|v| {
                                if let Value::Identifier(ident) = &*v.get() {
                                    if let Some(typ) = removed_items.get(ident) {
                                        let Type::Struct {  fields,.. } = &*typ.get() else {
                                            panic!("not a struct?");
                                        };
                                        fields
                                            .iter()
                                            .map(|NamedType { name, .. }| {
                                                Shared::new(Value::Identifier(destructed_ident(*ident, *name)))
                                            })
                                            .collect()
                                    } else {
                                        vec![v.clone()]
                                    }
                                } else {
                                    vec![v.clone()]
                                }
                            })
                            .collect();

                        vec![Shared::new(Statement::FunctionCall {
                            expression,
                            name: *name,
                            arguments,
                        })]
                    }
                    _ => vec![clone],
                }
            })
            .collect();

        block.set_statements(destructed);

        if let Terminator::Return(Value::Identifier(return_value_ident)) = block.terminator() {
            if let Some(typ) = removed_items.get(&return_value_ident) {
                let Type::Struct {  fields,.. } = &*typ.get() else {
                    panic!("not a struct?");
                };
                block.set_terminator(Terminator::Return(Value::Tuple(
                    fields
                        .iter()
                        .map(|NamedType { name, .. }| Value::Identifier(destructed_ident(return_value_ident, *name)))
                        .map(Shared::new)
                        .collect(),
                )));
            }
        }
    });
}

fn split_return(fn_def: &mut FunctionDefinition) {
    let Type::Struct { fields, .. } = (&*fn_def.signature.return_type.get()).clone() else {
        return;
    };

    fn_def.signature.return_type = Shared::new(Type::Tuple(
        fields
            .into_iter()
            .map(|NamedType { typ, .. }| typ)
            .collect(),
    ));
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

fn split_parameters(parameters: Shared<Vec<Parameter>>) -> HashMap<InternedString, Shared<Type>> {
    let mut removed = HashMap::default();

    let mut parameters = parameters.get_mut();
    *parameters = parameters
        .iter()
        .flat_map(|parameter| {
            if let Type::Struct { fields, .. } = &*parameter.typ.get() {
                removed.insert(parameter.name, parameter.typ.clone());

                fields
                    .iter()
                    .map(
                        |NamedType {
                             name: field_name,
                             typ,
                         }| Parameter {
                            name: destructed_ident(parameter.name, *field_name),
                            typ: typ.clone(),
                        },
                    )
                    .collect()
            } else {
                vec![parameter.clone()]
            }
        })
        .collect();

    removed
}
