use {
    crate::{
        arch::x86::memory::{PhysAddrExt, VirtAddrExt, VirtualMemoryArea},
        devices::{self, pcie::allocate_bars, BlockDevice, Device},
    },
    alloc::alloc::{alloc_zeroed, dealloc},
    byte_unit::Byte,
    core::{alloc::Layout, fmt::Debug, ptr::NonNull},
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

    allocate_bars(root, device_function);

    let transport = PciTransport::new::<VirtioHal>(root, device_function).unwrap();

    let blk = VirtIOBlk::<VirtioHal, _>::new(transport).unwrap();

    devices::manager::SharedDeviceManager::get().register_block_device(VirtioBlockDevice {
        blk,
        device_function,
    });
}

struct VirtioBlockDevice {
    blk: VirtIOBlk<VirtioHal, PciTransport>,
    device_function: DeviceFunction,
}

impl Debug for VirtioBlockDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "virtio block device @ {}, capacity: {:.2}, block size: {:.2}",
            self.device_function,
            Byte::from(self.size()).get_appropriate_unit(byte_unit::UnitType::Binary),
            Byte::from(self.block_size()).get_appropriate_unit(byte_unit::UnitType::Binary),
        )
    }
}

// safe because device is held behind mutex in device manager
unsafe impl Send for VirtioBlockDevice {}

impl Device for VirtioBlockDevice {
    fn configure(&mut self) {
        // no config needed?
    }
}

impl BlockDevice for VirtioBlockDevice {
    fn block_size(&self) -> usize {
        SECTOR_SIZE
    }

    fn size(&self) -> usize {
        usize::try_from(self.blk.capacity()).unwrap() * self.block_size()
    }

    fn read(&mut self, buf: &mut [u8], start_block_index: usize) -> Result<(), super::IoError> {
        self.blk
            .read_blocks(start_block_index, buf)
            .map_err(|e| panic!("{e:?}"))
    }

    fn write(&mut self, buf: &[u8], start_block_index: usize) -> Result<(), super::IoError> {
        self.blk
            .write_blocks(start_block_index, buf)
            .map_err(|e| panic!("{e:?}"))
    }
}
