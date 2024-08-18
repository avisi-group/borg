//! Interfaces for emulated guests

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::fmt::Debug,
};

/// Guest's external environment (the host system)
pub trait Environment {
    fn read_memory(&self, address: u64, data: &mut [u8]);
    fn write_memory(&self, address: u64, data: &[u8]);
}

/// Manufacturer of guest devices
pub trait DeviceFactory {
    fn create(
        &self,
        config: BTreeMap<String, String>,
        environment: Box<dyn Environment>,
    ) -> Arc<dyn Device>;
}

/// Emulated guest device
pub trait Device: Debug {
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
