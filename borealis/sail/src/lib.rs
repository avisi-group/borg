#![deny(missing_docs)]

//! SAIL language parser

pub mod error;

use {
    chumsky::{
        primitive::end,
        text::{self, TextParser},
        Parser,
    },
    error::Error,
};

/// Gets new instance of Sail parser
pub fn parser() -> impl Parser<char, Expr, Error = Error> {
    let int = text::int(10)
        .map(|s: String| Expr::Num(s.parse().unwrap()))
        .padded();

    int.then_ignore(end())
}

/// Parsed Sail program
#[derive(Debug)]
pub enum Expr {
    /// Numeric expression
    Num(f64),
}
