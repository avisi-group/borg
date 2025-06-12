use {
    crate::{
        guest::devices::virtio::devices::{
            ReadRegister, VIRTIO_DEV_BLK, VIRTIO_MAGIC, VIRTIO_VERSION, Virtio, WriteRegister,
        },
        host::objects::{
            Object, ObjectId, ObjectStore, ToIrqController, ToRegisterMappedDevice, ToTickable,
            device::{Device, MemoryMappedDevice},
        },
        util::any_as_u8_slice,
    },
    alloc::{collections::BTreeMap, sync::Arc},
    common::intern::InternedString,
    core::sync::atomic::Ordering,
    proc_macro_lib::guest_device_factory,
    spin::Mutex,
    virtio_bindings::virtio_blk::virtio_blk_config,
    virtio_drivers::transport::pci::VIRTIO_VENDOR_ID,
};

#[guest_device_factory(virtio_block)]
fn create_virtio_block(config: &BTreeMap<InternedString, InternedString>) -> Arc<dyn Device> {
    let dev = Arc::new(VirtioBlock::new(64));

    dev
}

#[derive(Debug)]
struct VirtioBlock {
    id: ObjectId,
    virtio: Mutex<Virtio>,
    config: virtio_blk_config,
}

impl VirtioBlock {
    fn new(irq_line: usize) -> Self {
        let mut celf = Self {
            id: ObjectId::new(),
            virtio: Mutex::new(Virtio::new(1, VIRTIO_DEV_BLK)),
            config: virtio_blk_config::default(),
        };

        celf.config.capacity = 1 * 1024 * 1024;
        celf.config.blk_size = 4096;

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

    fn read(&self, offset: u64, dest: &mut [u8]) {
        if offset <= 0xff {
            let register = ReadRegister::from_offset(offset);
            let read = self.virtio.lock().read_register(register);
            dest.copy_from_slice(&read.to_le_bytes());
        } else {
            let config_offset = usize::try_from(offset - 0x100).unwrap();

            log::warn!("reading config @ {config_offset:x}");

            let config = unsafe { any_as_u8_slice(&self.config) };
            let start = config_offset;
            let end = config_offset + dest.len();

            dest.copy_from_slice(&config[start..end]);
        }
    }

    fn write(&self, offset: u64, value: &[u8]) {
        let register = WriteRegister::from_offset(offset);
        let value = u32::from_le_bytes(value.try_into().unwrap());
        self.virtio.lock().write_register(register, value);
    }
}
