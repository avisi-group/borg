#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

pub mod ast;
pub mod error;
pub mod json;
pub mod load;
pub mod num;
pub mod parse_ast;
pub mod runtime;
pub mod type_check;
pub mod types;
pub mod visitor;
mod wrapper;

#[cfg(test)]
mod tests {
    use {
        crate::{runtime::RT, wrapper::util_dedup},
        proptest::{bits, collection::vec, prelude::*},
    };

    fn dedup(list: Vec<i32>) -> Vec<i32> {
        RT.lock()
            .execute(|rt| {
                unsafe { util_dedup(rt, list.into_iter().collect()) }
                    .unwrap()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<_>>()
            })
            .unwrap()
    }

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test(v in vec(bits::i32::ANY, 0..1000)) {
            let mut v_d = v.clone();
            v_d.sort();

            v_d.dedup();

            let mut out = dedup(v);
            out.sort();

            assert_eq!(out, v_d);
        }
    }
}
