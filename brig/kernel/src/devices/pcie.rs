use {
    super::Bus,
    crate::{
        arch::x86::memory::{PhysAddrExt, VirtualMemoryArea},
        devices::virtio::probe_virtio_block,
    },
    acpi::{mcfg::PciConfigEntry, PciConfigRegions},
    log::trace,
    phf::phf_map,
    virtio_drivers::transport::pci::bus::{BarInfo, Cam, DeviceFunction, MemoryBarType, PciRoot},
    x86_64::{
        structures::paging::{Page, PageSize, PageTableFlags, PhysFrame, Size4KiB},
        PhysAddr,
    },
};

pub struct PCIEBus;

impl Bus<PciConfigRegions<'_, alloc::alloc::Global>> for PCIEBus {
    fn probe(&self, probe_data: PciConfigRegions<'_, alloc::alloc::Global>) {
        probe_data.iter().for_each(enumerate);
    }
}

type ProbeFn = fn(&mut PciRoot, DeviceFunction);

static PCI_DRIVER_MAP: phf::Map<u32, ProbeFn> = phf_map! {
    0x1af41001u32 => probe_virtio_block,
    0x80862922u32 => probe_ich9r
};

pub fn enumerate(
    PciConfigEntry {
        bus_range,
        physical_address,
        ..
    }: PciConfigEntry,
) {
    let physical_address = PhysAddr::new(u64::try_from(physical_address).unwrap());
    log::debug!("enumerating pcie {:?} {:x}", bus_range, physical_address);

    let mut root = unsafe { PciRoot::new(physical_address.to_virt().as_mut_ptr(), Cam::Ecam) };

    for bus in bus_range {
        root.enumerate_bus(bus).for_each(|(dev_fn, dev_fn_info)| {
            let key = ((dev_fn_info.vendor_id as u32) << 16) | dev_fn_info.device_id as u32;

            if let Some(prober) = PCI_DRIVER_MAP.get(&key) {
                prober(&mut root, dev_fn);
            } else {
                log::warn!(
                    "unsupported pcie device {:04x}:{:04x}",
                    dev_fn_info.vendor_id,
                    dev_fn_info.device_id
                );
            }
        });
    }
}

fn probe_ich9r(_root: &mut PciRoot, _device_function: DeviceFunction) {
    trace!("probing sata controller");
}

/// Iterator over BARs in a device
struct BarIter<'root> {
    root: &'root mut PciRoot,
    dev_fn: DeviceFunction,
    current_index: u8,
}

impl<'root> BarIter<'root> {
    fn new(root: &'root mut PciRoot, dev_fn: DeviceFunction) -> Self {
        Self {
            root,
            dev_fn,
            current_index: 0,
        }
    }
}

impl<'root> Iterator for BarIter<'root> {
    type Item = BarInfo;

    fn next(&mut self) -> Option<Self::Item> {
        // BAR array is length 6
        if self.current_index >= 6 {
            return None;
        }

        let info = self.root.bar_info(self.dev_fn, self.current_index).unwrap();

        if info.takes_two_entries() {
            self.current_index += 2;
        } else {
            self.current_index += 1;
        }

        Some(info)
    }
}

/// Map all 64-bit memory BARs of a device to virtual memory
pub fn allocate_bars(root: &mut PciRoot, device_function: DeviceFunction) {
    BarIter::new(root, device_function)
        // remove IO and 32-bit memory BARs
        .filter_map(|bar| match bar {
            BarInfo::Memory {
                address_type: MemoryBarType::Width64,
                address,
                size,
                ..
            } => Some((address, size)),
            _ => None,
        })
        // for each mappable 64-bit BAR...
        .for_each(|(address, size)| {
            // ...calculate it's size in pages...
            let num_pages = u64::from(size) / Size4KiB::SIZE;

            (0..num_pages)
                // ...calculate the offset of each page...
                .map(|page_idx| page_idx * Size4KiB::SIZE)
                // ...calculate the physical and virtual addresses of each page...
                .map(|page_offset| {
                    let phys = PhysAddr::new(address + page_offset);
                    let virt = phys.to_virt();
                    (phys, virt)
                })
                // ...and map them
                .for_each(|(phys, virt)| {
                    VirtualMemoryArea::current().map_page(
                        Page::<Size4KiB>::from_start_address(virt).unwrap(),
                        PhysFrame::<Size4KiB>::from_start_address(phys).unwrap(),
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    );
                });
        });

    VirtualMemoryArea::current().invalidate();
}
