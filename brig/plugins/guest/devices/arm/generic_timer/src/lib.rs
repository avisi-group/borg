#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    log::error,
    plugins_rt::api::{
        PluginHeader, PluginHost,
        guest::{Device, DeviceFactory, Environment},
    },
    spin::Mutex,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".plugin_header")]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "generic_timer",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);

    //host.register_device("generic_timer", Box::new(GlobalInterruptControllerFactory));

    log::info!("registered generic_timer factory");
}
