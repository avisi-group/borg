use {
    alloc::{boxed::Box, collections::BTreeMap, string::String},
    core::{
        fmt::Display,
        mem::{size_of, MaybeUninit},
        slice,
    },
    plugins_api::IOMemoryHandler,
};

pub struct AddressSpace {
    regions: BTreeMap<usize, AddressSpaceRegion>,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self {
            regions: BTreeMap::new(),
        }
    }

    pub fn add_region(&mut self, region: AddressSpaceRegion) {
        log::trace!("addr-space: adding region {}", region);
        self.regions.insert(region.base, region);
    }

    pub fn find_region(&self, address: usize) -> Option<&AddressSpaceRegion> {
        let candidate = self
            .regions
            .upper_bound(core::ops::Bound::Included(&address))
            .prev()?;

        if address >= candidate.1.base && address < (candidate.1.base + candidate.1.size) {
            Some(candidate.1)
        } else {
            None
        }
    }
}

pub trait IoMemoryHandlerExt {
    fn read_fixed<H: IOMemoryHandler, T: Sized>(handler: H, offset: usize) -> T {
        let mut t = MaybeUninit::<T>::uninit();
        let buf = unsafe { slice::from_raw_parts_mut(t.as_mut_ptr() as *mut u8, size_of::<T>()) };
        handler.read(offset, buf);

        unsafe { t.assume_init() }
    }
}

pub enum AddressSpaceRegionKind {
    Ram,
    IO(Box<dyn IOMemoryHandler>),
}

pub struct AddressSpaceRegion {
    name: String,
    base: usize,
    size: usize,
    kind: AddressSpaceRegionKind,
}

impl AddressSpaceRegion {
    pub fn kind(&self) -> &AddressSpaceRegionKind {
        &self.kind
    }
}

impl Display for AddressSpaceRegion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "name={}, base={:x}, size={:x}",
            self.name, self.base, self.size
        )
    }
}

impl AddressSpaceRegion {
    pub fn new(name: String, base: usize, size: usize, kind: AddressSpaceRegionKind) -> Self {
        Self {
            name,
            base,
            size,
            kind,
        }
    }
}
