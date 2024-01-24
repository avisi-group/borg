use {
    bootloader_api::{
        info::{MemoryRegionKind, MemoryRegions},
        BootInfo,
    },
    buddy_system_allocator::LockedHeap,
    core::ops::Deref,
    x86_64::{
        structures::paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
            PhysFrame, Size1GiB, Size2MiB, Size4KiB,
        },
        PhysAddr, VirtAddr,
    },
};

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub const HEAP_START: u64 = 0x_ffff_a000_0000_0000; // +160TiB
pub const HEAP_SIZE: usize = 1 * 1024 * 1024;

pub fn init(boot_info: &'static BootInfo) {
    // virtual address of the start of mapped physical memory
    let phys_mem_start = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("No physical memory offset in boot info"),
    );

    // let pml4 = unsafe { (phys_mem_start.as_ptr::<u8>().add(0x2000)) as *mut [u64;
    // 512] }; pml4[]

    //OffsetPageTable::new(level_4_table, physical_memory_offset);

    let mut mapper = unsafe { offset_page_table(phys_mem_start) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    let heap_start = VirtAddr::new(HEAP_START);
    let heap_end = heap_start + HEAP_SIZE - 1u64;
    let heap_start_page = Page::<Size2MiB>::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);
    let page_range = Page::range_inclusive(heap_start_page, heap_end_page);

    for page in page_range {
        let frame = frame_allocator.allocate_frame().unwrap();
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, &mut frame_allocator)
                .unwrap()
                .flush()
        };
    }

    unsafe {
        HEAP_ALLOCATOR.lock().add_to_heap(
            usize::try_from(heap_start.as_u64()).unwrap(),
            usize::try_from(heap_end.as_u64()).unwrap(),
        )
    };
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn offset_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// A FrameAllocator that returns usable frames from the bootloader's memory
/// map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the
    /// passed memory map is valid. The main requirement is that all frames
    /// that are marked as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory
    fn usable_frames<S: PageSize>(&self) -> impl Iterator<Item = PhysFrame<S>> {
        // get usable regions from memory map
        let regions = self.memory_map.deref().iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable); // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe impl FrameAllocator<Size2MiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size2MiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe impl FrameAllocator<Size1GiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size1GiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

// use {
//     crate::{dbg, println},
//     bootloader_api::{info::MemoryRegionKind, BootInfo},
//     buddy_system_allocator::LockedHeap,
//     core::{
//         alloc::Layout,
//         ops::{Add, Deref},
//         ptr::NonNull,
//     },
//     x86_64::{
//         structures::paging::{PhysFrame, Size4KiB},
//         PhysAddr,
//     },
// };

// static PAGE_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

// pub fn init(boot_info: &BootInfo) {
//     dbg!(&boot_info);

//     // get usable regions from memory map
//     let regions = (boot_info.memory_regions).deref().iter().skip(1);
//     let usable_regions = regions.filter(|r| r.kind ==
// MemoryRegionKind::Usable); // map each region to its address range
//     let addr_ranges = usable_regions.map(|r| r.start..r.end);
//     // transform to an iterator of frame start addresses
//     let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
//     // create `PhysFrame` types from the start addresses
//     frame_addresses
//         .map(|addr|
// PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(addr)))
//         .for_each(|physical_frame| {
//             dbg!(&physical_frame);
//             unsafe {
//                 PAGE_ALLOCATOR.lock().add_to_heap(
//
// usize::try_from(physical_frame.start_address().as_u64()).unwrap(),
//                     usize::try_from(
//                         physical_frame
//                             .start_address()
//                             .add(physical_frame.size())
//                             .as_u64(),
//                     )
//                     .unwrap(),
//                 )
//             };
//         });

//     // for region in regions {
//     //     println!("{region:x?}");

//     //     if region.kind == MemoryRegionKind::Usable && region.start > 0 {
//     //
//     //     }
//     // }
// }

// pub fn alloc_zero_pages(count: usize) -> PhysAddr {
//     todo!()
// }

// pub fn alloc_pages(count: usize) -> PhysAddr {
//     PhysAddr::new(
//         PAGE_ALLOCATOR
//             .lock()
//             .alloc(Layout::from_size_align(0x1000 * count,
// 0x1000).expect("layout error"))             .expect(
//                 "failed to allocate
// page",
//             )
//             .as_ptr() as u64,
//     )
// }

// pub fn dealloc_pages(start: PhysAddr, count: usize) {
//     let start = NonNull::new(start.as_u64() as *mut _).expect(
//         "null
// pointer",
//     );

//     PAGE_ALLOCATOR.lock().dealloc(
//         start,
//         Layout::from_size_align(0x1000 * count, 0x1000).expect(
//             "layout
// error",
//         ),
//     );
// }
