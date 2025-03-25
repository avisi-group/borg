#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    plugins_rt::api::{
        PluginHeader, PluginHost,
        guest::{Device, DeviceFactory, Environment},
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

    host.register_device("pl011", Box::new(Pl011Factory));

    log::info!("registered pl011 factory");
}

struct Pl011Factory;

impl DeviceFactory for Pl011Factory {
    fn create(
        &self,
        _config: BTreeMap<String, String>,
        _guest_environment: Box<dyn Environment>,
    ) -> Arc<dyn Device> {
        Arc::new(Pl011)
    }
}

#[derive(Debug)]
struct Pl011;

impl Device for Pl011 {
    fn start(&self) {}
    fn stop(&self) {}

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
            (0x0000, [c]) => log::trace!("{}", *c as char),

            // todo: https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers
            (offset, value) => log::trace!("PL011: wrote {value:x?} @ {offset:x}"),
        }
    }
}
