use {
    crate::rudder::{
        statement::StatementKind, Block, ConstantValue, Context, Function, PrimitiveType,
        PrimitiveTypeClass, Statement, Type,
    },
    std::{fmt::Display, sync::Arc},
};

pub enum Severity {
    Error,
    Warning,
    Note,
}

pub enum Scope {
    FunctionLevel(Function),
    BlockLevel(Function, Block),
    StatementLevel(Function, Block, Statement),
}

pub struct ValidationMessage(Severity, Scope, String);

impl Display for ValidationMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.0 {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
            Severity::Note => "NOTE",
        };

        let scope = match &self.1 {
            Scope::FunctionLevel(f) => format!("{}", f.name()),
            Scope::BlockLevel(f, b) => format!("{} {}", f.name(), b.index()),
            Scope::StatementLevel(f, b, s) => format!("{} {} {}", f.name(), b.index(), s.name()),
        };

        write!(f, "{severity}: {scope}: {}", self.2)
    }
}

impl ValidationMessage {
    pub fn stmt_msg<T: ToString>(
        f: &Function,
        b: &Block,
        s: &Statement,
        v: Severity,
        m: T,
    ) -> Self {
        Self(
            v,
            Scope::StatementLevel(f.clone(), b.clone(), s.clone()),
            m.to_string(),
        )
    }

    pub fn stmt_warn<T: ToString>(f: &Function, b: &Block, s: &Statement, m: T) -> Self {
        Self::stmt_msg(f, b, s, Severity::Warning, m)
    }

    pub fn stmt_err<T: ToString>(f: &Function, b: &Block, s: &Statement, m: T) -> Self {
        Self::stmt_msg(f, b, s, Severity::Error, m)
    }
}

pub fn validate(ctx: &Context) -> Vec<ValidationMessage> {
    let messages = [check_constant_value_types(ctx), check_operand_types(ctx)];

    messages.into_iter().flatten().collect()
}

fn check_constant_value_types(ctx: &Context) -> Vec<ValidationMessage> {
    // iterate over every statement in every function, passing
    ctx.get_functions()
        .values()
        .map(|f| f.entry_block().iter().map(|b| (f.clone(), b)))
        .flatten()
        .map(|(f, b)| {
            b.clone()
                .statements()
                .into_iter()
                .map(move |s| ((b.clone(), f.clone()), s))
        })
        .flatten()
        .filter_map(|((b, f), s)| {
            if let StatementKind::Constant { typ, value } = s.kind() {
                Some(((s, b, f), (typ, value)))
            } else {
                None
            }
        })
        .filter_map(validate_constant_type)
        .collect()
}

fn check_operand_types(ctx: &Context) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();

    for (_, f) in ctx.get_functions() {
        for b in f.entry_block().iter() {
            for s in b.statements() {
                if let StatementKind::BinaryOperation { lhs, rhs, .. } = s.kind() {
                    if !lhs.typ().is_compatible_with(&rhs.typ()) {
                        messages.push(ValidationMessage::stmt_err(
                            &f,
                            &b,
                            &s,
                            "incompatible operand types in binary operation",
                        ));
                    }
                }
            }
        }
    }

    messages
}

fn validate_constant_type(
    ((stmt, block, f), (typ, value)): ((Statement, Block, Function), (Arc<Type>, ConstantValue)),
) -> Option<ValidationMessage> {
    match (&value, &*typ) {
        (
            ConstantValue::UnsignedInteger(_),
            Type::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::UnsignedInteger,
                ..
            }),
        )
        | (
            ConstantValue::SignedInteger(_),
            Type::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::SignedInteger,
                ..
            }),
        )
        | (
            ConstantValue::FloatingPoint(_),
            Type::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::FloatingPoint,
                ..
            }),
        )
        | (
            ConstantValue::Unit,
            Type::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::Unit,
                ..
            }),
        )
        | (ConstantValue::String(_), Type::String)
        | (ConstantValue::Rational(_), Type::Rational)
        | (ConstantValue::Tuple(_), Type::Tuple(_)) => None,

        _ => Some(ValidationMessage::stmt_warn(
            &f,
            &block,
            &stmt,
            format!("cannot use {typ:?} type for {value:?}"),
        )),
    }
}
