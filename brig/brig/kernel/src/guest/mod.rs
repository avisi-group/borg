use {
    crate::{
        devices::manager::SharedDeviceManager,
        fs::{tar::TarFilesystem, File, Filesystem},
        guest::memory::{AddressSpace, AddressSpaceRegion},
    },
    alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String},
    core::ptr,
    plugins_api::{GuestDevice, GuestDeviceFactory},
    spin::{Mutex, Once},
    x86::current::segmentation::{rdfsbase, wrfsbase},
};

pub mod config;
pub mod devices;
pub mod memory;

static mut GUEST: Once<Guest> = Once::INIT;
pub static mut GUEST_DEVICE_FACTORIES: Mutex<BTreeMap<String, Box<dyn GuestDeviceFactory>>> =
    Mutex::new(BTreeMap::new());

#[derive(Default)]
pub struct Guest {
    address_spaces: BTreeMap<String, Box<AddressSpace>>,
    devices: BTreeMap<String, Box<dyn GuestDevice>>,
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

        for (name, (region)) in regions {
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
    for (name, device) in config.devices {
        let factories = unsafe { GUEST_DEVICE_FACTORIES.lock() };

        let Some(factory) = factories.get(device.kind.as_str()) else {
            log::warn!("unsupported guest device type {}", device.kind);
            continue;
        };

        let dev = factory.create(
            [("tracer", "noop")]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .chain(device.extra.into_iter())
                .collect(),
        );
        guest.devices.insert(name.clone(), dev);

        // locate address space for attachment, if any
        // if let Some(attachment) = device.attach {
        //     let Some(io_handler) = dev.as_io_handler() else {
        //         panic!("attempting to attach non-mmio guest device");
        //     };

        //     if let Some(addrspace) =
        // guest.address_spaces.get_mut(&attachment.address_space) {
        //         let size = 0; // need to get this from device object io trait
        //         addrspace.add_region(AddressSpaceRegion::new(
        //             name,
        //
        // usize::from_str_radix(attachment.base.trim_start_matches("0x"),
        // 16).unwrap(),             size,
        //             memory::AddressSpaceRegionKind::IO(io_handler), /* need
        // to reference device
        //                                                              * object
        //                                                                io trait
        //                                                                */
        //         ));
        //     } else {
        //         panic!(
        //             "address space {} not configured for attaching device
        // {}",             attachment.address_space, name
        //         );
        //     }
        // } else if dev.as_io_handler().is_some() {
        //     panic!("io device missing address space attachment");
        // }
    }

    let temp_exec_ctx = Box::new(GuestExecutionContext {
        current_address_space: guest
            .address_spaces
            .get_mut(&String::from("as0"))
            .unwrap()
            .as_mut() as *mut AddressSpace,
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
