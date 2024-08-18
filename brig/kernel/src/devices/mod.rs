use {
    alloc::{boxed::Box, sync::Arc},
    core::{
        fmt::Debug,
        ops::{Deref, DerefMut},
    },
    spin::{Mutex, MutexGuard},
    thiserror_core as thiserror,
};

pub mod acpi;
pub mod guest;
pub mod lapic;
pub mod manager;
pub mod pcie;
pub mod pit;
pub mod serial;
pub mod virtio;

pub trait Bus<P> {
    fn probe(&self, probe_data: P);
}

#[derive(Debug, Clone)]
pub struct SharedDevice {
    inner: Arc<Mutex<Device>>,
}

impl SharedDevice {
    pub fn from_device(device: Device) -> Self {
        Self {
            inner: Arc::new(Mutex::new(device)),
        }
    }

    pub fn lock(&self) -> MutexGuard<Device> {
        self.inner.lock()
    }
}

// no clone for now
#[derive(Debug)]
pub enum Device {
    Block(Box<dyn BlockDevice>),
    #[allow(unused)]
    Net(Box<dyn NetDevice>),
    #[allow(unused)]
    Timer(Box<dyn Timer>),
}

impl Device {
    /// Panics if underlying device is not a block device
    pub fn as_block(&mut self) -> &mut Box<dyn BlockDevice> {
        let Device::Block(ref mut blk) = self else {
            panic!("not a block device");
        };

        blk
    }
}

impl From<Box<dyn BlockDevice>> for Device {
    fn from(value: Box<dyn BlockDevice>) -> Self {
        Self::Block(value)
    }
}

impl From<Box<dyn NetDevice>> for Device {
    fn from(value: Box<dyn NetDevice>) -> Self {
        Self::Net(value)
    }
}

impl From<Box<dyn Timer>> for Device {
    fn from(value: Box<dyn Timer>) -> Self {
        Self::Timer(value)
    }
}

/// Error type shared between block and network devices
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum IoError {
    /// Operation attempted to read or write past the end of the device
    _EndOfBlock,
}

pub trait BlockDevice: Debug + Send + Sync {
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

// Messy but required in order to pass a `dyn BlockDevice` as a generic argument
// implementing `BlockDevice`.
// See https://users.rust-lang.org/t/why-does-dyn-trait-not-implement-trait/30052
impl BlockDevice for Box<dyn BlockDevice> {
    fn size(&self) -> usize {
        self.deref().size()
    }

    fn block_size(&self) -> usize {
        self.deref().block_size()
    }

    fn read(
        &mut self,
        buf: &mut [u8],
        start_block_index: usize,
    ) -> Result<(), crate::devices::IoError> {
        self.deref_mut().read(buf, start_block_index)
    }

    fn write(
        &mut self,
        buf: &[u8],
        start_block_index: usize,
    ) -> Result<(), crate::devices::IoError> {
        self.deref_mut().write(buf, start_block_index)
    }
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

pub trait NetDevice: Debug + Send + Sync {}

pub trait Timer: Debug + Send + Sync {}
