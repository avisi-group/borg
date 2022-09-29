use std::ops::Range;

pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub struct Span(Range<usize>);

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self(r)
    }
}
