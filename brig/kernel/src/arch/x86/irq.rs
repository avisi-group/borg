use {
    crate::{
        arch::x86::{
            MachineContext,
            aarch64_mmu::guest_translate,
            memory::{
                GUEST_PHYSICAL_START, LOW_HALF_CANONICAL_END, VirtAddrExt, VirtualMemoryArea,
            },
        },
        dbt::models::ModelDevice,
        guest::memory::AddressSpaceRegionKind,
        qemu_exit,
    },
    alloc::alloc::alloc_zeroed,
    bitset_core::BitSet,
    common::intern::InternedString,
    core::alloc::Layout,
    proc_macro_lib::irq_handler,
    spin::Once,
    x86::irq::{
        BREAKPOINT_VECTOR, DIVIDE_ERROR_VECTOR, DOUBLE_FAULT_VECTOR,
        GENERAL_PROTECTION_FAULT_VECTOR, PAGE_FAULT_VECTOR,
    },
    x86_64::{
        VirtAddr,
        registers::control::Cr2,
        structures::{
            idt::{InterruptDescriptorTable, PageFaultErrorCode},
            paging::{Page, PageTableFlags, PhysFrame, Size4KiB, Translate},
        },
    },
};

/// Print supplied message at `error` level, then exit QEMU
///
/// Panicking inside IRQ handler results in infinite loop and clutters log
/// output
macro_rules! exit_with_message {
    ($($arg:tt)*) => {
        (|| {
            log::error!($($arg)*);
            qemu_exit()
        })()
    }
}
pub(crate) use exit_with_message;

static mut IRQ_MANAGER: Once<IrqManager> = Once::INIT;

pub fn init() {
    unsafe {
        IRQ_MANAGER.call_once(|| IrqManager::new());
        let irqm = IRQ_MANAGER.get_mut().unwrap();
        irqm.setup().unwrap();
        irqm.idt.load();
    };
}

pub fn assign_irq(nr: u8, handler: IrqHandlerFn) -> Result<(), IrqError> {
    let irqm = unsafe { IRQ_MANAGER.get_mut() }.unwrap();
    irqm.assign_irq(nr, handler)?;
    irqm.idt.load();
    Ok(())
}

struct IrqManager {
    idt: InterruptDescriptorTable,
    used: UsedInterruptVectors,
}

impl IrqManager {
    fn new() -> Self {
        Self {
            idt: InterruptDescriptorTable::new(),
            used: UsedInterruptVectors::new(),
        }
    }

    fn setup(&mut self) -> Result<(), IrqError> {
        unsafe {
            // page fault
            self.idt
                .page_fault
                .set_handler_addr(VirtAddr::from_ptr(page_fault_exception as *const u8));
            self.used.set(PAGE_FAULT_VECTOR);

            // general protection
            self.idt
                .general_protection_fault
                .set_handler_addr(VirtAddr::from_ptr(gpf_exception as *const u8));
            self.used.set(GENERAL_PROTECTION_FAULT_VECTOR);

            // breakpoint
            self.idt
                .breakpoint
                .set_handler_addr(VirtAddr::from_ptr(breakpoint_exception as *const u8));
            self.used.set(BREAKPOINT_VECTOR);

            // double fault
            self.idt
                .double_fault
                .set_handler_addr(VirtAddr::from_ptr(double_fault_exception as *const u8));
            self.used.set(DOUBLE_FAULT_VECTOR);

            // double fault
            self.idt
                .divide_error
                .set_handler_addr(VirtAddr::from_ptr(div0_exception as *const u8));
            self.used.set(DIVIDE_ERROR_VECTOR);
        };

        for (f, i) in [
            (dbt_handler_undefined_terminator as IrqHandlerFn, 0x50),
            (dbt_handler_default_terminator, 0x51),
            (dbt_handler_const_assert, 0x52),
            (dbt_handler_panic, 0x53),
        ] {
            self.assign_irq(i, f)?;
        }

        Ok(())
    }

    fn assign_irq(&mut self, nr: u8, handler: IrqHandlerFn) -> Result<(), IrqError> {
        if !self.used.get(nr) {
            unsafe { self.idt[nr].set_handler_addr(VirtAddr::from_ptr(handler as *const u8)) };
            self.used.set(nr);
            Ok(())
        } else {
            Err(IrqError::IrqAlreadyReserved(nr))
        }
    }
}

/// IRQ Error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum IrqError {
    /// Attempted to assign IRQ {0} but it is already in use
    IrqAlreadyReserved(u8),
}

pub type IrqHandlerFn = unsafe extern "C" fn();

pub fn _local_enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn local_disable() {
    x86_64::instructions::interrupts::disable();
}

