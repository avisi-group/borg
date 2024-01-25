#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(non_null_convenience)]
#![feature(slice_ptr_get)]

extern crate alloc;

use {
    crate::{
        backtrace::backtrace,
        memory::{
            HEAP_ALLOCATOR, HIGH_HALF_CANONICAL_END, HIGH_HALF_CANONICAL_START,
            PHYSICAL_MEMORY_MAP_OFFSET,
        },
    },
    bootloader_api::{config::Mapping, BootloaderConfig},
    byte_unit::Byte,
    core::panic::PanicInfo,
    x86::io::outw,
    x86_64::{PhysAddr, VirtAddr},
};

mod backtrace;
mod devices;
mod gdt;
mod interrupts;
mod logger;
mod memory;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory =
        Some(Mapping::FixedAddress(PHYSICAL_MEMORY_MAP_OFFSET.as_u64()));
    config.mappings.dynamic_range_start = Some(HIGH_HALF_CANONICAL_START.as_u64());
    config.mappings.dynamic_range_end = Some(HIGH_HALF_CANONICAL_END.as_u64());
    config
};

pub fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    assert_eq!(
        boot_info.physical_memory_offset.into_option().unwrap(),
        PHYSICAL_MEMORY_MAP_OFFSET.as_u64()
    );

    logger::init();
    backtrace::init(
        VirtAddr::new(boot_info.kernel_image_offset),
        PhysAddr::new(boot_info.kernel_addr),
        usize::try_from(boot_info.kernel_len).unwrap(),
    );

    gdt::init();
    interrupts::init();
    memory::init(boot_info);
    devices::init(boot_info.rsdp_addr.into_option().unwrap() as usize);

    // let a = Box::new(14u64);
    // let b = vec![0xFF00_FF00_FF00u64; 1_000];
    // println!("{a:?} {a:p} {} {:p}", b.len(), b.as_ptr());

    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let allocator = HEAP_ALLOCATOR.lock();
    let used = Byte::from(allocator.stats_alloc_actual())
        .get_appropriate_unit(byte_unit::UnitType::Binary);
    let total =
        Byte::from(allocator.stats_total_bytes()).get_appropriate_unit(byte_unit::UnitType::Binary);

    log::error!("{info}");
    log::error!("heap {:.2}/{:.2} used", used, total);

    backtrace();
    qemu_exit();
}

/// Exits QEMU
fn qemu_exit() -> ! {
    unsafe { outw(0x604, 0x2000) };
    loop {}
}
