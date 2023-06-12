//! Types and functions for interfacing with OCaml

use {
    common::intern::InternedString,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue, Value},
    serde::{Deserialize, Serialize},
    std::{ffi::OsStr, fmt::Display, path::PathBuf},
};

/// Kind identifier
///
/// In the Sail AST kind identifiers are strings, but they are invalid Rust
/// `String`s and so are represented here as a `Vec<u8>`.
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf, PartialEq, Eq)]
pub struct KindIdentifierInner(Vec<u8>);

unsafe impl FromValue for KindIdentifierInner {
    fn from_value(v: Value) -> Self {
        Self(<&[u8]>::from_value(v).to_owned())
    }
}

unsafe impl ToValue for KindIdentifierInner {
    fn to_value(&self, rt: &ocaml::Runtime) -> Value {
        (self.0.as_slice()).to_value(rt)
    }
}

/// Position of a character in a source file
///
/// Can be converted from `Lexing.position` value <https://v2.ocaml.org/api/Lexing.html>.

#[derive(Debug, Clone, FromValue, ToValue, PartialEq, Serialize, Deserialize, DeepSizeOf)]
pub struct Position {
    /// File name
    pub pos_fname: InternedString,
    /// Line number
    pub pos_lnum: Int,
    /// Character offset of beginning of line
    pub pos_bol: Int,
    /// Character offset of the position
    pub pos_cnum: Int,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            PathBuf::from(self.pos_fname.to_string())
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("?"),
            self.pos_lnum,
            self.pos_cnum - self.pos_bol
        )
    }
}
