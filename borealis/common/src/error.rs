//! Errors

use std::{
    io,
    path::{Path, PathBuf},
};

/// Path {path:?}
///
/// Wrapper around `std::io::Error` providing additional context (the path that caused the error).\
#[derive(Debug, displaydoc::Display, thiserror::Error)]
#[ignore_extra_doc_attributes]
pub struct IoErrCtx {
    #[source]
    inner: io::Error,
    path: PathBuf,
}

impl IoErrCtx {
    /// Create a new `IoErrCtx` from an `io::Error` and a path
    pub fn new<P: AsRef<Path>>(inner: io::Error, path: P) -> Self {
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
    /// # use {std::io, common::error::IoErrCtx};
    /// #
    /// # fn main() -> Result<(), IoErrCtx> {
    /// #    let mut input = String::new();
    /// #    let path = "demo.json";
    /// io::stdin().read_line(&mut input).map_err(|e| IoErrCtx::new(e, path))?;
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// can become
    ///
    /// ```
    /// # use {std::io, common::error::IoErrCtx};
    /// #
    /// # fn main() -> Result<(), IoErrCtx> {
    /// #    let mut input = String::new();
    /// #    let path = "demo.json";
    /// io::stdin().read_line(&mut input).map_err(IoErrCtx::f(path))?;
    /// #    Ok(())
    /// # }
    /// ```
    pub fn f<P: AsRef<Path>>(path: P) -> Box<dyn FnOnce(io::Error) -> Self> {
        let p = path.as_ref().to_owned();
        Box::new(|e| Self::new(e, p))
    }
}
