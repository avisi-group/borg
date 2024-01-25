use {
    bootloader_api::{info::MemoryRegionKind, BootInfo},
    buddy_system_allocator::LockedHeap,
    byte_unit::{Byte, UnitType},
    core::{alloc::Layout, ops::Deref},
    spin::Once,
    x86_64::{
        self,
        registers::control::{Cr3, Cr3Flags},
        structures::paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
            PhysFrame, Size1GiB, Size4KiB,
        },
        PhysAddr, VirtAddr,
    },
};

pub const HIGH_HALF_CANONICAL_START: VirtAddr = VirtAddr::new_truncate(0x_ffff_8000_0000_0000);
pub const HIGH_HALF_CANONICAL_END: VirtAddr = VirtAddr::new_truncate(0x_ffff_ffff_ffff_ffff);
pub const PHYSICAL_MEMORY_MAP_OFFSET: VirtAddr = VirtAddr::new_truncate(0xffff_8180_0000_0000);

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub static OFFSET_PAGE_TABLE: Once<OffsetPageTable<'static>> = Once::INIT;

pub fn init(boot_info: &'static BootInfo) {
    // virtual address of the start of mapped physical memory
    let phys_mem_start = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("No physical memory offset in boot info"),
    );

    // get usable regions from memory map and add to heap allocator
    (boot_info.memory_regions)
        .deref()
        .iter()
        .filter(|r| matches!(r.kind, MemoryRegionKind::Usable))
        .for_each(|region| {
            unsafe {
                HEAP_ALLOCATOR.lock().add_to_heap(
                    (phys_mem_start.as_u64() + region.start) as usize,
                    (phys_mem_start.as_u64() + region.end) as usize,
                )
            };
        });

    log::info!(
        "heap allocator initialized, {:.2} available",
        Byte::from(HEAP_ALLOCATOR.lock().stats_total_bytes())
            .get_appropriate_unit(UnitType::Binary)
    );

    VMA::current().map_page(
        Page::<Size1GiB>::from_start_address(VirtAddr::new(0xffff818800000000)).unwrap(),
        PhysFrame::<Size1GiB>::from_start_address(PhysAddr::new(0x800000000)).unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
    );

    VMA::current().invalidate();
}

/// Frame allocator that uses the global heap allocator, then translates virtual
/// addresses back to physical
struct HeapStealingFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for HeapStealingFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let virt_ptr = HEAP_ALLOCATOR
            .lock()
            .alloc(Layout::from_size_align(4096, 4096).unwrap())
            .unwrap();

        Some(
            PhysFrame::from_start_address(VirtAddr::from_ptr(virt_ptr.as_ptr()).to_phys()).unwrap(),
        )
    }
}

struct VMA {
    pml4_base: PhysAddr,
}

impl VMA {
    fn get_current_cr3() -> PhysAddr {
        Cr3::read().0.start_address()
    }

    pub fn current() -> Self {
        Self {
            pml4_base: Self::get_current_cr3(),
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

    fn get_pml4(&self) -> (PhysAddr, VirtAddr) {
        (self.pml4_base, self.pml4_base.to_virt())
    }

    fn get_pml4_ptr(&self) -> &mut PageTable {
        unsafe { &mut *(self.get_pml4().1.as_mut_ptr()) }
    }

    pub fn map_page<'a, S: PageSize + core::fmt::Debug>(
        &'a mut self,
        page: Page<S>,
        frame: PhysFrame<S>,
        flags: PageTableFlags,
    ) where
        OffsetPageTable<'a>: Mapper<S>,
    {
        let pml4 = self.get_pml4_ptr();

        let _ = unsafe {
            OffsetPageTable::new(pml4, PHYSICAL_MEMORY_MAP_OFFSET).map_to(
                page,
                frame,
                flags,
                &mut HeapStealingFrameAllocator,
            )
        }
        .unwrap();
    }

    pub fn translate_page<S: PageSize + core::fmt::Debug>(&self, page: Page<S>) -> PhysFrame<S>
    where
        OffsetPageTable<'static>: Mapper<S>,
    {
        let (level_4_table_frame, _) = Cr3::read();

        let phys = level_4_table_frame.start_address();
        let page_table_ptr = unsafe { &mut *(phys.to_virt().as_mut_ptr()) };

        unsafe {
            OffsetPageTable::new(
                page_table_ptr,
                VirtAddr::new(PHYSICAL_MEMORY_MAP_OFFSET.as_u64()),
            )
            .translate_page(page)
        }
        .unwrap()
    }
}

/// Address extension trait for additional methods on `PhysAddr`
pub trait PhysAddrExt {
    fn to_virt(&self) -> VirtAddr;
}

impl PhysAddrExt for PhysAddr {
    fn to_virt(&self) -> VirtAddr {
        VirtAddr::new(self.as_u64() + PHYSICAL_MEMORY_MAP_OFFSET.as_u64())
    }
}

/// Address extension trait for additional methods on `VirtAddr`
pub trait VirtAddrExt {
    fn to_phys(&self) -> PhysAddr;
}

impl VirtAddrExt for VirtAddr {
    fn to_phys(&self) -> PhysAddr {
        PhysAddr::new(self.as_u64() - PHYSICAL_MEMORY_MAP_OFFSET.as_u64())
    }
}
