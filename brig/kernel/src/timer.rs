use {
    crate::{arch::x86::irq::assign_irq, scheduler},
    core::sync::atomic::{AtomicU64, Ordering},
    proc_macro_lib::irq_handler,
};

static JIFFIES: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    assign_irq(0x20, timer_interrupt).unwrap();
}

pub fn current_milliseconds() -> u64 {
    JIFFIES.load(Ordering::Relaxed)
}

#[irq_handler(with_code = false)]
fn timer_interrupt() {
    JIFFIES.fetch_add(1, Ordering::Relaxed);

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
