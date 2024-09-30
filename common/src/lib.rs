#![no_std]

use {core::hash::BuildHasherDefault, twox_hash::XxHash64};

pub use hashbrown::hash_map::Entry;

/// HashMap with non-default hasher
pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<XxHash64>>;

/// HashSet with non-default hasher
pub type HashSet<T> = hashbrown::HashSet<T, BuildHasherDefault<XxHash64>>;
