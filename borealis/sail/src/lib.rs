#![deny(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::error::Error,
    ocaml::{List, Runtime},
    once_cell::sync::Lazy,
    parking_lot::RwLock,
};

pub mod ast;
pub mod error;
pub mod parser;
pub mod span;

/// OCaml runtime handle, initialised on first access
///
/// *Every* function referencing RT must either begin with RT.write() or it will possibly not be
/// initialised and cause a "boxroot is not setup" error. This error will be hard to diagnose as
/// it will be dependent on the order that other (correctly dereferencing RT and thus initialising
/// the runtime) functions are called. Need to investigate how this can be prevented.
static RT: Lazy<RwLock<Runtime>> = Lazy::new(|| RwLock::new(ocaml::runtime::init()));

ocaml::import! {
    fn internal_util_dedup(l: List<i32>) -> List<i32>;
}

/// Removes duplicate values in the supplied Vec
pub fn dedup(list: Vec<i32>) -> Result<Vec<i32>, Error> {
    RT.write();

    let mut l = List::empty();

    for element in list {
        l = unsafe { l.add(&RT.read(), &element) };
    }

    Ok(unsafe { internal_util_dedup(&RT.read(), l) }?.into_vec())
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
            v_d.dedup();
            v_d.sort();

            let mut out = dedup(v).unwrap();
            out.sort();
            assert_eq!(out, v_d);
        }
    }
}
