//! Types and functions for interfacing with OCaml

use {
    crate::intern::InternedStringKey,
    common::identifiable::{unique_id, Identifiable},
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    serde::{Deserialize, Serialize},
    std::{
        borrow::Cow,
        ffi::{OsStr, OsString},
        os::unix::ffi::OsStringExt,
        path::PathBuf,
    },
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
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Position {
    id: u64,
    /// File name
    pub pos_fname: InternedStringKey,
    /// Line number
    pub pos_lnum: Int,
    /// Character offset of beginning of line
    pub pos_bol: Int,
    /// Character offset of the position
    pub pos_cnum: Int,
}

impl Identifiable for Position {
    fn id(&self) -> u64 {
        self.id
    }
}

unsafe impl FromValue for Position {
    fn from_value(v: Value) -> Self {
        #[derive(FromValue)]
        struct RawPosition {
            pos_fname: &'static [u8],
            pos_lnum: Int,
            pos_bol: Int,
            pos_cnum: Int,
        }

        let raw = RawPosition::from_value(v);

        let path = PathBuf::from(OsString::from_vec(raw.pos_fname.to_owned()).to_owned());

        // if it is a path to a sail file, strip absolute path, otherwise ignore
        let normalized_path = match path.extension().and_then(OsStr::to_str) {
            Some("sail") => path.file_name().unwrap(),
            _ => path.as_os_str(),
        }
        .to_string_lossy();

        let pos_fname = match normalized_path {
            Cow::Borrowed(s) => InternedStringKey::from(s),
            Cow::Owned(s) => InternedStringKey::from(s),
        };

        Self {
            id: unique_id(),
            pos_fname,
            pos_lnum: raw.pos_lnum,
            pos_bol: raw.pos_bol,
            pos_cnum: raw.pos_cnum,
        }
    }
}
