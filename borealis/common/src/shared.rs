//! Abstract shared type

use {
    parking_lot::RwLock,
    std::{
        collections::hash_map::DefaultHasher,
        fmt::{self, Display, Formatter},
        hash::{Hash, Hasher},
        ops::{Deref, DerefMut},
        ptr,
        sync::Arc,
    },
};

/// Shared data
///
/// Abstracts over Rc, Arc, Mutex, RwLock
#[derive(Debug, Default)]
pub struct Shared<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Shared<T> {
    /// Create a new shared data
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(data)),
        }
    }

    /// Get an immutable reference to the data
    pub fn get(&self) -> impl Deref<Target = T> + '_ {
        self.inner.read()
    }

    /// Get a mutable reference to the data
    pub fn get_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.inner.write()
    }

    /// Returns a raw pointer to the data
    pub fn as_ptr(&self) -> *const T {
        self.inner.data_ptr()
    }

    /// Returns `true` if the two `Shared`s point to the same allocation
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        Arc::ptr_eq(&this.inner, &other.inner)
    }

    /// Creates a new weak pointer to the allocation
    pub fn downgrade(&self) -> Weak<T> {
        Weak {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

/// Weak reference to a `Shared``
#[derive(Debug)]
pub struct Weak<T> {
    inner: std::sync::Weak<RwLock<T>>,
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self {
            inner: std::sync::Weak::clone(&self.inner),
        }
    }
}

impl<T> Weak<T> {
    /// Upgrades the `Weak`` pointer to a `Shared`
    pub fn upgrade(&self) -> Option<Shared<T>> {
        self.inner.upgrade().map(|inner| Shared { inner })
    }

    /// Returns a raw pointer to the data
    pub fn as_ptr(&self) -> *const RwLock<T> {
        self.inner.as_ptr()
    }

    /// Returns `true` if the two `Shared`s point to the same allocation
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        std::sync::Weak::ptr_eq(&this.inner, &other.inner)
    }
}

/// Wrapper around a shared, mutable memory location, hashed by address
///
/// Used so that you can place `Rc<RefCell<T>>`'s into a `HashMap` directly,
/// instead of needing to associate some new key with each `T`.
#[derive(Debug, Clone)]
pub struct SharedKey<T>(Shared<T>);

impl<T> From<Shared<T>> for SharedKey<T> {
    fn from(inner: Shared<T>) -> Self {
        Self(inner)
    }
}

impl<T> Hash for SharedKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0.as_ptr(), state)
    }
}

impl<T> PartialEq for SharedKey<T> {
    fn eq(&self, other: &Self) -> bool {
        Shared::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for SharedKey<T> {}

impl<T> Display for SharedKey<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        write!(f, "{:016X}", state.finish())
    }
}

#[cfg(test)]
mod test {
    use {
        super::{Shared, SharedKey},
        pretty_assertions::{assert_eq, assert_ne},
        std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        },
    };

    #[test]
    fn not_equal() {
        let a = SharedKey(Shared::new(0u8));
        let b = SharedKey(Shared::new(0u8));

        assert_ne!(a, b);

        let state = DefaultHasher::new();

        let a_hash = {
            let mut state = state.clone();
            a.hash(&mut state);
            state.finish()
        };

        let b_hash = {
            let mut state = state.clone();
            b.hash(&mut state);
            state.finish()
        };

        assert_ne!(a_hash, b_hash);
    }

    #[test]
    fn equal() {
        let value = Shared::new(0u8);
        let a = SharedKey(value.clone());
        let b = SharedKey(value);

        assert_eq!(a, b);

        let state = DefaultHasher::new();

        let a_hash = {
            let mut state = state.clone();
            a.hash(&mut state);
            state.finish()
        };

        let b_hash = {
            let mut state = state.clone();
            b.hash(&mut state);
            state.finish()
        };

        assert_eq!(a_hash, b_hash);
    }
}
