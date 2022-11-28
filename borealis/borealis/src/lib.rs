#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    common::error::ErrCtx,
    std::{io, path::PathBuf},
};

pub mod genc;

pub use sail;

/// Borealis error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// IO error
    Io(#[from] ErrCtx<io::Error>),
    /// Error from Sail compiler
    Sail(#[from] sail::error::Error),
    /// GenC export directory {0:?} not found
    OutDirectoryNotFound(PathBuf),
    /// GenC export directory {0:?} not empty
    OutDirectoryNotEmpty(PathBuf),
}
