use {
    crate::{arch::x86::memory::HEAP_ALLOCATOR, guest::register_device_factory, print},
    alloc::{borrow::ToOwned, boxed::Box},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
    plugins_api::{
        PluginHost,
        object::{ObjectId, device::DeviceFactory, tickable::Tickable},
    },
};

pub struct Host;

impl PluginHost for Host {
    fn print_message(&self, msg: &str, bare: bool) {
        if bare {
            print!("{}", msg);
        } else {
            log::info!("{}", msg)
        }
    }

    fn allocator(&self) -> &'static dyn GlobalAlloc {
        &HEAP_ALLOCATOR
    }

    fn register_device_factory(&self, name: &'static str, factory: Box<dyn DeviceFactory>) {
        register_device_factory(name.to_owned(), factory);
    }

    fn panic(&self, info: &PanicInfo) {
        log::error!("{info}");
        panic!("plugin panic");
    }

    fn register_periodic_tick(&self, frequency: u64, callback: &dyn Tickable) {
        todo!()
    }
}
