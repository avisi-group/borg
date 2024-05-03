#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use core::alloc::GlobalAlloc;
use core::fmt::{self, Write};
use core::panic::PanicInfo;
use plugins_api::PluginHost;

#[global_allocator]
static mut ALLOCATOR: HostAllocator = HostAllocator::new();

#[no_mangle]
#[link_section = ".plugin_entrypoint"]
pub extern "C" fn entrypoint(host: &'static dyn PluginHost) {
    host.print_message("starting pl011");
    print_noalloc(host, format_args!("{:x}", 0x55));

    unsafe { ALLOCATOR.init(host.allocator()) };

    host.print_message("initialized allocator");

    print_noalloc(
        host,
        format_args!("allocator = {:p}", unsafe {
            ALLOCATOR.host.unwrap() as *const _
        }),
    );

    let a = alloc::boxed::Box::new(0x5555);
    print_noalloc(host, format_args!("a = {:p}", a));
    print_noalloc(host, format_args!("*a = {:x}", *a));

    let mut vec = alloc::vec::Vec::<u8>::new();
    print_noalloc(host, format_args!("vec = {:p}", &vec));
    print_noalloc(host, format_args!("vec = {:x?}", vec));
    vec.push(0);
    print_noalloc(host, format_args!("vec = {:x?}", vec));
    // 7 is okay, but 8 fails
    vec.reserve(8);
    print_noalloc(host, format_args!("vec = {:x?}", vec));
}

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

fn print_noalloc(host: &dyn PluginHost, args: fmt::Arguments) {
    pub struct ByteMutWriter<'a> {
        buf: &'a mut [u8],
        cursor: usize,
    }

    impl<'a> ByteMutWriter<'a> {
        pub fn new(buf: &'a mut [u8]) -> Self {
            ByteMutWriter { buf, cursor: 0 }
        }

        pub fn as_str(&self) -> &str {
            unsafe { core::str::from_utf8_unchecked(&self.buf[0..self.cursor]) }
        }

        #[inline]
        pub fn capacity(&self) -> usize {
            self.buf.len()
        }

        pub fn clear(&mut self) {
            self.cursor = 0;
        }

        pub fn len(&self) -> usize {
            self.cursor
        }

        pub fn empty(&self) -> bool {
            self.cursor == 0
        }

        pub fn full(&self) -> bool {
            self.capacity() == self.cursor
        }
    }

    impl fmt::Write for ByteMutWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let cap = self.capacity();
            for (i, &b) in self.buf[self.cursor..cap]
                .iter_mut()
                .zip(s.as_bytes().iter())
            {
                *i = b;
            }
            self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
            Ok(())
        }
    }

    let mut buf = [0u8; 64];
    let mut buf = ByteMutWriter::new(&mut buf[..]);
    write!(&mut buf, "{}", args);
    host.print_message(buf.as_str());
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // todo!
    loop {}
}
