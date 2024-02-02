use {
    crate::{
        arch::x86::{
            memory::{VirtAddrExt, VirtualMemoryArea, LOW_HALF_CANONICAL_END},
            MachineContext,
        },
        guest::memory::AddressSpaceRegionKind,
        scheduler,
    },
    core::alloc::Layout,
    proc_macro_lib::irq_handler,
    spin::Once,
    x86_64::{
        registers::control::Cr2,
        structures::{
            idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
            paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
        },
        VirtAddr,
    },
};

static IDT: Once<InterruptDescriptorTable> = Once::INIT;

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
    let faulting_address = Cr2::read();

    let error_code =
        PageFaultErrorCode::from_bits(unsafe { (*machine_context).error_code }).unwrap();

    if faulting_address <= LOW_HALF_CANONICAL_END {
        //  log::trace!("GUEST PAGE FAULT code {error_code:?} @ {faulting_address:?}");

        let exec_ctx = crate::guest::GuestExecutionContext::current();
        let addrspace = unsafe { &*exec_ctx.current_address_space };

        if let Some(rgn) = addrspace.find_region(faulting_address.as_u64() as usize) {
            //log::trace!("located region {}", rgn);

            match rgn.kind() {
                AddressSpaceRegionKind::RAM => {
                    let faulting_page = faulting_address.align_down(0x1000u64);
                    let backing_page = VirtAddr::from_ptr(unsafe {
                        alloc::alloc::alloc_zeroed(Layout::from_size_align(0x1000, 0x1000).unwrap())
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
            panic!("no region -- this is a real fault");
        }
    } else {
        panic!("HOST PAGE FAULT code {error_code:?} @ {faulting_address:?}");
    }
}

pub fn init() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        unsafe {
            idt.page_fault
                .set_handler_addr(VirtAddr::from_ptr(page_fault_exception as *const u8));
            idt[32].set_handler_addr(VirtAddr::from_ptr(timer_interrupt as *const u8));
        };

        idt
    });

    IDT.get().unwrap().load();
}

pub fn _local_enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn local_disable() {
    x86_64::instructions::interrupts::disable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::error!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT code {error_code}\n{:#?}",
        stack_frame
    );
}

extern "x86-interrupt" fn general_protection_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("general protection fault code {error_code}\n{stack_frame:#?}");
}
