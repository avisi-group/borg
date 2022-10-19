//! Error handling for Sail interface

use {
    crate::runtime::Request,
    ocaml::{interop::BoxRoot, CamlError, FromValue, Int, Value},
    std::{
        fmt::{Debug, Formatter},
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
    General(L, String),
    /// Unreachable error
    Unreachable(L, (String, Int, Int, Int, Int), Value, String),
    /// Todo error
    Todo(L, String),
    /// Syntax error
    Syntax(Position, String),
    /// Syntax location error
    SyntaxLocation(L, String),
    /// Lexical error
    Lexical(Position, String),
    /// Type error
    Type(L, String),
}

/// Location
#[derive(Debug, FromValue)]
pub enum L {
    /// Unknown location
    Unknown,
    /// Unique location
    Unique(Int, Box<L>),
    /// Generated location
    Generated(Box<L>),
    /// Range between two positions
    Range(Position, Position),
    /// Documented location
    Documented(String, Box<L>),
}

/// Position in a source file
#[derive(Debug, FromValue)]
pub struct Position {
    /// File name
    pos_fname: String,
    /// Line number
    pos_lnum: Int,
    /// Character offset of beginning of line
    pos_bol: Int,
    /// Character offset of the position
    pos_cnum: Int,
}
