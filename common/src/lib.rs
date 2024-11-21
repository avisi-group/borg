#![no_std]

extern crate alloc;

pub use hashbrown::hash_map::Entry;
use {core::hash::BuildHasherDefault, twox_hash::XxHash64};

pub mod arena;
pub mod id;
pub mod intern;
pub mod mask;
pub mod rudder;
pub mod width_helpers;

pub type Hasher = XxHash64;

/// HashMap with non-default hasher
pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<Hasher>>;

/// HashSet with non-default hasher
pub type HashSet<T> = hashbrown::HashSet<T, BuildHasherDefault<Hasher>>;
