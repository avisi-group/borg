//! Types and functions for interfacing with OCaml

use {
    common::{
        identifiable::{identifiable_fromvalue, unique_id, Identifiable},
        intern::InternedStringKey,
    },
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    serde::{Deserialize, Serialize},
    std::{ffi::OsStr, fmt::Display, path::PathBuf},
};

/// Kind identifier
///
/// In the Sail AST kind identifiers are strings, but they are invalid Rust Strings and so are represented here as a Vec<u8>.
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf, PartialEq, Eq)]
pub struct KindIdentifierInner(Vec<u8>);

unsafe impl FromValue for KindIdentifierInner {
    fn from_value(v: Value) -> Self {
        Self(<&[u8]>::from_value(v).to_owned())
    }
}

/// Position of a character in a source file
///
/// Can be converted from `Lexing.position` value <https://v2.ocaml.org/api/Lexing.html>.
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Position {
    /// File name
    pub pos_fname: InternedStringKey,
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

/// Wrapper to give enums an ID in the AST without affecting `FromValue`
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct EnumWrapper<T> {
    id: u32,
    /// Inner item
    pub inner: T,
}

impl<T> Identifiable for EnumWrapper<T> {
    fn id(&self) -> u32 {
        self.id
    }
}

unsafe impl<T: FromValue> FromValue for EnumWrapper<T> {
    fn from_value(v: ocaml::Value) -> Self {
        let inner = T::from_value(v);
        Self {
            id: unique_id(),
            inner,
        }
    }
}
