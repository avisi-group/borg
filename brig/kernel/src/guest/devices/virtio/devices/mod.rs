use {
    crate::guest::devices::virtio::queue::VirtQueue,
    alloc::vec::Vec,
    core::sync::atomic::{AtomicBool, AtomicU32, Ordering},
    virtio_drivers::transport::pci::VIRTIO_VENDOR_ID,
};

mod block;

const VIRTIO_MAGIC: u32 = u32::from_le_bytes([b'v', b'i', b'r', b't']);
const VIRTIO_VERSION: u32 = 0x2;
const VIRTIO_DEV_NET: u16 = 0x1;
const VIRTIO_DEV_BLK: u16 = 0x2;

#[derive(Debug, Clone, Copy)]
enum ReadRegister {
    Magic,
    Version,
    DeviceId,
    VendorId,
    DeviceFeatures,
    QueueReady,
    QueueNumMax,
    InterruptStatus,
    Status,
    ConfigGeneration,
}

impl ReadRegister {
    fn from_offset(offset: u64) -> Self {
        match offset {
            0x00 => Self::Magic,
            0x04 => Self::Version,
            0x08 => Self::DeviceId,
            0x0c => Self::VendorId,
            0x10 => Self::DeviceFeatures,
            0x34 => Self::QueueNumMax,
            0x44 => Self::QueueReady,
            0x60 => Self::InterruptStatus,
            0x70 => Self::Status,
            0xfc => Self::ConfigGeneration,
            _ => panic!("unknown read reg offset {offset:x}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum WriteRegister {
    DeviceFeaturesSelect,
    DriverFeatures,
    DriverFeaturesSelect,
    QueueSelect,
    QueueNum,
    QueueReady,
    QueueNotify,
    InterruptAcknowledge,
    Status,
    QueueDescriptorLow,
    QueueDescriptorHigh,
    QueueAvailableLow,
    QueueAvailableHigh,
    QueueUsedLow,
    QueueUsedHigh,
}

impl WriteRegister {
    fn from_offset(offset: u64) -> Self {
        match offset {
            0x14 => Self::DeviceFeaturesSelect,
            0x20 => Self::DriverFeatures,
            0x24 => Self::DriverFeaturesSelect,
            0x30 => Self::QueueSelect,
            0x38 => Self::QueueNum,
            0x44 => Self::QueueReady,
            0x50 => Self::QueueNotify,
            0x64 => Self::InterruptAcknowledge,
            0x70 => Self::Status,
            0x80 => Self::QueueDescriptorLow,
            0x84 => Self::QueueDescriptorHigh,
            0x90 => Self::QueueAvailableLow,
            0x94 => Self::QueueAvailableHigh,
            0xa0 => Self::QueueUsedLow,
            0xa4 => Self::QueueUsedHigh,
            _ => panic!("unknown write reg offset {offset:x}"),
        }
    }
}

#[derive(Debug)]
struct Virtio {
    device_id: u16,
    device_feature_select: bool,
    device_features: [u32; 2],
    driver_features_select: u32,
    driver_features: u32,
    isr: u32,
    status: u32,
    queues: Vec<VirtQueue>,
    queue_select: usize,
}

impl Virtio {
    fn new(num_queues: usize, device_id: u16) -> Self {
        assert!(num_queues > 0);
        Self {
            device_id,
            device_feature_select: false,
            device_features: [0, 0],
            isr: 0,
            status: 0,
            queues: (0..num_queues)
                .into_iter()
                .map(|i| VirtQueue::new(i))
                .collect(),
            queue_select: 0,
            driver_features_select: 0,
            driver_features: 0,
        }
    }

    fn selected_queue(&self) -> &VirtQueue {
        &self.queues[self.queue_select]
    }
    fn selected_queue_mut(&mut self) -> &mut VirtQueue {
        &mut self.queues[self.queue_select]
    }

    fn selected_device_feature(&self) -> u32 {
        self.device_features[usize::from(self.device_feature_select)]
    }

    fn set_host_feature(&mut self, idx: usize) {
        if idx > 31 {
            self.device_features[1] |= 1 << (idx - 32);
        } else {
            self.device_features[0] |= 1 << idx;
        }
    }

    fn clear_host_feature(&mut self, idx: usize) {
        if idx > 31 {
            self.device_features[1] &= !(1 << (idx - 32));
        } else {
            self.device_features[0] &= !(1 << idx);
        }
    }

    fn reset(&self) {
        // ???
    }

    fn read_register(&self, register: ReadRegister) -> u32 {
        use ReadRegister::*;

        let resp = match register {
            Magic => VIRTIO_MAGIC,
            Version => VIRTIO_VERSION,
            DeviceId => u32::from(self.device_id),
            VendorId => u32::from(VIRTIO_VENDOR_ID),
            DeviceFeatures => self.selected_device_feature(),
            QueueReady => u32::from(self.selected_queue().ready()),
            QueueNumMax => u32::try_from(self.selected_queue().num_max()).unwrap(),
            InterruptStatus => self.isr,
            Status => self.status,
            ConfigGeneration => 0,
        };

        log::error!("read reg {register:?}: {resp:?}");

        resp
    }

    fn write_register(&mut self, register: WriteRegister, value: u32) {
        use WriteRegister::*;

        log::error!("write reg {register:?}: {value:?}");

        match register {
            DeviceFeaturesSelect => {
                self.device_feature_select = match value {
                    0 => false,
                    1 => true,
                    _ => panic!(),
                };
            }
            Status => {
                self.status = value;

                if value == 0 {
                    self.driver_features_select = 0;
                    self.driver_features = 0;

                    self.reset();
                }
            }
            DriverFeatures => {
                self.driver_features = value;
            }
            DriverFeaturesSelect => {
                self.driver_features_select = value;
            }
            QueueSelect => {
                let idx = usize::try_from(value).unwrap();
                assert!(idx < self.queues.len());
                self.queue_select = idx;
            }
            QueueNum => {
                let num = usize::try_from(value).unwrap();
                self.selected_queue_mut().set_num(num);
            }
            QueueReady => {
                let ready = match value {
                    0 => false,
                    1 => true,
                    _ => panic!(),
                };
                self.selected_queue_mut().set_ready(ready);
            }
            QueueNotify => todo!(),
            InterruptAcknowledge => todo!(),
            QueueDescriptorLow => self.selected_queue_mut().set_descriptor_low(value),
            QueueDescriptorHigh => self.selected_queue_mut().set_descriptor_high(value),
            QueueAvailableLow => self.selected_queue_mut().set_available_low(value),
            QueueAvailableHigh => self.selected_queue_mut().set_available_high(value),
            QueueUsedLow => self.selected_queue_mut().set_used_low(value),
            QueueUsedHigh => self.selected_queue_mut().set_used_high(value),
        }
    }
}
