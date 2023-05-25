pub trait AnyExt: Iterator<Item = bool> {
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
