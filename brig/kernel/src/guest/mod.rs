use {
    crate::{
        guest::{
            config::DeviceAttachment,
            memory::{AddressSpace, AddressSpaceRegion},
        },
        host::{
            dbt::sysreg_helpers::{self},
            fs::Filesystem,
            objects::{ObjectStore, device::Device},
        },
        util::encode_sysreg_id,
    },
    alloc::{boxed::Box, collections::BTreeMap, sync::Arc},
    common::intern::InternedString,
    core::ptr,
    spin::Once,
    x86::current::segmentation::{rdfsbase, wrfsbase},
};

pub mod config;
pub mod devices;
pub mod memory;

pub static mut GUEST: Once<Guest> = Once::INIT;

// pub static mut GUEST_DEVICE_FACTORIES: Mutex<BTreeMap<String, Box<dyn
// DeviceFactory>>> =     Mutex::new(BTreeMap::new());

// pub fn register_device_factory(name: String, factory: Box<dyn DeviceFactory>)
// {     unsafe { GUEST_DEVICE_FACTORIES.lock() }.insert(name, factory);
// }

#[derive(Default)]
pub struct Guest {
    pub address_spaces: BTreeMap<InternedString, Box<AddressSpace>>,
    pub devices: BTreeMap<InternedString, Arc<dyn Device>>,
}

impl Guest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
pub struct GuestExecutionContext {
    pub current_address_space: *mut AddressSpace,
    pub interrupt_pending: u64,
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
pub fn start<FS: Filesystem>(guest_data: &mut FS) {
    //check each connected block device for guest config
    let config = config::load_from_fs(guest_data).unwrap();

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
        // let factories = unsafe { GUEST_DEVICE_FACTORIES.lock() };

        // let Some(factory) = factories.get(device_config.kind.as_str()) else {
        //     panic!("unsupported guest device type {}", device_config.kind);
        // };

        // let device = factory.create(
        //     ObjectStore::get(),
        //     [("tracer", "noop")]
        //         .into_iter()
        //         .map(|(k, v)| (k.to_owned(), v.to_owned()))
        //         .chain(device_config.extra.into_iter())
        //         .collect(),
        // );

        let device = devices::create_device(device_config.kind, &device_config.extra).unwrap();

        guest.devices.insert(name.clone(), device.clone());
        ObjectStore::global().insert(device.clone());
        ObjectStore::global().insert_alias(device.id(), name.clone());

        // locate address space for attachment, if any
        match device_config.attach {
            Some(DeviceAttachment::Memory {
                address_space,
                base,
            }) => {
                let mem_map_device = ObjectStore::global()
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
                let reg_map_device = ObjectStore::global()
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
        current_address_space: guest
            .address_spaces
            .get_mut(&("as0".into()))
            .unwrap()
            .as_mut() as *mut AddressSpace,
        interrupt_pending: 0,
    });

    log::debug!("activating guest execution context");
    temp_exec_ctx.activate();

    {
        for load in config.load {
            let data = guest_data.read_to_vec(&load.path).unwrap();
            let pointer = load.address as *mut u8;

            log::warn!("loading {:?} @ {:p}", load.path, pointer);

            unsafe {
                ptr::copy(data.as_ptr(), pointer, data.len());
            }
        }
    }

    // go go go (start all devices)
    log::warn!("starting guest");

    for (_, device) in guest
        .devices
        .iter()
        .filter(|(name, _)| **name != InternedString::from_static("core0"))
    {
        device.start();
    }
    guest
        .devices
        .get(&InternedString::from_static("core0"))
        .unwrap()
        .start();
}
