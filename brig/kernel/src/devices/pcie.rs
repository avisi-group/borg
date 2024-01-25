use {
    crate::{
        dbg,
        memory::{PhysAddrExt, VirtAddrExt, HEAP_ALLOCATOR},

    },
    acpi::mcfg::PciConfigEntry,
    alloc::vec,
    byte_unit::Byte,
    core::{alloc::Layout, ptr::NonNull},
    virtio_drivers::{
        device::blk::{VirtIOBlk, SECTOR_SIZE},
        transport::pci::{
            bus::{Cam, DeviceFunction, PciRoot},
            PciTransport,
        },
    },
    x86_64::{PhysAddr, VirtAddr},
};

pub fn enumerate(
    PciConfigEntry {
        bus_range,
        physical_address,
        ..
    }: PciConfigEntry,
) {
    let physical_address = PhysAddr::new(u64::try_from(physical_address).unwrap());
    log::debug!("pcie {:?} {:x}", bus_range, physical_address);

    let mut root = unsafe { PciRoot::new(physical_address.to_virt().as_mut_ptr(), Cam::MmioCam) };

    for bus in bus_range {
        root.enumerate_bus(bus).for_each(|(dev_fn, dev_fn_info)| {
            match (dev_fn_info.vendor_id, dev_fn_info.device_id) {
                (0x1af4, 0x1001) => {
                    register_pcie_virtio_block(&mut root, dev_fn);
                }
                (vid, did) => log::warn!("unsupported pcie device {vid}:{did}"),
            }
        });
    }
}

struct NoopHal;

unsafe impl virtio_drivers::Hal for NoopHal {
    fn dma_alloc(
        pages: usize,
        direction: virtio_drivers::BufferDirection,
    ) -> (virtio_drivers::PhysAddr, NonNull<u8>) {
        let ptr = HEAP_ALLOCATOR
            .lock()
            .alloc(
                Layout::from_size_align(
                    pages * virtio_drivers::PAGE_SIZE,
                    virtio_drivers::PAGE_SIZE,
                )
                .unwrap(),
            )
            .unwrap();

        (
            VirtAddr::from_ptr(ptr.as_ptr())
                .to_phys()
                .as_u64()
                .try_into()
                .unwrap(),
            ptr,
        )
    }

    unsafe fn dma_dealloc(
        paddr: virtio_drivers::PhysAddr,
        vaddr: NonNull<u8>,
        pages: usize,
    ) -> i32 {
        todo!()
    }

    unsafe fn mmio_phys_to_virt(paddr: virtio_drivers::PhysAddr, _size: usize) -> NonNull<u8> {
        let physical_address = PhysAddr::new(u64::try_from(paddr).unwrap());
        NonNull::new(physical_address.to_virt().as_mut_ptr()).unwrap()
    }

    unsafe fn share(
        buffer: NonNull<[u8]>,
        direction: virtio_drivers::BufferDirection,
    ) -> virtio_drivers::PhysAddr {
        let allocation = HEAP_ALLOCATOR
            .lock()
            .alloc(Layout::from_size_align(buffer.len(), 16).unwrap())
            .unwrap();

        dbg!((direction, buffer.len()));

        match direction {
            virtio_drivers::BufferDirection::DeviceToDriver
            | virtio_drivers::BufferDirection::Both => {
                // do copy
                allocation.copy_from(buffer.as_non_null_ptr(), buffer.len());
            }

            virtio_drivers::BufferDirection::DriverToDevice => {
                // do nothing
            }
        }

        VirtAddr::from_ptr(allocation.as_ptr())
            .to_phys()
            .as_u64()
            .try_into()
            .unwrap()
    }

    unsafe fn unshare(
        paddr: virtio_drivers::PhysAddr,
        buffer: NonNull<[u8]>,
        direction: virtio_drivers::BufferDirection,
    ) {
        todo!()
    }
}

fn register_pcie_virtio_block(root: &mut PciRoot, device_function: DeviceFunction) {
    let transport = PciTransport::new::<NoopHal>(root, device_function).unwrap();
    dbg!(&transport);
    let mut disk = VirtIOBlk::<NoopHal, _>::new(transport).unwrap();
    log::trace!(
        "VirtIO block device: {}",
        Byte::from(disk.capacity() * SECTOR_SIZE as u64)
            .get_appropriate_unit(byte_unit::UnitType::Binary)
    );
    let mut buf = vec![0u8; 4096];
    log::trace!("{:p}", buf.as_ptr());
    disk.read_blocks(0, &mut buf).unwrap();
    dbg!(buf);
}
