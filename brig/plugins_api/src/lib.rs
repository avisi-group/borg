#![no_std]

use core::alloc::GlobalAlloc;

pub trait PluginHost: Send + Sync {
    /// Prints a message to the console, returning the length of the string (as a test value)
    fn print_message(&self, msg: &str) -> usize;

    fn allocator(&self) -> &dyn GlobalAlloc;
}
