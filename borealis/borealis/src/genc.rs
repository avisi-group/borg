//! GenC output generation
//!
//! Responsiblity of callee to insert tabs and newlines

use {
    crate::Error,
    std::{
        collections::HashMap,
        fmt::Display,
        fs::{self, File},
        io::{self},
        path::Path,
    },
};

const MAIN_FILENAME: &str = "main.genc";
const ISA_FILENAME: &str = "isa.genc";
const EXECUTE_FILENAME: &str = "execute.genc";
const BEHAVIOURS_FILENAME: &str = "behaviours.genc";

struct Context {
    name: String,
    ident_mangle_counter: u32,
}

impl Context {
    /// Generates a new unique identifier
    fn generate_ident(&mut self) -> String {
        let ident = n_to_ident(self.ident_mangle_counter);
        self.ident_mangle_counter += 1;
        ident
    }
}

trait ContextWrite {
    fn write<W: io::Write>(&self, ctx: &mut Context, w: &mut W) -> io::Result<()>;
}

/// GenC description of an instruction set architecture
pub struct Description {
    /// Main GenC file
    pub main: Main,
    /// ISA GenC file
    pub isa: Isa,
    /// Execute GenC file
    pub execute: Execute,
    /// Behaviours GenC file
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

        let mut context = Context {
            name: "test".to_lowercase(),
            ident_mangle_counter: 0,
        };

        self.main
            .write(&mut context, &mut File::create(path.join(MAIN_FILENAME))?)?;

        self.isa
            .write(&mut context, &mut File::create(path.join(ISA_FILENAME))?)?;

        self.execute.write(
            &mut context,
            &mut File::create(path.join(EXECUTE_FILENAME))?,
        )?;

        self.behaviours.write(
            &mut context,
            &mut File::create(path.join(BEHAVIOURS_FILENAME))?,
        )?;

        Ok(())
    }

    #[doc(hidden)]
    pub fn empty() -> Self {
        Description {
            main: Main {
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
                predicated: false,
                fetchsize: 32,
                formats: vec![Format {
                    contents: "%sf:1 %op:1 %S:1 0x11:5 %shift:2 %imm12:12 %rn:5 %rd:5".to_owned(),
                }],
            },
            execute: Execute {},
            behaviours: Behaviours {},
        }
    }
}

/// Main GenC file
#[derive(Debug)]
pub struct Main {
    /// Endianness of the architecture
    pub endianness: Endianness,

    /// Word size in bytes
    pub wordsize: u32,

    /// Register definitions
    pub registers: Vec<RegisterSpace>,
}

impl ContextWrite for Main {
    fn write<W: io::Write>(&self, ctx: &mut Context, w: &mut W) -> io::Result<()> {
        write_header(w)?;
        writeln!(w, "AC_ARCH({})", ctx.name)?;
        writeln!(w, "{{")?;

        writeln!(
            w,
            "\tac_mem Data({}, {}, {}, {});",
            self.wordsize, self.wordsize, self.endianness, 0
        )?;
        writeln!(
            w,
            "\tac_mem Fetch({}, {}, {}, {});",
            self.wordsize, self.wordsize, self.endianness, 1
        )?;
        writeln!(w, "")?;
        writeln!(w, "\tac_wordsize {};", self.wordsize)?;
        writeln!(w, "")?;

        for register in &self.registers {
            write!(w, "{}", register)?;
        }

        writeln!(w, "")?;

        writeln!(w, "\tARCH_CTOR(arm) {{")?;
        writeln!(w, "\t\tac_isa(\"{}\");", ISA_FILENAME)?;
        writeln!(w, "\t\tset_endian(\"{}\");", self.endianness)?;
        writeln!(w, "\t}};")?;

        writeln!(w, "}};")?;

        Ok(())
    }
}

/// ISA GenC file
pub struct Isa {
    /// ?
    pub predicated: bool,

    /// Instruction size in bytes
    pub fetchsize: u32,

    /// Instruction decode format strings
    pub formats: Vec<Format>,
}

