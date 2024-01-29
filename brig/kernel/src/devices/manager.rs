use {
    crate::{
        devices::{BlockDevice, Device},
        rand::new_uuid_v4,
    },
    alloc::{borrow::ToOwned, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec},
    spin::{Mutex, Once},
    uuid::Uuid,
};

static DEVICE_MANAGER: Once<SharedDeviceManager> = Once::INIT;

pub fn init() {
    DEVICE_MANAGER.call_once(|| SharedDeviceManager::default());
}

#[derive(Debug, Default)]
pub struct SharedDeviceManager {
    inner: Mutex<InnerDeviceManager>,
}

#[derive(Debug, Default)]
pub struct InnerDeviceManager {
    devices: BTreeMap<Uuid, Arc<Mutex<dyn Device>>>,
    aliases: BTreeMap<String, Uuid>,
}

impl SharedDeviceManager {
    pub fn get() -> &'static SharedDeviceManager {
        DEVICE_MANAGER.get().unwrap()
    }

    fn register_device<D: Device + 'static>(&self, device: D) -> Uuid {
        let id = new_uuid_v4();
        log::debug!("registering device {id:?}: {device:?}");
        let shared_device = Arc::new(Mutex::new(device));
        let res = self.inner.lock().devices.insert(id, shared_device);

        if res.is_some() {
            panic!("device {id:?} already registered");
        }

        id
    }

    fn count_block_devices(&self) -> usize {
        self.inner.lock().devices.values().filter(|d| true).count()
    }

    pub fn register_block_device<B: BlockDevice + 'static>(&self, device: B) -> Uuid {
        let id = self.register_device(device);

        let name = format!("disk{}", self.count_block_devices());
        self.add_alias(id, name);

        id
    }

    pub fn add_alias<S: AsRef<str>>(&self, device: Uuid, alias: S) {
        self.inner
            .lock()
            .aliases
            .insert(alias.as_ref().to_owned(), device);
    }

    pub fn remove_alias<S: AsRef<str>>(&self, alias: S) {
        self.inner.lock().aliases.remove(alias.as_ref());
    }

    /// This could be more efficient by returning an iterator, but would require
    /// holding the lock and possibly risking deadlock.
    pub fn get_aliases<S: AsRef<str>>(&self, device: Uuid) -> Vec<String> {
        self.inner
            .lock()
            .aliases
            .iter()
            .filter(|(k, v)| **v == device)
            .map(|(k, _)| k)
            .cloned()
            .collect()
    }

    pub fn lookup_alias<S: AsRef<str>>(&self, alias: S) -> Option<Uuid> {
        self.inner.lock().aliases.get(alias.as_ref()).copied()
    }

    pub fn _get_device(&self, name: String) -> Option<Arc<Mutex<dyn Device>>> {
        // self.inner.lock().devices.get(&name).cloned()
        todo!()
    }
}
