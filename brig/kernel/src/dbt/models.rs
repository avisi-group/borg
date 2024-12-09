use {
    crate::{
        dbt::init_register_file,
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
        guest::register_device_factory,
    },
    alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::btree_map::BTreeMap,
        string::{String, ToString},
        sync::Arc,
        vec::Vec,
    },
    common::rudder::Model,
    core::fmt::{self, Debug},
    plugins_api::{
        guest::{Device, DeviceFactory},
        util::parse_hex_prefix,
    },
    spin::Mutex,
};

static MODEL_MANAGER: Mutex<BTreeMap<String, Arc<Model>>> = Mutex::new(BTreeMap::new());

pub fn register_model(name: &str, model: Model) {
    log::info!("registering {name:?} ISA model");
    let model = Arc::new(model);
    MODEL_MANAGER.lock().insert(name.to_owned(), model.clone());

    register_device_factory(
        name.to_string(),
        Box::new(ModelDeviceFactory::new(name.to_string(), model)),
    )
}

pub fn get(name: &str) -> Option<Arc<Model>> {
    MODEL_MANAGER.lock().get(name).cloned()
}

pub fn load_all(device: &SharedDevice) {
    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());

    log::info!("loading models");
    // todo: don't hardcode this, load all .postcards?
    ["aarch64.postcard"]
        .into_iter()
        .map(|path| {
            (
                path.strip_suffix(".postcard").unwrap(),
                fs.open(path).unwrap().read_to_vec().unwrap(),
            )
        })
        .map(|(name, data)| (name, postcard::from_bytes(&data).unwrap()))
        .for_each(|(name, model)| {
            register_model(name, model);
        });
}

/// Factory for creating execution instances for a supplied model
struct ModelDeviceFactory {
    name: String,
    model: Arc<Model>,
}

impl ModelDeviceFactory {
    fn new(name: String, model: Arc<Model>) -> Self {
        Self { name, model }
    }
}

impl DeviceFactory for ModelDeviceFactory {
    fn create(
        &self,
        config: BTreeMap<String, String>,
        _environment: Box<dyn plugins_api::guest::Environment>,
    ) -> Arc<dyn plugins_api::guest::Device> {
        let initial_pc = config
            .get("initial_pc")
            .map(parse_hex_prefix)
            .unwrap()
            .unwrap();

        Arc::new(ModelDevice::new(
            self.name.clone(),
            self.model.clone(),
            initial_pc,
        ))
    }
}

struct ModelDevice {
    name: String,
    model: Arc<Model>,
    register_file: Vec<u8>,
}

impl ModelDevice {
    fn new(name: String, model: Arc<Model>, initial_pc: u64) -> Self {
        let mut register_file = alloc::vec![0u8; model.register_file_size()];

        init_register_file(&*model, register_file.as_mut_ptr());

        unsafe {
            *(register_file.as_mut_ptr().add(model.reg_offset("_PC")) as *mut u64) = initial_pc
        }

        Self {
            name,
            model,
            register_file,
        }
    }
}

impl Debug for ModelDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModelDevice({})", self.name)
    }
}

impl Device for ModelDevice {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn address_space_size(&self) -> u64 {
        u64::MAX
    }

    fn read(&self, _offset: u64, _value: &mut [u8]) {
        todo!()
    }

    fn write(&self, _offset: u64, _value: &[u8]) {
        todo!()
    }
}