impl ContextWrite for Isa {
    fn write<W: io::Write>(&self, ctx: &mut Context, w: &mut W) -> io::Result<()> {
        write_header(w)?;
        writeln!(w, "AC_ISA({})", ctx.name)?;
        writeln!(w, "{{")?;

        writeln!(w, "\tac_fetchsize {};", self.fetchsize)?;
        writeln!(
            w,
            "\tac_predicated \"{}\";",
            match self.predicated {
                true => "yes",
                false => "no",
            }
        )?;

        for format in &self.formats {
            format.write(ctx, w)?;
        }

        writeln!(w, "")?;
        writeln!(w, "\tISA_CTOR(arm) {{")?;
        writeln!(w, "\t\tac_behaviours(\"{}\");", BEHAVIOURS_FILENAME)?;
        writeln!(w, "\t\tac_execute(\"{}\");", EXECUTE_FILENAME)?;
        writeln!(w, "\t}};")?;

        writeln!(w, "}}")?;

        Ok(())
    }
}

/// Execution GenC file containing instruction implementations
pub struct Execute {}

impl ContextWrite for Execute {
    fn write<W: io::Write>(&self, _ctx: &mut Context, w: &mut W) -> io::Result<()> {
        write_header(w)?;

        Ok(())
    }
}

/// Behaviours GenC file
pub struct Behaviours {}

impl ContextWrite for Behaviours {
    fn write<W: io::Write>(&self, _ctx: &mut Context, w: &mut W) -> io::Result<()> {
        write_header(w)?;

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
    /// Format string
    pub contents: String,
}

impl ContextWrite for Format {
    fn write<W: io::Write>(&self, ctx: &mut Context, w: &mut W) -> io::Result<()> {
        let format_ident = ctx.generate_ident();
        let instruction = "addi";

        writeln!(w, "\tac_format {} = \"{}\";", format_ident, self.contents)?;
        writeln!(w, "\tac_instruction<{}> {};", format_ident, instruction)?;

        Ok(())
    }
}

/// Creates a file and writes the initial comment
fn write_header<W: io::Write>(w: &mut W) -> io::Result<()> {
    writeln!(w, "/* GENERATED BY BOREALIS */")?;
    writeln!(w, "")?;
    Ok(())
}

/// Converts the supplied integer into base26 ASCII
fn n_to_ident(mut n: u32) -> String {
    let mut s = String::new();

    loop {
        if n < 26 {
            s.push(char::from_u32(n + 'a' as u32).unwrap());
            break;
        } else {
            s.push(char::from_u32((n % 26) + 'a' as u32).unwrap());

            n /= 26;
            n -= 1;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use {
        crate::genc::{n_to_ident, Context, ContextWrite, Description},
        proptest::prelude::*,
    };

    #[test]
    fn ident_mangler() {
        assert_eq!(n_to_ident(0), "a");
        assert_eq!(n_to_ident(1), "b");
        assert_eq!(n_to_ident(25), "z");
        assert_eq!(n_to_ident(26), "aa");
        assert_eq!(n_to_ident(27), "ba");
        assert_eq!(n_to_ident(50), "ya");
        assert_eq!(n_to_ident(51), "za");
        assert_eq!(n_to_ident(52), "ab");
        assert_eq!(n_to_ident(216222910), "uidder");
    }

    proptest! {
        #[test]
        fn mangler_doesnt_crash(n in 0u32..u32::MAX) {
           let _s = n_to_ident(n);
        }
    }

    #[test]
    fn snapshot() {
        let mut context = Context {
            name: "aarch64".to_owned(),
            ident_mangle_counter: 0,
        };

        let Description {
            main,
            isa,
            execute,
            behaviours,
        } = Description::empty();

        let main = {
            let mut buf = vec![];
            main.write(&mut context, &mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        };

        let isa = {
            let mut buf = vec![];
            isa.write(&mut context, &mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        };

        let execute = {
            let mut buf = vec![];
            execute.write(&mut context, &mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        };

        let behaviours = {
            let mut buf = vec![];
            behaviours.write(&mut context, &mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        };

        insta::assert_snapshot!(format!(
            "main.genc:\n{}\n\n\nisa.genc:\n{}\n\n\nexecute.genc:\n{}\n\n\nbehaviours.genc:\n{}\n\n\n",
            main, isa, execute, behaviours
        ));
    }
}
