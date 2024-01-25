pub mod acpi;
pub mod lapic;
pub mod pcie;
pub mod pit;
pub mod serial;

pub fn init(rsdp_phys: usize) {
    acpi::init(rsdp_phys);
    lapic::init();
}

struct DeviceManager {}

trait Device {
    fn name(&self) -> &'static str;
}

trait BlockDevice {
    fn read();
    fn write();
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