#[irq_handler(with_code = true)]
fn page_fault_exception(machine_context: *mut MachineContext) {
    let faulting_address = Cr2::read().unwrap();

    let error_code =
        PageFaultErrorCode::from_bits(unsafe { (*machine_context).error_code }).unwrap();

    if faulting_address <= LOW_HALF_CANONICAL_END {
        log::debug!("guest fault @ {faulting_address:#x}");
        let exec_ctx = crate::guest::GuestExecutionContext::current();
        let addrspace = unsafe { &*exec_ctx.current_address_space };

        let device = unsafe {
            crate::guest::GUEST
                .get()
                .unwrap()
                .devices
                .get("core0")
                .unwrap()
        }
        .as_any()
        .downcast_ref::<ModelDevice>()
        .unwrap();

        let mmu_enabled = *device.get_register_mut::<u64>("SCTLR_EL1_bits") & 1 == 1;

        // correct the address as it was masked off in emitter.rs:read/write-memory
        let unmasked_address =
            VirtAddr::new((((faulting_address.as_u64() as i64) << 24) >> 24) as u64);

        let guest_physical = if mmu_enabled {
            // translate:
            // * walk guest page tables from top level page table translate faulting address
            // * if it doesnt exist: guest page fault
            // * if it does exist but is invalid (write to a read only mapped page)
            // * or it works, we get a guest physical address, we do the next logic on line
            //   186 and map it as writeable, but if it was a read then map as read only
            // * map that guest physical address into the correct location in host virtual
            //   memory

            guest_translate(device, unmasked_address.as_u64()).unwrap()
        } else {
            unmasked_address.as_u64()
        };

        let guest_backing_frame = VirtualMemoryArea::current()
            .opt
            .translate_addr((GUEST_PHYSICAL_START + guest_physical).align_down(0x1000u64));

        log::debug!("guest backing frame: {guest_backing_frame:x?}");

        // have we already allocated this gues physical address?
        let backing_page = match guest_backing_frame {
            None => {
                // No existing backing page, so lookup what to do.
                if let Some(rgn) = addrspace.find_region(guest_physical) {
                    // Physical address lies within a valid guest region, determine region type...
                    match rgn.kind() {
                        AddressSpaceRegionKind::Ram => {
                            // Physical address lies within a RAM-backed region, so allocate a
                            // backing page.
                            let backing_page = VirtAddr::from_ptr(unsafe {
                                alloc_zeroed(Layout::from_size_align(0x1000, 0x1000).unwrap())
                            })
                            .to_phys();

                            // Map the allocated backing page into the 1-1 guest phyical memory area
                            VirtualMemoryArea::current().map_page(
                                Page::<Size4KiB>::from_start_address(
                                    (GUEST_PHYSICAL_START + guest_physical).align_down(0x1000u64),
                                )
                                .unwrap(),
                                PhysFrame::from_start_address(backing_page).unwrap(),
                                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                            );

                            log::debug!(
                                "allocated backing page {backing_page:x?} -> {:x?}",
                                (GUEST_PHYSICAL_START + guest_physical).align_down(0x1000u64)
                            );

                            backing_page
                        }
                        _ => {
                            // Physical address is not in RAM-backed region; could be a device...
                            exit_with_message!("fault in non-ram memory @ {guest_physical:x?}")
                        }
                    }
                } else {
                    // Physical address not in valid guest region -- real fault.
                    exit_with_message!(
                        "GUEST PAGE FAULT code {error_code:?} @ {guest_physical:x?}: no region -- this is a real fault"
                    )
                }
            }
            Some(phys_addr) => {
                // Backing page already exists at this host physical address
                phys_addr
            }
        };

        log::debug!(
            "guest backing page: {backing_page:x?} mapping to {:x?}",
            faulting_address.align_down(0x1000u64)
        );

        VirtualMemoryArea::current().map_page_propagate_invalidation(
            Page::<Size4KiB>::from_start_address(faulting_address.align_down(0x1000u64)).unwrap(),
            PhysFrame::from_start_address(backing_page).unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
    } else {
        exit_with_message!("HOST PAGE FAULT code {error_code:?} @ {faulting_address:?}");
    }
}

#[irq_handler(with_code = false)]
fn div0_exception() {
    exit_with_message!("EXCEPTION: DIVIDE BY 0");
}

#[irq_handler(with_code = false)]
fn breakpoint_exception() {
    exit_with_message!("EXCEPTION: BREAKPOINT");
}

#[irq_handler(with_code = true)]
fn double_fault_exception() {
    exit_with_message!("EXCEPTION: DOUBLE-FAULT");
}

#[irq_handler(with_code = true)]
fn gpf_exception(machine_context: *mut MachineContext) {
    exit_with_message!(
        "EXCEPTION: GENERAL PROTECTION FAULT\nrip = {:x}",
        unsafe { &*machine_context }.rip
    );

    crate::qemu_exit();
}

#[irq_handler(with_code = true)]
fn dbt_handler_undefined_terminator(_machine_context: *mut MachineContext) {
    exit_with_message!("DBT interrupt: undefined terminator")
}

#[irq_handler(with_code = true)]
fn dbt_handler_default_terminator(_machine_context: *mut MachineContext) {
    exit_with_message!("DBT interrupt: default terminator")
}

#[irq_handler(with_code = true)]
fn dbt_handler_const_assert(_machine_context: *mut MachineContext) {
    exit_with_message!("DBT interrupt: const assert")
}

#[irq_handler(with_code = true)]
fn dbt_handler_panic(machine_context: *mut MachineContext) {
    let meta = unsafe { &*machine_context }.r15;

    let key = (meta >> 32) as u32;
    let function_name = InternedString::from_raw(key - 1);

    let block = (meta >> 16) as u16;
    let statement = meta as u16;

    exit_with_message!(
        "DBT interrupt: statement {statement:x} failed assert in block {block:x} of {function_name:?}"
    )
}

struct UsedInterruptVectors([u64; 4]);

impl UsedInterruptVectors {
    pub fn new() -> Self {
        Self([0; 4])
    }

    pub fn set(&mut self, nr: u8) {
        self.0.bit_set(usize::from(nr));
    }

    pub fn get(&mut self, nr: u8) -> bool {
        self.0.bit_test(usize::from(nr))
    }
}
