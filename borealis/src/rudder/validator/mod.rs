use {
    common::{
        arena::Ref,
        intern::InternedString,
        rudder::{
            Model,
            block::Block,
            constant::Constant,
            function::Function,
            statement::Statement,
            types::{PrimitiveType, Type},
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
            if let Statement::Constant(value) = s.get(b.get(f.arena()).arena()) {
                Some(((s, b, f), (value.typ(), value.clone())))
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
                    if lhs.get(s_arena).typ(s_arena).unwrap()
                        != rhs.get(s_arena).typ(s_arena).unwrap()
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
    ((stmt, block, f), (typ, value)): ((Ref<Statement>, Ref<Block>, &Function), (Type, Constant)),
) -> Option<ValidationMessage> {
    match (&value, &typ) {
        (Constant::UnsignedInteger { .. }, Type::Primitive(PrimitiveType::UnsignedInteger(_)))
        | (Constant::SignedInteger { .. }, Type::Primitive(PrimitiveType::SignedInteger(_)))
        | (Constant::FloatingPoint { .. }, Type::Primitive(PrimitiveType::FloatingPoint(_)))
        | (Constant::String(_), Type::String)
        | (Constant::Tuple(_), Type::Tuple(_)) => None,

        _ => Some(ValidationMessage::stmt_warn(
            f.name(),
            format!("{}", block.index()).into(),
            stmt.to_string(&block.get(f.arena()).arena()).into(),
            format!("cannot use {typ:?} type for {value:?}"),
        )),
    }
}
