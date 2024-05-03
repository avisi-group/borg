#![no_std]

use brig::plugins::Plugin;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn name(&self) -> &'static str {
        "test"
    }

    fn superspecificferdianame(&self, a: u32) -> u32 {
        a.pow(3) + 5
    }
}

static TEST_PLUGIN: TestPlugin = TestPlugin;

#[no_mangle]
#[link_section = ".plugins"]
pub static TEST_PLUGIN_R: &'static dyn Plugin = &TEST_PLUGIN;
