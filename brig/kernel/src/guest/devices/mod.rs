use {crate::guest::memory::IOMemoryHandler, alloc::rc::Rc};

pub mod arm64generic;
pub mod pl011;
pub mod virtio;

pub trait GuestDevice {
    fn start(&self);
    fn stop(&self);
    fn as_io_handler(self: Rc<Self>) -> Option<Rc<dyn IOMemoryHandler>>;
}
