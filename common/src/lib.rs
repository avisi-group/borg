#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub use hashbrown::hash_map::Entry;
use {
    alloc::{string::String, vec::Vec},
    core::hash::BuildHasherDefault,
    serde::{Deserialize, Serialize},
};

pub mod arena;
pub mod id;
pub mod intern;
pub mod mask;
pub mod rudder;
pub mod width_helpers;

pub type Hasher = twox_hash::XxHash64;

/// HashMap with non-default hasher
pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<Hasher>>;

/// HashSet with non-default hasher
pub type HashSet<T> = hashbrown::HashSet<T, BuildHasherDefault<Hasher>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum TestConfig {
    // Do not run tests
    None,
    // Only run the specified tests
    Include(Vec<String>),
    // Run all tests except those specified
    Exclude(Vec<String>),
    // Run all tests
    All,
}
