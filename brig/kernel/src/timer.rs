use {
    crate::{arch::x86::irq, devices, scheduler},
    core::sync::atomic::{AtomicU64, Ordering},
    proc_macro_lib::irq_handler,
    x86_64::instructions::interrupts::int3,
};

pub fn init() {
    devices::pic::init(0x20, 0x28);
    devices::pic::enable_irq(0x0);
    devices::pit::init();
    log::debug!("here");

    irq::assign_irq(0x20, timer_interrupt).unwrap();
    // use PIT to calibrate LAPIC

    // use LAPIC
    log::debug!("done here");
}

fn timer_end_interrupt() {}

#[irq_handler(with_code = false)]
fn timer_interrupt() {
    log::debug!("timer interrupt");

    JIFFIES.fetch_add(1, Ordering::Relaxed);

    // scheduler::schedule();

    // timer_end_interrupt();
}

static JIFFIES: AtomicU64 = AtomicU64::new(1);
static JIFFIES_DIVISOR: AtomicU64 = AtomicU64::new(1);

pub fn current_milliseconds() -> u64 {
    JIFFIES.load(Ordering::Relaxed) / JIFFIES_DIVISOR.load(Ordering::Relaxed)
}
