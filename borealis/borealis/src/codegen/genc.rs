//! GenC code generation from BOOM structures

use crate::boom;

/// Used to render items to GenC strings
pub trait Render {
    /// Renders `self` as GenC
    fn render(&self) -> String;
}

impl Render for boom::NamedType {
    fn render(&self) -> String {
        format!("{} {}", self.typ.render(), self.name)
    }
}

impl Render for boom::Type {
    fn render(&self) -> String {
        use boom::Type::*;

        match self {
            Unit => "void".to_string(),
            Fbits(len, _) => match *len {
                0 => panic!("unexpected 0 length bitvector"),
                1..=8 => "uint8".to_owned(),
                9..=16 => "uint16".to_owned(),
                17..=32 => "uint32".to_owned(),
                33..=64 => "uint64".to_owned(),
                65..=128 => "uint128".to_owned(),
                _ => panic!("bitvector length exceeds 128 bits, not representable in GenC"),
            },
            t => panic!("{t:?}"),
        }
    }
}
