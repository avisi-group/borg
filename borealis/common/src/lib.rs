#![deny(missing_docs)]

//! Logic and structures shared among many crates in workspace

use {std::hash::BuildHasherDefault, twox_hash::XxHash64};

pub mod identifiable;
pub mod intern;
pub mod shared_key;

/// HashMap with non-default hasher
pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<XxHash64>>;

/// HashSet with non-default hasher
pub type HashSet<T> = std::collections::HashSet<T, BuildHasherDefault<XxHash64>>;
