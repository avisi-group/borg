use crate::guest::{devices::GuestDevice, memory::IOMemoryHandler};

pub struct VirtIOBlock;

impl GuestDevice for VirtIOBlock {
    fn start(&self) {}

    fn stop(&self) {}

    fn as_io_handler(
        self: alloc::rc::Rc<Self>,
    ) -> Option<alloc::rc::Rc<dyn crate::guest::memory::IOMemoryHandler>> {
        Some(self.clone())
    }
}

impl IOMemoryHandler for VirtIOBlock {
    fn read(&self, _offset: usize, _buf: &mut [u8]) {
        todo!()
    }

    fn write(&self, _offset: usize, _buf: &[u8]) {
        todo!()
    }
}

impl VirtIOBlock {
    pub fn new() -> Self {
        Self
    }
}
