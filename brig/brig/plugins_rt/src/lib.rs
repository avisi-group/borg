#![no_std]

//! Brig plugin runtime: allocation, panics, logging

extern crate alloc;

pub use plugins_api as api;

use {api::PluginHost, core::panic::PanicInfo};

mod allocator;
pub mod host;
mod log;

/// Initializes plugin runtime
///
/// * Configures global allocator to use the host allocator
/// * Configures global variable containing reference to `dyn PluginHost`
///     * which allows for `log` macro usage
pub fn init(host: &'static dyn PluginHost) {
    host::init(host);
    allocator::init();
    log::init();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    host::get().print_message("panic!");
    loop {
        unsafe { core::arch::asm!("nop") };
    }
}
