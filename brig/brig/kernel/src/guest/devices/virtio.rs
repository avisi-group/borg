use {
    alloc::{boxed::Box, collections::BTreeMap, string::String},
    plugins_api::{GuestDevice, GuestDeviceFactory, IOMemoryHandler},
};

pub struct VirtIOBlock;
pub struct VirtIOBlockFactory;

impl GuestDevice for VirtIOBlock {
    fn start(&mut self) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }

    fn as_io_handler(self: Box<Self>) -> Option<Box<dyn IOMemoryHandler>> {
        todo!()
    }
}

impl IOMemoryHandler for VirtIOBlock {
    fn read(&self, _offset: u64, _buf: &mut [u8]) {
        todo!()
    }

    fn write(&self, _offset: u64, _buf: &[u8]) {
        todo!()
    }
}

impl VirtIOBlock {
    pub fn new() -> Self {
        Self
    }
}

impl GuestDeviceFactory for VirtIOBlockFactory {
    fn create(&self, _config: BTreeMap<String, String>) -> Box<dyn GuestDevice> {
        Box::new(VirtIOBlock::new())
    }
}
