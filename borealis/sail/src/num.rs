//! Wrappers around ocaml `Num` arbitrary-precision integer and rational
//! arithmetic library

use {
    crate::{
        visitor::{Visitor, Walkable},
        wrapper::{bigint_to_string, string_to_bigint},
    },
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue, Value},
    serde::{Deserialize, Serialize},
    std::str::FromStr,
};

/// Arbitrary precision number from `Num.num` OCaml type
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Num {
    /// Integer
    Int(Int),
    /// Big integer
    BigInt(BigInt),
    /// Ratio of big integers
    Ratio(Ratio),
}

/// Signed big integer from `Nat_big_num.num` OCaml type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BigInt(pub num_bigint::BigInt);

impl Walkable for BigInt {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

unsafe impl FromValue for BigInt {
    fn from_value(v: Value) -> Self {
        let rt = unsafe { ocaml::Runtime::recover_handle() };

        let s = unsafe { bigint_to_string(rt, v) }.unwrap().unwrap();

        Self(num_bigint::BigInt::from_str(&s).unwrap())
    }
}

unsafe impl ToValue for BigInt {
    fn to_value(&self, rt: &ocaml::Runtime) -> Value {
        let s = num_bigint::BigInt::to_string(&self.0);
        unsafe { string_to_bigint(rt, s) }.unwrap().unwrap()
    }
}

impl DeepSizeOf for BigInt {
    fn deep_size_of_children(&self, _: &mut deepsize::Context) -> usize {
        (self.0.bits() / 8) as usize
    }
}

/// Ratio of big integers from `num.Ratio.ratio` OCaml type
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
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
        crate::{num::BigInt, runtime::RT, wrapper},
        proptest::prelude::*,
    };

    proptest! {
        /// Check passing num_bigint::BigInt to OCaml `Num` and back through string representations
        #[test]
        fn add_num(a: Vec<u8>, b: Vec<u8>) {
            let a = num_bigint::BigInt::from_signed_bytes_be(&a);
            let b = num_bigint::BigInt::from_signed_bytes_be(&b);
            let c_true = &a + &b;

            // calculate by passing into OCaml
            let c = RT
                .lock()
                .execute(move |rt| {
                    unsafe { wrapper::add_num(rt, BigInt(a), BigInt(b)) }
                        .unwrap()
                        .unwrap()
                })
                .unwrap()
                .0;

            assert_eq!(c_true, c);
        }
    }
}
