//! Span type

use std::{ops::Range, path::PathBuf};

/// Type alias for tuple `(T, Span)`
pub type Spanned<T> = (T, Span);

/// Span over a Sail source file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    path: PathBuf,
    range: Range<usize>,
}
