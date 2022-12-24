//! Wrappers around ocaml `Num` arbitrary-precision integer and rational
//! arithmetic library

use {
    crate::{
        visitor::{Visitor, Walkable},
        wrapper::internal_bigint_to_string,
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

        let s = unsafe { internal_bigint_to_string(rt, v) }
            .unwrap()
            .unwrap();

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
        crate::{wrapper::internal_add_num, RT},
        num_bigint::BigInt,
        proptest::prelude::*,
        std::str::FromStr,
    };

    fn runtime_add_num(a: String, b: String) -> String {
        RT.lock()
            .execute(move |rt| unsafe { internal_add_num(rt, a, b) }.unwrap().unwrap())
            .unwrap()
    }

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
                BigInt::from_str(&runtime_add_num(a_str.clone(), b_str.clone())).unwrap()
            };

            assert_eq!(c_true, c);
        }
    }
}
