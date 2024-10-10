use {
    alloc::vec::Vec,
    core::{
        fmt::Debug,
        hash::{Hash, Hasher},
        marker::PhantomData,
    },
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Arena<T> {
    vec: Vec<T>,
    #[cfg(debug_assertions)]
    id: crate::id::Id,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            #[cfg(debug_assertions)]
            id: crate::id::Id::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> Ref<T> {
        self.vec.push(t);
        Ref {
            index: self.vec.len() - 1,
            #[cfg(debug_assertions)]
            arena: self.id,
            _phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> Vec<T> {
        self.vec
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ref<T> {
    index: usize,
    #[cfg(debug_assertions)]
    arena: crate::id::Id,
    _phantom: PhantomData<T>,
}

impl<T> Ref<T> {
    pub fn get_mut<'reph, 'arena: 'reph>(&self, arena: &'arena mut Arena<T>) -> &'reph mut T {
        #[cfg(debug_assertions)]
        assert_eq!(arena.id, self.arena);

        arena.vec.get_mut(self.index).unwrap()
    }

    pub fn get<'reph, 'arena: 'reph>(&self, arena: &'arena Arena<T>) -> &'reph T {
        #[cfg(debug_assertions)]
        assert_eq!(arena.id, self.arena);

        arena.vec.get(self.index).unwrap()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T> Hash for Ref<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Eq for Ref<T> {}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ref<T> {}

impl<T> Debug for Ref<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if cfg!(debug_assertions) {
            write!(f, "ref{} (arena {})", self.index(), self.arena)
        } else {
            write!(f, "ref{}", self.index())
        }
    }
}
