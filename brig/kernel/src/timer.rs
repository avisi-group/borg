use {
    crate::{arch::x86::irq::assign_irq, println, scheduler},
    core::sync::atomic::{AtomicU64, Ordering},
    proc_macro_lib::irq_handler,
    x86::time::rdtscp,
};

const ENABLE_MEASUREMENTS: bool = true;

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

pub struct Measurement {
    start: u64,
    prev: u64,
}

impl Measurement {
    pub fn start() -> Self {
        let now = if ENABLE_MEASUREMENTS {
            unsafe { rdtscp().0 }
        } else {
            0
        };

        Self {
            start: now,
            prev: now,
        }
    }

    pub fn trigger(&mut self, msg: &str) {
        if ENABLE_MEASUREMENTS {
            let now = unsafe { rdtscp().0 };
            let delta_start = now - self.start;
            let delta_prev = now - self.prev;

            println!(
                "MEASUREMENT: msg={} ds={}ms, dp={}ms",
                msg, delta_start, delta_prev
            );

            self.prev = unsafe { rdtscp().0 };
        }
    }
}
