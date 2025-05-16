use {
    crate::host::objects::device::{Device, RegisterMappedDevice},
    alloc::{collections::BTreeMap, sync::Arc},
    common::hashmap::HashMap,
    spin::{Lazy, Mutex},
};

enum Handler {
    Device(Arc<dyn RegisterMappedDevice>),
    // ttbr0
}

static SYSREG_HANDLERS: Lazy<Mutex<HashMap<u64, Handler>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

pub fn register_device(id: u64, device: Arc<dyn RegisterMappedDevice>) {
    SYSREG_HANDLERS.lock().insert(id, Handler::Device(device));
}

pub fn handler_exists(reg: u64) -> bool {
    let res = SYSREG_HANDLERS.lock().contains_key(&reg);
    log::debug!("{reg:x}: {res:?}");
    res
}

pub fn sys_reg_read(reg: u64) -> u64 {
    let guard = SYSREG_HANDLERS.lock();
    let handler = guard.get(&reg).unwrap();

    let mut result = [0u8; 8];

    match handler {
        Handler::Device(dev) => dev.read(reg, &mut result),
    }

    u64::from_le_bytes(result)
}

pub fn sys_reg_write(reg: u64, value: u64, len: u8) {
    let guard = SYSREG_HANDLERS.lock();
    let handler = guard.get(&reg).unwrap();

    // TODO: 'len'
    match handler {
        Handler::Device(dev) => dev.write(reg, value.to_le_bytes().as_slice()),
    }
}
