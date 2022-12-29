//! Trait and derive macro for uniquely identifying nodes

use std::sync::atomic::{AtomicU32, Ordering};

pub use borealis_macro::identifiable_fromvalue;

#[doc(hidden)]
/// Gets a new, unique, u64
pub fn unique_id() -> u32 {
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    let num = COUNTER.fetch_add(1, Ordering::SeqCst);

    if num == u32::MAX {
        panic!("AtomicU32 COUNTER overflowed");
    }

    num
}

/// Trait for identifying items
pub trait Identifiable {
    /// Gets the unique identifier
    fn id(&self) -> u32;
}
