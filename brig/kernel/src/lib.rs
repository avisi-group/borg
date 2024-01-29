#![no_std]
#![feature(abi_x86_interrupt)] // needed for interrupts
#![feature(allocator_api)] // needed for pci config regions
#![feature(naked_functions)] // for interrupts with glorious purpose

extern crate alloc;

use {
    crate::arch::x86::{
        backtrace::backtrace,
        memory::{HIGH_HALF_CANONICAL_END, HIGH_HALF_CANONICAL_START, PHYSICAL_MEMORY_OFFSET},
    },
    bootloader_api::{config::Mapping, BootInfo, BootloaderConfig},
    byte_unit::{Byte, UnitType::Binary},
    core::panic::PanicInfo,
    log::trace,
    x86::io::outw,
};

mod arch;
mod devices;
mod fs;
mod guest;
mod logger;
mod rand;
mod scheduler;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::FixedAddress(PHYSICAL_MEMORY_OFFSET.as_u64()));
    config.mappings.dynamic_range_start = Some(HIGH_HALF_CANONICAL_START.as_u64());
    config.mappings.dynamic_range_end = Some(HIGH_HALF_CANONICAL_END.as_u64());
    config.kernel_stack_size = 0x10_0000;
    config
};

pub fn start(boot_info: &'static mut BootInfo) -> ! {
    // note: logging device initialized internally before platform
    logger::init();

    // required for generating UUIDs
    rand::init();

    // Host machine initialisation
    arch::platform_init(boot_info);
    scheduler::init();

    // search all drives for guest tar
    /* let (config, kernel, _dt) = {
        // todo: maybeuninit
        let mut buf = vec![0u8; device.size()];
        device.read(&mut buf, 0).unwrap();
        guest::config::load_guest_config(&buf).unwrap()
    };
    log::trace!("kernel len: {:#x}, got config: {:#?}", kernel.len(), config);*/

    scheduler::spawn(continue_start1);
    scheduler::spawn(continue_start2);

    scheduler::start();

    loop {
        x86_64::instructions::hlt();
    }
}

pub fn continue_start1() {
    loop {
        trace!("hello from thread 1");
        for _ in 0..100000000 {
            //
        }
    }
}

pub fn continue_start2() {
    loop {
        trace!("hello from thread 2");
        for _ in 0..10000000 {
            //
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arch::x86::irq::local_disable();
    let (used, total) = arch::x86::memory::stats();

    log::error!("{info}");
    log::error!(
        "heap {:.2}/{:.2} used",
        Byte::from(used).get_appropriate_unit(Binary),
        Byte::from(total).get_appropriate_unit(Binary),
    );

    backtrace();
    qemu_exit();
}

/// Exits QEMU
fn qemu_exit() -> ! {
    unsafe { outw(0x604, 0x2000) };
    loop {
        x86_64::instructions::hlt();
    }
}
