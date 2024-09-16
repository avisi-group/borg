use {
    crate::boom::{
        control_flow::Terminator, passes::Pass, visitor::Visitor, Ast, Expression, FunctionDefinition, NamedType,
        NamedValue, Parameter, Statement, Type, Value,
    },
    common::{intern::InternedString, shared::Shared, HashMap},
};

#[derive(Debug, Default)]
pub struct DestructStructs;

impl DestructStructs {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for DestructStructs {
    fn name(&self) -> &'static str {
        "DestructStructs"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        // split struct registers into a register per field, returning the names and
        // types of the *removed* registers
        let struct_registers = handle_registers(&mut ast.get_mut().registers);

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
                let mut removed_structs = split_parameters(def.signature.parameters.clone());
                removed_structs.extend(struct_registers.clone());
                destruct_local_structs(functions, removed_structs, def)
            });
        }

        false
    }
}

fn handle_registers(registers: &mut HashMap<InternedString, Shared<Type>>) -> HashMap<InternedString, Vec<NamedType>> {
    let mut to_remove = HashMap::default();
    let mut to_add = vec![];

    registers.iter().for_each(|(name, typ)| {
        if let Type::Struct { fields, .. } = &*typ.get() {
            to_remove.insert(*name, fields.clone());
            to_add.extend(
                fields
                    .iter()
                    .map(|NamedType { name: field_name, typ }| (destructed_ident(*name, *field_name), typ.clone())),
            );
        }
    });

    for name in to_remove.keys() {
        registers.remove(name).unwrap();
    }
    registers.extend(to_add);

    to_remove
}

fn destruct_local_structs(
    functions: &HashMap<InternedString, FunctionDefinition>,
    mut structs: HashMap<InternedString, Vec<NamedType>>,
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

                        structs.insert(*variable_name, fields.clone());

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

                        let Some(fields) = structs.get(dest) else {
                            return vec![clone];
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
                                let fields = structs.get(ident).unwrap_or_else(|| {
                                    panic!("attempting to assign non struct value identifier {ident:?}")
                                });

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
                            _ => todo!(),
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
                                return vec![clone];
                            };

                            if let Type::Tuple(return_types) = &*def.signature.return_type.get() {
                                let Expression::Identifier(dest) = expression else {
                                    // only visiting each statement once so this should be true (for now, unions
                                    // break this)
                                    panic!();
                                };

                                let fields = structs.get(dest).unwrap();

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
                                    if let Some(fields) = structs.get(ident) {
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
            if let Some(fields) = structs.get(&return_value_ident) {
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
        fields.into_iter().map(|NamedType { typ, .. }| typ).collect(),
    ));
}

fn destructed_ident(local_variable_name: InternedString, field_name: InternedString) -> InternedString {
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

fn split_parameters(parameters: Shared<Vec<Parameter>>) -> HashMap<InternedString, Vec<NamedType>> {
    let mut removed = HashMap::default();

    let mut parameters = parameters.get_mut();
    *parameters = parameters
        .iter()
        .flat_map(|parameter| {
            if let Type::Struct {
                name: struct_name,
                fields,
            } = &*parameter.typ.get()
            {
                removed.insert(*struct_name, fields.clone());

                fields
                    .iter()
                    .map(|NamedType { name: field_name, typ }| Parameter {
                        name: destructed_ident(parameter.name, *field_name),
                        typ: typ.clone(),
                        is_ref: false,
                    })
                    .collect()
            } else {
                vec![parameter.clone()]
            }
        })
        .collect();

    removed
}
