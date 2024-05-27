use {
    crate::{arch::x86::memory::HEAP_ALLOCATOR, guest::GUEST_DEVICE_FACTORIES, print},
    alloc::{borrow::ToOwned, boxed::Box},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
    plugins_api::PluginHost,
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

    fn register_device(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn plugins_api::GuestDeviceFactory>,
    ) {
        unsafe { GUEST_DEVICE_FACTORIES.lock() }.insert(name.to_owned(), guest_device_factory);
    }

    fn panic(&self, info: &PanicInfo) {
        log::error!("{info}");
        panic!("plugin panic");
    }
}
