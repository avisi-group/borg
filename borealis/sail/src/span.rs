use std::ops::Range;

pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone)]
pub struct Span(Range<usize>);

impl Span {
    pub fn into_inner(self) -> Range<usize> {
        self.0
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self(r)
    }
}
