use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Arena<T> {
    vec: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn insert(&mut self, t: T) -> Ref<T> {
        self.vec.push(t);
        Ref {
            index: self.vec.len() - 1,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Ref<T> {
    index: usize,
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Eq for Ref<T> {}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for Ref<T> {}

impl<T> Ref<T> {
    pub fn get_mut<'s, 'c: 's>(&self, arena: &'c mut Arena<T>) -> &'s mut T {
        arena.vec.get_mut(self.index).unwrap()
    }

    pub fn get<'s, 'c: 's>(&self, arena: &'c Arena<T>) -> &'s T {
        arena.vec.get(self.index).unwrap()
    }
}
