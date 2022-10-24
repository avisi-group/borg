//! OCaml Interface types and functions

use ocaml::{FromValue, Int, Value};

/// OCaml strings are byte arrays and may contain valid UTF-8 contents or arbitrary bytes
///
/// When converting from Value will attempt to parse as a `String`, falling back to `Vec<u8>` on error
#[derive(Debug, Clone)]
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

/// Position in a source file
#[derive(Debug, Clone, FromValue)]
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

#[derive(Debug, Clone, FromValue)]
pub struct BigNum(Value);

unsafe impl Send for BigNum {}
unsafe impl Sync for BigNum {}

#[derive(Debug, Clone, FromValue)]
pub struct Rational(Value);

unsafe impl Send for Rational {}
unsafe impl Sync for Rational {}
