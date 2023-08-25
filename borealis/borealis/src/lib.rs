#![warn(missing_docs)]

//! Sail frontend for GenSim

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Statement},
        codegen::{
            functions::{generate_enums, generate_fns},
            instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
        },
        genc_model::{
            Bank, Behaviours, Description, Endianness, Instruction, RegisterSpace, Slot, Typ, View,
        },
        passes::execute_passes,
    },
    common::{intern::INTERNER, HashMap},
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{info, trace},
    lz4_flex::frame::FrameDecoder as Lz4Decoder,
    sail::{
        jib_ast::Definition, load_from_config, runtime::DEFAULT_RUNTIME_THREAD_STACK_SIZE,
        sail_ast::Ast,
    },
    std::{
        cell::RefCell,
        collections::LinkedList,
        ffi::OsStr,
        fs::File,
        io::{self, BufReader},
        path::{Path, PathBuf},
        rc::Rc,
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
                "IsZero",
                "GetSlice_int",
                "integer_logical_shiftedreg_decode",
                "DecodeShift",
                "integer_logical_shiftedreg",
                "ShiftReg",
                "branch_conditional_cond_decode",
                "branch_conditional_cond",
                "integer_insext_insert_movewide_decode",
                "integer_insext_insert_movewide",
                //"PostDecode",
                //"HaveBTIExt",
                //"HasArchVersion",
                //"BranchTargetCheck",
                //"ConditionHolds",
                //"BranchTo",
                //"AArch64_BranchAddr",
                //"UsingAArch32",
                //"IsZeroBit",
                //"integer_logical_immediate_decode",
                //"DecodeBitMasks",
                //"integer_logical_immediate",
            ]
            .contains(&k.as_ref())
            {
                def.entry_block = ControlFlowBlock::new();
                def.entry_block.set_statements(vec![Rc::new(RefCell::new(
                    Statement::FunctionCall {
                        expression: None,
                        name: "trap".into(),
                        arguments: vec![],
                    },
                ))])
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
    let functions = generate_fns(ast.clone(), instruction_names);

    let constants = {
        let mut constants = HashMap::default();
        let enum_constants = generate_enums(ast.clone());

        constants.extend(enum_constants);

        constants.insert("v85_implemented".into(), (Typ::Uint8, 1));
        constants.insert("v84_implemented".into(), (Typ::Uint8, 1));
        constants.insert("v83_implemented".into(), (Typ::Uint8, 1));
        constants.insert("v82_implemented".into(), (Typ::Uint8, 1));
        constants.insert("v81_implemented".into(), (Typ::Uint8, 1));

        constants
    };

    Description {
        name: "arm64".to_owned(),
        endianness: Endianness::LittleEndian,
        wordsize: 64,
        fetchsize: 32,
        predicated: false,
        registers: vec![
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
                size: 16,
                views: vec![
                    View::Slot(Slot {
                        name: "reg_PC".into(),
                        typ: genc_model::Typ::Uint64,
                        width: 8,
                        offset: 0,
                        tag: Some("PC".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_SP".into(),
                        typ: genc_model::Typ::Uint64,
                        width: 8,
                        offset: 8,
                        tag: Some("SP".into()),
                    }),
                ],
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
        ],
        instructions,
        behaviours: Behaviours {
            handle_exception: "".to_owned(),
            reset: "".to_owned(),
            irq: "".to_owned(),
            mmu_fault: "".to_owned(),
            page_fault: "".to_owned(),
            undefined_instruction: "".to_owned(),
            single_step: "".to_owned(),
            undef: "".to_owned(),
            custom: vec![],
        },
        helpers: functions,
        features: ["EMULATE_LINUX_ARCHSIM".into()].into_iter().collect(),
        constants,
    }
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
