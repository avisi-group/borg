use alloc::string::String;

pub mod tar;

pub trait Filesystem<F: File> {
    fn open(&self, filename: String) -> Result<F, ()>;
}

pub trait File {
    fn read(&self, buffer: &mut [u8], offset: usize);
}
