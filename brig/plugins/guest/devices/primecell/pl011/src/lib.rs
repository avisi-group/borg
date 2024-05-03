#![no_std]
extern crate alloc;

use alloc::rc::Rc;
use brig::guest::{
    devices::{GuestDevice, GuestDeviceFactory},
    memory::IOMemoryHandler,
};
use brig::plugins::Plugin;

pub struct PluginInst;

impl Plugin for PluginInst {
    fn name(&self) -> &'static str {
        "pl011"
    }

    fn superspecificferdianame(&self, _: u32) -> u32 {
        0xDEADBEEF
    }
}

pub struct PL011;
pub struct PL011Factory;

impl PL011 {
    pub fn new() -> Self {
        Self
    }
}

impl GuestDevice for PL011 {
    fn start(&self) {}

    fn stop(&self) {}

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

impl GuestDeviceFactory for PL011Factory {
    fn create(&self) -> Rc<dyn GuestDevice> {
        Rc::new(PL011::new())
    }
}
