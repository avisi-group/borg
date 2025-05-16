use {
    crate::{
        arch::x86::memory::HEAP_ALLOCATOR, guest::register_device_factory, object_store, print,
        timer::register_tickable,
    },
    alloc::{borrow::ToOwned, boxed::Box},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
    embedded_time::duration::Nanoseconds,
    plugins_api::{
        PluginHost,
        object::{ObjectId, ObjectStore, device::DeviceFactory, tickable::Tickable},
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

    fn register_periodic_tick(&self, interval: Nanoseconds<u64>, tickable: &dyn Tickable) {
        let tickable = object_store::get().get_tickable(tickable.id()).unwrap();
        register_tickable(interval, tickable);
    }

    fn object_store(&self) -> &dyn ObjectStore {
        object_store::get()
    }
}
