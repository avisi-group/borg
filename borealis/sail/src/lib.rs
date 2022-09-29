//! SAIL language parser

mod dead_end;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod span;

pub use crate::lexer::lexer;
