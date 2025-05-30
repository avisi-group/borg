use {
    crate::{guest::config, host::objects::device::Device},
    alloc::{collections::btree_map::BTreeMap, sync::Arc},
    common::intern::InternedString,
    linkme::distributed_slice,
};

pub mod arm;
pub mod primecell;

//pub type DeviceConfig =

#[distributed_slice]
pub static DEVICE_FACTORIES: [(&str, fn(&config::Device) -> Arc<dyn Device>)];

pub fn create_device(config: &config::Device) -> Option<Arc<dyn Device>> {
    DEVICE_FACTORIES
        .iter()
        .find(|(n, _)| *n == config.kind.as_ref())
        .map(|(_, f)| f(config))
}
