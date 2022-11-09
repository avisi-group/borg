//! GenC output generation
//!
//! Responsiblity of callee to insert tabs and newlines

use {
    crate::Error,
    std::{
        collections::HashMap,
        fmt::{self, Display},
        fs::{self, File},
        io::{self, Write},
        path::Path,
    },
};

const MAIN_FILENAME: &str = "main.genc";
const ISA_FILENAME: &str = "isa.genc";
const EXECUTE_FILENAME: &str = "execute.genc";
const BEHAVIOURS_FILENAME: &str = "behaviours.genc";

/// GenC description of an instruction set architecture
pub struct Description {
    pub main: Main,
    pub isa: Isa,
    pub execute: Execute,
    pub behaviours: Behaviours,
}

impl Description {
    /// Export a GenC description to the supplied empty directory
    pub fn export<P: AsRef<Path>>(&self, path: P, force: bool) -> Result<(), Error> {
        let path = path.as_ref();

        let count = fs::read_dir(&path)
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

        write!(File::create(path.join(MAIN_FILENAME))?, "{}", self.main)?;
        write!(File::create(path.join(ISA_FILENAME))?, "{}", self.isa)?;
        write!(
            File::create(path.join(EXECUTE_FILENAME))?,
            "{}",
            self.execute
        )?;
        write!(
            File::create(path.join(BEHAVIOURS_FILENAME))?,
            "{}",
            self.behaviours
        )?;
        Ok(())
    }

    #[doc(hidden)]
    pub fn empty() -> Self {
        Description {
            main: Main {
                name: "arm64".to_owned(),
                endianness: Endianness::LittleEndian,
                wordsize: 64,
                registers: vec![RegisterSpace {
                    size: 248,
                    banks: vec![Bank {
                        name: "RBX".to_owned(),
                        typ: Typ::Uint64,
                        offset: 0,
                        count: 31,
                        stride: 8,
                        element_count: 1,
                        element_size: 8,
                        element_stride: 8,
                    }],
                }],
            },
            isa: Isa {
                name: "arm64".to_owned(),
                predicated: false,
                fetchsize: 32,
                instruction_formats: vec![],
            },
            execute: Execute {},
            behaviours: Behaviours {},
        }
    }
}

/// Main GenC file
#[derive(Debug)]
pub struct Main {
    /// Name of the architecture (e.g. "aarch64" or "riscv")
    pub name: String,

    /// Endianness of the architecture
    pub endianness: Endianness,

    /// Word size in bytes
    pub wordsize: u32,

    /// Register definitions
    pub registers: Vec<RegisterSpace>,
}

impl Display for Main {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f)?;
        writeln!(f, "AC_ARCH({})", self.name)?;
        writeln!(f, "{{")?;

        writeln!(
            f,
            "\tac_mem Data({}, {}, {}, {});",
            self.wordsize, self.wordsize, self.endianness, 0
        )?;
        writeln!(
            f,
            "\tac_mem Fetch({}, {}, {}, {});",
            self.wordsize, self.wordsize, self.endianness, 1
        )?;
        writeln!(f, "")?;
        writeln!(f, "\tac_wordsize {};", self.wordsize)?;
        writeln!(f, "")?;

        for register in &self.registers {
            write!(f, "{}", register)?;
        }

        writeln!(f, "")?;

        writeln!(f, "\tARCH_CTOR(arm) {{")?;
        writeln!(f, "\t\tac_isa(\"{}\");", ISA_FILENAME)?;
        writeln!(f, "\t\tset_endian(\"{}\");", self.endianness)?;
        writeln!(f, "\t}};")?;

        writeln!(f, "}};")?;

        Ok(())
    }
}

/// ISA GenC file
pub struct Isa {
    /// Name of the ISA (e.g. "aarch64" or "riscv")
    pub name: String,
    /// ?
    pub predicated: bool,
    /// Instruction size in bytes
    pub fetchsize: u32,

    pub instruction_formats: Vec<Format>,
}

impl Display for Isa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f)?;
        writeln!(f, "AC_ISA({})", self.name)?;
        writeln!(f, "{{")?;

        writeln!(f, "\tac_fetchsize {};", self.fetchsize)?;
        writeln!(
            f,
            "\tac_predicated \"{}\";",
            match self.predicated {
                true => "yes",
                false => "no",
            }
        )?;

        writeln!(f, "")?;

        writeln!(f, "\tISA_CTOR(arm) {{")?;
        writeln!(f, "\t\tac_behaviours(\"{}\");", BEHAVIOURS_FILENAME)?;
        writeln!(f, "\t\tac_execute(\"{}\");", EXECUTE_FILENAME)?;
        writeln!(f, "\t}};")?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

/// Execution GenC file containing instruction implementations
pub struct Execute {}

impl Display for Execute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f)?;

        Ok(())
    }
}

/// Behaviours GenC file
pub struct Behaviours {}

impl Display for Behaviours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_header(f)?;

        Ok(())
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

impl Display for Endianness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endianness::BigEndian => write!(f, "big"),
            Endianness::LittleEndian => write!(f, "little"),
        }
    }
}

/// Definition of a register space
#[derive(Debug)]
pub struct RegisterSpace {
    /// Size in bytes of the register space
    pub size: u32,
    /// Register banks in register space
    pub banks: Vec<Bank>,
}

impl Display for RegisterSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tac_regspace({}) {{", self.size)?;
        for bank in &self.banks {
            write!(f, "{}", bank)?;
        }
        writeln!(f, "\t}}")
    }
}

/// Register bank
#[derive(Debug)]
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

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\t\tbank {} ({}, {}, {}, {}, {}, {}, {});",
            self.name,
            self.typ,
            self.offset,
            self.count,
            self.stride,
            self.element_count,
            self.element_size,
            self.element_stride
        )
    }
}

/// Data type
#[derive(Debug)]
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

impl Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Uint8 => "uint8",
                Self::Uint16 => "uint16",
                Self::Uint32 => "uint32",
                Self::Uint64 => "uint64",
                Self::Uint128 => "uint128",
                Self::Float => "float",
                Self::Double => "double",
            }
        )
    }
}

/// Instructions defined for an architecture definition.
#[derive(Debug)]
pub struct Instructions(pub HashMap<String, InstructionDefinition>);

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Definition of the syntax and semantics of a single instruction
#[derive(Debug)]
pub struct InstructionDefinition {
    /// Instruction format string used for decoding
    pub format: Format,
    /// GenC execution behaviour
    pub execute: String,
}

impl Display for InstructionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Format string describing the en/decoding of an instruction
#[derive(Debug)]
pub struct Format {
    /// Identifier for the format
    pub name: String,
    /// Format string
    pub contents: String,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Creates a file and writes the initial comment
fn write_header(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "/* GENERATED BY BOREALIS */")?;
    writeln!(f, "")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::genc::Description;

    #[test]
    fn genc_display_snapshot_main() {
        let Description { main, .. } = Description::empty();
        insta::assert_display_snapshot!(main);
    }

    #[test]
    fn genc_display_snapshot_isa() {
        let Description { isa, .. } = Description::empty();
        insta::assert_display_snapshot!(isa);
    }

    #[test]
    fn genc_display_snapshot_behaviours() {
        let Description { behaviours, .. } = Description::empty();
        insta::assert_display_snapshot!(behaviours);
    }

    #[test]
    fn genc_display_snapshot_execute() {
        let Description { execute, .. } = Description::empty();
        insta::assert_display_snapshot!(execute);
    }
}
