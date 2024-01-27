use {
    alloc::{collections::BTreeMap, format, string::String, sync::Arc},
    core::borrow::BorrowMut,
};

pub mod acpi;
pub mod lapic;
pub mod pcie;
pub mod pit;
pub mod serial;
pub mod virtio;

pub fn init() {}

struct DeviceManager {
    devices: BTreeMap<String, Arc<dyn Device>>,
    next_block_device_id: u32,
}

impl DeviceManager {
    fn register_device<D: Device>(&mut self, name: String, device: D) -> Arc<D> {
        // let mut dev = Arc::new(device);
        // dev.borrow_mut().configure();

        // self.devices.insert(name, dev);

        // dev
        todo!()
    }

    fn register_block_device<D: BlockDevice>(&mut self, device: D) -> Arc<dyn BlockDevice> {
        // let name = format!("disk{}", self.next_block_device_id);
        // self.next_block_device_id += 1;

        // self.register_device(name, device)
        todo!()
    }

    pub fn get_device(&self, name: String) -> Option<Arc<dyn Device>> {
        self.devices.get(&name).cloned()
    }
}

pub trait Device {
    fn configure(&mut self);
}

pub trait Bus<P> {
    fn probe(&self, probe_data: P);
}

// pub struct NetDeviceManager(BTreeMap<String, Arc<dyn BlockDevice>>);

// trait NetDevice {
//     fn read(&self, mac: u64, buf: &[u8]);
//     fn write(&self, mac: u64, buf: &[u8]);
// }

// pub struct BlockDeviceManager(BTreeMap<String, Arc<dyn BlockDevice>>);

pub trait BlockDevice: Device {
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
