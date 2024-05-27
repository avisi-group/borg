use {
    alloc::{string::String, vec::Vec},
    thiserror_core as thiserror,
};

pub mod tar;

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// File or directory {0:?} was not found
    NotFound(String),
    /// IO error occurred while reading or writing
    #[allow(unused)]
    Io,
}

/// Filesystem
///
/// `'fs` is the lifetime of the filesystem, and is used by implementations of
/// the `File` trait to hold a reference to the parent filesystem.
pub trait Filesystem<'fs, F: File<'fs>> {
    fn open<S: AsRef<str>>(&'fs mut self, filename: S) -> Result<F, Error>;
}

pub trait File<'fs> {
    fn read(&self, buffer: &mut [u8], offset: usize) -> Result<(), Error>;

    fn size(&self) -> usize;

    fn read_to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut buf = alloc::vec![0u8; self.size()];
        self.read(&mut buf, 0)?;
        Ok(buf)
    }
}
