//! Sail2 language parser

use {
    ocaml::{List, Runtime},
    once_cell::sync::Lazy,
};

pub mod ast;
//pub mod error;
pub mod parser;
pub mod span;

/// OCaml runtime handle, initialised on first access
///
/// *Every* function referencing RT must use &*RT or it will possibly not be initialised and cause
/// a "boxroot is not setup" error. This error will be hard to diagnose as it will be dependent on
/// the order that other (correctly dereferencing RT and thus initialising the runtime) functions
/// are called. Need to investigate how this can be prevented.
static RT: Lazy<Runtime> = Lazy::new(ocaml::runtime::init);

ocaml::import! {
    fn internal_dedup(l: List<i32>) -> List<i32>;
}

pub fn dedup(list: Vec<i32>) -> Vec<i32> {
    let rt = &*RT;

    let mut l = List::empty();

    for element in list {
        l = unsafe { l.add(rt, &element) };
    }

    unsafe { internal_dedup(rt, l) }.unwrap().into_vec()
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

            let mut out = dedup(v);
            out.sort();
            assert_eq!(out, v_d);
        }
    }
}
