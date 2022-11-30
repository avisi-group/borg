//! Types and functions for interfacing with OCaml

use {
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    once_cell::sync::Lazy,
    serde::{Deserialize, Serialize},
};

static INTERNER: Lazy<lasso::ThreadedRodeo> = Lazy::new(lasso::ThreadedRodeo::new);

/// Key for an interned string
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct Key(lasso::Spur);

impl Key {
    /// Create a new interned string
    pub fn new<A: AsRef<str>>(str: A) -> Self {
        Self(INTERNER.get_or_intern(str))
    }

    /// Create a new interned string from a static str
    pub fn from_static(key: &'static str) -> Self {
        Self(INTERNER.get_or_intern_static(key))
    }
}

impl From<lasso::Spur> for Key {
    fn from(spur: lasso::Spur) -> Self {
        Self(spur)
    }
}

impl From<String> for Key {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<&'_ str> for Key {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        INTERNER.resolve(&self.0).fmt(f)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        INTERNER.resolve(&self.0).fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        std::borrow::Cow::<'de, str>::deserialize(deserializer).map(Self::new)
    }
}

impl serde::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

unsafe impl FromValue for Key {
    fn from_value(v: Value) -> Self {
        let s = OCamlString::from_value(v);
        match s {
            OCamlString::String(s) => Key::new(s),
            _ => unimplemented!(),
        }
    }
}

impl DeepSizeOf for Key {
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        0
    }

    fn deep_size_of(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

/// OCaml string
///
/// OCaml strings are byte arrays. They *may* contain valid UTF-8 contents or
/// could be arbitrary bytes. Conversion from opaque `ocaml::Value` will attempt
/// to parse as a `String`, falling back to `Vec<u8>` on error.
#[cfg_attr(not(test), derive(Serialize))]
#[derive(Debug, Clone, Deserialize, DeepSizeOf)]
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

/// If the OCamlString contains a path to a sail source file, strip absolute all
/// of path except filename to prevent breaking snapshot tests.
#[cfg(test)]
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

/// Position of a character in a source file
///
/// Can be converted from `Lexing.position` value <https://v2.ocaml.org/api/Lexing.html>.
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Position {
    /// File name
    pub pos_fname: Key,
    /// Line number
    pub pos_lnum: Int,
    /// Character offset of beginning of line
    pub pos_bol: Int,
    /// Character offset of the position
    pub pos_cnum: Int,
}
