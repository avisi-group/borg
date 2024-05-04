#![no_std]

use core::alloc::GlobalAlloc;

pub trait PluginHost: Send + Sync {
    /// Prints a message to the console
    fn print_message(&self, msg: &str);

    fn allocator(&self) -> &dyn GlobalAlloc;
}
