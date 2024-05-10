use {
    crate::arch::{
        x86::memory::{AlignedAllocator, VirtualMemoryArea},
        PAGE_SIZE,
    },
    alloc::{boxed::Box, vec::Vec},
    core::pin::Pin,
    elfloader::{
        arch::x86_64::RelocationTypes::{R_AMD64_64, R_AMD64_GLOB_DAT, R_AMD64_RELATIVE},
        xmas_elf::program,
        ElfBinary, ElfLoader, ElfLoaderErr, Flags, ProgramHeader, RelocationEntry, RelocationType,
        VAddr,
    },
    x86_64::{structures::paging::PageTableFlags, VirtAddr},
};

pub struct SharedObject {
    allocation: Pin<Box<[u8], AlignedAllocator<PAGE_SIZE>>>,
}

impl SharedObject {
    /// Loads the shared object into memory from an ELF
    pub fn from_elf(elf: &ElfBinary) -> Self {
        // calculate (aligned) highest virtual address in *loaded* ELF file
        let size = usize::try_from(
            elf.program_headers()
                .filter(|header| matches!(header.get_type().unwrap(), program::Type::Load))
                .map(|header| header.virtual_addr() + header.mem_size())
                .max()
                .unwrap(),
        )
        .unwrap()
        .next_multiple_of(PAGE_SIZE);

        let mut obj = {
            // todo: if someone finds a better way of doing this please let me know
            let mut vec = Vec::with_capacity_in(size, AlignedAllocator::<PAGE_SIZE>);
            for _ in 0..size {
                vec.push(0);
            }
            let allocation = Pin::new(vec.into_boxed_slice());

            Self { allocation }
        };

        let range = obj.allocation.as_ptr_range();
        let virt_range = VirtAddr::from_ptr(range.start)..VirtAddr::from_ptr(range.end);

        VirtualMemoryArea::current().update_flags_range(
            virt_range,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE, // removing  "NOEXECUTE" flag
        );

        elf.load(&mut SharedObjectLoader::new(&elf, &mut obj))
            .unwrap();

        obj
    }

    /// Finds where a given address in the ELF has been allocated in guest
    /// virtual memory
    pub fn translate_virt_addr(&self, elf_address: u64) -> VirtAddr {
        VirtAddr::new(self.allocation.as_ptr() as u64 + elf_address)
    }
}

struct SharedObjectLoader<'so> {
    symbol_values: Vec<u64>,
    object: &'so mut SharedObject,
}

impl<'so> SharedObjectLoader<'so> {
    fn new(binary: &ElfBinary<'_>, object: &'so mut SharedObject) -> Self {
        let mut symbol_values = Vec::new();
        binary
            .for_each_symbol(|s| symbol_values.push(object.translate_virt_addr(s.value()).as_u64()))
            .unwrap();

        Self {
            symbol_values,
            object,
        }
    }
}

impl<'so> ElfLoader for SharedObjectLoader<'so> {
    fn allocate(&mut self, header: ProgramHeader) -> Result<(), ElfLoaderErr> {
        // no-op becuase we do one large allocation ahead of time

        // assert header resides within already-allocated range
        let header_max = usize::try_from(header.virtual_addr() + header.mem_size()).unwrap();
        assert!(header_max < self.object.allocation.len());

        Ok(())
    }

    fn load(&mut self, _flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
        let base = usize::try_from(base).unwrap();

        self.object.allocation[base..base + region.len()].copy_from_slice(region);

        Ok(())
    }

    fn relocate(&mut self, entry: RelocationEntry) -> Result<(), ElfLoaderErr> {
        match entry.rtype {
            // B + A
            // This is a relative relocation, add the offset (where we put our
            // binary in the vspace) to the addend and we're done.
            RelocationType::x86_64(R_AMD64_RELATIVE) => {
                // this type requires addend to be present
                let addend = entry.addend.unwrap();

                // find address we're writing into
                // * find which region entry.offset belongs to
                // * add offset to region aligned_base
                // * now have pointer to offset within its region
                let alloc_offset = self.object.translate_virt_addr(entry.offset);

                // calculate data we are writing
                // * find region addend belongs to
                // * add addend to region aligned_base
                let alloc_addend = self.object.translate_virt_addr(addend);

                unsafe { *alloc_offset.as_mut_ptr() = alloc_addend.as_u64() };

                Ok(())
            }
            // S
            RelocationType::x86_64(R_AMD64_GLOB_DAT) => {
                let value = self.symbol_values[usize::try_from(entry.index).unwrap()];

                unsafe {
                    *self.object.translate_virt_addr(entry.offset).as_mut_ptr() = value;
                };

                Ok(())
            }
            // S + A
            RelocationType::x86_64(R_AMD64_64) => {
                let value = self.symbol_values[usize::try_from(entry.index).unwrap()];

                let addend = entry.addend.unwrap();

                unsafe {
                    *self.object.translate_virt_addr(entry.offset).as_mut_ptr() = value + addend
                };

                Ok(())
            }
            _ => {
                panic!("unimplemented rtype: {:#x?}", entry.rtype);
            }
        }
    }
}
