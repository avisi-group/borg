use {
    super::Bus,
    crate::{arch::x86::memory::PhysAddrExt, devices::virtio::probe_virtio_block},
    acpi::{mcfg::PciConfigEntry, PciConfigRegions},
    log::trace,
    phf::phf_map,
    virtio_drivers::transport::pci::bus::{Cam, DeviceFunction, PciRoot},
    x86_64::PhysAddr,
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
