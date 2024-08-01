use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    plugins_api::guest::{Device, DeviceFactory, Environment},
};

#[derive(Debug)]
pub struct VirtIOBlock;

pub struct VirtIOBlockFactory;

impl Device for VirtIOBlock {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn address_space_size(&self) -> u64 {
        todo!()
    }

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

impl DeviceFactory for VirtIOBlockFactory {
    fn create(
        &self,
        _config: BTreeMap<String, String>,
        _env: Box<dyn Environment>,
    ) -> Arc<dyn Device> {
        Arc::new(VirtIOBlock::new())
    }
}
