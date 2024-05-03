use {
    crate::{
        devices::SharedDevice,
        fs::{
            tar::{TarFile, TarFilesystem},
            File, Filesystem,
        },
    },
    alloc::{alloc::alloc_zeroed, vec::Vec},
    core::alloc::Layout,
    elfloader::{
        arch::x86_64::RelocationTypes::{R_AMD64_64, R_AMD64_GLOB_DAT, R_AMD64_RELATIVE},
        ElfBinary, ElfLoader, ElfLoaderErr, Flags, LoadableHeaders, RelocationEntry,
        RelocationType, VAddr,
    },
    plugins_api::PluginHost,
    x86_64::VirtAddr,
    xmas_elf::program::Type,
};

struct Loader<'binary, 'contents> {
    binary: &'binary ElfBinary<'contents>,

    symbol_values: Vec<u64>,
    // address -> (index, size)
    // section_addresses: BTreeMap<u64, (u16, u64)>,
    allocation: *mut u8,
}

impl<'binary, 'contents> Loader<'binary, 'contents> {
    pub fn new(binary: &'binary ElfBinary<'contents>, allocation: *mut u8) -> Self {
        // load symbol values
        let mut symbol_values = Vec::new();
        binary
            .for_each_symbol(|s| symbol_values.push(s.value()))
            .unwrap();

        let mut this = Self {
            binary,
            symbol_values,
            allocation,
        };

        // need to translate symbol values
        this.symbol_values = this
            .symbol_values
            .iter()
            .map(|value| (this.translate_virt_addr(*value).as_u64()))
            .collect();

        this
    }

    /// Finds where a given address in the ELF has been allocated in guest
    /// virtual memory
    pub fn translate_virt_addr(&self, elf_address: u64) -> VirtAddr {
        VirtAddr::new(self.allocation as u64 + elf_address)
    }
}

impl<'binary, 'contents> ElfLoader for Loader<'binary, 'contents> {
    fn allocate(&mut self, _load_headers: LoadableHeaders) -> Result<(), ElfLoaderErr> {
        // todo: asset header resides within already-allocated range
        Ok(())
    }

    fn load(&mut self, flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
        // let Some((&candidate_base, (allocation, size))) = self
        //     .mapping
        //     .upper_bound(core::ops::Bound::Included(&base))
        //     .peek_prev()
        // else {
        //     panic!("could not find allocation for {base:x}");
        // };

        // log::info!("found allocation, base = {:x}, candidate_base = {:x}, allocation = {:x}, region.len() = {:x}, size = {:x}", base,candidate_base, allocation, region.len(), size);

        // // lower bounds check
        // if !(candidate_base..candidate_base + size).contains(&base) {
        //     panic!(
        //         "base {base:x} was not found in candidate region
        // {candidate_base:x}..{:x}",         candidate_base + size
        //     )
        // }

        // // upper bounds check
        // if !(candidate_base..=candidate_base + size)
        //     .contains(&(base + u64::try_from(region.len()).unwrap()))
        // {
        //     panic!("load region exceeded allocation")
        // }

        // let offset = isize::try_from(base).unwrap() -
        // isize::try_from(candidate_base).unwrap();

        let alloc_location = unsafe { self.allocation.offset(base as isize) };

        log::info!("copying region to {:p}", alloc_location);

        unsafe { core::ptr::copy(region.as_ptr(), alloc_location, region.len()) }

        Ok(())
    }

    fn relocate(&mut self, entry: RelocationEntry) -> Result<(), ElfLoaderErr> {
        // log::info!(
        //     "RELOCATE {:x} {:x} {:?}",
        //     entry.offset,
        //     entry.addend.unwrap_or_default(),
        //     entry.rtype
        // );

        match entry.rtype {
            // B + A
            RelocationType::x86_64(R_AMD64_RELATIVE) => {
                // this type requires addend to be present
                let addend = entry.addend.unwrap();

                // find address we're writing into
                // * find which region entry.offset belongs to
                // * add offset to region aligned_base
                // * now have pointer to offset within its region

                let alloc_offset = self.translate_virt_addr(entry.offset);

                // calculate data we are writing
                // * find region addend belongs to
                // * add addend to region aligned_base
                let alloc_addend = self.translate_virt_addr(addend);

                // This is a relative relocation, add the offset (where we put our
                // binary in the vspace) to the addend and we're done.
                // log::info!(
                //     "relocate type offset = {:x}, addend = {:x} => alloc {:x?} {:x?}",
                //     entry.offset,
                //     addend,
                //     alloc_offset,
                //     alloc_addend
                // );

                unsafe { *alloc_offset.as_mut_ptr() = alloc_addend.as_u64() };

                Ok(())
            }
            // S
            RelocationType::x86_64(R_AMD64_GLOB_DAT) => {
                let value = self.symbol_values[usize::try_from(entry.index).unwrap()];

                unsafe {
                    *self.translate_virt_addr(entry.offset).as_mut_ptr() = value;
                };

                Ok(())
            }
            // S + A
            RelocationType::x86_64(R_AMD64_64) => {
                // lookup symbol
                let value = self.symbol_values[usize::try_from(entry.index).unwrap()];
                //  log::info!("AMD64_64 symbol {:x} = {:x}", entry.index, value);

                let addend = entry.addend.unwrap();
                unsafe { *self.translate_virt_addr(entry.offset).as_mut_ptr() = value + addend };

                Ok(())
            }
            _ => {
                panic!("unimplemented rtype: {:#x?}", entry.rtype);
            }
        }
    }
}

pub fn load_all(device: &SharedDevice) {
    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());
    load_one(fs.open("plugins/libtest.so").unwrap());
    load_one(fs.open("plugins/libpl011.so").unwrap());
    panic!("end of plugin load");
}

pub fn load_one<'fs>(file: TarFile<'fs>) {
    let contents = file.read_to_vec().unwrap();
    let binary = ElfBinary::new(contents.as_slice()).unwrap();

    // calculate (aligned) highest virtual address in *loaded* ELF file
    let highest_virt_addr = binary
        .program_headers()
        .filter(|header| matches!(header.get_type().unwrap(), Type::Load))
        .map(|header| ((header.virtual_addr() + header.mem_size()) + header.align()))
        .max()
        .unwrap();

    let alloc =
        unsafe { alloc_zeroed(Layout::from_size_align(highest_virt_addr as usize, 4096).unwrap()) };

    log::info!("elf backing allocation: {alloc:p} {highest_virt_addr:x}");

    let mut loader = Loader::new(&binary, alloc);
    binary.load(&mut loader).expect("Can't load the binary?");

    let header = binary
        .file
        .find_section_by_name(".plugin_entrypoint")
        .unwrap();
    let translated_header_address = loader.translate_virt_addr(header.address());

    log::info!("translated_header_address: {translated_header_address:x}");

    unsafe {
        let entrypoint =
            core::mem::transmute::<_, fn(&dyn PluginHost)>(translated_header_address.as_u64());
        (entrypoint)(&BrigHost);
    }
}

struct BrigHost;

impl PluginHost for BrigHost {
    fn print_message(&self, msg: &str) {
        log::info!("got message from plugin: {}", msg);
    }

    fn allocator(&self) -> &'static dyn core::alloc::GlobalAlloc {
        &crate::arch::x86::memory::HEAP_ALLOCATOR
    }
}
