#![no_std]
#![feature(abi_x86_interrupt)] // needed for interrupts
#![feature(allocator_api)] // needed for pci config regions

extern crate alloc;

use {
    crate::{
        arch::x86::{
            backtrace::backtrace,
            memory::{
                HIGH_HALF_CANONICAL_END, HIGH_HALF_CANONICAL_START, PHYSICAL_MEMORY_MAP_OFFSET,
            },
        },
        error::Error,
    },
    bootloader_api::{config::Mapping, BootloaderConfig},
    byte_unit::{Byte, UnitType::Binary},
    core::panic::PanicInfo,
    x86::io::outw,
    x86_64::{PhysAddr, VirtAddr},
};

mod arch;
mod devices;
mod error;
mod guest;
mod logger;
mod sched;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory =
        Some(Mapping::FixedAddress(PHYSICAL_MEMORY_MAP_OFFSET.as_u64()));
    config.mappings.dynamic_range_start = Some(HIGH_HALF_CANONICAL_START.as_u64());
    config.mappings.dynamic_range_end = Some(HIGH_HALF_CANONICAL_END.as_u64());
    config.kernel_stack_size = 0x10_0000;
    config
};

pub fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    match main(boot_info) {
        Ok(_) => unreachable!(),
        Err(e) => panic!("kernel error: {e:?}"),
    }
}

fn main(boot_info: &'static bootloader_api::BootInfo) -> Result<(), Error> {
    assert_eq!(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("physical memory offset missing from boot info"),
        PHYSICAL_MEMORY_MAP_OFFSET.as_u64()
    );

    logger::init();

    // Host machine initialisation
    arch::x86::backtrace::init(
        VirtAddr::new(boot_info.kernel_image_offset),
        PhysAddr::new(boot_info.kernel_addr),
        usize::try_from(boot_info.kernel_len)?,
    );
    arch::x86::init_system();
    arch::x86::memory::init(boot_info)?;
    devices::init();
    arch::x86::init_platform(PhysAddr::new(boot_info.rsdp_addr.into_option().unwrap()));
    sched::init();

    // Guest machine initialisation

    // let a = Box::new(14u64);
    // let b = vec![0xFF00_FF00_FF00u64; 1_000];
    // println!("{a:?} {a:p} {} {:p}", b.len(), b.as_ptr());
    Ok(())
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //todo disable interrupts
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
    loop {}
}
