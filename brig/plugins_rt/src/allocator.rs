use {crate::host, core::alloc::GlobalAlloc};

#[global_allocator]
static mut ALLOCATOR: RefAllocator = RefAllocator(None);

pub(crate) fn init() {
    unsafe { ALLOCATOR.0 = Some(host::get().allocator()) };
}

struct RefAllocator(Option<&'static dyn GlobalAlloc>);

unsafe impl Send for RefAllocator {}
unsafe impl Sync for RefAllocator {}

unsafe impl GlobalAlloc for RefAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { (self.0).unwrap().alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        unsafe { (self.0).unwrap().dealloc(ptr, layout) }
    }
}
