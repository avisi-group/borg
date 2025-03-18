#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub use hashbrown::hash_map::Entry;
use {
    alloc::{string::String, vec::Vec},
    serde::{Deserialize, Serialize},
};

pub mod arena;
pub mod id;
pub mod intern;
pub mod mask;
pub mod rudder;
pub mod width_helpers;

pub mod modname {
    use {
        alloc::{self, alloc::Global},
        core::{alloc::Allocator, hash::BuildHasherDefault},
    };

    pub type Hasher = twox_hash::XxHash64;

    /// HashMap with XxHash64 hasher and custom allocator
    pub type HashMapA<K, V, A> = hashbrown::HashMap<K, V, BuildHasherDefault<Hasher>, A>;

    pub fn hashmap_in<K, V, A: Allocator>(allocator: A) -> HashMapA<K, V, A> {
        HashMapA::with_hasher_in(Default::default(), allocator)
    }

    /// HashSet with XxHash64 hasher and custom allocator
    pub type HashSetA<T, A> = hashbrown::HashSet<T, BuildHasherDefault<Hasher>, A>;

    pub fn hashset_in<T, A: Allocator>(allocator: A) -> HashSetA<T, A> {
        HashSetA::with_hasher_in(Default::default(), allocator)
    }

    /// HashMap with XxHash64 hasher
    pub type HashMap<K, V> = HashMapA<K, V, Global>;

    /// HashSet with XxHash64 hasher
    pub type HashSet<T> = HashSetA<T, Global>;
}

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
