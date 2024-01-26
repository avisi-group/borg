pub mod acpi;
pub mod lapic;
pub mod pcie;
pub mod pit;
pub mod serial;
pub mod virtio;

pub fn init() {}

struct DeviceManager {}

impl DeviceManager {
    pub fn register_device<D: Device>(&self, device: D) -> &D {
        todo!()
    }
}

pub trait Device {
    fn configure(&mut self);
}

pub trait Bus<P> {
    fn probe(&self, probe_data: P);
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
