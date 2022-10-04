//! Error handling for SAIL parser

use {
    crate::lexer::Token,
    ariadne::{Color, Fmt, Label, Report, ReportKind},
    chumsky::error::Simple,
    std::{fmt::Debug, hash::Hash, ops::Range},
};

/// Supertrait for input types allowed in lexer and parser
pub trait Input: Hash + Eq + Debug {}

impl Input for char {}

impl Input for Token {}

/// Parse error
#[derive(Debug)]
pub struct Error<I: Input>(pub Simple<I>);

impl<I: Input> Error<I> {
    /// Create a diagnostics `ariadne::Report` from an Error
    pub fn into_report(&self) -> Report {
        let e = &self.0;

        let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
            msg.clone()
        } else {
            format!(
                "{}{}, expected {}",
                if e.found().is_some() {
                    "Unexpected token"
                } else {
                    "Unexpected end of input"
                },
                if let Some(label) = e.label() {
                    format!(" while parsing {}", label)
                } else {
                    String::new()
                },
                if e.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    e.expected()
                        .map(|expected| match expected {
                            Some(expected) => format!("{expected:?}"),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            )
        };

        let report = Report::build(ReportKind::Error, (), e.span().start)
            .with_code(3)
            .with_message(msg)
            .with_label(
                Label::new(e.span())
                    .with_message(match e.reason() {
                        chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                        _ => format!(
                            "Unexpected {}",
                            e.found()
                                .map(|c| format!("token {}", format!("{c:?}").fg(Color::Red)))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                    })
                    .with_color(Color::Red),
            );

        let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                Label::new(span.clone())
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        format!("{delimiter:?}").fg(Color::Yellow)
                    ))
                    .with_color(Color::Yellow),
            ),
            chumsky::error::SimpleReason::Unexpected => report,
            chumsky::error::SimpleReason::Custom(_) => report,
        };

        report.finish()
    }
}

impl<I: Input> chumsky::Error<I> for Error<I> {
    type Span = Range<usize>;

    type Label = &'static str;

    fn expected_input_found<Iter: IntoIterator<Item = Option<I>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<I>,
    ) -> Self {
        Self(Simple::expected_input_found(span, expected, found))
    }

    fn with_label(self, label: Self::Label) -> Self {
        Self(self.0.with_label(label))
    }

    fn merge(self, other: Self) -> Self {
        Self(self.0.merge(other.0))
    }
}

impl<I: Input> Into<Simple<I>> for Error<I> {
    fn into(self) -> Simple<I> {
        self.0
    }
}
