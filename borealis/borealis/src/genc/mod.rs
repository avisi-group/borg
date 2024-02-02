//! GenC model filesystem representation
//!
//! `Description` is a GenC ISA description which can be used to generate it's
//! constituent files, the

use {
    crate::{
        genc::{
            files::{Execute, Files, Format, Function, Isa, Main},
            format::InstructionFormat,
        },
        Error,
    },
    common::{intern::InternedString, HashMap, HashSet},
    errctx::PathCtx,
    std::{
        fmt::Display,
        fs::{create_dir_all, File},
        io::Write as _,
        path::{Path, PathBuf},
    },
};

pub mod codegen;
mod files;
pub mod format;

const MAIN_FILENAME: &str = "main.genc";
const ISA_FILENAME: &str = "isa.genc";
const EXECUTE_FILENAME: &str = "execute.genc";
const BEHAVIOURS_FILENAME: &str = "behaviours.genc";

/// GenC description of an instruction set architecture
#[derive(Debug)]
pub struct Description {
    /// Name of the architecture (e.g. "arm64" or "riscv")
    pub name: String,

    /// Endianness of the architecture
    pub endianness: Endianness,

    /// Word size in bytes
    pub wordsize: u32,

    /// Instruction size in bytes
    pub fetchsize: u32,

    /// ?
    pub predicated: bool,

    /// Register definitions
    pub registers: Vec<RegisterSpace>,

    /// Instruction definitions
    pub instructions: HashMap<String, Instruction>,

    /// GenC behaviours
    pub behaviours: Behaviours,

    /// Helper functions
    pub helpers: Vec<HelperFunction>,

    /// AC features
    pub features: HashSet<String>,

    /// Constant values
    pub constants: HashMap<InternedString, (Typ, u64)>,
}

/// Export a GenC description to the supplied empty directory
pub fn export<P: AsRef<Path>>(description: &Description, path: P) -> Result<(), Error> {
    let path = path.as_ref();

    create_dir_all(path).map_err(PathCtx::f(path))?;

    let Files {
        main,
        isa,
        execute,
        behaviours,
    } = description.files();

    write_file(main, path.join(MAIN_FILENAME))?;
    write_file(isa, path.join(ISA_FILENAME))?;
    write_file(execute, path.join(EXECUTE_FILENAME))?;
    write_file(behaviours, path.join(BEHAVIOURS_FILENAME))?;

    Ok(())
}

/// Creates and writes an value implementing `Display` to a file at the supplied
/// path.
fn write_file<D: Display>(contents: D, path: PathBuf) -> Result<(), Error> {
    let mut file = File::create(&path).map_err(PathCtx::f(&path))?;
    writeln!(file, "{contents}").map_err(PathCtx::f(&path))?;
    Ok(())
}

impl Description {
    /// Creates the internal representation of the GenC files
    fn files(&self) -> Files {
        // `main.genc` file
        let main = Main {
            name: self.name.clone(),
            endianness: self.endianness,
            wordsize: self.wordsize,
            registers: self.registers.clone(),
            features: self.features.clone(),
            constants: self.constants.clone(),
        };

        // construct Vec of Formats
        let formats = self
            .instructions
            .iter()
            .map(
                |(
                    instruction_ident,
                    Instruction {
                        format,
                        disasm,
                        is_branch,
                        ..
                    },
                )| Format {
                    format_ident: format!("fmt_{instruction_ident}"),
                    instruction_ident: instruction_ident.clone(),
                    contents: format.to_string(),
                    disasm: disasm.clone(),
                    is_branch: *is_branch,
                },
            )
            .collect();

        // `isa.genc` file containg arch info and instruction decoding
        let isa = Isa {
            name: self.name.clone(),
            predicated: self.predicated,
            fetchsize: self.fetchsize,
            formats,
        };

        // semantics of each instruction
        let execute = Execute(
            self.instructions
                .iter()
                .map(
                    |(
                        instruction_ident,
                        Instruction {
                            execute, is_branch, ..
                        },
                    )| Function::Execute {
                        name: instruction_ident.clone(),
                        body: if *is_branch {
                            "\twrite_register(reg_PC_target, read_pc() + 4);\n".to_owned()
                                + execute
                                + "\n\twrite_pc(read_register(reg_PC_target));\n"
                        } else {
                            execute.clone()
                        },
                    },
                )
                .chain(self.helpers.iter().cloned().map(
                    |HelperFunction {
                         return_type,
                         parameters,
                         name,
                         body,
                     }| Function::Helper {
                        return_type,
                        parameters,
                        name,
                        body,
                    },
                ))
                .collect(),
        );

        let required_behaviours = [
            Function::Behaviour {
                global: true,
                name: "handle_exception".into(),
                body: self.behaviours.handle_exception.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "reset".into(),
                body: self.behaviours.reset.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "irq".into(),
                body: self.behaviours.irq.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "mmu_fault".into(),
                body: self.behaviours.mmu_fault.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "page_fault".into(),
                body: self.behaviours.page_fault.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "undefined_instruction".into(),
                body: self.behaviours.undefined_instruction.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "single_step".into(),
                body: self.behaviours.single_step.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "undef".into(),
                body: self.behaviours.undef.clone(),
            },
        ];

        let behaviours = files::Behaviours(
            self.behaviours
                .custom
                .iter()
                .map(|CustomBehaviour { name, body }| Function::Behaviour {
                    name: name.clone(),
                    body: body.clone(),
                    global: false,
                })
                .chain(required_behaviours)
                .collect::<Vec<_>>(),
        );

        Files {
            main,
            isa,
            execute,
            behaviours,
        }
    }
}

