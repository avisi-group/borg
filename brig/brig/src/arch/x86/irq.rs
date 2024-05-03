use {
    crate::{
        arch::x86::{
            memory::{VirtAddrExt, VirtualMemoryArea, LOW_HALF_CANONICAL_END},
            MachineContext,
        },
        guest::memory::AddressSpaceRegionKind,
        qemu_exit, scheduler,
    },
    alloc::{alloc::alloc_zeroed, collections::BTreeSet},
    core::alloc::Layout,
    proc_macro_lib::irq_handler,
    spin::Once,
    x86::irq::{BREAKPOINT_VECTOR, GENERAL_PROTECTION_FAULT_VECTOR, PAGE_FAULT_VECTOR},
    x86_64::{
        registers::control::Cr2,
        structures::{
            idt::{InterruptDescriptorTable, PageFaultErrorCode},
            paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
        },
        VirtAddr,
    },
};

static mut IRQ_MANAGER: Once<IrqManager> = Once::INIT;

struct IrqManager {
    idt: InterruptDescriptorTable,
    avail: BTreeSet<u8>,
}

#[irq_handler(with_code = false)]
fn timer_interrupt() {
    scheduler::schedule();

    unsafe {
        crate::devices::lapic::LAPIC
            .get()
            .unwrap()
            .lock()
            .inner
            .end_of_interrupt()
    };
}

#[irq_handler(with_code = true)]
fn page_fault_exception(machine_context: *mut MachineContext) {
    let faulting_address = Cr2::read().unwrap();

    let error_code =
        PageFaultErrorCode::from_bits(unsafe { (*machine_context).error_code }).unwrap();

    if faulting_address <= LOW_HALF_CANONICAL_END {
        //  log::trace!("GUEST PAGE FAULT code {error_code:?} @ {faulting_address:?}");

        let exec_ctx = crate::guest::GuestExecutionContext::current();
        let addrspace = unsafe { &*exec_ctx.current_address_space };

        if let Some(rgn) = addrspace.find_region(faulting_address.as_u64() as usize) {
            //log::trace!("located region {}", rgn);

            match rgn.kind() {
                AddressSpaceRegionKind::Ram => {
                    let faulting_page = faulting_address.align_down(0x1000u64);
                    let backing_page = VirtAddr::from_ptr(unsafe {
                        alloc_zeroed(Layout::from_size_align(0x1000, 0x1000).unwrap())
                    })
                    .to_phys();

                    // log::trace!("mapping va={:x} to pa={:x}", faulting_page, backing_page);

                    VirtualMemoryArea::current().map_page(
                        Page::<Size4KiB>::from_start_address(faulting_page).unwrap(),
                        PhysFrame::from_start_address(backing_page).unwrap(),
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    );
                }
                _ => {
                    panic!("cannot alloc non-ram");
                }
            }
        } else {
            panic!("GUEST PAGE FAULT code {error_code:?} @ {faulting_address:?}: no region -- this is a real fault");
        }
    } else {
        panic!("HOST PAGE FAULT code {error_code:?} @ {faulting_address:?}");
    }
}

pub fn init() {
    unsafe {
        IRQ_MANAGER.call_once(|| IrqManager {
            idt: InterruptDescriptorTable::new(),
            avail: BTreeSet::new(),
        });

        IRQ_MANAGER.get_mut().unwrap().init_default();
    }

    //IRQ_MANAGER.init_default();

    //    IRQ_MANAGER.get().unwrap().load();
}

enum IrqError {
    IrqAlreadyReserved,
    NoAvailableIrqs,
}

pub type IrqHandlerFn = unsafe extern "C" fn();

impl IrqManager {
    pub fn init_default(&'static mut self) {
        self.assign_irq(BREAKPOINT_VECTOR, breakpoint_exception);
        self.assign_irq(PAGE_FAULT_VECTOR, page_fault_exception);
        self.assign_irq(GENERAL_PROTECTION_FAULT_VECTOR, gpf_exception);

        // TODO: Pop this out
        self.reserve_irq(0x20, timer_interrupt);

        for i in 32..=255 {
            self.avail.insert(i);
        }

        self.idt.load();
    }

    pub fn assign_irq(&mut self, nr: u8, handler: IrqHandlerFn) {
        unsafe {
            match nr {
                PAGE_FAULT_VECTOR => {
                    self.idt
                        .page_fault
                        .set_handler_addr(VirtAddr::from_ptr(handler as *const u8));
                }
                GENERAL_PROTECTION_FAULT_VECTOR => {
                    self.idt
                        .general_protection_fault
                        .set_handler_addr(VirtAddr::from_ptr(handler as *const u8));
                }
                nr => {
                    self.idt[nr].set_handler_addr(VirtAddr::from_ptr(handler as *const u8));
                }
            }
        }
    }

    pub fn reserve_irq(&mut self, nr: u8, handler: IrqHandlerFn) -> Result<(), IrqError> {
        if self.avail.contains(&nr) {
            return Err(IrqError::IrqAlreadyReserved);
        }

        self.assign_irq(nr, handler);

        Ok(())
    }

    pub fn allocate_irq(&mut self, handler: IrqHandlerFn) -> Result<(), IrqError> {
        let nr = self.avail.pop_first().ok_or(IrqError::NoAvailableIrqs)?;

        self.assign_irq(nr, handler);

        Ok(())
    }
}

pub fn _local_enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn local_disable() {
    x86_64::instructions::interrupts::disable();
}

#[irq_handler(with_code = false)]
fn breakpoint_exception() {
    log::error!("EXCEPTION: BREAKPOINT");
}

#[irq_handler(with_code = true)]
fn double_fault_exception() {
    log::error!("EXCEPTION: DOUBLE-FAULT");
}

#[irq_handler(with_code = true)]
fn gpf_exception(machine_context: *mut MachineContext) {
    log::error!(
        "EXCEPTION: GENERAL PROTECTION FAULT\nrip = {:x}",
        unsafe { &*machine_context }.rip
    );

    crate::qemu_exit();
}
