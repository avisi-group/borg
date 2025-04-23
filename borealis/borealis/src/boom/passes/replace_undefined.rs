//! Replace undefined values with some constant actual value (0 or equivalent)

use {
    crate::boom::{
        self, Ast, Bit, Expression, FunctionDefinition, Literal, NamedType, NamedValue, Size,
        Statement, Type, Value,
        passes::{Pass, any::AnyExt},
    },
    common::id::Id,
    num_bigint::BigInt,
    sailrs::shared::Shared,
};

#[derive(Debug, Default)]
pub struct ReplaceUndefined;

impl ReplaceUndefined {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for ReplaceUndefined {
    fn name(&self) -> &'static str {
        "ReplaceUndefined"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get()
            .functions
            .values()
            .map(|def| replace_undefined(def))
            .any()
    }
}

fn replace_undefined(def: &FunctionDefinition) -> bool {
    def.entry_block.iter().for_each(|b| {
        if let boom::control_flow::Terminator::Return(Some(value)) = b.terminator() {
            if let Value::Literal(lit) = value {
                if let Literal::Undefined = *lit.get() {
                    let return_type = &*def.signature.return_type.as_ref().unwrap().get();

                    let value = Shared::new(make_value_from_type(return_type));

                    let return_value_ident = format!("undefined_return_{}", Id::new()).into();

                    let mut statements = b.statements();
                    statements.push(Shared::new(Statement::VariableDeclaration {
                        name: return_value_ident,
                        typ: Shared::new(return_type.clone()),
                    }));
                    statements.push(Shared::new(Statement::Copy {
                        expression: Expression::Identifier(return_value_ident),
                        value,
                    }));
                    b.set_statements(statements);

                    b.set_terminator(boom::control_flow::Terminator::Return(Some(
                        Value::Identifier(return_value_ident),
                    )));
                }
            }
        }
    });

    // oneshot
    false
}

fn make_value_from_type(typ: &Type) -> Value {
    match typ {
        Type::Struct { name, fields } => Value::Struct {
            name: *name,
            fields: fields
                .iter()
                .map(|NamedType { name, typ }| NamedValue {
                    name: *name,
                    value: Shared::new(make_value_from_type(&*typ.get())),
                })
                .collect(),
        },
        Type::Union { .. } => todo!(),
        Type::Tuple(types) => Value::Tuple(
            types
                .iter()
                .map(|typ| make_value_from_type(&*typ.get()))
                .map(Shared::new)
                .collect(),
        ),
        _ => Value::Literal(Shared::new(make_literal_from_type(typ))),
    }
}

fn make_literal_from_type(typ: &Type) -> Literal {
    match typ {
        Type::Unit => Literal::Unit,
        Type::String => todo!(),
        Type::Bool => Literal::Bool(false),
        Type::Bit => todo!(),
        Type::Real => todo!(),
        Type::Float => todo!(),
        Type::Integer { .. } => Literal::Int(BigInt::ZERO),
        Type::Bits {
            size: Size::Unknown,
        } => Literal::Bits(vec![Bit::Zero; 64]),
        Type::Bits {
            size: Size::Static(size),
        } => Literal::Bits(vec![Bit::Zero; *size]),
        Type::Constant(_) => todo!(),

        Type::Vector { .. } => todo!(),
        Type::FixedVector { .. } => todo!(),
        Type::Reference(_) => todo!(),
        Type::Union { .. } | Type::Struct { .. } | Type::Tuple(_) => unreachable!(),
    }
}
