use {
    common::{
        arena::Ref,
        intern::InternedString,
        rudder::{
            block::Block,
            constant_value::ConstantValue,
            function::Function,
            statement::Statement,
            types::{PrimitiveType, PrimitiveTypeClass, Type},
            Model,
        },
    },
    std::fmt::Display,
};

pub enum Severity {
    Error,
    Warning,
    Note,
}

pub enum Scope {
    FunctionLevel(InternedString),
    BlockLevel(InternedString, InternedString),
    StatementLevel(InternedString, InternedString, InternedString),
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
            Scope::FunctionLevel(f) => format!("{f}"),
            Scope::BlockLevel(f, b) => format!("{f} {b}"),
            Scope::StatementLevel(f, b, s) => format!("{f} {b} {s}"),
        };

        write!(f, "{severity}: {scope}: {}", self.2)
    }
}

impl ValidationMessage {
    pub fn stmt_msg<T: ToString>(
        f: InternedString,
        b: InternedString,
        s: InternedString,
        v: Severity,
        m: T,
    ) -> Self {
        Self(v, Scope::StatementLevel(f, b, s), m.to_string())
    }

    pub fn stmt_warn<T: ToString>(
        f: InternedString,
        b: InternedString,
        s: InternedString,
        m: T,
    ) -> Self {
        Self::stmt_msg(f, b, s, Severity::Warning, m)
    }

    pub fn stmt_err<T: ToString>(
        f: InternedString,
        b: InternedString,
        s: InternedString,
        m: T,
    ) -> Self {
        Self::stmt_msg(f, b, s, Severity::Error, m)
    }
}

pub fn validate(ctx: &Model) -> Vec<ValidationMessage> {
    let messages = [check_constant_value_types(ctx), check_operand_types(ctx)];

    messages.into_iter().flatten().collect()
}

fn check_constant_value_types(ctx: &Model) -> Vec<ValidationMessage> {
    // iterate over every statement in every function, passing
    ctx.functions()
        .values()
        .map(|f| f.block_iter().map(move |b| (f, b)))
        .flatten()
        .map(|(f, b)| {
            b.get(f.arena())
                .statements()
                .into_iter()
                .map(move |s| ((b, f), *s))
        })
        .flatten()
        .filter_map(|((b, f), s)| {
            if let Statement::Constant { typ, value } = s.get(b.get(f.arena()).arena()) {
                Some(((s, b, f), (typ.clone(), value.clone())))
            } else {
                None
            }
        })
        .filter_map(validate_constant_type)
        .collect()
}

fn check_operand_types(ctx: &Model) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();

    for (_, f) in ctx.functions() {
        for b in f.block_iter() {
            let s_arena = b.get(f.arena()).arena();

            for s in b.get(f.arena()).statements() {
                if let Statement::BinaryOperation { lhs, rhs, .. } = s.get(s_arena) {
                    if !lhs
                        .get(s_arena)
                        .typ(s_arena)
                        .is_compatible_with(&rhs.get(s_arena).typ(s_arena))
                    {
                        messages.push(ValidationMessage::stmt_err(
                            f.name(),
                            format!("{}", b.index()).into(),
                            s.to_string(s_arena).into(),
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
    ((stmt, block, f), (typ, value)): (
        (Ref<Statement>, Ref<Block>, &Function),
        (Type, ConstantValue),
    ),
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
            f.name(),
            format!("{}", block.index()).into(),
            stmt.to_string(&block.get(f.arena()).arena()).into(),
            format!("cannot use {typ:?} type for {value:?}"),
        )),
    }
}
