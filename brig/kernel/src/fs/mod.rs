pub mod tar;

/// Filesystem
///
/// `'fs` is the lifetime of the filesystem, and is used by implementations of
/// the `File` trait to hold a reference to the parent filesystem.
pub trait Filesystem<'fs, F: File<'fs>> {
    fn open<S: AsRef<str>>(&'fs mut self, filename: S) -> Result<F, ()>;
}

pub trait File<'fs> {
    fn read(&self, buffer: &mut [u8], offset: usize);
}
