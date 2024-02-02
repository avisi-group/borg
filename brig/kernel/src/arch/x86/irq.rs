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
    spin::Once,
    x86_64::{
        registers::control::Cr2,
        structures::{
            idt::{InterruptDescriptorTable, InterruptStackFrame},
            paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
        },
        VirtAddr,
    },
};

static IDT: Once<InterruptDescriptorTable> = Once::INIT;

macro_rules! irq_handler {
    ($f:ident) => {
        #[naked]
        unsafe extern "C" fn $f() -> ! {
            core::arch::asm!(
                concat!(
                    "
    push $0
    push %rax
	push %rcx
	push %rdx
	push %rbx
	push %rbp
	push %rsi
	push %rdi
	push %r8
	push %r9
	push %r10
	push %r11
	push %r12
	push %r13
	push %r14
	push %r15
    mov %rsp, %gs:0
    mov %rsp, %rdi
    call ",
                    stringify!($f),
                    "_handler
    mov %gs:0, %rsp
    pop %r15
	pop %r14
	pop %r13
	pop %r12
	pop %r11
	pop %r10
	pop %r9
	pop %r8
	pop %rdi
	pop %rsi
	pop %rbp
	pop %rbx
	pop %rdx
	pop %rcx
	pop %rax
    add $8, %rsp
    iretq"
                ),
                options(att_syntax, noreturn)
            );
        }
    };
}

macro_rules! irq_handler_with_code {
    ($f:ident) => {
        #[naked]
        unsafe extern "C" fn $f() -> ! {
            core::arch::asm!(
                concat!(
                    "
    push %rax
	push %rcx
	push %rdx
	push %rbx
	push %rbp
	push %rsi
	push %rdi
	push %r8
	push %r9
	push %r10
	push %r11
	push %r12
	push %r13
	push %r14
	push %r15
    mov %rsp, %gs:0
    mov %rsp, %rdi
    call ",
                    stringify!($f),
                    "_handler
    mov %gs:0, %rsp
    pop %r15
	pop %r14
	pop %r13
	pop %r12
	pop %r11
	pop %r10
	pop %r9
	pop %r8
	pop %rdi
	pop %rsi
	pop %rbp
	pop %rbx
	pop %rdx
	pop %rcx
	pop %rax
    add $8, %rsp
    iretq"
                ),
                options(att_syntax, noreturn)
            );
        }
    };
}

irq_handler!(timer_irq);

#[no_mangle]
extern "C" fn timer_irq_handler() {
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

irq_handler_with_code!(page_fault_exception);

#[no_mangle]
extern "C" fn page_fault_exception_handler(machine_context: *mut MachineContext) {
    let faulting_address = Cr2::read();

    let error_code = unsafe { (*machine_context).error_code };

    if faulting_address <= LOW_HALF_CANONICAL_END {
        log::trace!("GUEST PAGE FAULT code {error_code:?} @ {faulting_address:?}");

        let exec_ctx = crate::guest::GuestExecutionContext::current();
        let addrspace = unsafe { &*exec_ctx.current_address_space };

        if let Some(rgn) = addrspace.find_region(faulting_address.as_u64() as usize) {
            match rgn.kind() {
                AddressSpaceRegionKind::RAM => {
                    let faulting_page = faulting_address.align_down(0x1000u64);
                    let backing_page = VirtAddr::from_ptr(unsafe {
                        alloc::alloc::alloc_zeroed(Layout::from_size_align_unchecked(
                            0x1000, 0x1000,
                        ))
                    })
                    .to_phys();

                    log::trace!("mapping {:x} to {:x}", faulting_page, backing_page);

                    unsafe {
                        VirtualMemoryArea::current().map_page(
                            Page::<Size4KiB>::from_start_address_unchecked(faulting_page),
                            PhysFrame::from_start_address_unchecked(backing_page),
                            PageTableFlags::WRITABLE,
                        );
                    }
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
        unsafe {
            idt.page_fault
                .set_handler_addr(VirtAddr::from_ptr(page_fault_exception as *const u8))
        };
        idt.general_protection_fault
            .set_handler_fn(general_protection_handler);
        unsafe { idt[32].set_handler_addr(VirtAddr::from_ptr(timer_irq as *const u8)) };
        idt.double_fault.set_handler_fn(double_fault_handler);

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
