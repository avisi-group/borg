#![no_std]
#![no_main]

use kernel::BOOTLOADER_CONFIG;

bootloader_api::entry_point!(kernel::start, config = &BOOTLOADER_CONFIG);
