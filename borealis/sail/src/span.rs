use std::ops::Range;

pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl chumsky::Span for Span {
    type Context = ();

    type Offset = ();

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        todo!()
    }

    fn context(&self) -> Self::Context {
        todo!()
    }

    fn start(&self) -> Self::Offset {
        todo!()
    }

    fn end(&self) -> Self::Offset {
        todo!()
    }
}
