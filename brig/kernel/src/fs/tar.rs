use {
    crate::{
        devices::BlockDevice,
        fs::{File, Filesystem},
    },
    alloc::{string::String, vec::Vec},
    tar_no_std::{ArchiveEntry, TarArchive},
};

pub struct TarFilesystem<'device, B> {
    dev: &'device mut B,
    archive: TarArchive,
}

impl<'device, 'fs, B: BlockDevice> TarFilesystem<'device, B> {
    pub fn mount(dev: &'device mut B) -> Self {
        // read entire file into memory and create tar archive
        let archive = {
            let mut buf = alloc::vec![0u8; dev.size()];
            dev.read(&mut buf, 0).unwrap();
            TarArchive::new(buf.into())
        };

        Self { dev, archive }
    }

    pub fn open<S: AsRef<str>>(&'fs mut self, filename: S) -> TarFile<'device, 'fs, B> {
        let entry = self
            .archive
            .entries()
            .find(|e| *e.filename() == *(filename.as_ref().trim_start_matches("/")))
            .unwrap();

        TarFile { fs: self, entry }
    }
}

pub struct TarFile<'device, 'fs, B> {
    fs: &'fs TarFilesystem<'device, B>,
    entry: ArchiveEntry<'fs>,
}

impl<'device, 'fs, B: BlockDevice> TarFile<'device, 'fs, B> {
    pub fn read(&self, buffer: &mut [u8], offset: usize) {
        buffer.copy_from_slice(&self.entry.data()[offset..]);
    }

    pub fn read_to_vec(&self) -> Vec<u8> {
        let mut buf = alloc::vec![0u8; self.size()];
        self.read(&mut buf, 0);
        buf
    }

    pub fn size(&self) -> usize {
        self.entry.size()
    }
}

// impl<'device, 'fs, B: BlockDevice> File<'fs> for TarFile<'device, 'fs, B> {
//     fn read(&self, _buffer: &mut [u8], _offset: usize) {
//         todo!()
//     }
// }

// impl<'device, 'fs, B: BlockDevice> Filesystem<'fs, TarFile<'device, 'fs, B>>
//     for TarFilesystem<'device, B>
// {
//     fn open<S: AsRef<str>>(&'fs mut self, filename: S) ->
// Result<TarFile<'device, 'fs, B>, ()> {         let mut buf = vec![0u8;
// self.dev.size()];         self.dev.read(&mut buf, 0);

//         let tar = TarArchiveRef::new(&buf);
//         tar.entries()
//             .find(|e| *e.filename() ==
// *(filename.as_ref().trim_start_matches("./")))             .ok_or(())?;
//         // .ok_or(ConfigLoadError::FileNotFoundInTar(path.to_owned()));

//         Ok(TarFile {
//             fs: &self,
//             _offset: 0,
//             _size: 0,
//         })
//     }
// }
