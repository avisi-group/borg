#![no_std]

extern crate alloc;

use {
    alloc::{format, vec::Vec},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
    plugins_api::PluginHost,
};

static mut HOST: Option<&'static dyn PluginHost> = None;

fn host() -> &'static dyn PluginHost {
    unsafe { HOST.unwrap() }
}

#[global_allocator]
static mut ALLOCATOR: HostAllocator = HostAllocator::new();

#[no_mangle]
#[link_section = ".plugin_entrypoint"]
pub extern "Rust" fn entrypoint(supplied_host: &'static dyn PluginHost) {
    unsafe { HOST = Some(supplied_host) };
    unsafe { ALLOCATOR.init(host().allocator()) };

    let mut vec = Vec::new();
    for i in 0..32 {
        vec.push(i);
    }
    vec.extend_from_slice(b"test string");

    host().print_message(&format!("hello from pl011! {:?}", vec));
}

/// TODO: move me to plugins_api and create a `bootstrap()` method on host that
/// initializes it
struct HostAllocator {
    host: Option<&'static dyn GlobalAlloc>,
}

unsafe impl Send for HostAllocator {}
unsafe impl Sync for HostAllocator {}

impl HostAllocator {
    pub const fn new() -> Self {
        Self { host: None }
    }

    pub fn init(&mut self, allocator: &'static dyn GlobalAlloc) {
        self.host = Some(allocator);
    }
}

unsafe impl GlobalAlloc for HostAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        ((self.host).unwrap()).alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        ((self.host).unwrap()).dealloc(ptr, layout)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    host().print_message("panic!");
    loop {
        unsafe { core::arch::asm!("nop") };
    }
}
