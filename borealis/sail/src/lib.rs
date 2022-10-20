//! Rust interface to `Sail` compiler library

use {
    crate::{error::Error, runtime::Runtime},
    once_cell::sync::Lazy,
    parking_lot::Mutex,
};

pub mod ast;
pub mod error;
pub mod parser;
mod runtime;
pub mod span;

/// Global runtime shared by all public functions
static RT: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(Runtime::new()));

/// Removes duplicate values in the supplied Vec
pub fn dedup(list: Vec<i32>) -> Result<Vec<i32>, Error> {
    RT.lock().dedup(list)
}

#[cfg(test)]
mod tests {
    use {
        crate::dedup,
        proptest::{bits, collection::vec, prelude::*},
    };

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test(v in vec(bits::i32::ANY, 0..1000)) {
            let mut v_d = v.clone();
            v_d.sort();
            v_d.dedup();

            let mut out = dedup(v).unwrap();
            out.sort();
            assert_eq!(out, v_d);
        }
    }
}
