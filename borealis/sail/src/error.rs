//! Error handling for Sail interface

use ocaml::FromValue;

use {
    crate::runtime::Request,
    std::{
        fmt::Debug,
        sync::mpsc::{RecvError, SendError},
    },
};

/// Sail error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Error returned by OCaml function: {0:?}
    OCamlFunction(ocaml::Error),

    /// Error when communicating with runtime worker thread: {0}
    RuntimeCommunication(#[from] ChannelError),

    /// Error due to exception thrown in OCaml: {0}
    OCamlException(#[from] OCamlException),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<ocaml::Error> for Error {
    fn from(e: ocaml::Error) -> Self {
        Self::OCamlFunction(e)
    }
}

/// MPSC channel error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum ChannelError {
    /// Sending failed: {0}
    Send(#[from] SendError<Request>),
    /// Receiving failed: {0}
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

/// Exception in OCaml: {0:?}
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub struct OCamlException(String);

unsafe impl FromValue for OCamlException {
    fn from_value(v: ocaml::Value) -> Self {
        Self(String::from_value(v))
    }
}
