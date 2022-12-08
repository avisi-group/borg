//! GenC description generation

use {
    crate::{
        genc::ir::{Execute, Files, Format, Function, FunctionKind, Isa, Main},
        Error,
    },
    common::error::ErrCtx,
    std::{
        collections::HashMap,
        fmt::Display,
        fs::{read_dir, File},
        io::{self, Write as _},
        path::{Path, PathBuf},
    },
};

mod from_ast;
mod ir;

const MAIN_FILENAME: &str = "main.genc";
const ISA_FILENAME: &str = "isa.genc";
const EXECUTE_FILENAME: &str = "execute.genc";
const BEHAVIOURS_FILENAME: &str = "behaviours.genc";

/// Export a GenC description to the supplied empty directory
pub fn export<P: AsRef<Path>>(
    description: &Description,
    path: P,
    force: bool,
) -> Result<(), Error> {
    let path = path.as_ref();

    let count = read_dir(&path)
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Error::OutDirectoryNotFound(path.to_owned())
            } else {
                ErrCtx::new(e, path).into()
            }
        })?
        .count();

    // if force is not set and count is not zero, return a directory not empty error
    if !force && count != 0 {
        return Err(Error::OutDirectoryNotEmpty(path.to_owned()));
    }

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

/// Creates and writes
pub fn write_file<D: Display>(contents: D, path: PathBuf) -> Result<(), Error> {
    let mut file = File::create(&path).map_err(ErrCtx::f(&path))?;
    writeln!(file, "{}", contents).map_err(ErrCtx::f(&path))?;
    Ok(())
}

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

    ///
    pub behaviours: Behaviours,
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
        };

        // construct Vec of Formats
        let formats = self
            .instructions
            .iter()
            .map(|(instruction_ident, Instruction { format, .. })| Format {
                format_ident: format!("fmt_{}", instruction_ident),
                instruction_ident: instruction_ident.clone(),
                contents: format.clone(),
            })
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
                    |(instruction_ident, Instruction { execute, .. })| Function {
                        kind: FunctionKind::Execute,
                        name: instruction_ident.clone(),
                        body: execute.clone(),
                    },
                )
                .collect(),
        );

        let required_behaviours = [
            Function {
                kind: FunctionKind::Behaviour,
                name: "handle_exception".to_owned(),
                body: self.behaviours.handle_exception.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "reset".to_owned(),
                body: self.behaviours.reset.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "irq".to_owned(),
                body: self.behaviours.irq.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "mmu_fault".to_owned(),
                body: self.behaviours.mmu_fault.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "page_fault".to_owned(),
                body: self.behaviours.page_fault.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "undefined_instruction".to_owned(),
                body: self.behaviours.undefined_instruction.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "single_step".to_owned(),
                body: self.behaviours.single_step.clone(),
            },
            Function {
                kind: FunctionKind::Behaviour,
                name: "undef".to_owned(),
                body: self.behaviours.undef.clone(),
            },
        ];

        let behaviours = ir::Behaviours(
            self.behaviours
                .custom
                .iter()
                .map(|CustomBehaviour { name, body }| ir::Function {
                    kind: FunctionKind::Behaviour,
                    name: name.clone(),
                    body: body.clone(),
                })
                .chain(required_behaviours.into_iter())
                .collect::<Vec<_>>(),
        );

        Files {
            main,
            isa,
            execute,
            behaviours,
        }
    }

    #[doc(hidden)]
    pub fn empty() -> Self {
        Description {
            name: "aarch64".to_owned(),
            endianness: Endianness::LittleEndian,
            wordsize: 64,
            fetchsize: 32,
            predicated: false,
            registers: vec![
                RegisterSpace {
                    size: 248,
                    views: vec![
                        View::Bank(Bank {
                            name: "RBX".to_owned(),
                            typ: Typ::Uint64,
                            offset: 0,
                            count: 31,
                            stride: 8,
                            element_count: 1,
                            element_size: 8,
                            element_stride: 8,
                        }),
                        View::Bank(Bank {
                            name: "RBW".to_owned(),
                            typ: Typ::Uint32,
                            offset: 0,
                            count: 31,
                            stride: 8,
                            element_count: 1,
                            element_size: 4,
                            element_stride: 4,
                        }),
                    ],
                },
                RegisterSpace {
                    size: 16,
                    views: vec![View::Slot(Slot {
                        name: "PC".to_owned(),
                        typ: Typ::Uint64,
                        width: 8,
                        offset: 0,
                        tag: Some("PC".to_owned()),
                    })],
                },
            ],
            instructions: HashMap::from([(
                "addi".to_owned(),
                Instruction {
                    format: "%sf:1 %op:1 %S:1 0x11:5 %shift:2 %imm12:12 %rn:5 %rd:5".to_owned(),
                    execute: "return;".to_owned(),
                },
            )]),
            behaviours: Behaviours {
                handle_exception: "".to_owned(),
                reset: "".to_owned(),
                irq: "".to_owned(),
                mmu_fault: "".to_owned(),
                page_fault: "".to_owned(),
                undefined_instruction: "".to_owned(),
                single_step: "".to_owned(),
                undef: "".to_owned(),
                custom: vec![CustomBehaviour {
                    name: "custom".to_owned(),
                    body: "return;".to_owned(),
                }],
            },
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
    pub format: String,
    /// GenC execution behaviour
    pub execute: String,
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

#[cfg(test)]
mod tests {
    use crate::genc::{ir::Files, Description};

    #[test]
    fn snapshot() {
        let Files {
            main,
            isa,
            execute,
            behaviours,
        } = Description::empty().files();

        insta::assert_snapshot!(format!(
            "main.genc:\n{}\n\n\nisa.genc:\n{}\n\n\nexecute.genc:\n{}\n\n\nbehaviours.genc:\n{}\n\n\n",
            main, isa, execute, behaviours
        ));
    }
}
