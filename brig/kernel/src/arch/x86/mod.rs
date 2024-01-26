pub mod backtrace;
mod gdt;
mod irq;
pub mod memory;

use {
    crate::devices::{acpi, lapic, Bus},
    x86_64::PhysAddr,
};

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

static SYSTEM_BUS: X86SystemBus = X86SystemBus;

pub fn init_system() {
    gdt::init();
    irq::init();
}

pub fn init_platform(rsdp_phys: PhysAddr) {
    SYSTEM_BUS.probe(X86SystemBusProbeData { rsdp_phys });
}
