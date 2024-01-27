use {
    crate::devices::{BlockDevice, Device},
    alloc::{collections::BTreeMap, string::String, sync::Arc},
    spin::{Mutex, Once},
};

static mut DEVICE_MANAGER: Once<SharedDeviceManager> = Once::INIT;

pub fn init() {
    unsafe {
        DEVICE_MANAGER.call_once(|| SharedDeviceManager(Mutex::new(BTreeMap::new())));
    }
}

#[derive(Debug, Default)]
pub struct SharedDeviceManager(Mutex<BTreeMap<String, Arc<Mutex<dyn Device>>>>);

impl SharedDeviceManager {
    pub fn get() -> &'static SharedDeviceManager {
        unsafe { DEVICE_MANAGER.get().unwrap() }
    }

    pub fn register_device<D: Device + 'static>(&self, name: String, device: D) {
        log::debug!("registering device {name:?}: {device:?}");
        let shared_device = Arc::new(Mutex::new(device));
        let res = self.0.lock().insert(name.clone(), shared_device);

        if res.is_some() {
            panic!("device {name:?} already registered");
        }
    }

    pub fn register_block_device<B: BlockDevice + 'static>(&self, name: String, device: B) {
        self.register_device(name, device);
    }

    pub fn _get_device(&self, name: String) -> Option<Arc<Mutex<dyn Device>>> {
        self.0.lock().get(&name).cloned()
    }
}
