#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    plugins_rt::{
        api::{
            PluginHeader, PluginHost,
            object::{
                Object, ObjectId, ToDevice, ToMemoryMappedDevice, ToRegisterMappedDevice,
                ToTickable,
                device::{Device, DeviceFactory, MemoryMappedDevice},
            },
        },
        get_host,
    },
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".plugin_header")]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "pl011",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);

    host.register_device_factory(
        "pl011",
        Box::new(Pl011Factory {
            id: ObjectId::new(),
        }),
    );

    log::info!("registered pl011 factory");
}

struct Pl011Factory {
    id: ObjectId,
}

impl Object for Pl011Factory {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl ToDevice for Pl011Factory {}
impl ToTickable for Pl011Factory {}
impl ToRegisterMappedDevice for Pl011Factory {}
impl ToMemoryMappedDevice for Pl011Factory {}

impl DeviceFactory for Pl011Factory {
    fn create(&self, _config: BTreeMap<String, String>) -> Arc<dyn Device> {
        Arc::new(Pl011 {
            id: ObjectId::new(),
        })
    }
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
            (0x0000, [c]) => get_host().print_message(&alloc::format!("{}", *c as char), true),

            // todo: https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers
            (offset, value) => log::trace!("PL011: wrote {value:x?} @ {offset:x}"),
        }
    }
}
