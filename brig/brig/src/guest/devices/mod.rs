use {crate::guest::memory::IOMemoryHandler, alloc::rc::Rc};

pub mod arch;
pub mod virtio;

pub trait GuestDevice {
    fn start(&self);
    fn stop(&self);
    fn as_io_handler(self: Rc<Self>) -> Option<Rc<dyn IOMemoryHandler>>;
}

pub trait GuestDeviceFactory
{
    fn create(&self) -> Rc<dyn GuestDevice>;
}
