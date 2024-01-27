use {
    crate::{
        devices::BlockDevice,
        fs::{File, Filesystem},
    },
    alloc::{string::String, sync::Arc},
};

struct TarFilesystem {
    dev: Arc<dyn BlockDevice>,
}

struct TarFile {
    _dev: Arc<dyn BlockDevice>,
    _offset: usize,
    _size: usize,
}

impl File for TarFile {
    fn read(&self, _buffer: &mut [u8], _offset: usize) {
        todo!()
    }
}

impl Filesystem<TarFile> for TarFilesystem {
    fn open(&self, _filename: String) -> Result<TarFile, ()> {
        Ok(TarFile {
            _dev: self.dev.clone(),
            _offset: 0,
            _size: 0,
        })
    }
}

impl TarFilesystem {
    pub fn _mount(dev: Arc<dyn BlockDevice>) -> Self {
        Self { dev }
    }
}
