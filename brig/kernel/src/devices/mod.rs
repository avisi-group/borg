use alloc::{collections::BTreeMap, string::String, sync::Arc};

pub mod acpi;
pub mod lapic;
pub mod pcie;
pub mod pit;
pub mod serial;
pub mod virtio;

pub fn init() {}

struct DeviceManager {
    devices: BTreeMap<String, Arc<dyn Device>>,
}

impl DeviceManager {
    pub fn register_device<D: Device>(&self, device: D) -> &D {
        todo!()
    }

    pub fn get_device(&self, name: String) -> Arc<dyn Device> {
        todo!();
    }

    pub fn get_block_device(&self, name: String) -> Arc<dyn BlockDevice> {
        todo!();
    }
}

pub trait Device {
    fn configure(&mut self);
}

pub trait Bus<P> {
    fn probe(&self, probe_data: P);
}

pub struct NetDeviceManager(BTreeMap<String, Arc<dyn BlockDevice>>);

trait NetDevice {
    fn read(&self, mac: u64, buf: &[u8]);
    fn write(&self, mac: u64, buf: &[u8]);
}

pub struct BlockDeviceManager(BTreeMap<String, Arc<dyn BlockDevice>>);

trait BlockDevice {
    fn read(&self, buf: &[u8]);
    fn write(&self, buf: &[u8]);
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
