#![no_std]

extern crate alloc;

use {
    alloc::{format, vec::Vec},
    plugins_rt::{
        api::{PluginHeader, PluginHost},
        host,
    },
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "pl011",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);

    let mut vec = Vec::new();
    for i in 0..32 {
        vec.push(i);
    }
    vec.extend_from_slice(b"test string");

    host::get().print_message(&format!("hello from pl011! {:?}", vec));
}
