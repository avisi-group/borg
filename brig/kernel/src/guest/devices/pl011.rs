use crate::guest::{devices::GuestDevice, memory::IOMemoryHandler};

pub struct PL011;

impl PL011 {
    pub fn new() -> Self {
        Self
    }
}

impl GuestDevice for PL011 {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn as_io_handler(self: alloc::rc::Rc<Self>) -> Option<alloc::rc::Rc<dyn IOMemoryHandler>> {
        Some(self.clone())
    }
}

impl IOMemoryHandler for PL011 {
    fn read(&self, _offset: usize, _buf: &mut [u8]) {
        todo!()
    }

    fn write(&self, _offset: usize, _buf: &[u8]) {
        todo!()
    }
}
