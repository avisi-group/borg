#![no_std]

//! Plugin API definitions
//!
//! Plugins should depend on `plugins_rt`, which re-exports `plugins_api`. The brig kernel depends on `plugins_api` directly.

use core::alloc::GlobalAlloc;

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
    fn print_message(&self, msg: &str);

    /// Gets a reference to the plugin host's global allocator
    fn allocator(&self) -> &dyn GlobalAlloc;
}
