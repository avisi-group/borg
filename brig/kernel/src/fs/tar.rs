use {
    crate::{
        devices::BlockDevice,
        fs::{Error, File, Filesystem},
    },
    alloc::borrow::ToOwned,
    tar_no_std::{ArchiveEntry, TarArchive},
};

pub struct TarFilesystem<'device, B> {
    _dev: &'device mut B,
    archive: TarArchive,
}

impl<'device, B: BlockDevice> TarFilesystem<'device, B> {
    pub fn mount(dev: &'device mut B) -> Self {
        // read entire file into memory and create tar archive
        let archive = {
            let mut buf = alloc::vec![0u8; dev.size()];
            dev.read(&mut buf, 0).unwrap();
            TarArchive::new(buf.into())
        };

        Self { _dev: dev, archive }
    }
}

impl<'device, 'fs, B: BlockDevice> Filesystem<'fs, TarFile<'fs>> for TarFilesystem<'device, B> {
    fn open<S: AsRef<str>>(&'fs mut self, filename: S) -> Result<TarFile<'fs>, Error> {
        let entry = self
            .archive
            .entries()
            .find(|e| *e.filename() == *(filename.as_ref().trim_start_matches('/')))
            .ok_or(Error::NotFound(filename.as_ref().to_owned()))?;

        Ok(TarFile { entry })
    }
}

pub struct TarFile<'fs> {
    entry: ArchiveEntry<'fs>,
}

impl<'fs> File<'fs> for TarFile<'fs> {
    fn read(&self, buffer: &mut [u8], offset: usize) -> Result<(), Error> {
        buffer.copy_from_slice(&self.entry.data()[offset..]);
        Ok(())
    }

    fn size(&self) -> usize {
        self.entry.size()
    }
}
