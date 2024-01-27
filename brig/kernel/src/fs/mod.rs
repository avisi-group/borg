use alloc::{string::String, sync::Arc};

use crate::devices::BlockDevice;

pub trait Filesystem<F: File> {
    fn open(&self, filename: String) -> Result<F, ()>;
}

pub trait File {
    fn read(&self, buffer: &mut [u8], offset: usize);
}

struct TarFilesystem {
    dev: Arc<dyn BlockDevice>
}

struct TarFile
{
    parent: usize,
    offset: usize,
    size: usize
}

impl File for TarFile {
    fn read(&self, buffer: &mut [u8], offset: usize) {
        todo!()
    }
}

impl Filesystem<TarFile> for TarFilesystem {
    fn open(&self, filename: String) -> Result<TarFile, ()> {
        Ok(TarFile {
            parent: 0,
            offset: 0,
            size: 0
        })
    }
}

impl TarFilesystem {
    pub fn mount(dev: Arc<dyn BlockDevice>) -> Self {
        Self {
            dev
        }
    }
}
