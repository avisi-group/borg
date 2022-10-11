//! Error handling for SAIL parser

use {
    ariadne::{Color, Fmt, Label, Report, ReportKind},
    std::{fmt::Debug, hash::Hash, ops::Range},
};

/// Parse error
#[derive(Debug)]
pub struct Error();

impl Error {
    /// Create a diagnostics `ariadne::Report` from an Error
    pub fn into_report(&self) -> Report {
        let e = &self.0;

        let msg = format!(
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
        );

        let report = Report::build(ReportKind::Error, (), e.span().start)
            .with_code(3)
            .with_message(msg)
            .with_label(
                Label::new(e.span())
                    .with_message(match e.reason() {
                        _ => format!(
                            "Unexpected {}",
                            e.found()
                                .map(|c| format!("token {}", format!("{c:?}").fg(Color::Red)))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                    })
                    .with_color(Color::Red),
            );

        report.finish()
    }
}
