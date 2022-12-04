//! Trait and derive macro for uniquely identifying nodes

use core::panic;
use std::sync::atomic::{AtomicU64, Ordering};

pub use borealis_macro::identifiable_fromvalue;

#[doc(hidden)]
/// Gets a new, unique, u64
pub fn unique_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let num = COUNTER.fetch_add(1, Ordering::SeqCst);

    if num == u64::MAX {
        // how on earth did you create more than 18446744073709551615 identifiers?
        panic!("AtomicU64 COUNTER overflowed");
    }

    num
}

/// Trait for identifying items
pub trait Identifiable {
    /// Gets the unique identifier
    fn id(&self) -> u64;
}
