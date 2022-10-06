//! Sail2 language parser

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod span;

pub use crate::{lexer::lexer, parser::parser};
