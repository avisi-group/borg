//! OCaml Interface types and functions

use {
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    serde::{Deserialize, Serialize},
};

/// OCaml strings are byte arrays and may contain valid UTF-8 contents or arbitrary bytes
///
/// When converting from Value will attempt to parse as a `String`, falling back to `Vec<u8>` on error
#[cfg_attr(not(feature = "redact"), derive(Serialize))]
#[derive(Debug, Clone, Deserialize, DeepSizeOf)]
pub enum OCamlString {
    String(String),
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

/// Kind identifiers zeroed for tests to prevent snapshot tests from breaking
#[cfg(feature = "redact")]
impl Serialize for OCamlString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OCamlString::String(s) => {
                let path = std::path::PathBuf::from(s);

                // if it is a path to a sail file, strip absolute path, otherwise ignore
                let s = match path.extension().and_then(std::ffi::OsStr::to_str) {
                    Some("sail") => path.file_name().unwrap().to_string_lossy().to_string(),
                    _ => s.clone(),
                };

                serializer.serialize_newtype_variant("OCamlString", 0, "String", &s)
            }
            OCamlString::Vec(v) => serializer.serialize_newtype_variant("OCamlString", 0, "Vec", v),
        }
    }
}

/// Position in a source file
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Position {
    /// File name
    pub pos_fname: OCamlString,
    /// Line number
    pub pos_lnum: Int,
    /// Character offset of beginning of line
    pub pos_bol: Int,
    /// Character offset of the position
    pub pos_cnum: Int,
}
