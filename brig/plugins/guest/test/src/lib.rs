#![no_std]

use {
    core::{
        fmt::{self, Write},
        panic::PanicInfo,
    },
    plugins_api::PluginHost,
};

#[no_mangle]
#[link_section = ".plugin_entrypoint"]
pub extern "C" fn entrypoint(host: &dyn PluginHost) {
    host.print_message("hello from test!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // todo!
    loop {}
}
