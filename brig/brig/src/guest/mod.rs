use {
    crate::{
        devices::manager::SharedDeviceManager,
        guest::{
            devices::{core::Interpreter, GuestDevice},
            memory::{AddressSpace, AddressSpaceRegion},
        },
    },
    alloc::{boxed::Box, collections::BTreeMap, rc::Rc, string::String},
    core::ptr,
    spin::Once,
    x86::current::segmentation::{rdfsbase, wrfsbase},
};

pub mod config;
pub mod devices;
pub mod memory;

static mut GUEST: Once<Guest> = Once::INIT;

#[derive(Default)]
pub struct Guest {
    address_spaces: BTreeMap<String, Box<AddressSpace>>,
    devices: BTreeMap<String, Rc<dyn GuestDevice>>,
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

    let (config, kernel, dtb) = config::load_from_device(&device).unwrap();

    log::debug!("kernel len: {:#x}, got config: {:#?}", kernel.len(), config);

    unsafe { GUEST.call_once(Guest::new) };
    let guest = unsafe { GUEST.get_mut() }.unwrap();

    // create memory
    for (name, regions) in config.memory {
        let mut addrspace = AddressSpace::new();

        for (name, region) in regions {
            let (start, end) = (
                usize::from_str_radix(region.start.trim_start_matches("0x"), 16).unwrap(),
                usize::from_str_radix(region.end.trim_start_matches("0x"), 16).unwrap(),
            );

            addrspace.add_region(AddressSpaceRegion::new(
                name,
                start,
                end - start,
                memory::AddressSpaceRegionKind::Ram,
            ));
        }

        guest.address_spaces.insert(name, Box::new(addrspace));
    }

    // create devices, including cores
    for (name, device) in config.devices {
        let dev: Rc<dyn GuestDevice> = match device.kind.as_str() {
            "core" => match device
                .extra
                .get("arch".into())
                .expect("missing arch definition")
                .as_str()
            {
                "arm64" => Rc::new(self::devices::core::GenericCore::new::<
                    arch::State,
                    Interpreter,
                >()),
                _ => {
                    panic!("unsupported core arch")
                }
            },
            "pl011" => Rc::new(self::devices::pl011::PL011::new()),
            "virtio-blk" => Rc::new(self::devices::virtio::VirtIOBlock::new()),
            _ => {
                panic!("unsupported guest device type {}", device.kind);
            }
        };

        guest.devices.insert(name.clone(), dev.clone());

        // locate address space for attachment, if any
        if let Some(attachment) = device.attach {
            let Some(io_handler) = dev.as_io_handler() else {
                panic!("attempting to attach non-mmio guest device");
            };

            if let Some(addrspace) = guest.address_spaces.get_mut(&attachment.address_space) {
                let size = 0; // need to get this from device object io trait
                addrspace.add_region(AddressSpaceRegion::new(
                    name,
                    usize::from_str_radix(attachment.base.trim_start_matches("0x"), 16).unwrap(),
                    size,
                    memory::AddressSpaceRegionKind::IO(io_handler), /* need to reference device
                                                                     * object io trait */
                ));
            } else {
                panic!(
                    "address space {} not configured for attaching device {}",
                    attachment.address_space, name
                );
            }
        } else if dev.as_io_handler().is_some() {
            panic!("io device missing address space attachment");
        }
    }

    let temp_exec_ctx = Box::new(GuestExecutionContext {
        current_address_space: guest
            .address_spaces
            .get_mut(&String::from("as0"))
            .unwrap()
            .as_mut() as *mut AddressSpace,
    });

    temp_exec_ctx.activate();

    // initiate boot protocol
    match config.boot {
        config::BootProtocol::Arm64Linux(_) => {
            // todo read kernel from TAR

            // read header from kernel
            let header = unsafe { &*(kernel.as_ptr() as *const Arm64KernelHeader) };
            assert_eq!(ARM64_MAGIC, header.magic);

            // load kernel and dtb into guest physical memory
            unsafe {
                ptr::copy(
                    kernel.as_ptr(),
                    (usize::try_from(header.text_offset).unwrap() + KERNEL_LOAD_BIAS) as *mut u8,
                    kernel.len(),
                );

                ptr::copy(dtb.as_ptr(), (DTB_LOAD_OFFSET) as *mut u8, dtb.len());
            }
        }
    }

    // go go go (start all devices)
    for device in guest.devices.values() {
        device.start();
    }
}

const KERNEL_LOAD_BIAS: usize = 0x8000_0000;
const DTB_LOAD_OFFSET: usize = 0x9000_0000;

const ARM64_MAGIC: u32 = 0x644d5241;

#[derive(Debug)]
#[repr(C)]
struct Arm64KernelHeader {
    code0: u32,
    code1: u32,
    text_offset: u64,
    image_size: u64,
    flags: u64,
    res2: u64,
    res3: u64,
    res4: u64,
    magic: u32,
    res5: u32,
}
