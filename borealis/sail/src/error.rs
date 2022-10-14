//! Error handling for Sail interface

use std::fmt::Debug;

/// Sail error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Error returned by OCaml function interface: {0:?}
    Interface(ocaml::Error),
}

impl From<ocaml::Error> for Error {
    fn from(e: ocaml::Error) -> Self {
        Self::Interface(e)
    }
}
