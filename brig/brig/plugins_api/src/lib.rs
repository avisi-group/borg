#![no_std]

//! Plugin API definitions
//!
//! Plugins should depend on `plugins_rt`, which re-exports `plugins_api`. The
//! brig kernel depends on `plugins_api` directly.

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::{alloc::GlobalAlloc, num::ParseIntError, panic::PanicInfo},
};

/// Header information for the plugin, stored in the `.plugin_header` section
#[derive(Debug)]
pub struct PluginHeader {
    /// Name of the plugin
    pub name: &'static str,
    /// Entrypoint function of the plugin
    pub entrypoint: fn(&'static dyn PluginHost),
}

pub trait PluginHost: Send + Sync {
    /// Prints a message to the console
    fn print_message(&self, msg: &str, bare: bool);

    /// Gets a reference to the plugin host's global allocator
    fn allocator(&self) -> &dyn GlobalAlloc;

    fn register_device(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn GuestDeviceFactory>,
    );

    /// Panic from plugin
    fn panic(&self, info: &PanicInfo);
}

pub trait GuestDeviceFactory {
    fn create(
        &self,
        config: BTreeMap<String, String>,
        interpreter_host: Box<dyn InterpreterHost>,
    ) -> Arc<dyn GuestDevice>;

    // todo: create_arch, create_timer, create_serial, etc
}

pub trait GuestDevice {
    fn start(&self);
    fn stop(&self);

    /// Size of the device's IO memory address space in bytes
    /// (I.E. the maximum valid sum of `offset` and `value.len()` in the `read`
    /// and `write` methods)
    fn address_space_size(&self) -> u64;

    /// Read `value.len()` bytes from the device starting at `offset`
    fn read(&self, offset: u64, value: &mut [u8]);

    /// Write `value` bytes into the device starting at `offset`
    fn write(&self, offset: u64, value: &[u8]);
}

/// Todo make this more general? or more specific? or roll it into PluginHost
pub trait InterpreterHost {
    fn read_memory(&self, address: u64, data: &mut [u8]);
    fn write_memory(&self, address: u64, data: &[u8]);
}

/// Parses `0x`-prefixed, underscore separated hexadecimal values (like a memory
/// address)
///
/// Shouldn't really live here, ideally in some common utility crate, but
/// `plugins_api` is sorta serving that purpose
pub fn parse_hex_prefix<S: AsRef<str>>(s: S) -> Result<u64, ParseIntError> {
    // remove any underscores
    let s = s.as_ref().replace('_', "");
    // remove prefix
    let s = s.trim_start_matches("0x");

    u64::from_str_radix(s, 16)
}
