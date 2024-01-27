use {
    crate::{
        arch::x86::memory::{PhysAddrExt, VirtualMemoryArea},
        devices::{pcie::PCIEBus, Bus},
    },
    acpi::{AcpiHandler, AcpiTables, PciConfigRegions, PhysicalMapping},
    core::ptr::NonNull,
    x86_64::{
        structures::paging::{Page, PageTableFlags, PhysFrame, Size1GiB},
        PhysAddr, VirtAddr,
    },
};

pub struct ACPIBus;

impl Bus<PhysAddr> for ACPIBus {
    fn probe(&self, probe_data: PhysAddr) {
        let tables =
            unsafe { AcpiTables::from_rsdp(Handler, probe_data.as_u64() as usize) }.unwrap();

        // todo: do this dynamically
        VirtualMemoryArea::current().map_page(
            Page::<Size1GiB>::from_start_address(VirtAddr::new(0xffff818800000000)).unwrap(),
            PhysFrame::<Size1GiB>::from_start_address(PhysAddr::new(0x800000000)).unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
        VirtualMemoryArea::current().invalidate();

        PCIEBus.probe(PciConfigRegions::new(&tables).unwrap())
    }
}

#[derive(Clone)]
struct Handler;

impl AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        let virt_addr = PhysAddr::new(u64::try_from(physical_address).unwrap()).to_virt();

        PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr()).unwrap(),
            size,
            size,
            Self,
        )
    }

    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) {}
}
