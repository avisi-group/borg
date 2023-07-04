#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    crate::{
        codegen::{
            functions::generate_fns,
            instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
        },
        genc_model::{Description, Instruction},
        passes::execute_passes,
    },
    common::intern::INTERNER,
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{info, trace},
    lz4_flex::frame::FrameDecoder as Lz4Decoder,
    sail::{
        jib_ast::Definition, load_from_config, runtime::DEFAULT_RUNTIME_THREAD_STACK_SIZE,
        sail_ast::Ast,
    },
    std::{
        collections::LinkedList,
        ffi::OsStr,
        fs::File,
        io::{self, BufReader},
        path::{Path, PathBuf},
        thread,
    },
};

pub mod boom;
pub mod codegen;
pub mod genc_model;
pub mod passes;

/// Borealis error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Unrecognized format of input file {0:?}
    UnrecognizedFormat(PathBuf),
    /// Bincode deserialization failed
    Bincode(#[from] Box<bincode::ErrorKind>),
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

    info!("Generating BOOM from JIB");
    let ast = boom::Ast::from_jib(jib_ast);

    info!("Running passes on BOOM");
    execute_passes(ast.clone());

    // set up entrypoints in GenC execute behaviours
    let (instruction_names, instructions) = get_instruction_entrypoint_fns(sail_ast)
        .into_iter()
        .map(|clause| generate_execute_entrypoint(ast.clone(), &clause))
        .map(
            |(instruction_name, mangled_name, format, execute, disasm)| {
                (
                    instruction_name,
                    (
                        mangled_name.to_string(),
                        Instruction {
                            format,
                            execute,
                            disasm,
                        },
                    ),
                )
            },
        )
        .unzip();

    // generate all functions, using the names of the
    let mut functions = generate_fns(ast, instruction_names);

    // generate register spaces
    let _registers = 0;

    // create empty GenC description
    let mut description = Description::empty();

    description.helpers.append(&mut functions);

    description.instructions = instructions;

    description
}

/// Load Sail model AST and JIB from either a `sail.json` or compressed
/// bincode-serialised format, at the supplied path.
pub fn load_sail<P: AsRef<Path>>(path: P) -> Result<(Ast, LinkedList<Definition>), Error> {
    let path = path.as_ref();

    let (ast, jib) = match path.extension().and_then(OsStr::to_str) {
        Some("json") => {
            info!("Loading Sail config {:?}", path);
            load_from_config(path)?
        }
        Some("lz4") => {
            info!("Deserializing compressed bincode {:?}", path);
            deserialize_compressed_ast(path)?
        }
        _ => return Err(Error::UnrecognizedFormat(path.to_owned())),
    };

    trace!(
        "Size: AST {} bytes, JIB {} bytes",
        ast.deep_size_of(),
        jib.deep_size_of()
    );
    trace!(
        "INTERNER size: {} bytes, {} strings",
        INTERNER.current_memory_usage(),
        INTERNER.len()
    );

    Ok((ast, jib))
}

/// Deserializes an AST from a compressed bincode reader.
///
/// Internally, deserialization is performed on a new thread with a sufficient
/// stack size to perform the deserialization.
pub fn deserialize_compressed_ast<P: AsRef<Path>>(
    path: P,
) -> Result<(Ast, LinkedList<Definition>), Error> {
    let file_reader = BufReader::new(File::open(&path).map_err(PathCtx::f(&path))?);

    let thread = thread::Builder::new().stack_size(DEFAULT_RUNTIME_THREAD_STACK_SIZE);

    let handle = thread
        .spawn(move || bincode::deserialize_from(Lz4Decoder::new(file_reader)))
        .map_err(PathCtx::f(&path))?;

    let out = handle
        .join()
        .expect("Failed to join on deserializing thread")?;

    Ok(out)
}
