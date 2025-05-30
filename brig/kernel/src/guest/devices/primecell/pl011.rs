use {
    crate::{
        guest::config,
        host::objects::{
            Object, ObjectId, ObjectStore, ToIrqController, ToRegisterMappedDevice, ToTickable,
            device::{Device, MemoryMappedDevice},
        },
    },
    alloc::{collections::BTreeMap, sync::Arc},
    common::intern::InternedString,
    proc_macro_lib::guest_device_factory,
};

#[guest_device_factory(pl011)]
fn create_pl011(_config: &config::Device) -> Arc<dyn Device> {
    let dev = Arc::new(Pl011 {
        id: ObjectId::new(),
    });

    dev
}

#[derive(Debug)]
struct Pl011 {
    id: ObjectId,
}

impl Object for Pl011 {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl ToTickable for Pl011 {}
impl ToRegisterMappedDevice for Pl011 {}
impl ToIrqController for Pl011 {}

impl Device for Pl011 {
    fn start(&self) {}
    fn stop(&self) {}
}

impl MemoryMappedDevice for Pl011 {
    fn address_space_size(&self) -> u64 {
        0x1000
    }

    /// Read `value.len()` bytes from the device starting at `offset`
    fn read(&self, _offset: u64, value: &mut [u8]) {
        // return all zeros for now
        value.fill(0);
    }

    /// Write `value` bytes into the device starting at `offset`
    fn write(&self, offset: u64, value: &[u8]) {
        match (offset, value) {
            (0x0000, _) => crate::print!("{}", value[0] as char),

            // todo: https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers
            (offset, value) => log::debug!("PL011: wrote {value:x?} @ {offset:x}"),
        }
    }
}
