#![no_std]

extern crate alloc;

use {
    aarch64_interpreter::{Aarch64Interpreter, TracerKind},
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    plugins_rt::api::{
        guest::{Device, DeviceFactory, Environment},
        util::parse_hex_prefix,
        PluginHeader, PluginHost,
    },
    spin::Mutex,
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

impl DeviceFactory for Aarch64InterpreterFactory {
    // todo: find a way of passing some config to guest device creation: json?
    // key-value?
    fn create(
        &self,
        config: BTreeMap<String, String>,
        environment: Box<dyn Environment>,
    ) -> Arc<dyn Device> {
        let tracer = match config.get("tracer").map(String::as_str) {
            Some("log") => TracerKind::Log,
            Some("noop") | None => TracerKind::Noop,
            Some(t) => panic!("unknown tracer {t:?}"),
        };

        let initial_pc = config
            .get("initial_pc")
            .map(parse_hex_prefix)
            .unwrap()
            .unwrap();

        Arc::new(Aarch64InterpreterDevice(Mutex::new(
            Aarch64Interpreter::new(initial_pc, tracer, environment),
        )))
    }
}

#[derive(Debug)]
struct Aarch64InterpreterDevice(Mutex<Aarch64Interpreter>);

impl Device for Aarch64InterpreterDevice {
    fn start(&self) {
        self.0.lock().run();
    }
    fn stop(&self) {
        todo!()
    }

    fn address_space_size(&self) -> u64 {
        0x0
    }

    fn read(&self, _: u64, _: &mut [u8]) {
        panic!("cannot read aarch64 interpreter")
    }
    fn write(&self, _: u64, _: &[u8]) {
        panic!("cannot write aarch64 interpreter")
    }
}
