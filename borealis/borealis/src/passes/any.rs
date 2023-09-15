//! Extension trait for `any` method on iterators of bools.

/// Extension trait for `any` method on iterators of bools.
pub trait AnyExt: Iterator<Item = bool> {
    /// Non-short circuiting, boolean-only version of `Iterator::any`.
    fn any(self) -> bool
    where
        Self: Sized,
    {
        let mut flag = false;

        self.for_each(|b| {
            flag |= b;
        });
        flag
    }
}

impl<I: Iterator<Item = bool>> AnyExt for I {}
