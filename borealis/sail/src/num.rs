//! Wrappers around ocaml `Num` arbitrary-precision integer and rational
//! arithmetic library

use {
    crate::{runtime::internal_bigint_to_string, types::OCamlString},
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, Value},
    serde::{Deserialize, Serialize},
    std::str::FromStr,
};

/// Arbitrary precision number from `Num.num` OCaml type
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Num {
    /// Integer
    Int(Int),
    /// Big integer
    BigInt(BigInt),
    /// Ratio of big integers
    Ratio(Ratio),
}

/// Signed big integer from `Nat_big_num.num` OCaml type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigInt(pub num_bigint::BigInt);

unsafe impl FromValue for BigInt {
    fn from_value(v: Value) -> Self {
        let rt = unsafe { ocaml::Runtime::recover_handle() };

        let s = match dbg!(unsafe { internal_bigint_to_string(rt, v) })
            .unwrap()
            .unwrap()
        {
            OCamlString::String(s) => s,
            OCamlString::Vec(_) => panic!("invalid UTF-8 when converting bigint to string"),
        };

        Self(num_bigint::BigInt::from_str(&s).unwrap())
    }
}

impl DeepSizeOf for BigInt {
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        (self.0.bits() / 8) as usize
    }
}

/// Ratio of big integers from `num.Ratio.ratio` OCaml type
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Ratio {
    /// Numerator
    pub numerator: BigInt,
    /// Denominator
    pub denominator: BigInt,
    /// Normalized
    pub normalized: bool,
}
