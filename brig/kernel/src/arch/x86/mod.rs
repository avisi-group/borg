use {
    crate::{
        arch::x86::memory::PHYSICAL_MEMORY_OFFSET,
        devices::{acpi, lapic, Bus},
    },
    bootloader_api::BootInfo,
    x86_64::{PhysAddr, VirtAddr},
};

pub mod backtrace;
mod gdt;
pub mod irq;
pub mod memory;

pub fn init(
    BootInfo {
        memory_regions,
        physical_memory_offset,
        rsdp_addr,
        kernel_addr,
        kernel_len,
        kernel_image_offset,
        ..
    }: &BootInfo,
) {
    // if physical memory offset was wrong, all phys-virt conversions would be wrong
    assert_eq!(
        PHYSICAL_MEMORY_OFFSET.as_u64(),
        physical_memory_offset
            .into_option()
            .expect("physical memory offset missing from boot info"),
        "physical memory offset reported by bootloader should be {:x}",
        PHYSICAL_MEMORY_OFFSET
    );

    // pass physical and virtual addresses of kernel for backtrace symbol
    // resolution, if we crash from here on out we want a nice pretty backtrace
    backtrace::init(
        VirtAddr::new(*kernel_image_offset),
        PhysAddr::new(*kernel_addr),
        usize::try_from(*kernel_len).unwrap(),
    );

    // initialize heap, from here on out we have a global allocator and the `alloc`
    // crate works
    memory::heap_init(memory_regions);



    // initialize global descriptor table and interrupts
    gdt::init();
    irq::init();

    // probe system bus, this bootstraps device enumeration and initialization
    SYSTEM_BUS.probe(X86SystemBusProbeData {
        rsdp_phys: PhysAddr::new(rsdp_addr.into_option().unwrap()),
    });
}

static SYSTEM_BUS: X86SystemBus = X86SystemBus;

struct X86SystemBus;

struct X86SystemBusProbeData {
    rsdp_phys: PhysAddr,
}

impl Bus<X86SystemBusProbeData> for X86SystemBus {
    fn probe(&self, probe_data: X86SystemBusProbeData) {
        acpi::ACPIBus.probe(probe_data.rsdp_phys);
        lapic::init();
    }
}
