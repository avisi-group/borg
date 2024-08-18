#![no_std]

//! Brig plugin runtime: allocation, panics, logging

extern crate alloc;

pub use {crate::host::get as get_host, plugins_api as api};

use {api::PluginHost, core::panic::PanicInfo};

mod allocator;
mod host;
mod logger;

/// Initializes plugin runtime
///
/// * Configures global allocator to use the host allocator
/// * Configures global variable containing reference to `dyn PluginHost`
///     * which allows for `log` macro usage
pub fn init(host: &'static dyn PluginHost) {
    host::init(host);
    allocator::init();
    logger::init();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    host::get().panic(info);
    loop {}
}
