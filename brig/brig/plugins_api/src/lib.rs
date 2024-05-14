#![no_std]

//! Plugin API definitions
//!
//! Plugins should depend on `plugins_rt`, which re-exports `plugins_api`. The
//! brig kernel depends on `plugins_api` directly.

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String},
    core::{alloc::GlobalAlloc, panic::PanicInfo},
};

/// Header information for the plugin, stored in the `.plugin_header` section
#[derive(Debug)]
pub struct PluginHeader {
    /// Name of the plugin
    pub name: &'static str,
    /// Entrypoint function of the plugin
    pub entrypoint: fn(&'static dyn PluginHost),
}

pub trait PluginHost: Send + Sync {
    /// Prints a message to the console
    fn print_message(&self, msg: &str, bare: bool);

    /// Gets a reference to the plugin host's global allocator
    fn allocator(&self) -> &dyn GlobalAlloc;

    fn register_device(
        &self,
        name: &'static str,
        guest_device_factory: Box<dyn GuestDeviceFactory>,
    );

    /// Panic from plugin
    fn panic(&self, info: &PanicInfo);
}

pub trait GuestDeviceFactory {
    fn create(&self, config: BTreeMap<String, String>) -> Box<dyn GuestDevice>;

    // todo: create_arch, create_timer, create_serial, etc
}

pub trait GuestDevice {
    fn start(&mut self);
    fn stop(&mut self);
    fn as_io_handler(self: Box<Self>) -> Option<Box<dyn IOMemoryHandler>>;
}

pub trait IOMemoryHandler {
    fn read(&self, offset: usize, buf: &mut [u8]);
    fn write(&self, offset: usize, buf: &[u8]);
}

// /// Factor of ArchitectureExecutors, used to create new instances of an
// /// `ArchitectureExecutor`.
// pub trait ArchitectureExecutorFactory: Send + Sync {
//     fn new(&self, guest_memory_base: usize, initial_pc: usize) -> Box<dyn
// ArchitectureExecutor>; }

// pub trait ArchitectureExecutor {
//     // can't have generic methods on dyn traits
//     // fn write_register<T>(&mut self, offset: isize, value: T);
//     // fn read_register<T: Copy>(&self, offset: isize) -> T;

//     fn get_pc(&self) -> usize;

//     fn guest_memory_base(&self) -> usize;

//     fn step(&mut self, amount: StepAmount) -> StepResult;

//     fn instructions_retired(&self) -> u64;
// }

// pub enum StepAmount {
//     Instruction,
//     BasicBlock,
//     Continuous,
// }

// pub enum StepResult {
//     Ok,
//     Halt,
// }