/// Endianness of an instruction set architecture
#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    /// Most significant byte of a word at the smallest memory address
    BigEndian,
    /// Most significant byte of a word at the largest memory address
    LittleEndian,
}

/// Definition of a register space
#[derive(Debug, Clone)]
pub struct RegisterSpace {
    /// Size in bytes of the register space
    pub size: u32,
    /// Views in register space
    pub views: Vec<View>,
}

/// Register view
#[derive(Debug, Clone)]
pub enum View {
    /// Register bank
    Bank(Bank),
    /// Register slot
    Slot(Slot),
}

/// Register bank
#[derive(Debug, Clone)]
pub struct Bank {
    /// Identifier for bank
    pub name: String,
    /// Register type
    pub typ: Typ,
    /// Offset
    pub offset: u32,
    /// Number of registers
    pub count: u32,
    /// Register stride
    pub stride: u32,
    /// Number of elements
    pub element_count: u32,
    /// Size of elements
    pub element_size: u32,
    /// Element stride
    pub element_stride: u32,
}

/// Register slot
#[derive(Debug, Clone)]
pub struct Slot {
    /// Identifier for slot
    pub name: String,

    /// Register type
    pub typ: Typ,

    /// Register slot width
    pub width: u32,

    /// Register slot offset in register space
    pub offset: u32,

    /// Optional tag
    pub tag: Option<String>,
}

/// Data type
#[derive(Debug, Clone)]
pub enum Typ {
    /// Unsigned 8-bit integer
    Uint8,
    /// Unsigned 16-bit integer
    Uint16,
    /// Unsigned 32-bit integer
    Uint32,
    /// Unsigned 64-bit integer
    Uint64,
    /// Unsigned 128-bit integer
    Uint128,
    /// 32-bit floating point
    Float,
    /// 64-bit floating point
    Double,
}

/// Definition of the syntax and semantics of a single instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Instruction format string used for decoding
    pub format: InstructionFormat,
    /// GenC execution behaviour
    pub execute: String,
    /// GenC disassembly behaviour
    pub disasm: String,
    /// Is branch instruction (modifies PC)
    pub is_branch: bool,
}

/// Required and custom behaviours for architecture
#[derive(Debug, Clone)]
pub struct Behaviours {
    /// Exception handler
    pub handle_exception: String,
    /// Reset
    pub reset: String,
    /// Interrupt request
    pub irq: String,
    /// Memory management unit fault
    pub mmu_fault: String,
    /// Page fault
    pub page_fault: String,
    /// Undefined instruction
    pub undefined_instruction: String,
    /// Single step
    pub single_step: String,
    /// Undefined
    pub undef: String,
    /// Additional, non-required behaviours
    pub custom: Vec<CustomBehaviour>,
}

/// Custom behaviour
#[derive(Debug, Clone)]
pub struct CustomBehaviour {
    /// Name of behaviour
    pub name: String,
    /// Function body
    pub body: String,
}

/// GenC internal helper function
#[derive(Debug, Clone)]
pub struct HelperFunction {
    /// Function return type
    pub return_type: String,
    /// Function parameters
    pub parameters: String,
    /// Function name
    pub name: String,
    /// Function body
    pub body: String,
}
