use {
    crate::genc::{files::write_header, BEHAVIOURS_FILENAME, EXECUTE_FILENAME},
    std::fmt::{self, Display, Formatter},
};

/// ISA GenC file
#[derive(Debug, Clone)]
pub struct Isa {
    pub name: String,

    pub predicated: bool,

    /// Instruction size in bytes
    pub fetchsize: u32,

    /// Instruction decode format strings
    pub formats: Vec<Format>,
}

impl Display for Isa {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

        writeln!(f)?;

        for format in &self.formats {
            write!(f, "{format}")?;
        }

        writeln!(f)?;
        writeln!(f, "\tISA_CTOR(arm) {{")?;

        for Format {
            instruction_ident,
            disasm,
            is_branch,
            ..
        } in &self.formats
        {
            writeln!(f, "\t\t{instruction_ident}.set_decoder();")?;

            if !disasm.is_empty() {
                writeln!(f, "\t\t{instruction_ident}.set_asm({disasm});")?;
            }

            writeln!(
                f,
                "\t\t{instruction_ident}.set_behaviour({instruction_ident});",
            )?;

            if *is_branch {
                writeln!(f, "\t\t{instruction_ident}.set_end_of_block();")?;
                writeln!(f, "\t\t{instruction_ident}.set_variable_jump();")?;
            }
            writeln!(f)?;
        }

        writeln!(f, "\t\tac_behaviours(\"{BEHAVIOURS_FILENAME}\");")?;
        writeln!(f, "\t\tac_execute(\"{EXECUTE_FILENAME}\");")?;
        writeln!(f, "\t}};")?;

        writeln!(f, "}};")?;

        Ok(())
    }
}

/// Format string describing the en/decoding of an instruction
#[derive(Debug, Clone)]
pub struct Format {
    /// Format identifier
    pub format_ident: String,
    /// Instruction identifier
    pub instruction_ident: String,
    /// Format string
    pub contents: String,
    /// Disassembly string
    pub disasm: String,
    /// Is branch instruction, if true, adds `set_end_of_block` and
    /// `set_variable_jump` attributes
    pub is_branch: bool,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "\tac_format {} = \"{}\";",
            self.format_ident, self.contents
        )?;
        writeln!(
            f,
            "\tac_instr<{}> {};",
            self.format_ident, self.instruction_ident
        )?;
        writeln!(f, "\tac_behaviour {};", self.instruction_ident)?;
        writeln!(f)?;

        Ok(())
    }
}
