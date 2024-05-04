#![no_std]

use plugins_rt::{
    api::{PluginHeader, PluginHost},
    host,
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "test plugin",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);
    host::get().print_message("hello from test!");
}
