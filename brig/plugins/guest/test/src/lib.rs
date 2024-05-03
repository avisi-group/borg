#![no_std]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use plugins_api::PluginHost;

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
