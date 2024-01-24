#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use {
    alloc::{boxed::Box, vec},
    bootloader_api::{config::Mapping, BootloaderConfig},
    core::panic::PanicInfo,
};

mod console;
mod gdt;
mod interrupts;
mod memory;
mod serial;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.mappings.dynamic_range_start = Some(0x_FFFF_8000_0000_0000);
    config.mappings.dynamic_range_end = Some(0x_FFFF_FFFF_FFFF_FFFF);
    config
};

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    memory::init(boot_info);

    dbg!(&boot_info);

    let a = Box::new(14u64);
    let b = vec![0xFF00_FF00_FF00u64; 1_000];
    println!("{a:?} {a:p} {} {:p}", b.len(), b.as_ptr());

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
