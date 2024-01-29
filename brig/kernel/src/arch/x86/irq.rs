use {
    spin::Once,
    x86_64::{
        registers::control::Cr2,
        structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
        VirtAddr,
    },
};

static IDT: Once<InterruptDescriptorTable> = Once::INIT;

#[naked]
unsafe fn foo() -> ! {
    core::arch::asm!("1: jmp 1b", options(att_syntax, noreturn));
}

pub fn init() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_handler);
        unsafe { idt[32].set_handler_addr(VirtAddr::from_ptr(foo as *const u8)) };
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    });
}

pub fn local_enable() {
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

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!(
        "PAGE FAULT code {error_code:?} @ {:?}\n{stack_frame:#?}",
        Cr2::read()
    );
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    log::warn!("timer interrupt");
    loop {}

    unsafe {
        crate::devices::lapic::LAPIC
            .get()
            .unwrap()
            .lock()
            .inner
            .end_of_interrupt()
    };
}

extern "x86-interrupt" fn general_protection_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("general protection fault code {error_code}\n{stack_frame:#?}");
}
