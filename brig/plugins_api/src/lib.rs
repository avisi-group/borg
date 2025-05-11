#![no_std]

//! Plugin API definitions
//!
//! Plugins should depend on `plugins_rt`, which re-exports `plugins_api`. The
//! brig kernel depends on `plugins_api` directly.

extern crate alloc;

use {
    crate::object::{ObjectId, device::DeviceFactory, tickable::Tickable},
    alloc::{boxed::Box, sync::Arc},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
};

pub mod object;
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
    fn register_device_factory(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn DeviceFactory>,
    );

    fn register_periodic_tick(&self, frequency: u64, tickable: &dyn Tickable);

    /// Panic from plugin
    fn panic(&self, info: &PanicInfo);
}
