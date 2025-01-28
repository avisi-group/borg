#![no_std]
#![feature(abi_x86_interrupt)] // needed for interrupts
#![feature(allocator_api)] // needed for pci config regions and alignedallocator
#![feature(naked_functions)] // for interrupts with glorious purpose
#![feature(btree_cursors)]

extern crate alloc;

use {
    crate::{
        arch::x86::{
            backtrace::backtrace,
            memory::{HIGH_HALF_CANONICAL_END, HIGH_HALF_CANONICAL_START, PHYSICAL_MEMORY_OFFSET},
        },
        dbt::models,
        devices::manager::SharedDeviceManager,
        fs::{tar::TarFilesystem, File, Filesystem},
        logger::WRITER,
    },
    bootloader_api::{config::Mapping, BootInfo, BootloaderConfig},
    byte_unit::{Byte, UnitType::Binary},
    core::panic::PanicInfo,
    x86::io::outw,
};

mod arch;
mod dbt;
mod devices;
mod fs;
pub mod guest;
mod logger;
pub mod plugins;
mod rand;
mod scheduler;
mod tasks;
mod tests;

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

    arch::CoreStorage::init_self();

    // required for generating UUIDs
    rand::init();

    // Host machine initialisation
    arch::platform_init(boot_info);
    tasks::init();

    // occurs per core
    tasks::register_scheduler();

    {
        let continue_start_task = tasks::create_task(continue_start);
        continue_start_task.start();
    }

    scheduler::local_run();
}

fn continue_start() {
    // let serial_in_task = tasks::create_task(serial_in);
    // serial_in_task.start();

    let device_manager = SharedDeviceManager::get();
    let device = device_manager
        .get_device_by_alias("disk00:03.0")
        .expect("disk not found");

    plugins::load_all(&device);
    models::load_all(&device);

    let test_config = {
        let mut dev = device.lock();
        let mut fs = TarFilesystem::mount(dev.as_block());
        let file = fs
            .open("test_config.postcard")
            .expect("missing test configuration file")
            .read_to_vec()
            .unwrap();
        postcard::from_bytes(&file).unwrap()
    };
    tests::run(test_config);

    guest::start();
}

fn serial_in() {
    let mut buf = [0u8; 64];

    loop {
        let read = unsafe { WRITER.get_mut() }
            .expect("WRITER not initialized")
            .read_bytes(&mut buf);

        if read > 0 {
            match core::str::from_utf8(&buf[..read]) {
                Ok(s) => match s {
                    "\u{3}" => {
                        log::error!("received Ctrl-C, terminating");
                        qemu_exit();
                    }
                    _ => log::debug!("{:?}", s),
                },
                Err(e) => log::error!("serial port received invalid UTF-8 {:?}", e),
            }
        }

        // todo nap time for a little bit
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
