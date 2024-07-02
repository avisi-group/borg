#![no_std]

extern crate alloc;

use {
    aarch64_interpreter::{Aarch64Interpreter, TracerKind},
    alloc::{boxed::Box, collections::BTreeMap, string::String},
    plugins_rt::api::{GuestDevice, GuestDeviceFactory, IOMemoryHandler, PluginHeader, PluginHost},
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "aarch64",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);
    log::info!("loading aarch64");

    plugins_rt::get_host().register_device("aarch64", Box::new(Aarch64InterpreterFactory));
}

struct Aarch64InterpreterFactory;

impl GuestDeviceFactory for Aarch64InterpreterFactory {
    // todo: find a way of passing some config to guest device creation: json?
    // key-value?
    fn create(&self, config: BTreeMap<String, String>) -> Box<dyn GuestDevice> {
        const GUEST_MEMORY_BASE: usize = 0;
        const INITIAL_PC: usize = 0x8000_0000;

        let tracer = match config.get("tracer").map(String::as_str) {
            Some("log") => TracerKind::Log,
            Some("noop") | None => TracerKind::Noop,
            Some(t) => panic!("unknown tracer {t:?}"),
        };

        Box::new(Aarch64InterpreterDevice(Aarch64Interpreter::new(
            GUEST_MEMORY_BASE,
            INITIAL_PC,
            tracer,
        )))
    }
}

struct Aarch64InterpreterDevice(Aarch64Interpreter);

// impl guestdevice for architectureexecutor?
impl GuestDevice for Aarch64InterpreterDevice {
    fn start(&mut self) {
        self.0.run();
    }
    fn stop(&mut self) {
        todo!()
    }
    fn as_io_handler(self: Box<Self>) -> Option<Box<dyn IOMemoryHandler>> {
        None
    }
}
