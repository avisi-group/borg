use {
    crate::{
        arch::{self, x86::MachineContext},
        devices::lapic::LAPIC,
    },
    alloc::collections::LinkedList,
    log::trace,
    spin::Once,
    x86::current::segmentation::{rdgsbase, wrgsbase},
};

struct Task {
    context: MachineContext,
}

impl Task {
    pub fn new(entry_point: fn()) -> Self {
        let mut context = MachineContext::empty();
        context.rflags = 2;
        context.rip = task_wrapper as u64;
        context.rdi = entry_point as u64;
        context.rsp = 0; // TODO: Allocate Stack

        Self { context }
    }

    pub fn get_current() -> &'static Task {
        let gs = unsafe { rdgsbase() };
        unsafe { &*(gs as *const Task) }
    }

    pub fn start(&self) {
        todo!()
    }

    pub fn stop(&self) {
        todo!()
    }
}

fn task_wrapper(cb: fn()) {
    cb();

    Task::get_current().stop();
    loop {}
}

fn idle_task() {
    loop {
        // TODO: asm pause
    }
}

struct Scheduler {
    tasks: LinkedList<Task>,
}

static mut SCHEDULER: Once<Scheduler> = Once::INIT;

pub fn init() {
    trace!("sched: init");

    unsafe {
        SCHEDULER.call_once(|| Scheduler {
            tasks: LinkedList::new(),
        });

        // TODO: abstract to arch-specific
        wrgsbase(idle_task as u64);
    }
}

pub fn spawn(cb: fn()) {
    unsafe {
        SCHEDULER.get_mut().unwrap().tasks.push_back(Task::new(cb));
    }
}

pub fn run() -> ! {
    trace!("sched: running...");

    unsafe {
        LAPIC.get_mut().unwrap().lock().start_periodic(100);
    }

    //arch::x86::irq::local_enable();

    loop {}
}
