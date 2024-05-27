use {
    crate::{
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
        plugins::{host::Host, shared_object::SharedObject},
    },
    alloc::collections::BTreeMap,
    elfloader::ElfBinary,
    plugins_api::PluginHeader,
    spin::Mutex,
};

mod host;
mod shared_object;

static PLUGIN_REGISTRY: PluginRegistry = PluginRegistry::new();

struct PluginRegistry {
    plugins: Mutex<BTreeMap<&'static str, Plugin>>,
}

impl PluginRegistry {
    pub const fn new() -> Self {
        Self {
            plugins: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn register(&self, plugin: Plugin) {
        self.plugins.lock().insert(plugin.header.name, plugin);
    }
}

struct Plugin {
    // whole point of this is to prevent the shared object getting deallocated
    _object: SharedObject,
    // todo: figure out how to make the header the lifetime of the shared object
    header: &'static PluginHeader,
}

impl Plugin {
    pub fn load(data: &[u8]) -> Self {
        let binary = ElfBinary::new(data).unwrap();

        let _object = SharedObject::from_elf(&binary);

        let header = binary.file.find_section_by_name(".plugin_header").unwrap();
        let translated_header_address = _object.translate_virt_addr(header.address());
        let header = unsafe { &*translated_header_address.as_ptr::<PluginHeader>() };

        Self { _object, header }
    }
}

pub fn load_all(device: &SharedDevice) {
    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());

    log::info!("loading plugins");
    // todo: don't hardcode this, load everything in plugins directory
    [
        "plugins/libtest.so",
        "plugins/libpl011.so",
        "plugins/libaarch64.so",
    ]
    .into_iter()
    .map(|path| fs.open(path).unwrap().read_to_vec().unwrap())
    .map(|data| Plugin::load(&data))
    .for_each(|plugin| {
        // run entrypoint and register plugin
        (plugin.header.entrypoint)(&Host);
        PLUGIN_REGISTRY.register(plugin);
    });
}
