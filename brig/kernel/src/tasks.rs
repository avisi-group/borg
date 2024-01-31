use {
    crate::{
        arch::{self, x86::MachineContext, PAGE_SIZE},
        dbg,
        devices::lapic::LAPIC,
        scheduler::{self, Scheduler},
    },
    alloc::{
        alloc::{alloc_zeroed, dealloc},
        collections::LinkedList,
        rc::{Rc, Weak},
        sync::Arc,
    },
    core::{
        alloc::Layout,
        borrow::BorrowMut,
        mem::size_of,
        panic,
        ptr::{null, null_mut},
    },
    log::trace,
    spin::{Mutex, Once},
    x86::{
        current::segmentation::{rdgsbase, wrgsbase},
        segmentation::gs,
    },
};

/// Task stack size in bytes
const STACK_SIZE: usize = PAGE_SIZE * 2;
const STACK_ALLOC_LAYOUT: Layout =
    unsafe { Layout::from_size_align_unchecked(STACK_SIZE, PAGE_SIZE) };

static TASK_MANAGER: Once<Mutex<TaskManager>> = Once::INIT;

struct TaskManager {
    tasks: LinkedList<Task>,
    schedulers: LinkedList<&'static mut Scheduler>,
}

#[derive(Clone)]
pub struct Task {
    inner: Rc<InnerTask>,
}

impl PartialEq for &Task {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

pub struct InnerTask {
    tcb: TaskControlBlock,
    stack: *mut u8,
}

impl Drop for InnerTask {
    fn drop(&mut self) {
        unsafe { dealloc(self.stack, STACK_ALLOC_LAYOUT) }
    }
}

#[repr(C)]
pub struct TaskControlBlock {
    pub context: *mut MachineContext,
    pub parent: Weak<InnerTask>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: LinkedList::new(),
            schedulers: LinkedList::new(),
        }
    }

    pub fn register_scheduler(&mut self, scheduler: &'static mut Scheduler) {
        // TODO: Cannot be called remotely
        self.schedulers.push_back(scheduler);
    }

    pub fn create_task(&mut self, entry_point: fn()) -> Task {
        let task = Task::new(entry_point);
        self.tasks.push_back(task.clone());
        task
    }

    pub fn start_task(&mut self, task: &Task) {
        self.schedulers
            .back_mut()
            .unwrap()
            .add_to_runqueue(task.clone());
    }

    pub fn stop_task(&mut self, task: &Task) {
        self.schedulers
            .back_mut()
            .unwrap()
            .remove_from_runqueue(task);
        self.tasks = self
            .tasks
            .clone()
            .into_iter()
            .filter(|t| t != task)
            .collect();
    }

    pub fn suspend_task(&mut self, task: &Task) {
        todo!();
    }

    pub fn resume_task(&mut self, task: &Task) {
        todo!();
    }
}

impl Task {
    fn new(entry_point: fn()) -> Self {
        let stack = unsafe { alloc_zeroed(STACK_ALLOC_LAYOUT) };
        let stack_end = unsafe { stack.add(STACK_ALLOC_LAYOUT.size()) };

        let context =
            unsafe { &mut *(stack_end.sub(size_of::<MachineContext>()) as *mut MachineContext) };

        context.rflags = 0x202; // RSVD | IF
        context.rip = u64::try_from(task_wrapper as usize).unwrap();
        context.rdi = u64::try_from(entry_point as usize).unwrap();
        context.rsp = u64::try_from(stack_end as usize).unwrap();
        context.cs = 0x8; // TODO: Less magic
        context.ss = 0x10;

        let inner = Rc::new_cyclic(|weak| InnerTask {
            tcb: TaskControlBlock {
                context: context as *mut MachineContext,
                parent: weak.clone(),
            },
            stack,
        });

        Self { inner }
    }

    pub fn get_tcb(&self) -> &TaskControlBlock {
        &self.inner.tcb
    }

    pub fn start(&self) {
        TASK_MANAGER.get().unwrap().lock().start_task(self);
    }

    pub fn stop(&self) {
        TASK_MANAGER.get().unwrap().lock().stop_task(self);
    }

    pub fn current() -> Task {
        let tcb = unsafe { &*(rdgsbase() as *const TaskControlBlock) };
        Task {
            inner: tcb.parent.upgrade().unwrap(),
        }
    }
}

extern "C" fn task_wrapper(cb: extern "C" fn()) {
    cb();

    Task::current().stop();

    loop {}
}

// safe because TaskManager is kept behind mutex??
unsafe impl Send for TaskManager {}

pub fn init() {
    TASK_MANAGER.call_once(|| Mutex::new(TaskManager::new()));
}

pub fn create_task(ep: fn()) -> Task {
    TASK_MANAGER.get().unwrap().lock().create_task(ep)
}

fn idle_task_ep() {
    loop {
        unsafe {
            core::arch::asm!("pause");
        }
    }
}

pub fn create_idle_task() -> Task {
    Task::new(idle_task_ep)
}

pub fn register_scheduler() {
    // create new scheduler on current core
    let scheduler = Scheduler::new();
    crate::arch::Core::this_mut().set(scheduler);

    // register it with task manager
    TASK_MANAGER
        .get()
        .unwrap()
        .lock()
        .register_scheduler(crate::arch::Core::this_mut().get().unwrap());
}
