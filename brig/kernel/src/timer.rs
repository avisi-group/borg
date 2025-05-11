use {
    crate::{arch::x86::irq::assign_irq, println, scheduler},
    alloc::{collections::BTreeMap, sync::Arc, vec::Vec},
    core::sync::atomic::{AtomicU64, Ordering},
    plugins_api::object::tickable::Tickable,
    proc_macro_lib::irq_handler,
    spin::{Lazy, Mutex},
    x86::time::rdtscp,
};

const ENABLE_MEASUREMENTS: bool = true;
const JIFFIES_PER_SECOND: u64 = 1000;

static TICKABLES: Lazy<Mutex<Vec<TickableState>>> = Lazy::new(|| Mutex::new(alloc::vec![]));

static JIFFIES: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    assign_irq(0x20, timer_interrupt).unwrap();
}

pub fn current_milliseconds() -> u64 {
    JIFFIES.load(Ordering::Relaxed)
}

#[irq_handler(with_code = false)]
fn timer_interrupt() {
    let jiffies = JIFFIES.fetch_add(1, Ordering::Relaxed);

    handle_tickables(jiffies);

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

fn handle_tickables(current_jiffies: u64) {
    TICKABLES
        .lock()
        .iter_mut()
        .filter(|s| current_jiffies >= s.last_tick_jiffy + s.jiffy_interval)
        .for_each(|s| {
            s.tickable.tick();
            s.last_tick_jiffy = current_jiffies;
        });
}

pub fn register_tickable(frequency: u64, tickable: Arc<dyn Tickable>) {
    TICKABLES.lock().push(TickableState {
        tickable,
        jiffy_interval: JIFFIES_PER_SECOND / frequency,
        last_tick_jiffy: 0,
    });
}

struct TickableState {
    tickable: Arc<dyn Tickable>,
    jiffy_interval: u64,
    last_tick_jiffy: u64,
}
