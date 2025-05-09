use {
    alloc::sync::Arc,
    common::hashmap::{HashMap, HashSet},
    plugins_api::object::{
        Object, ObjectId, ObjectStore,
        device::{Device, MemoryMappedDevice, RegisterMappedDevice},
        tickable::Tickable,
    },
    spin::{Lazy, Mutex},
};

static STORE: Lazy<SimpleStore> = Lazy::new(|| SimpleStore::new());

pub fn get() -> &'static impl ObjectStore {
    &*STORE
}

struct SimpleStore {
    state: Mutex<SimpleStoreState>,
}

#[derive(Default)]
struct SimpleStoreState {
    objects: HashMap<ObjectId, Arc<dyn Object>>,
    devices: HashSet<ObjectId>,
    memory_mapped_devices: HashSet<ObjectId>,
    register_mapped_devices: HashSet<ObjectId>,
    tickables: HashSet<ObjectId>,
}

impl SimpleStore {
    fn new() -> Self {
        Self {
            state: Mutex::new(SimpleStoreState::default()),
        }
    }
}

impl ObjectStore for SimpleStore {
    fn insert(&self, object: Arc<dyn Object>) {
        let mut guard = self.state.lock();

        if object.clone().to_device().is_some() {
            guard.devices.insert(object.id());
        }
        if object.clone().to_memory_mapped_device().is_some() {
            guard.memory_mapped_devices.insert(object.id());
        }

        if object.clone().to_register_mapped_device().is_some() {
            guard.register_mapped_devices.insert(object.id());
        }

        if object.clone().to_tickable().is_some() {
            guard.tickables.insert(object.id());
        }

        guard.objects.insert(object.id(), object);
    }

    fn get(&self, id: ObjectId) -> Option<Arc<dyn Object>> {
        self.state.lock().objects.get(&id).cloned()
    }

    fn get_device(&self, id: ObjectId) -> Option<Arc<dyn Device>> {
        let state = self.state.lock();

        if state.devices.contains(&id) {
            Some(state.objects.get(&id).unwrap().clone().to_device().unwrap())
        } else {
            None
        }
    }

    fn get_memory_mapped_device(&self, id: ObjectId) -> Option<Arc<dyn MemoryMappedDevice>> {
        let state = self.state.lock();

        if state.memory_mapped_devices.contains(&id) {
            Some(
                state
                    .objects
                    .get(&id)
                    .unwrap()
                    .clone()
                    .to_memory_mapped_device()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn get_register_mapped_device(&self, id: ObjectId) -> Option<Arc<dyn RegisterMappedDevice>> {
        let state = self.state.lock();

        if state.register_mapped_devices.contains(&id) {
            Some(
                state
                    .objects
                    .get(&id)
                    .unwrap()
                    .clone()
                    .to_register_mapped_device()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn get_tickable(&self, id: ObjectId) -> Option<Arc<dyn Tickable>> {
        let state = self.state.lock();

        if state.tickables.contains(&id) {
            Some(
                state
                    .objects
                    .get(&id)
                    .unwrap()
                    .clone()
                    .to_tickable()
                    .unwrap(),
            )
        } else {
            None
        }
    }
}
