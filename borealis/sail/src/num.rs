//! Wrappers around ocaml `Num` arbitrary-precision integer and rational
//! arithmetic library

use {
    crate::{
        runtime::internal_bigint_to_string,
        types::OCamlString,
        visitor::{Visitor, Walkable},
    },
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

impl Walkable for BigInt {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

unsafe impl FromValue for BigInt {
    fn from_value(v: Value) -> Self {
        let rt = unsafe { ocaml::Runtime::recover_handle() };

        let s = match unsafe { internal_bigint_to_string(rt, v) }
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
    fn deep_size_of_children(&self, _: &mut deepsize::Context) -> usize {
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

#[cfg(test)]
mod tests {
    use {
        crate::{types::OCamlString, RT},
        num_bigint::BigInt,
        proptest::prelude::*,
        std::str::FromStr,
    };

    proptest! {
        /// Check passing num_bigint::BigInt to OCaml `Num` and back through string representations
        #[test]
        fn add_num(a: Vec<u8>, b: Vec<u8>) {
            let a = BigInt::from_signed_bytes_be(&a);
            let b = BigInt::from_signed_bytes_be(&b);
            let c_true = &a + &b;

            // calculate by passing into OCaml
            let c = {
                let a_str = a.to_string();
                let b_str = b.to_string();
                match RT.lock().add_num(a_str.clone(), b_str.clone()).unwrap() {
                    OCamlString::String(s) => BigInt::from_str(&s).unwrap(),
                    OCamlString::Vec(v) => panic!("{:?}", v)
                }
            };

            assert_eq!(c_true, c);
        }
    }
}
