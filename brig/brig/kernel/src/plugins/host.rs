use {
    crate::{arch::x86::memory::HEAP_ALLOCATOR, guest::GUEST_DEVICE_FACTORIES},
    alloc::{borrow::ToOwned, boxed::Box},
    core::alloc::GlobalAlloc,
    plugins_api::PluginHost,
};

pub struct Host;

impl PluginHost for Host {
    fn print_message(&self, msg: &str) {
        log::info!("{}", msg)
    }

    fn allocator(&self) -> &'static dyn GlobalAlloc {
        &HEAP_ALLOCATOR
    }

    fn register_device(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn plugins_api::GuestDeviceFactory>,
    ) {
        unsafe { GUEST_DEVICE_FACTORIES.lock() }.insert(name.to_owned(), guest_device_factory);
    }
}
