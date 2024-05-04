#![no_std]

use {core::panic::PanicInfo, plugins_api::PluginHost};

#[no_mangle]
#[link_section = ".plugin_entrypoint"]
pub extern "Rust" fn entrypoint(host: &dyn PluginHost) {
    host.print_message("hello from test!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // todo!
    loop {}
}
