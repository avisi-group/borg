//! Trait and derive macro for uniquely identifying nodes

use core::{
    fmt::{self, Debug, Display, LowerHex},
    sync::atomic::{AtomicU32, Ordering},
};

/// Unique identifier
#[derive(Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Id(u32);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    /// Creates a new, unique ID
    pub fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        let num = COUNTER.fetch_add(1, Ordering::SeqCst);

        if num == u32::MAX {
            panic!("COUNTER overflowed");
        }

        Self(num)
    }
}

impl LowerHex for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({:x})", self)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
