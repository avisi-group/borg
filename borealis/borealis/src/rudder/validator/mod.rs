use {
    crate::{
        rudder::{
            statement::StatementKind, Block, ConstantValue, Function, Model, PrimitiveType,
            PrimitiveTypeClass, Statement, Type,
        },
        util::arena::Ref,
    },
    std::fmt::Display,
};

pub enum Severity {
    Error,
    Warning,
    Note,
}

pub enum Scope<'f> {
    FunctionLevel(&'f Function),
    BlockLevel(&'f Function, Ref<Block>),
    StatementLevel(&'f Function, Ref<Block>, Statement),
}

pub struct ValidationMessage<'f>(Severity, Scope<'f>, String);

impl<'f> Display for ValidationMessage<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.0 {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
            Severity::Note => "NOTE",
        };

        let scope = match &self.1 {
            Scope::FunctionLevel(f) => format!("{}", f.name()),
            Scope::BlockLevel(f, b) => format!("{} {b:?}", f.name()), //todo: fix block ref debug printing to be the block index
            Scope::StatementLevel(f, b, s) => format!("{} {b:?} {}", f.name(), s.name()),
        };

        write!(f, "{severity}: {scope}: {}", self.2)
    }
}

impl<'f> ValidationMessage<'f> {
    pub fn stmt_msg<T: ToString>(
        f: &'f Function,
        b: Ref<Block>,
        s: &Statement,
        v: Severity,
        m: T,
    ) -> Self {
        Self(v, Scope::StatementLevel(f, b, s.clone()), m.to_string())
    }

    pub fn stmt_warn<T: ToString>(f: &'f Function, b: Ref<Block>, s: &Statement, m: T) -> Self {
        Self::stmt_msg(f, b, s, Severity::Warning, m)
    }

    pub fn stmt_err<T: ToString>(f: &'f Function, b: Ref<Block>, s: &Statement, m: T) -> Self {
        Self::stmt_msg(f, b, s, Severity::Error, m)
    }
}

pub fn validate(ctx: &Model) -> Vec<ValidationMessage> {
    let messages = [check_constant_value_types(ctx), check_operand_types(ctx)];

    messages.into_iter().flatten().collect()
}

fn check_constant_value_types(ctx: &Model) -> Vec<ValidationMessage> {
    // iterate over every statement in every function, passing
    ctx.get_functions()
        .values()
        .map(|f| f.block_iter().map(move |b| (f, b)))
        .flatten()
        .map(|(f, b)| {
            b.get(f.block_arena())
                .statements()
                .into_iter()
                .map(move |s| ((b, f), s))
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

fn check_operand_types(ctx: &Model) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();

    for (_, f) in ctx.get_functions() {
        for b in f.block_iter() {
            for s in b.get(f.block_arena()).statements() {
                if let StatementKind::BinaryOperation { lhs, rhs, .. } = s.kind() {
                    if !lhs.typ().is_compatible_with(&rhs.typ()) {
                        messages.push(ValidationMessage::stmt_err(
                            &f,
                            b,
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
    ((stmt, block, f), (typ, value)): ((Statement, Ref<Block>, &Function), (Type, ConstantValue)),
) -> Option<ValidationMessage> {
    match (&value, &typ) {
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
            block,
            &stmt,
            format!("cannot use {typ:?} type for {value:?}"),
        )),
    }
}
