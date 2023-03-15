#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    crate::{
        genc::{Description, Instruction},
        instruction::{get_instructions, process_instruction},
    },
    color_eyre::{eyre::eyre, Result},
    errctx::PathCtx,
    lz4_flex::frame::FrameDecoder as Lz4Decoder,
    sail::{jib_ast::Definition, runtime::DEFAULT_RUNTIME_THREAD_STACK_SIZE, sail_ast::Ast},
    std::{collections::LinkedList, io, path::PathBuf, thread},
};

pub mod boom;
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
pub fn sail_to_genc(sail_ast: &Ast, jib_ast: &LinkedList<Definition>) -> Description {
    // crate::instruction::execute::pretty_print::print_ast(jib_ast);

    let instructions = get_instructions(sail_ast);

    let mut description = Description::empty();

    description.instructions = instructions
        .into_iter()
        .map(|clause| process_instruction(jib_ast, &clause))
        .map(|(name, format, execute)| (name.to_string(), Instruction { format, execute }))
        .collect();

    description
}

/// Deserializes an AST from a compressed bincode reader.
///
/// Internally, deserialization is performed on a new thread with a sufficient stack size to perform the deserialization.
pub fn deserialize_compressed_ast<R: io::Read + Send + 'static>(
    reader: R,
) -> Result<(Ast, LinkedList<Definition>)> {
    let thread = thread::Builder::new().stack_size(DEFAULT_RUNTIME_THREAD_STACK_SIZE);

    let handle = thread.spawn(move || bincode::deserialize_from(Lz4Decoder::new(reader)))?;

    let out = handle
        .join()
        .map_err(|_| eyre!("Failed to join aassociated thread"))??;

    Ok(out)
}
