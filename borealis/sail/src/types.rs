//! Types and functions for interfacing with OCaml

use {
    common::{identifiable::identifiable_fromvalue, intern::InternedStringKey},
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    serde::{Deserialize, Serialize},
};

/// OCaml string
///
/// OCaml strings are byte arrays. They *may* contain valid UTF-8 contents or
/// could be arbitrary bytes. Conversion from opaque `ocaml::Value` will attempt
/// to parse as a `String`, falling back to `Vec<u8>` on error.
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub enum OCamlString {
    /// UTF-8 string
    String(String),
    /// Byte array
    Vec(Vec<u8>),
}

unsafe impl FromValue for OCamlString {
    fn from_value(v: Value) -> Self {
        let vec = <&[u8]>::from_value(v).to_owned();
        match String::from_utf8(vec.clone()) {
            Ok(s) => Self::String(s),
            Err(_) => Self::Vec(vec),
        }
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
