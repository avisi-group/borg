use {
    crate::{
        guest::devices::virtio::devices::{
            ReadRegister, VIRTIO_DEV_BLK, VIRTIO_MAGIC, VIRTIO_VERSION, Virtio, WriteRegister,
        },
        host::objects::{
            Object, ObjectId, ObjectStore, ToIrqController, ToRegisterMappedDevice, ToTickable,
            device::{Device, MemoryMappedDevice},
        },
    },
    alloc::{collections::BTreeMap, sync::Arc},
    common::intern::InternedString,
    core::sync::atomic::Ordering,
    proc_macro_lib::guest_device_factory,
    spin::Mutex,
    virtio_drivers::transport::pci::VIRTIO_VENDOR_ID,
};

#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed)]
struct Geometry {
    cylinders: u16,
    heads: u8,
    sectors: u8,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed)]
struct Topology {
    physical_block_exp: u8,
    alignment_offset: u8,
    min_io_size: u16,
    opt_io_size: u32,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed)]
struct Config {
    capacity: u64,
    size_max: u32,
    seg_max: u32,
    geometry: Geometry,
    block_size: u32,
    topology: Topology,
    reserved: u8,
}

#[guest_device_factory(virtio_block)]
fn create_virtio_block(_config: &BTreeMap<InternedString, InternedString>) -> Arc<dyn Device> {
    let dev = Arc::new(VirtioBlock::new());

    dev
}

#[derive(Debug)]
struct VirtioBlock {
    id: ObjectId,
    virtio: Mutex<Virtio>,
    config: Config,
}

impl VirtioBlock {
    fn new() -> Self {
        let mut celf = Self {
            id: ObjectId::new(),
            virtio: Mutex::new(Virtio::new(1, VIRTIO_DEV_BLK)),
            config: Config::default(),
        };

        celf.config.capacity = 8192;
        celf.config.block_size = 4096;

        celf.virtio.lock().set_host_feature(6);
        celf.virtio.lock().set_host_feature(32);

        celf
    }
}

impl Object for VirtioBlock {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl ToTickable for VirtioBlock {}
impl ToRegisterMappedDevice for VirtioBlock {}
impl ToIrqController for VirtioBlock {}

impl Device for VirtioBlock {
    fn start(&self) {}
    fn stop(&self) {}
}

impl MemoryMappedDevice for VirtioBlock {
    fn address_space_size(&self) -> u64 {
        0x1000
    }

    fn read(&self, offset: u64, value: &mut [u8]) {
        let register = ReadRegister::from_offset(offset);
        let read = self.virtio.lock().read_register(register);
        value.copy_from_slice(&read.to_le_bytes());
    }

    fn write(&self, offset: u64, value: &[u8]) {
        let register = WriteRegister::from_offset(offset);
        let value = u32::from_le_bytes(value.try_into().unwrap());
        self.virtio.lock().write_register(register, value);
    }
}
