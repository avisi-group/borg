#![warn(missing_docs)]

//! Sail frontend for GenSim

use {common::error::IoErrCtx, std::path::PathBuf};

pub mod genc;

pub use sail;

/// Borealis error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// IO error
    Io(#[from] IoErrCtx),
    /// Error from Sail compiler
    Sail(#[from] sail::error::Error),
    /// GenC export directory {0:?} not found
    OutDirectoryNotFound(PathBuf),
    /// GenC export directory {0:?} not empty
    OutDirectoryNotEmpty(PathBuf),
}
