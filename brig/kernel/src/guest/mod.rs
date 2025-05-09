use {
    crate::{
        dbt::sysreg_helpers::{self},
        devices::manager::SharedDeviceManager,
        fs::{File, Filesystem, tar::TarFilesystem},
        guest::{
            config::DeviceAttachment,
            memory::{AddressSpace, AddressSpaceRegion},
        },
        object_store,
    },
    alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::ptr,
    plugins_api::{
        object::{
            ObjectStore,
            device::{Device, DeviceFactory},
        },
        util::encode_sysreg_id,
    },
    spin::{Mutex, Once},
    x86::current::segmentation::{rdfsbase, wrfsbase},
};

pub mod config;
pub mod memory;

pub static mut GUEST: Once<Guest> = Once::INIT;

pub static mut GUEST_DEVICE_FACTORIES: Mutex<BTreeMap<String, Box<dyn DeviceFactory>>> =
    Mutex::new(BTreeMap::new());

pub fn register_device_factory(name: String, factory: Box<dyn DeviceFactory>) {
    unsafe { GUEST_DEVICE_FACTORIES.lock() }.insert(name, factory);
}

#[derive(Default)]
pub struct Guest {
    pub address_spaces: BTreeMap<String, Box<AddressSpace>>,
    pub devices: BTreeMap<String, Arc<dyn Device>>,
}

impl Guest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
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
    //check each connected block device for guest config
    let device_manager = SharedDeviceManager::get();
    let device = device_manager
        .get_device_by_alias("disk00:03.0")
        .expect("disk not found");

    let config = config::load_from_device(&device).unwrap();

    log::debug!("got config: {:#x?}", config);

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
            panic!("unsupported guest device type {}", device_config.kind);
        };

        let device = factory.create(
            [("tracer", "noop")]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .chain(device_config.extra.into_iter())
                .collect(),
        );
        guest.devices.insert(name.clone(), device.clone());

        // locate address space for attachment, if any
        match device_config.attach {
            Some(DeviceAttachment::Memory {
                address_space,
                base,
            }) => {
                let mem_map_device = object_store::get()
                    .get_memory_mapped_device(device.id())
                    .unwrap();

                if let Some(addrspace) = guest.address_spaces.get_mut(&address_space) {
                    addrspace.add_region(AddressSpaceRegion::new(
                        name,
                        base,
                        mem_map_device.address_space_size(),
                        memory::AddressSpaceRegionKind::IO(mem_map_device.clone()),
                    ));
                } else {
                    panic!(
                        "address space {} not configured for attaching device {}",
                        address_space, name
                    );
                }
            }
            Some(DeviceAttachment::SysReg(sysregs)) => {
                let reg_map_device = object_store::get()
                    .get_register_mapped_device(device.id())
                    .unwrap();

                sysregs
                    .iter()
                    .map(|(_, [op0, op1, crn, crm, op2])| {
                        encode_sysreg_id(*op0, *op1, *crn, *crm, *op2)
                    })
                    .for_each(|id| {
                        sysreg_helpers::register_device(id, reg_map_device.clone());
                    });
            }
            None => (),
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

            log::warn!("loading {:?} @ {:p}", load.path, pointer);

            unsafe {
                ptr::copy(data.as_ptr(), pointer, data.len());
            }
        }
    }

    // go go go (start all devices)
    log::warn!("starting guest");

    for device in guest.devices.values_mut() {
        device.start();
    }
}
