use {
    crate::{devices::pcie, memory::PhysAddrExt},
    acpi::{AcpiHandler, AcpiTables, PciConfigRegions, PhysicalMapping},
    core::ptr::NonNull,
    x86_64::PhysAddr,
};

pub fn init(rsdp: usize) {
    let tables = unsafe { AcpiTables::from_rsdp(Handler, rsdp) }.unwrap();
    PciConfigRegions::new(&tables)
        .unwrap()
        .iter()
        .for_each(|entry| pcie::enumerate(entry));
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
