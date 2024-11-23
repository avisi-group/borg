use crate::boom::control_flow::Terminator;
use itertools::Itertools;
use std::fmt;
use std::fmt::Display;

use {
    crate::boom::{
        control_flow::ControlFlowBlock, passes::Pass, Ast, Expression, Literal, NamedType,
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

/// Either a destination or a source location
#[derive(Debug, Clone)]
enum DataLocation {
    Identifier(InternedString),
    Fields {
        root: InternedString,
        fields: Vec<InternedString>,
    },
}

impl Display for DataLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{ident}"),
            Self::Fields { root, fields } => {
                write!(f, "{root}.{}", fields.iter().join("."))
            }
        }
    }
}

impl DataLocation {
    pub fn root(&self) -> InternedString {
        match self {
            DataLocation::Identifier(root) | DataLocation::Fields { root, .. } => *root,
        }
    }

    pub fn to_ident(&self) -> InternedString {
        match self {
            DataLocation::Identifier(ident) => *ident,
            DataLocation::Fields { root, fields } => {
                format!("{root}_{}", fields.iter().join("_")).into()
            }
        }
    }

    pub fn try_from(value: Shared<Value>) -> Option<Self> {
        fn build_fields(value: Shared<Value>) -> (InternedString, Vec<InternedString>) {
            let mut fields = vec![];

            let mut current_value = value.get().clone();
            loop {
                match current_value {
                    Value::Identifier(root) => {
                        fields.reverse();
                        return (root, fields);
                    }
                    Value::Field { value, field_name } => {
                        fields.push(field_name);
                        current_value = value.get().clone();
                    }
                    _ => panic!(),
                }
            }
        }

        match &*value.get() {
            Value::Identifier(ident) => Some(Self::Identifier(*ident)),
            Value::Field { .. } => {
                let (root, fields) = build_fields(value.clone());
                Some(Self::Fields { root, fields })
            }
            _ => None,
        }
    }
}

impl From<&Expression> for DataLocation {
    fn from(value: &Expression) -> Self {
        fn build_fields(expr: &Expression) -> (InternedString, Vec<InternedString>) {
            let mut fields = vec![];

            let mut current_expr = expr.clone();
            loop {
                match current_expr {
                    Expression::Identifier(root) => {
                        fields.reverse();
                        return (root, fields);
                    }
                    Expression::Field { expression, field } => {
                        fields.push(field);
                        current_expr = *expression;
                    }
                    _ => panic!(),
                }
            }
        }

        match value {
            Expression::Identifier(dst) => Self::Identifier(*dst),
            Expression::Field { .. } => {
                let (root, fields) = build_fields(value);
                Self::Fields { root, fields }
            }
            _ => panic!(),
        }
    }
}

impl Pass for DestructUnions {
    fn name(&self) -> &'static str {
        "DestructUnions"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let composites = {
            let mut map = HashMap::default();
            map.extend(ast.get().unions.iter().map(|(name, fields)| {
                (
                    *name,
                    Shared::new(Type::Union {
                        name: *name,
                        fields: fields.clone(),
                    }),
                )
            }));
            map.extend(ast.get().structs.iter().map(|(name, fields)| {
                (
                    *name,
                    Shared::new(Type::Struct {
                        name: *name,
                        fields: fields.clone(),
                    }),
                )
            }));
            map
        };

        let destructed_registers = destruct_registers(ast.clone());

        // function signatures need fixing first, then storing for lookup when handling function calls
        let destructed_return_type_by_function = destruct_function_return_types(ast.clone());
        let destructed_parameters_by_function = destruct_function_parameters(ast.clone());

        ast.get().functions.iter().for_each(|(name, def)| {
            destruct_locals(
                *name,
                &composites,
                &destructed_registers,
                &destructed_return_type_by_function,
                &destructed_parameters_by_function,
                def.entry_block.clone(),
            );
        });

        false
    }
}

