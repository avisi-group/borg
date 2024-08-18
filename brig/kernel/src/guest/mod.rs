use {
    crate::{
        devices::manager::SharedDeviceManager,
        fs::{tar::TarFilesystem, File, Filesystem},
        guest::memory::{AddressSpace, AddressSpaceRegion, AddressSpaceRegionKind},
    },
    alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::ptr,
    plugins_api::guest,
    spin::{Mutex, Once},
    x86::current::segmentation::{rdfsbase, wrfsbase},
};

pub mod config;
pub mod devices;
pub mod memory;

static mut GUEST: Once<Guest> = Once::INIT;

static mut GUEST_DEVICE_FACTORIES: Mutex<BTreeMap<String, Box<dyn guest::DeviceFactory>>> =
    Mutex::new(BTreeMap::new());

pub fn register_device_factory(name: String, factory: Box<dyn guest::DeviceFactory>) {
    unsafe { GUEST_DEVICE_FACTORIES.lock() }.insert(name, factory);
}

#[derive(Default)]
pub struct Guest {
    address_spaces: BTreeMap<String, Box<AddressSpace>>,
    devices: BTreeMap<String, Arc<dyn guest::Device>>,
}

impl Guest {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct GuestExecutionContext {
    pub current_address_space: *mut AddressSpace,
}

impl GuestExecutionContext {
    pub fn activate(self: Box<Self>) {
        unsafe {
            wrfsbase(Box::into_raw(self) as u64);
        }
    }

    pub fn current() -> &'static Self {
        unsafe { &*(rdfsbase() as *const Self) }
    }
}

/// Start guest emulation
pub fn start() {
    log::info!("starting guest");

    //check each connected block device for guest config
    let device_manager = SharedDeviceManager::get();
    let device = device_manager
        .get_device_by_alias("disk00:03.0")
        .expect("disk not found");

    let config = config::load_from_device(&device).unwrap();

    log::debug!("got config: {:#?}", config);

    unsafe { GUEST.call_once(Guest::new) };
    let guest = unsafe { GUEST.get_mut() }.unwrap();

    // create memory
    for (name, regions) in config.memory {
        let mut addrspace = AddressSpace::new();

        for (name, region) in regions {
            addrspace.add_region(AddressSpaceRegion::new(
                name,
                region.start,
                region.end - region.start,
                memory::AddressSpaceRegionKind::Ram,
            ));
        }

        guest.address_spaces.insert(name, Box::new(addrspace));
    }

    // create devices, including cores
    for (name, device_config) in config.devices {
        let factories = unsafe { GUEST_DEVICE_FACTORIES.lock() };

        let Some(factory) = factories.get(device_config.kind.as_str()) else {
            log::warn!("unsupported guest device type {}", device_config.kind);
            continue;
        };

        let guest_environment = Box::new(BrigGuestEnvironment {
            address_space: &(**guest.address_spaces.get("as0").unwrap()) as *const _,
        });

        let device = factory.create(
            [("tracer", "noop")]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .chain(device_config.extra.into_iter())
                .collect(),
            guest_environment,
        );
        guest.devices.insert(name.clone(), device.clone());

        // locate address space for attachment, if any
        if let Some(attachment) = device_config.attach {
            if let Some(addrspace) = guest.address_spaces.get_mut(&attachment.address_space) {
                addrspace.add_region(AddressSpaceRegion::new(
                    name,
                    attachment.base,
                    device.address_space_size(),
                    memory::AddressSpaceRegionKind::IO(device.clone()),
                ));
            } else {
                panic!(
                    "address space {} not configured for attaching device {}",
                    attachment.address_space, name
                );
            }
        }
    }

    let temp_exec_ctx = Box::new(GuestExecutionContext {
        current_address_space: guest.address_spaces.get_mut("as0").unwrap().as_mut()
            as *mut AddressSpace,
    });

    log::debug!("activating guest execution context");
    temp_exec_ctx.activate();

    {
        let mut device = device.lock();
        let mut fs = TarFilesystem::mount(device.as_block());

        for load in config.load {
            let data = fs.open(&load.path).unwrap().read_to_vec().unwrap();
            let pointer = load.address as *mut u8;

            log::info!("loading {:?} @ {:p}", load.path, pointer);

            unsafe {
                ptr::copy(data.as_ptr(), pointer, data.len());
            }
        }
    }

    // go go go (start all devices)
    log::info!("starting guest");

    for device in guest.devices.values_mut() {
        device.start();
    }
}

struct BrigGuestEnvironment {
    address_space: *const AddressSpace,
}

impl guest::Environment for BrigGuestEnvironment {
    fn read_memory(&self, address: u64, data: &mut [u8]) {
        let region = unsafe { &*self.address_space }
            .find_region(address)
            .unwrap();

        match region.kind() {
            // just read bytes
            AddressSpaceRegionKind::Ram => unsafe {
                ptr::copy(address as *const u8, data.as_mut_ptr(), data.len())
            },
            // or forward the request on to the IO handler
            AddressSpaceRegionKind::IO(device) => device.read(address - region.base(), data),
        }
    }

    fn write_memory(&self, address: u64, data: &[u8]) {
        // lookup address in address space
        let region = unsafe { &*self.address_space }
            .find_region(address)
            .unwrap();

        match region.kind() {
            // just write bytes
            AddressSpaceRegionKind::Ram => unsafe {
                ptr::copy(data.as_ptr(), address as *mut u8, data.len())
            },
            // or forward the request on to the IO handler
            AddressSpaceRegionKind::IO(device) => device.write(address - region.base(), data),
        }
    }
}
