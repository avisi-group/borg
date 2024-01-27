use {
    super::BlockDevice,
    crate::{
        arch::x86::memory::{PhysAddrExt, VirtAddrExt, VirtualMemoryArea},
        devices::Device,
        guest,
    },
    alloc::{
        alloc::{alloc_zeroed, dealloc},
        vec,
    },
    byte_unit::Byte,
    core::{alloc::Layout, ptr::NonNull},
    log::trace,
    virtio_drivers::{
        device::blk::{VirtIOBlk, SECTOR_SIZE},
        transport::pci::{
            bus::{Command, DeviceFunction, PciRoot},
            PciTransport,
        },
    },
    x86_64::{PhysAddr, VirtAddr},
};

struct VirtioHal;

unsafe impl virtio_drivers::Hal for VirtioHal {
    fn dma_alloc(
        pages: usize,
        _direction: virtio_drivers::BufferDirection,
    ) -> (virtio_drivers::PhysAddr, NonNull<u8>) {
        let ptr = unsafe {
            alloc_zeroed(
                Layout::from_size_align(
                    pages * virtio_drivers::PAGE_SIZE,
                    virtio_drivers::PAGE_SIZE,
                )
                .unwrap(),
            )
        };

        let paddr = VirtAddr::from_ptr(ptr)
            .to_phys()
            .as_u64()
            .try_into()
            .unwrap();

        let vaddr = NonNull::new(ptr).unwrap();

        (paddr, vaddr)
    }

    unsafe fn dma_dealloc(
        paddr: virtio_drivers::PhysAddr,
        vaddr: NonNull<u8>,
        pages: usize,
    ) -> i32 {
        log::trace!("dma_dealloc: {paddr:x} {vaddr:p} {pages:x}");
        dealloc(
            vaddr.as_ptr(),
            Layout::from_size_align(pages * virtio_drivers::PAGE_SIZE, virtio_drivers::PAGE_SIZE)
                .unwrap(),
        );

        0
    }

    unsafe fn mmio_phys_to_virt(paddr: virtio_drivers::PhysAddr, _size: usize) -> NonNull<u8> {
        let physical_address = PhysAddr::new(u64::try_from(paddr).unwrap());
        NonNull::new(physical_address.to_virt().as_mut_ptr()).unwrap()
    }

    unsafe fn share(
        buffer: NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) -> virtio_drivers::PhysAddr {
        VirtualMemoryArea::current()
            .translate_address(VirtAddr::from_ptr(buffer.as_ptr() as *const u8))
            .unwrap()
            .as_u64()
            .try_into()
            .unwrap()
    }

    unsafe fn unshare(
        _paddr: virtio_drivers::PhysAddr,
        _buffer: NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) {
    }
}

pub fn probe_virtio_block(root: &mut PciRoot, device_function: DeviceFunction) {
    trace!("probing virtio block device");

    root.set_command(
        device_function,
        Command::IO_SPACE | Command::MEMORY_SPACE | Command::BUS_MASTER,
    );

    let transport = PciTransport::new::<VirtioHal>(root, device_function).unwrap();

    let mut disk = VirtIOBlk::<VirtioHal, _>::new(transport).unwrap();
    let len = usize::try_from(disk.capacity()).unwrap() * SECTOR_SIZE;

    log::trace!(
        "VirtIO block device: {}",
        Byte::from(len).get_appropriate_unit(byte_unit::UnitType::Binary)
    );

    let (config, kernel, _dt) = {
        // todo: maybeuninit
        let mut buf = vec![0u8; len];
        disk.read_blocks(0, &mut buf).unwrap();
        guest::config::load_guest_config(&buf).unwrap()
    };

    log::trace!("kernel len: {:#x}, got config: {:#?}", kernel.len(), config);
}

// struct VirtIOBlockDevice;

// impl Device for VirtIOBlockDevice {
//     fn configure(&mut self) {
//         todo!()
//     }
// }

// impl BlockDevice for VirtIOBlockDevice {
//     fn read(&self, buf: &[u8]) {
//         todo!()
//     }

//     fn write(&self, buf: &[u8]) {
//         todo!()
//     }
// }
