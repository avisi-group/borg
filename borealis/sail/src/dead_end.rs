#[allow(deprecated)]
use chumsky::{
    debug::Debugger,
    error::{Error, Located},
    recovery::Strategy,
    stream::Stream,
};

/// Recovery strategy for unrecoverable inputs
pub(crate) struct DeadEnd;

#[allow(deprecated)]
impl<I: Clone + PartialEq, O, E: Error<I>> Strategy<I, O, E> for DeadEnd {
    fn recover<D: Debugger, P: chumsky::Parser<I, O, Error = E>>(
        &self,
        recovered_errors: Vec<Located<I, P::Error>>,
        fatal_error: Located<I, P::Error>,
        _parser: P,
        _debugger: &mut D,
        _stream: &mut Stream<'_, I, <P::Error as Error<I>>::Span>,
    ) -> (
        Vec<Located<I, P::Error>>,
        Result<(O, Option<Located<I, P::Error>>), Located<I, P::Error>>,
    ) {
        (recovered_errors, Err(fatal_error))
    }
}
