//! Error handling for Sail interface

use {
    crate::{
        ast::{Position, L},
        runtime::Request,
    },
    ocaml::{CamlError, FromValue, Int, Value},
    std::{
        fmt::Debug,
        sync::mpsc::{RecvError, SendError},
    },
};

/// Sail error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Error returned by OCaml function
    OCamlFunction(ocaml::Error),

    /// Error when communicating with runtime worker thread
    RuntimeCommunication(#[from] ChannelError),

    /// Error returned by Sail wrapper function
    Wrapper(#[from] WrapperError),
}

// required as ocaml::Error can contain raw pointers
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<ocaml::Error> for Error {
    fn from(e: ocaml::Error) -> Self {
        if let ocaml::Error::Caml(CamlError::Exception(val)) = &e {
            // unsafe if `from` is called outside of the worker thread
            let msg = unsafe { val.exception_to_string() }.unwrap_or_else(|e| {
                format!(
                    "Failed to convert exception to string due to UTF-8 error: {}",
                    e
                )
            });

            Self::OCamlFunction(ocaml::Error::Error(msg.into()))
        } else {
            Self::OCamlFunction(e)
        }
    }
}

/// MPSC channel error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum ChannelError {
    /// Sending failed
    Send(#[from] SendError<Request>),
    /// Receiving failed
    Receive(#[from] RecvError),
}

impl From<SendError<Request>> for Error {
    fn from(e: SendError<Request>) -> Self {
        Error::RuntimeCommunication(e.into())
    }
}

impl From<RecvError> for Error {
    fn from(e: RecvError) -> Self {
        Error::RuntimeCommunication(e.into())
    }
}

/// Error from OCaml wrapper function around Sail library functions
#[derive(Debug, displaydoc::Display, thiserror::Error, FromValue)]
pub enum WrapperError {
    /// Exception occurred: "{message}" with backtrace: "{backtrace}"
    Exception {
        /// Exception message
        message: String,
        /// Recorded backtrace
        backtrace: String,
    },
    /// Sail error: {0:?}
    Sail(SailError),
}

/// Sail library error
#[derive(Debug, displaydoc::Display, thiserror::Error, FromValue)]
pub enum SailError {
    /// General error
    General(L, OCamlString),
    /// Unreachable error
    Unreachable(L, (OCamlString, Int, Int, Int, Int), Value, OCamlString),
    /// Todo error
    Todo(L, OCamlString),
    /// Syntax error
    Syntax(Position, OCamlString),
    /// Syntax location error
    SyntaxLocation(L, OCamlString),
    /// Lexical error
    Lexical(Position, OCamlString),
    /// Type error
    Type(L, OCamlString),
}

/// OCaml strings are byte arrays and may contain valid UTF-8 contents or arbitrary bytes
///
/// When converting from Value will attempt to parse as a `String`, falling back to `Vec<u8>` on error
#[derive(Debug, Clone)]
pub enum OCamlString {
    String(String),
    Vec(Vec<u8>),
}

unsafe impl FromValue for OCamlString {
    fn from_value(v: Value) -> Self {
        let vec = Vec::<u8>::from_value(v);
        match String::from_utf8(vec.clone()) {
            Ok(s) => Self::String(s),
            Err(_) => Self::Vec(vec),
        }
    }
}
