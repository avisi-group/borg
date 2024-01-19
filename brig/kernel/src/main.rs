#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use {
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
    config
};

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    memory::init(boot_info);

    dbg!(&boot_info);

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
