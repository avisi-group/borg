#![no_std]

//! Plugin API definitions
//!
//! Plugins should depend on `plugins_rt`, which re-exports `plugins_api`. The
//! brig kernel depends on `plugins_api` directly.

extern crate alloc;

use {
    alloc::boxed::Box,
    core::{alloc::GlobalAlloc, panic::PanicInfo},
};

pub mod guest;
pub mod util;

/// Header information for the plugin, stored in the `.plugin_header` section
#[derive(Debug)]
pub struct PluginHeader {
    /// Name of the plugin
    pub name: &'static str,
    /// Entrypoint function of the plugin
    pub entrypoint: fn(&'static dyn PluginHost),
}

/// Interface for plugins to the parent host
pub trait PluginHost: Send + Sync {
    /// Prints a message to the console
    fn print_message(&self, msg: &str, bare: bool);

    /// Gets a reference to the plugin host's global allocator
    fn allocator(&self) -> &dyn GlobalAlloc;

    /// Register a new kind of guest device with the host
    fn register_device(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn guest::DeviceFactory>,
    );

    /// Panic from plugin
    fn panic(&self, info: &PanicInfo);
}
