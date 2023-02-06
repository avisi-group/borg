#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    crate::{
        genc::{Description, Instruction},
        instruction::{get_instructions, process_instruction},
    },
    errctx::PathCtx,
    sail::ast::Ast,
    std::{io, path::PathBuf},
};

pub mod genc;
pub mod instruction;

/// Borealis error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// IO error
    Io(#[from] PathCtx<io::Error>),
    /// Error from Sail compiler
    Sail(#[from] sail::error::Error),
    /// GenC export directory {0:?} not found
    OutDirectoryNotFound(PathBuf),
    /// GenC export directory {0:?} not empty
    OutDirectoryNotEmpty(PathBuf),
}

/// Compiles a Sail ISA specification to a GenC description
pub fn sail_to_genc(ast: &Ast) -> Description {
    let instructions = get_instructions(ast);

    let mut description = Description::empty();

    description.instructions = instructions
        .into_iter()
        .map(|clause| process_instruction(ast, &clause))
        .map(|(name, format, execute)| (name.to_string(), Instruction { format, execute }))
        .collect();

    description
}
