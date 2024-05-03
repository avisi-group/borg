use {
    crate::{
        arch::x86::memory::VirtualMemoryArea,
        devices::SharedDevice,
        fs::{
            tar::{TarFile, TarFilesystem},
            File, Filesystem,
        },
    },
    alloc::{
        alloc::alloc_zeroed, borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String,
        vec::Vec,
    },
    core::{alloc::Layout, default, ops::Range},
    elfloader::{
        arch::x86_64::RelocationTypes::{R_AMD64_64, R_AMD64_GLOB_DAT, R_AMD64_RELATIVE},
        ElfBinary, ElfLoader, ElfLoaderErr, Entry, Flags, LoadableHeaders, RelocationEntry,
        RelocationType, VAddr,
    },
    x86_64::{
        structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
        VirtAddr,
    },
    xmas_elf::sections::SectionData,
};

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn superspecificferdianame(&self, a: u32) -> u32;
}

struct Loader<'binary, 'contents> {
    binary: &'binary ElfBinary<'contents>,

    symbol_values: Vec<u64>,
    // address -> (index, size)
    // section_addresses: BTreeMap<u64, (u16, u64)>,
    allocation: *mut u8,
}

impl<'binary, 'contents> Loader<'binary, 'contents> {
    pub fn new(binary: &'binary ElfBinary<'contents>, allocation: *mut u8) -> Self {
        let mut symbol_values = Vec::new();

        // let section_addresses = binary
        //     .file
        //     .section_iter()
        //     .enumerate()
        //     .map(|(i, s)| (s.address(), (u16::try_from(i).unwrap(), s.size())))
        //     .collect::<BTreeMap<_, _>>();
        let symbol_section = binary.file.find_section_by_name(".dynsym").unwrap();
        let symbol_table = symbol_section.get_data(&binary.file).unwrap();
        match symbol_table {
            SectionData::DynSymbolTable64(entries) => {
                for entry in entries {
                    symbol_values.push(entry.value());
                }
            }

            _ => panic!(),
        }

        //   let symbol_values = symbol_values.into_iter().map(|(_, value)|
        // value).collect();

        let mut this = Self {
            binary,
            symbol_values,
            allocation,
        };

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
        // for header in load_headers {
        //     let aligned_base = VirtAddr::new(header.virtual_addr())
        //         .align_down(header.align())
        //         .as_u64();

        //     // add the difference lost between base and aligned base to the size
        //     // equivalent to aligning mem_size up to the next page size
        //     let adjusted_size = header.mem_size() + (header.virtual_addr() -
        // aligned_base);

        //     let layout = Layout::from_size_align(
        //         usize::try_from(adjusted_size).unwrap(),
        //         usize::try_from(header.align()).unwrap(),
        //     )
        //     .unwrap();

        //     let alloc = VirtAddr::from_ptr(unsafe { alloc_zeroed(layout) });

        //     log::info!(
        //         "allocate base = {:x} size = {:x} align = {:x} => aligned base {:x}
        // adjusted size {:x} @ {:x}",         header.virtual_addr(),
        //         header.mem_size(),
        //         header.align(),
        //         aligned_base,
        //         adjusted_size,
        //         alloc
        //     );

        //     self.mapping.insert(aligned_base, (alloc, adjusted_size));
        // }
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
}

pub fn load_one<'fs>(file: TarFile<'fs>) {
    let contents = file.read_to_vec().unwrap();
    let binary = ElfBinary::new(contents.as_slice()).expect("Got proper ELF file");

    let max_size = binary
        .program_headers()
        .filter(|header| matches!(header.get_type().unwrap(), xmas_elf::program::Type::Load))
        .map(|header| ((header.virtual_addr() + header.mem_size()) + header.align()))
        .max()
        .unwrap();

    let alloc = unsafe { alloc_zeroed(Layout::from_size_align(max_size as usize, 4096).unwrap()) };

    log::info!("elf backing allocation: {alloc:p} {max_size:x}");

    let mut loader = Loader::new(&binary, alloc);
    binary.load(&mut loader).expect("Can't load the binary?");

    //panic!("sausage");

    let header = binary.file.find_section_by_name(".plugins").unwrap();
    let translated_header_address = loader.translate_virt_addr(header.address());

    unsafe {
        let ptr_to_plugin_trait_ptr = translated_header_address.as_ptr::<*const dyn Plugin>();
        let plugin_trait_ptr = *ptr_to_plugin_trait_ptr;
        let plugin_trait_ref = &*plugin_trait_ptr;

        let plugin_data_ptr = (*(ptr_to_plugin_trait_ptr as *const u64)) as *const ();
        let plugin_vtable_ptr =
            (*(ptr_to_plugin_trait_ptr as *const u64).offset(1)) as *const [u64; 5];

        log::info!("ptr_to_plugin_trait_ptr = {ptr_to_plugin_trait_ptr:p}");
        log::info!("plugin_trait_ptr = {plugin_trait_ptr:p}",);
        log::info!("plugin_data_ptr = {plugin_data_ptr:p}",);
        log::info!("plugin_vtable_ptr = {plugin_vtable_ptr:p}",);

        log::info!("plugin_vtable = {:x?}", *plugin_vtable_ptr);

        log::info!("{:x}", plugin_trait_ref.superspecificferdianame(7));
        log::info!("{}", plugin_trait_ref.name());
    }
    // register here
}
