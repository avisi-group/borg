use crate::{
    guest::{devices::GuestDevice, memory::IOMemoryHandler},
    tasks::{create_task, Task},
};

pub struct Arm64Generic {
    execution_thread: Task,
}

impl GuestDevice for Arm64Generic {
    fn start(&self) {
        self.execution_thread.start();
    }

    fn stop(&self) {
        self.execution_thread.stop();
    }

    fn as_io_handler(self: alloc::rc::Rc<Self>) -> Option<alloc::rc::Rc<dyn IOMemoryHandler>> {
        None
    }
}

impl Arm64Generic {
    pub fn new() -> Self {
        Self {
            execution_thread: create_task(execution_thread),
        }
    }
}

fn execution_thread() {
    log::trace!("running guest core");
    todo!("fetch-decode-execute goes here");
}
