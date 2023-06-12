//! Shared reference, hashed by address

use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    ptr,
    rc::Rc,
};

/// Wrapper around a shared, mutable memory location, hashed by address
///
/// Used so that you can place `Rc<RefCell<T>>`'s into a `HashMap` directly,
/// instead of needing to associate some new key with each `T`.
#[derive(Debug, Clone)]
pub struct SharedKey<T>(Rc<RefCell<T>>);

impl<T> From<Rc<RefCell<T>>> for SharedKey<T> {
    fn from(inner: Rc<RefCell<T>>) -> Self {
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
        Rc::ptr_eq(&self.0, &other.0)
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
        super::SharedKey,
        pretty_assertions::{assert_eq, assert_ne},
        std::{
            cell::RefCell,
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
            rc::Rc,
        },
    };

    #[test]
    fn not_equal() {
        let a = SharedKey(Rc::new(RefCell::new(0u8)));
        let b = SharedKey(Rc::new(RefCell::new(0u8)));

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
        let value = Rc::new(RefCell::new(0u8));
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