/// split locally declared unions into a tag and a local variable the size of
/// the largest value?
fn destruct_locals(
    function_name: InternedString,
    composites: &HashMap<InternedString, Shared<Type>>,
    destructed_registers: &HashMap<InternedString, Shared<Type>>,
    destructed_return_types_by_function: &HashMap<InternedString, Shared<Type>>,
    destructed_parameters_by_function: &HashMap<
        InternedString,
        HashMap<InternedString, Shared<Type>>,
    >,
    entry_block: ControlFlowBlock,
) {
    let mut destructed_local_variables = destructed_parameters_by_function
        .get(&function_name)
        .cloned()
        .unwrap_or_default();

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
                        if is_type_composite(typ.clone()) {
                            destructed_local_variables.insert(*variable_name, typ.clone());
                            destruct_variable(*variable_name, typ.clone())
                                .into_iter()
                                .map(|(name, typ)| Statement::VariableDeclaration { name, typ })
                                .map(Shared::new)
                                .collect()
                        } else {
                            return vec![clone];
                        }
                    }
                    Statement::Copy { expression, value } => {
                        let destination = DataLocation::from(expression);

                        // if source is not a location, it won't be a struct identifier or field access so return early
                        let Some(source) = DataLocation::try_from(value.clone()) else {
                            return vec![Shared::new(Statement::Copy {
                                expression: Expression::Identifier(destination.to_ident()),
                                value: value.clone(),
                            })];
                        };

                        match (
                            destructed_local_variables
                                .get(&source.root())
                                .or_else(|| destructed_registers.get(&source.root())),
                            destructed_local_variables
                                .get(&destination.root())
                                .or_else(|| destructed_registers.get(&destination.root())),
                        ) {
                            (Some(src_root_typ), Some(dst_root_typ)) => {
                                let (src_outer_ident, src_outer_type) =
                                    traverse_typ_from_location(src_root_typ.clone(), &source);
                                let (dst_outer_ident, dst_outer_type) =
                                    traverse_typ_from_location(dst_root_typ.clone(), &destination);

                                destruct_variable(src_outer_ident, src_outer_type)
                                    .into_iter()
                                    .zip(
                                        destruct_variable(dst_outer_ident, dst_outer_type)
                                            .into_iter(),
                                    )
                                    .map(|((src, _), (dst, _))| Statement::Copy {
                                        expression: Expression::Identifier(dst),
                                        value: Shared::new(Value::Identifier(src)),
                                    })
                                    .map(Shared::new)
                                    .collect()
                            }

                            (Some(src_root_typ), None) => {
                                let (src_outer_ident, _) =
                                    traverse_typ_from_location(src_root_typ.clone(), &source);

                                let DataLocation::Identifier(dst) = destination else {
                                    panic!();
                                };

                                vec![Shared::new(Statement::Copy {
                                    expression: Expression::Identifier(dst),
                                    value: Shared::new(Value::Identifier(src_outer_ident)),
                                })]
                            }
                            (None, Some(dst_root_typ)) => {
                                let (dst_outer_ident, _) =
                                    traverse_typ_from_location(dst_root_typ.clone(), &destination);

                                let DataLocation::Identifier(src) = source else {
                                    panic!();
                                };

                                vec![Shared::new(Statement::Copy {
                                    expression: Expression::Identifier(dst_outer_ident),
                                    value: Shared::new(Value::Identifier(src)),
                                })]
                            }
                            (None, None) => vec![clone],
                        }
                    }

                    Statement::FunctionCall {
                        expression,
                        name,
                        arguments,
                    } => {
                        // function name is a union tag constructor
                        if let Some((fields, tag)) = is_union_constructor(*name, composites) {
                            create_union_construction_copies(expression, arguments, fields, tag)
                        } else {
                            // fix composite destination
                            let expression = match expression {
                                Some(Expression::Identifier(ident)) => {
                                    if let Some(typ) = destructed_return_types_by_function.get(name)
                                    {
                                        Some(Expression::Tuple(
                                            destruct_variable(*ident, typ.clone())
                                                .into_iter()
                                                .map(|(name, _)| Expression::Identifier(name))
                                                .collect(),
                                        ))
                                    } else {
                                        Some(Expression::Identifier(*ident))
                                    }
                                }
                                Some(Expression::Field { expression, field }) => {
                                    match &**expression {
                                        Expression::Identifier(root) => {
                                            Some(Expression::Identifier(struct_field_ident(
                                                *root, *field,
                                            )))
                                        }
                                        Expression::Field { .. } => panic!("too deep"),
                                        _ => panic!(),
                                    }
                                }
                                Some(e) => panic!("{e:?}"),
                                None => None,
                            };

                            let arguments = arguments
                                .iter()
                                .flat_map(|arg| {
                                    if let Value::Identifier(ident) = &*arg.get() {
                                        if let Some(typ) =
                                            destructed_local_variables.get(ident).or_else(|| {
                                                // unlikely but may as well check
                                                destructed_registers.get(ident)
                                            })
                                        {
                                            destruct_variable(*ident, typ.clone())
                                                .into_iter()
                                                .map(|(new_ident, _)| Value::Identifier(new_ident))
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

        if let Terminator::Return(Some(Value::Identifier(return_value_ident))) = block.terminator()
        {
            // done to distinguish structs of one element (converted to unary tuple) from a single return value
            if let Some(typ) = destructed_local_variables.get(&return_value_ident) {
                block.set_terminator(Terminator::Return(Some(Value::Tuple(
                    destruct_variable(return_value_ident, typ.clone())
                        .into_iter()
                        .map(|(name, _)| Value::Identifier(name))
                        .map(Shared::new)
                        .collect(),
                ))));
            }
        }
    });
}

fn create_union_construction_copies(
    expression: &Option<Expression>,
    arguments: &Vec<Shared<Value>>,
    fields: Vec<NamedType>,
    tag: usize,
) -> Vec<Shared<Statement>> {
    if let Some(Expression::Identifier(ident)) = expression {
        assert_eq!(1, arguments.len());

        fields
            .iter()
            .enumerate()
            .map(|(i, nt)| Statement::Copy {
                expression: Expression::Identifier(union_value_ident(*ident, nt.name)),
                value: if i == tag {
                    arguments[0].clone()
                } else {
                    default_value(nt.typ.clone())
                },
            })
            .chain([Statement::Copy {
                expression: Expression::Identifier(union_tag_ident(*ident)),
                value: Shared::new(Value::Literal(Shared::new(Literal::Int(tag.into())))),
            }])
            .map(Shared::new)
            .collect()
    } else {
        panic!();
    }
}

/// Destructure a composite variable (name and type combination) into primitive types with unique identifiers
fn destruct_variable(
    root_name: InternedString,
    typ: Shared<Type>,
) -> Vec<(InternedString, Shared<Type>)> {
    match &*typ.get() {
        Type::Struct { fields, .. } => fields
            .iter()
            .map(
                |NamedType {
                     name: field_name,
                     typ,
                 }| (struct_field_ident(root_name, *field_name), typ.clone()),
            )
            .collect::<Vec<_>>(),
        Type::Union { fields, .. } => {
            // create a tag variable
            [(
                union_tag_ident(root_name),
                Shared::new(Type::Integer {
                    size: Size::Static(32),
                }),
            )]
            .into_iter()
            .chain(
                // and value variables for each variant
                fields
                    .iter()
                    .enumerate()
                    .map(|(_tag, NamedType { name, typ })| {
                        (union_value_ident(root_name, *name), typ.clone())
                    }),
            )
            .collect()
        }
        _ => vec![(root_name, typ.clone())],
    }
    .into_iter()
    .map(|(name, typ)| {
        if is_type_composite(typ.clone()) {
            destruct_variable(name, typ.clone())
        } else {
            vec![(name, typ)]
        }
    })
    .flatten()
    .collect()
}

/// Identifier for a union's tag
fn union_tag_ident(ident: InternedString) -> InternedString {
    format!("{ident}_tag").into()
}

/// Identifier for a union's values
fn union_value_ident(root: InternedString, variant: InternedString) -> InternedString {
    format!("{root}_{variant}").into()
}

/// Identifier for a struct's values
fn struct_field_ident(root: InternedString, field: InternedString) -> InternedString {
    format!("{root}_{field}").into()
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
        Type::Bits { size } => Shared::new(Value::Literal(Shared::new(Literal::Bits(vec![
            0.into(
            );
            match size {
                Size::Static(n) => *n,
                Size::Unknown => 1,
            }
        ])))),
        t => todo!("{t:?}"),
    }
}

/// If the supplied identifier is a constructor for a union variant, return the fields of that union and the tag of that variant
fn is_union_constructor(
    ident: InternedString,
    composites: &HashMap<InternedString, Shared<Type>>,
) -> Option<(Vec<NamedType>, usize)> {
    composites
        .values()
        .filter_map(|c| {
            if let Type::Union { fields, .. } = &*c.get() {
                Some(fields.clone())
            } else {
                None
            }
        })
        .flat_map(|nts| {
            nts.clone()
                .into_iter()
                .enumerate()
                .map(move |(i, nt)| (nts.clone(), i, nt))
        })
        .find(|(_, _, nt)| nt.name == ident)
        .map(|(nts, tag, _)| (nts, tag))
}

fn is_type_composite(typ: Shared<Type>) -> bool {
    matches!(&*typ.get(), Type::Union { .. } | Type::Struct { .. })
}

fn destruct_function_parameters(
    ast: Shared<Ast>,
) -> HashMap<InternedString, HashMap<InternedString, Shared<Type>>> {
    ast.get_mut()
        .functions
        .iter_mut()
        // // filter out functions with no composite parameters
        // .filter(|(_, def)| {
        //     def.signature
        //         .parameters
        //         .get()
        //         .iter()
        //         .any(|p| is_type_composite(p.typ.clone()).is_some())
        // })
        .map(|(name, def)| {
            let mut destructed_parameters = HashMap::default();

            let new_parameters = def
                .signature
                .parameters
                .get()
                .clone()
                .into_iter()
                .map(|p| (p.name, p.typ.clone(), is_type_composite(p.typ)))
                .map(|(name, typ, is_composite)| {
                    if is_composite {
                        destructed_parameters.insert(name, typ.clone());
                        destruct_variable(name, typ)
                    } else {
                        vec![(name, typ)]
                    }
                })
                .flatten()
                .map(|(name, typ)| Parameter { name, typ })
                .collect::<Vec<_>>();

            *def.signature.parameters.get_mut() = new_parameters;

            //  = Some(Shared::new(Type::Tuple(
            // destruct_variable("".into(), &kind)
            //     .into_iter()
            //     .map(|(_, typ)| typ)
            //     .collect(),

            (*name, destructed_parameters)
        })
        .collect::<HashMap<_, _>>()
}

fn destruct_function_return_types(ast: Shared<Ast>) -> HashMap<InternedString, Shared<Type>> {
    ast.get_mut()
        .functions
        .iter_mut()
        .filter(|(_, def)| {
            matches!(
                def.signature
                    .return_type
                    .as_ref()
                    .map(|t| is_type_composite(t.clone())),
                Some(true)
            )
        })
        .map(|(name, def)| {
            let typ = def.signature.return_type.clone().unwrap();

            def.signature.return_type = Some(Shared::new(Type::Tuple(
                destruct_variable("return".into(), typ.clone())
                    .into_iter()
                    .map(|(_, typ)| typ)
                    .collect(),
            )));

            (*name, typ)
        })
        .collect::<HashMap<_, _>>()
}

fn destruct_registers(ast: Shared<Ast>) -> HashMap<InternedString, Shared<Type>> {
    let union_regs = ast
        .get_mut()
        .registers
        .iter()
        .filter(|(_, typ)| is_type_composite((*typ).clone()))
        .map(|(name, typ)| (*name, typ.clone()))
        .collect::<HashMap<_, _>>();

    for (register_name, typ) in &union_regs {
        ast.get_mut().registers.remove(register_name);

        ast.get_mut()
            .registers
            .extend(destruct_variable(*register_name, typ.clone()));
    }

    union_regs
}

/// Determine the identifer and type given a root typ and a data location
///
/// todo: explain that better
fn traverse_typ_from_location(
    typ: Shared<Type>,
    location: &DataLocation,
) -> (InternedString, Shared<Type>) {
    match location {
        // data location is of type Kind
        DataLocation::Identifier(name) => (*name, typ),
        //
        DataLocation::Fields { root, fields } => {
            let mut current_type = typ.get().clone();
            let mut current_identifier = root.clone();

            for field in fields {
                match current_type.clone() {
                    Type::Union { fields: _, .. } => {
                        // let next = fields.iter().find(|nt| nt.name == *field).unwrap();
                        // current_type = next.typ.get().clone();
                        // current_identifier = union_value_ident(current_identifier, *field);
                        todo!();
                    }

                    Type::Struct { fields, .. } => {
                        let next = fields.iter().find(|nt| nt.name == *field).unwrap();
                        current_type = next.typ.get().clone();
                        current_identifier = struct_field_ident(current_identifier, *field);
                    }

                    _ => panic!(),
                }
            }

            (current_identifier, Shared::new(current_type))
        }
    }
}

// fn destruct_expression_destination(
//     expression: &Expression,
//     destructed_registers: &HashMap<InternedString, Kind>,
//     destructed_variables: &HashMap<InternedString, Kind>,
// ) -> Expression {
//     match expression {
//         Expression::Identifier(ident) => {
//             if let Some(fields) = destructed_registers
//                 .get(ident)
//                 .or_else(|| destructed_variables.get(ident))
//             {
//                 Expression::Tuple(
//                     destruct_variable(*ident, fields)
//                         .into_iter()
//                         .map(|(ident, _)| Expression::Identifier(ident))
//                         .collect(),
//                 )
//             } else {
//                 Expression::Identifier(*ident)
//             }
//         }
//         Expression::Field { expression, field } => match &**expression {
//             Expression::Identifier(root) => {
//                 Expression::Identifier(struct_field_ident(*root, *field))
//             }
//             Expression::Field { expression, field } => todo!("gotta go deeper"),
//             _ => panic!(),
//         },
//         _ => panic!(),
//     }
// }
