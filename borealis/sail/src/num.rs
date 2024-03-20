//! Wrappers around ocaml `Num` arbitrary-precision integer and rational
//! arithmetic library

use {
    crate::ffi::{bigint_to_string, string_to_bigint},
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue, Value},
    rkyv::{
        ser::ScratchSpace,
        vec::{ArchivedVec, VecResolver},
        Fallible,
    },
    std::str::FromStr,
};

/// Arbitrary precision number from `Num.num` OCaml type
#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum Num {
    /// Integer
    Int(Int),
    /// Big integer
    BigInt(BigInt),
    /// Ratio of big integers
    Ratio(Ratio),
}

/// Ratio of big integers from `num.Ratio.ratio` OCaml type
#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub struct Ratio {
    /// Numerator
    pub numerator: BigInt,
    /// Denominator
    pub denominator: BigInt,
    /// Normalized
    pub normalized: bool,
}

/// Signed big integer from `Nat_big_num.num` OCaml type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BigInt(pub num_bigint::BigInt);

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

impl rkyv::Archive for BigInt {
    type Archived = ArchivedVec<u8>;

    type Resolver = VecResolver;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.0.to_signed_bytes_be().as_slice(), pos, resolver, out);
    }
}

impl<S: Fallible + ScratchSpace + rkyv::ser::Serializer> rkyv::Serialize<S> for BigInt {
    fn serialize(
        &self,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as rkyv::Fallible>::Error> {
        ArchivedVec::serialize_from_slice(self.0.to_signed_bytes_be().as_slice(), serializer)
    }
}

impl<D: Fallible> rkyv::Deserialize<BigInt, D> for ArchivedVec<u8> {
    fn deserialize(&self, deserializer: &mut D) -> Result<BigInt, <D as Fallible>::Error> {
        ArchivedVec::deserialize(&self, deserializer)
            .map(|digits: Vec<u8>| BigInt(num_bigint::BigInt::from_signed_bytes_be(&digits)))
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{ffi, num::BigInt, runtime::RT},
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
                    unsafe { ffi::add_num(rt, BigInt(a), BigInt(b)) }
                        .unwrap()
                        .unwrap()
                })
                .unwrap()
                .0;

            assert_eq!(c_true, c);
        }
    }
}
