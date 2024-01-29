use {
    crate::devices::{BlockDevice, Device},
    alloc::{collections::BTreeMap, format, string::String, sync::Arc},
    spin::{Mutex, Once},
};

static mut DEVICE_MANAGER: Once<SharedDeviceManager> = Once::INIT;

pub fn init() {
    unsafe {
        DEVICE_MANAGER.call_once(|| SharedDeviceManager {
            devices: Mutex::new(BTreeMap::new()),
        });
    }
}

#[derive(Debug, Default)]
pub struct SharedDeviceManager {
    devices: Mutex<BTreeMap<String, Arc<Mutex<dyn Device>>>>,
}

impl SharedDeviceManager {
    pub fn get() -> &'static SharedDeviceManager {
        unsafe { DEVICE_MANAGER.get().unwrap() }
    }

    fn register_device<D: Device + 'static>(&self, name: String, device: D) {
        log::debug!("registering device {name:?}: {device:?}");
        let shared_device = Arc::new(Mutex::new(device));
        let res = self.devices.lock().insert(name.clone(), shared_device);

        if res.is_some() {
            panic!("device {name:?} already registered");
        }
    }

    fn count_block_devices(&self) -> usize {
        self.devices.lock().values().filter(|d| true).count()
    }

    pub fn register_block_device<B: BlockDevice + 'static>(&self, device: B) {
        let name = format!("disk{}", self.count_block_devices());
        //   let id = uuid::new();
        self.register_device(name, device);
        // id
    }

    // pub add_alias()
    // pub remove_alias()
    // pub get_aliases()

    pub fn _get_device(&self, name: String) -> Option<Arc<Mutex<dyn Device>>> {
        self.devices.lock().get(&name).cloned()
    }
}
