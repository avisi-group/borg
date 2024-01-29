use {core::fmt::Debug, thiserror_core as thiserror};

pub mod acpi;
pub mod lapic;
pub mod manager;
pub mod pcie;
pub mod pit;
pub mod serial;
pub mod virtio;

pub trait Bus<P> {
    fn probe(&self, probe_data: P);
}

pub trait Device: Debug + Send {
    fn configure(&mut self);
}

/// Error type shared between block and network devices
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum IoError {
    /// Operation attempted to read or write past the end of the device
    _EndOfBlock,
}

pub trait BlockDevice: Device {
    /// Returns the size of the device in bytes
    fn size(&self) -> usize;

    /// Returns the block size of the device
    fn block_size(&self) -> usize;

    /// Reads `buf.len()` bytes into `buf` starting from the block with the
    /// supplied index.
    fn read(&mut self, buf: &mut [u8], start_block_index: usize) -> Result<(), IoError>;

    /// Writes `buf` starting from the block with the supplied index.
    fn write(&mut self, buf: &[u8], start_block_index: usize) -> Result<(), IoError>;
}

// Box<dyn Device> -> name, id, etc, kind/downcast

// dyn PcieDevice

// dyn BlockDevice

// dyn Virtiodevice
// platform responsible for discovery
// probe interface responsible for finding devices and probing other things
// "platform.probe() -> acpi.probe() -> discover lapic -> discover pcie"

// device manager remmebrs and provides access for devices
// need specialised subsystems for kinds of devices
// e.g. "blockdevicemanager"
