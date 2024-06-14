use {
    aarch64_interpreter::{Aarch64Interpreter, TracerKind},
    rustix::mm::{MapFlags, ProtFlags},
};

const GUEST_MEMORY_BASE: usize = 0x1_0000;
const GUEST_MEMORY_ALLOC_SIZE: usize = 12 * 1024 * 1024 * 1024;

const DTB: &[u8] = include_bytes!("/workspaces/borg/borealis/data/sail-arm/arm-v9.4-a/sail.dtb");
const BOOTLOADER: &[u8] =
    include_bytes!("/workspaces/borg/borealis/data/sail-arm/arm-v9.4-a/bootloader.bin");
const IMAGE: &[u8] = include_bytes!("/workspaces/borg/borealis/data/sail-arm/arm-v9.4-a/Image");

mod logger;

fn main() {
    logger::init();

    // create guest virtual memory?
    let _mmap0 = unsafe {
        rustix::mm::mmap_anonymous(
            GUEST_MEMORY_BASE as *mut _,
            GUEST_MEMORY_ALLOC_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::FIXED | MapFlags::PRIVATE,
        )
    }
    .unwrap();
    let _mmap1 = unsafe {
        rustix::mm::mmap_anonymous(
            0x40_7a00_0000 as *mut _,
            GUEST_MEMORY_ALLOC_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::FIXED | MapFlags::PRIVATE,
        )
    }
    .unwrap();
    let _mmap2 = unsafe {
        rustix::mm::mmap_anonymous(
            0x7fc0_0780_0000 as *mut _,
            GUEST_MEMORY_ALLOC_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::FIXED | MapFlags::PRIVATE,
        )
    };

    // -b 0x80000000,bootloader.bin -b 0x81000000,sail.dtb -b 0x82080000,Image

    unsafe {
        // copy bootloader
        write_ram(BOOTLOADER, 0x8000_0000);
        // copy dtb
        write_ram(DTB, 0x8100_0000);
        // copy kernel
        write_ram(IMAGE, 0x8208_0000);
    }

    let mut interpreter = Aarch64Interpreter::new(GUEST_MEMORY_BASE, 0x8000_0000, TracerKind::Sail);
    interpreter.run();
}

unsafe fn write_ram(data: &[u8], guest_address: usize) {
    // speedy version
    // core::ptr::copy(
    //     data.as_ptr(),
    //     (GUEST_MEMORY_BASE + guest_address) as *mut u8,
    //     data.len(),
    // );

    // tracing version
    for (i, byte) in data.iter().enumerate() {
        let byte_address = guest_address + i;
        unsafe { *((GUEST_MEMORY_BASE + byte_address) as *mut u8) = *byte };
        println!("[Sail] mem {byte_address:016x} <- {byte:016x}");
    }
}
