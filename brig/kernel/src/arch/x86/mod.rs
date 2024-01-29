use {
    crate::{
        arch::x86::memory::PHYSICAL_MEMORY_OFFSET,
        devices::{self, acpi, lapic, Bus},
        qemu_exit,
    },
    bootloader_api::BootInfo,
    log::trace,
    x86::controlregs::{cr0, cr0_write, cr4, cr4_write, Cr0, Cr4},
    x86_64::{
        registers::model_specific::{Efer, EferFlags},
        PhysAddr, VirtAddr,
    },
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

    // update control-regs
    update_cregs();

    // initialize heap, from here on out we have a global allocator and the `alloc`
    // crate works
    memory::heap_init(memory_regions);

    // initialize global descriptor table and interrupts
    gdt::init();
    irq::init();

    // initialize device manager ready to register detected devices
    devices::manager::init();

    // probe system bus, this bootstraps device enumeration and initialization
    SYSTEM_BUS.probe(X86SystemBusProbeData {
        rsdp_phys: PhysAddr::new(rsdp_addr.into_option().unwrap()),
    });
}

fn update_cregs() {
    // enable wp
    let mut cr0 = unsafe { cr0() };
    cr0 |= Cr0::CR0_WRITE_PROTECT;

    trace!("cr0={cr0:?}");
    unsafe {
        cr0_write(cr0);
    }

    // enable fsgsbase, pse, pge
    let mut cr4 = unsafe { cr4() };

    cr4 |= Cr4::CR4_ENABLE_FSGSBASE | Cr4::CR4_ENABLE_PSE | Cr4::CR4_ENABLE_GLOBAL_PAGES;
    trace!("cr4={cr4:?}");

    unsafe {
        cr4_write(cr4);
    }

    // enable sce
    let mut efer = Efer::read();
    efer |= EferFlags::SYSTEM_CALL_EXTENSIONS;
    trace!("efer={efer:?}");

    unsafe {
        Efer::write(efer);
    }
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

#[repr(C)]
pub struct MachineContext {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
}

impl MachineContext {
    pub fn empty() -> Self {
        Self {
            rax: 0,
            rcx: 0,
            rdx: 0,
            rbx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rflags: 0,
            rip: 0,
        }
    }
}
