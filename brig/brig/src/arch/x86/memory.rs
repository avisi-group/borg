use {
    alloc::alloc::alloc_zeroed,
    bootloader_api::info::{MemoryRegionKind, MemoryRegions},
    buddy_system_allocator::LockedHeap,
    byte_unit::{Byte, UnitType},
    core::{alloc::Layout, ops::Deref},
    x86_64::{
        registers::control::{Cr3, Cr3Flags},
        structures::paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTableFlags, PhysFrame,
            Size4KiB, Translate,
        },
        PhysAddr, VirtAddr,
    },
};

pub const _LOW_HALF_CANONICAL_START: VirtAddr = VirtAddr::new_truncate(0x0000_0000_0000_0000);
pub const LOW_HALF_CANONICAL_END: VirtAddr = VirtAddr::new_truncate(0x0000_7fff_ffff_ffff);
pub const HIGH_HALF_CANONICAL_START: VirtAddr = VirtAddr::new_truncate(0xffff_8000_0000_0000);
pub const HIGH_HALF_CANONICAL_END: VirtAddr = VirtAddr::new_truncate(0xffff_ffff_ffff_ffff);
pub const PHYSICAL_MEMORY_OFFSET: VirtAddr = VirtAddr::new_truncate(0xffff_8180_0000_0000);

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<64> = LockedHeap::empty();

/// Initialize the global heap allocator backed by the usable memory regions
/// supplied by the bootloader
pub fn heap_init(memory_regions: &MemoryRegions) {
    // get usable regions from memory map and add to heap allocator
    for region in memory_regions
        .deref()
        .iter()
        .filter(|r| matches!(r.kind, MemoryRegionKind::Usable))
    {
        unsafe {
            HEAP_ALLOCATOR.lock().add_to_heap(
                usize::try_from(PhysAddr::new(region.start).to_virt().as_u64()).unwrap(),
                usize::try_from(PhysAddr::new(region.end).to_virt().as_u64()).unwrap(),
            )
        };
    }

    log::info!(
        "heap allocator initialized, {:.2} available",
        Byte::from(HEAP_ALLOCATOR.lock().stats_total_bytes())
            .get_appropriate_unit(UnitType::Binary)
    );
}

/// Returns the number of bytes used by and number of bytes available to the
/// heap allocator
pub fn stats() -> (usize, usize) {
    unsafe { HEAP_ALLOCATOR.force_unlock() };
    let allocator = HEAP_ALLOCATOR.lock();
    (
        allocator.stats_alloc_actual(),
        allocator.stats_total_bytes(),
    )
}

/// Frame allocator that uses the global heap allocator, then translates virtual
/// addresses back to physical
struct HeapStealingFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for HeapStealingFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let new_frame = unsafe { alloc_zeroed(Layout::from_size_align(4096, 4096).unwrap()) };

        Some(PhysFrame::from_start_address(VirtAddr::from_ptr(new_frame).to_phys()).unwrap())
    }
}

pub struct VirtualMemoryArea {
    pml4_base: PhysAddr,
    opt: OffsetPageTable<'static>,
}

impl VirtualMemoryArea {
    fn get_current_cr3() -> PhysAddr {
        Cr3::read().0.start_address()
    }

    pub fn current() -> Self {
        let pml4_base = Self::get_current_cr3();
        let pml4_virt = pml4_base.to_virt();
        let pml4_table = unsafe { &mut *(pml4_virt.as_mut_ptr()) };

        Self {
            pml4_base,
            opt: unsafe { OffsetPageTable::new(pml4_table, PHYSICAL_MEMORY_OFFSET) },
        }
    }

    pub fn invalidate(&self) {
        assert!(Self::get_current_cr3() == self.pml4_base);
        self.activate();
    }

    pub fn activate(&self) {
        unsafe {
            Cr3::write(
                PhysFrame::from_start_address(self.pml4_base).unwrap(),
                Cr3Flags::empty(),
            );
        }
    }

    pub fn map_page<S: PageSize + core::fmt::Debug>(
        &mut self,
        page: Page<S>,
        frame: PhysFrame<S>,
        flags: PageTableFlags,
    ) where
        OffsetPageTable<'static>: Mapper<S>,
    {
        unsafe {
            let _ = self
                .opt
                .map_to(page, frame, flags, &mut HeapStealingFrameAllocator)
                .unwrap();
        }
    }

    /*pub fn translate_page<'a, S: PageSize + core::fmt::Debug>(
        &'a self,
        page: Page<S>,
    ) -> Option<PhysFrame<S>>
    where
        OffsetPageTable<'a>: Mapper<S>,
    {
        let page_table_ref = self.get_pml4_ptr();
        let page_table = unsafe {
            OffsetPageTable::new(
                page_table_ref,
                VirtAddr::new(PHYSICAL_MEMORY_MAP_OFFSET.as_u64()),
            )
        };

        page_table.translate_page(page).ok()
    }*/

    pub fn translate_address(&self, addr: VirtAddr) -> Option<PhysAddr> {
        let r = self.opt.translate_addr(addr);

        log::trace!("translating {:x} to {:?}", addr, r);

        r
    }
}

/// Address extension trait for additional methods on `PhysAddr`
pub trait PhysAddrExt {
    fn to_virt(&self) -> VirtAddr;
}

impl PhysAddrExt for PhysAddr {
    fn to_virt(&self) -> VirtAddr {
        VirtAddr::new(self.as_u64() + PHYSICAL_MEMORY_OFFSET.as_u64())
    }
}

/// Address extension trait for additional methods on `VirtAddr`
pub trait VirtAddrExt {
    fn to_phys(&self) -> PhysAddr;
}

impl VirtAddrExt for VirtAddr {
    fn to_phys(&self) -> PhysAddr {
        PhysAddr::new(self.as_u64() - PHYSICAL_MEMORY_OFFSET.as_u64())
    }
}
