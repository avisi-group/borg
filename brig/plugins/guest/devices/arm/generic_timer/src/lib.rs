#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::sync::atomic::{AtomicU64, Ordering},
    plugins_rt::{
        api::{
            PluginHeader, PluginHost,
            object::{
                Object, ObjectId, ToDevice, ToMemoryMappedDevice, ToRegisterMappedDevice,
                ToTickable,
                device::{Device, DeviceFactory, RegisterMappedDevice},
                tickable::Tickable,
            },
            util::encode_sysreg_id,
        },
        get_host,
    },
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".plugin_header")]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "generic_timer",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);

    host.register_device_factory(
        "generic_timer",
        Box::new(GenericTimerFactory {
            id: ObjectId::new(),
        }),
    );

    log::info!("registered generic_timer factory");
}

struct GenericTimerFactory {
    id: ObjectId,
}

impl Object for GenericTimerFactory {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl ToDevice for GenericTimerFactory {}
impl ToTickable for GenericTimerFactory {}
impl ToRegisterMappedDevice for GenericTimerFactory {}
impl ToMemoryMappedDevice for GenericTimerFactory {}

impl DeviceFactory for GenericTimerFactory {
    fn create(&self, _config: BTreeMap<String, String>) -> Arc<dyn Device> {
        Arc::new(GenericTimer {
            id: ObjectId::new(),
            counter: AtomicU64::new(0),
        })
    }
}

const CNTKCTL_EL1: u64 = encode_sysreg_id(3, 0, 14, 1, 0);
const CNTFRQ_EL0: u64 = encode_sysreg_id(3, 3, 14, 0, 0);
const CNTPCT_EL0: u64 = encode_sysreg_id(3, 3, 14, 0, 1);
const CNTVCT_EL0: u64 = encode_sysreg_id(3, 3, 14, 0, 2);
const CNTP_TVAL_EL0: u64 = encode_sysreg_id(3, 3, 14, 2, 0);
const CNTP_CTL_EL0: u64 = encode_sysreg_id(3, 3, 14, 2, 1);
const CNTP_CVAL_EL0: u64 = encode_sysreg_id(3, 3, 14, 2, 2);
const CNTVOFF_EL2: u64 = encode_sysreg_id(3, 4, 14, 0, 3);
const CNTPS_TVAL_EL1: u64 = encode_sysreg_id(3, 7, 14, 2, 0);
const CNTPS_CTL_EL1: u64 = encode_sysreg_id(3, 7, 14, 2, 1);
const CNTPS_CVAL_EL1: u64 = encode_sysreg_id(3, 7, 14, 2, 2);
const CNTV_TVAL_EL0: u64 = encode_sysreg_id(3, 3, 14, 3, 0);
const CNTV_CTL_EL0: u64 = encode_sysreg_id(3, 3, 14, 3, 1);
const CNTV_CVAL_EL0: u64 = encode_sysreg_id(3, 3, 14, 3, 2);

#[derive(Debug)]
struct GenericTimer {
    id: ObjectId,
    counter: AtomicU64,
}

impl ToMemoryMappedDevice for GenericTimer {}

impl Tickable for GenericTimer {
    fn tick(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
}

impl Object for GenericTimer {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl Device for GenericTimer {
    fn start(&self) {
        get_host().register_periodic_tick(1000, self);
    }

    fn stop(&self) {}
}

impl RegisterMappedDevice for GenericTimer {
    fn read(&self, sys_reg_id: u64, value: &mut [u8]) {
        match sys_reg_id {
            CNTKCTL_EL1 => todo!(),
            CNTFRQ_EL0 => todo!(),
            CNTPCT_EL0 => todo!(),
            CNTVCT_EL0 => {
                value.copy_from_slice(&self.counter.load(Ordering::Relaxed).to_le_bytes());
            }
            CNTP_TVAL_EL0 => todo!(),
            CNTP_CTL_EL0 => todo!(),
            CNTP_CVAL_EL0 => todo!(),
            CNTVOFF_EL2 => todo!(),
            CNTPS_TVAL_EL1 => todo!(),
            CNTPS_CTL_EL1 => todo!(),
            CNTPS_CVAL_EL1 => todo!(),
            CNTV_TVAL_EL0 => todo!(),
            CNTV_CTL_EL0 => todo!(),
            CNTV_CVAL_EL0 => todo!(),
            _ => panic!("read unknown sys_reg_id {sys_reg_id:x}"),
        }
    }

    fn write(&self, sys_reg_id: u64, value: &[u8]) {
        match sys_reg_id {
            CNTKCTL_EL1 => todo!(),
            CNTFRQ_EL0 => todo!(),
            CNTPCT_EL0 => todo!(),
            CNTVCT_EL0 => todo!(),
            CNTP_TVAL_EL0 => todo!(),
            CNTP_CTL_EL0 => todo!(),
            CNTP_CVAL_EL0 => todo!(),
            CNTVOFF_EL2 => todo!(),
            CNTPS_TVAL_EL1 => todo!(),
            CNTPS_CTL_EL1 => todo!(),
            CNTPS_CVAL_EL1 => todo!(),
            CNTV_TVAL_EL0 => todo!(),
            CNTV_CTL_EL0 => todo!(),
            CNTV_CVAL_EL0 => todo!(),
            _ => panic!("write unknown sys_reg_id {sys_reg_id:x}"),
        }
    }
}
