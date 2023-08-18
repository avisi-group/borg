#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    crate::{
        boom::control_flow::ControlFlowBlock,
        codegen::{
            functions::generate_fns,
            instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
        },
        genc_model::{Bank, Description, Instruction, RegisterSpace, Slot, View},
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
    // uncomment me to dump the JIB AST to stdout
    // sail::jib_ast::pretty_print::print_ast(jib_ast);
    // panic!();

    info!("Generating BOOM from JIB");
    let ast = boom::Ast::from_jib(jib_ast);

    // filter out non addsub functions
    let funcs = ast
        .borrow()
        .functions
        .clone()
        .into_iter()
        .map(|(k, mut def)| {
            // if it's not an allowlisted function, delete the body
            if ![
                "integer_arithmetic_addsub_immediate_decode",
                "integer_arithmetic_addsub_immediate",
                "id",
                "AddWithCarry",
            ]
            .contains(&k.as_ref())
            {
                def.entry_block = ControlFlowBlock::new();
            }

            (k, def)
        })
        .collect();
    ast.borrow_mut().functions = funcs;

    info!("Running passes on BOOM");
    execute_passes(ast.clone());

    // let mut buf = String::new();
    // crate::boom::pretty_print::print_ast(&mut buf, ast.clone());
    // write!(&mut File::create("target/ast.boom").unwrap(), "{buf}").unwrap();

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
        .unzip::<_, _, _, _>();

    // generate all functions, using the names of the instructions as the entrypoint
    let mut functions = generate_fns(ast.clone(), instruction_names);

    // generate register spaces
    let _registers = 0;

    // create empty GenC description
    let mut description = Description::empty();

    description.helpers.append(&mut functions);

    description.instructions = instructions;

    description.registers = vec![
        RegisterSpace {
            size: 256,
            views: vec![View::Bank(Bank {
                name: "reg_RB".into(),
                typ: genc_model::Typ::Uint64,
                offset: 0,
                count: 31,
                stride: 8,
                element_count: 1,
                element_size: 8,
                element_stride: 8,
            })],
        },
        RegisterSpace {
            size: 8,
            views: vec![View::Slot(Slot {
                name: "reg_PC".into(),
                typ: genc_model::Typ::Uint64,
                width: 8,
                offset: 0,
                tag: Some("PC".into()),
            })],
        },
        RegisterSpace {
            size: 4,
            views: vec![
                View::Slot(Slot {
                    name: "reg_N".into(),
                    typ: genc_model::Typ::Uint8,
                    width: 1,
                    offset: 0,
                    tag: Some("N".into()),
                }),
                View::Slot(Slot {
                    name: "reg_Z".into(),
                    typ: genc_model::Typ::Uint8,
                    width: 1,
                    offset: 1,
                    tag: Some("Z".into()),
                }),
                View::Slot(Slot {
                    name: "reg_C".into(),
                    typ: genc_model::Typ::Uint8,
                    width: 1,
                    offset: 2,
                    tag: Some("C".into()),
                }),
                View::Slot(Slot {
                    name: "reg_V".into(),
                    typ: genc_model::Typ::Uint8,
                    width: 1,
                    offset: 3,
                    tag: Some("V".into()),
                }),
            ],
        },
    ];

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
