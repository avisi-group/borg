use {
    crate::{gdt, println},
    lazy_static::lazy_static,
    x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder},
    x86_64::{
        registers::control::Cr2,
        structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
    },
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
    static ref LAPIC: LocalApic = {
        let physical = unsafe { xapic_base() } + 0x8000000000;

        let mut lapic = LocalApicBuilder::new()
            .timer_vector(0x20)
            .error_vector(0x21)
            .spurious_vector(0x22)
            .set_xapic_base(physical)
            .build()
            .unwrap_or_else(|err| panic!("{}", err));
        unsafe {
            lapic.enable();
        }
        lapic
    };
}

pub fn init() {
    IDT.load();
    // lazy_static::initialize(&LAPIC);
    // x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
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
    println!(
        "PAGE FAULT code {error_code:?} @ {:?}\n{stack_frame:#?}",
        Cr2::read()
    );
}
