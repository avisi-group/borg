//! Common error utilities

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

/// Wrapper around an error providing additional context (the path that caused the error).
#[derive(Debug, thiserror::Error)]
pub struct ErrCtx<E> {
    /// Inner error
    #[source]
    pub inner: E,
    /// Path associated with error
    pub path: PathBuf,
}

impl<E> Display for ErrCtx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

impl<E> ErrCtx<E> {
    /// Create a new `IoErrCtx` from an `io::Error` and a path
    pub fn new<P: AsRef<Path>>(inner: E, path: P) -> Self {
        Self {
            inner,
            path: path.as_ref().to_owned(),
        }
    }

    /// Convenience method that creates a closure which converts an `io::Error` to a `IoErrCtx`
    ///
    /// This is designed for simplifying the conversion of `io::Error`s using `.map_err`.
    ///
    /// For example
    ///
    /// ```
    /// # use {std::{io, path::PathBuf}, common::error::ErrCtx};
    /// #
    /// #   let res: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    /// #   let path = PathBuf::from("example");
    /// let res = res.map_err(|e| ErrCtx::new(e, path));
    /// #   assert_eq!(format!("{res:?}"), "Err(ErrCtx { inner: Custom { kind: Other, error: \"oh no!\" }, path: \"example\" })");
    /// ```
    ///
    /// can become
    ///
    /// ```
    /// # use {std::{io, path::PathBuf}, common::error::ErrCtx};
    /// #
    /// #   let res: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    /// #   let path = PathBuf::from("example");
    /// let res = res.map_err(ErrCtx::f(path));
    /// #   assert_eq!(format!("{res:?}"), "Err(ErrCtx { inner: Custom { kind: Other, error: \"oh no!\" }, path: \"example\" })");
    /// ```
    pub fn f<P: AsRef<Path>>(path: P) -> Box<dyn FnOnce(E) -> Self> {
        let p = path.as_ref().to_owned();
        Box::new(|e| Self::new(e, p))
    }
}
