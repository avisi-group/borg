use {
    crate::genc_model::{
        files::write_header, Bank, Endianness, RegisterSpace, Slot, Typ, View, ISA_FILENAME,
    },
    common::{intern::InternedString, HashMap, HashSet},
    std::fmt::{self, Display, Formatter},
};

/// Main GenC file
#[derive(Debug, Clone)]
pub struct Main {
    pub name: String,

    /// Endianness of the architecture
    pub endianness: Endianness,

    /// Word size in bytes
    pub wordsize: u32,

    /// Register definitions
    pub registers: Vec<RegisterSpace>,

    pub features: HashSet<String>,

    pub constants: HashMap<InternedString, (Typ, u64)>,
}

impl Display for Main {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
        writeln!(f)?;
        writeln!(f, "\tac_wordsize {};", self.wordsize)?;
        writeln!(f)?;

        for register in &self.registers {
            write!(f, "{register}")?;
            writeln!(f)?;
        }

        writeln!(f)?;

        writeln!(f, "\tac_features {{")?;
        for feature in &self.features {
            writeln!(f, "\t\tfeature {feature};")?;
        }
        writeln!(f, "\t}}")?;

        writeln!(f)?;

        writeln!(f, "\tARCH_CTOR(arm) {{")?;
        writeln!(f, "\t\tac_isa(\"{ISA_FILENAME}\");")?;
        writeln!(f, "\t\tset_endian(\"{}\");", self.endianness)?;

        for (name, (typ, value)) in &self.constants {
            writeln!(f, "\t\tset_constant({name}, {typ}, {value});")?;
        }

        writeln!(f, "\t}};")?;

        writeln!(f, "}};")?;

        Ok(())
    }
}

impl Display for Endianness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Endianness::BigEndian => write!(f, "big"),
            Endianness::LittleEndian => write!(f, "little"),
        }
    }
}

impl Display for RegisterSpace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "\tac_regspace({}) {{", self.size)?;
        for view in &self.views {
            write!(f, "{view}")?;
        }
        writeln!(f, "\t}}")
    }
}

impl Display for View {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            View::Bank(bank) => write!(f, "{bank}"),
            View::Slot(slot) => write!(f, "{slot}"),
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\t\tslot {} ({}, {}, {})",
            self.name, self.typ, self.width, self.offset
        )?;

        if let Some(tag) = &self.tag {
            write!(f, " {tag}")?;
        }

        writeln!(f, ";")?;

        Ok(())
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl Display for Typ {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
