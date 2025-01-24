//! Borealis Object Of Machine, Internal intermediate representation used to
//! convert JIB AST to GenC AST

#![allow(missing_docs)]

use common::boom::Bit;

pub mod control_flow;
pub mod passes;
pub mod pretty_print;
pub mod visitor;

/// Converts a sequence of bits to an integer
pub fn bits_to_int<B: AsRef<[Bit]>>(bits: B) -> u64 {
    let bits = bits.as_ref();

    assert!(bits.iter().all(Bit::is_fixed));

    bits.iter().rev().fold(0, |acc, bit| acc << 1 | bit.value())
}
