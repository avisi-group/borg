//! GenC description generation

use {
    crate::{
        genc::ir::{Behaviours, Execute, Files, Format, Function, FunctionKind, Isa, Main},
        Error,
    },
    std::{
        collections::HashMap,
        fs::{read_dir, File},
        io::{self, Write as _},
        path::Path,
    },
};

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
                e.into()
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

    writeln!(File::create(path.join(MAIN_FILENAME))?, "{}", main)?;
    writeln!(File::create(path.join(ISA_FILENAME))?, "{}", isa)?;
    writeln!(File::create(path.join(EXECUTE_FILENAME))?, "{}", execute)?;
    writeln!(
        File::create(path.join(BEHAVIOURS_FILENAME))?,
        "{}",
        behaviours
    )?;

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

        let behaviours = Behaviours(vec![Function {
            kind: FunctionKind::Behaviour,
            name: "handle_exception".to_owned(),
            body: "".to_owned(),
        }]);

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
                        high: 8,
                        low: 0,
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

    pub high: u32,

    pub low: u32,

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
