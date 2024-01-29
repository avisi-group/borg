use {
    crate::{
        arch::{self, x86::MachineContext},
        devices::lapic::LAPIC,
    },
    alloc::{alloc::alloc_zeroed, collections::LinkedList, sync::Arc},
    core::{alloc::Layout, mem::size_of},
    log::trace,
    spin::{Mutex, Once},
    x86::current::segmentation::{rdgsbase, wrgsbase},
};

/// Task stack size in bytes
const STACK_SIZE: usize = 0x2000;

static SCHEDULER: Once<Mutex<Scheduler>> = Once::INIT;

struct Task {
    tcb: TaskControlBlock,
    stack: *const u8,
}

#[repr(C, packed)]
struct TaskControlBlock {
    tag: u64,
    context: *mut MachineContext,
}

impl Task {
    pub fn new(entry_point: fn()) -> Self {
        let stack = unsafe { alloc_zeroed(Layout::from_size_align(STACK_SIZE, 0x1000).unwrap()) };
        let stack_end = unsafe { stack.add(STACK_SIZE) };

        let context =
            unsafe { &mut *(stack_end.sub(size_of::<MachineContext>()) as *mut MachineContext) };

        context.rflags = 0x202; // RSVD | IF
        context.rip = u64::try_from(task_wrapper as usize).unwrap();
        context.rdi = u64::try_from(entry_point as usize).unwrap();
        context.rsp = u64::try_from(stack_end as usize).unwrap();
        context.cs = 0x8; // TODO: Less magic
        context.ss = 0x10;

        Self {
            tcb: TaskControlBlock {
                tag: 0,
                context: context as *mut MachineContext,
            },
            stack,
        }
    }

    /// Get the Task of the current context
    ///
    /// Todo: this sure ain't static
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

extern "C" fn task_wrapper(cb: extern "C" fn()) {
    trace!("task wrapper");
    cb();
    trace!("task complete");

    //Task::get_current().stop();
}

fn idle_task_ep() {
    trace!("idle task entry point");

    loop {
        trace!("idling");
        for _ in 0..100000000 {
            unsafe {
                core::arch::asm!("");
            }
        }
    }
}

struct Scheduler {
    tasks: LinkedList<Arc<Task>>,
    run_queue: LinkedList<Arc<Task>>,
    null_ctx: MachineContext,
    idle_task: Task,
}

// safe because scheduler is kept behind mutex
unsafe impl Send for Scheduler {}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: LinkedList::new(),
            run_queue: LinkedList::new(),
            null_ctx: MachineContext::empty(),
            idle_task: Task::new(idle_task_ep),
        }
    }

    pub fn activate(t: &Task) {
        // TODO: abstract to arch-specific
        unsafe {
            wrgsbase(&t.tcb as *const TaskControlBlock as u64);
            //trace!("{}", &*t.tcb.context);
        }
    }
}

pub fn init() {
    trace!("sched: init");

    SCHEDULER.call_once(|| Mutex::new(Scheduler::new()));

    let base = &SCHEDULER.get().unwrap().lock().null_ctx as *const MachineContext as u64;
    unsafe {
        wrgsbase(base);
    }
}

pub fn spawn(cb: fn()) {
    let task = Arc::new(Task::new(cb));

    SCHEDULER
        .get()
        .unwrap()
        .lock()
        .tasks
        .push_back(task.clone());

    SCHEDULER.get().unwrap().lock().run_queue.push_back(task);
}

pub fn start() {
    trace!("task scheduler started");

    LAPIC.get().unwrap().lock().start_periodic(100);

    arch::x86::irq::local_enable();
}

pub fn schedule() {
    let mut scheduler = SCHEDULER.get().unwrap().lock();

    if scheduler.run_queue.is_empty() {
        Scheduler::activate(&scheduler.idle_task);
    } else {
        let next = if scheduler.run_queue.len() == 1 {
            scheduler.run_queue.front().unwrap().as_ref()
        } else {
            let front = scheduler.run_queue.pop_front().unwrap();

            scheduler.run_queue.push_back(front);

            scheduler.run_queue.back().unwrap().as_ref()
        };

        Scheduler::activate(next);
    }
}
