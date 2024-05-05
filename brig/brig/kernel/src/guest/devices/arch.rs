use {
    alloc::{boxed::Box, collections::BTreeMap},
    plugins_api::ArchitectureExecutorFactory,
    spin::Mutex,
};

static ARCH_EXEC_FACTORIES: Mutex<BTreeMap<&'static str, Box<dyn ArchitectureExecutorFactory>>> =
    Mutex::new(BTreeMap::new());

pub fn register_architecture_executor(
    name: &'static str,
    arch_exec: Box<dyn ArchitectureExecutorFactory>,
) {
    ARCH_EXEC_FACTORIES.lock().insert(name, arch_exec);
}

/*pub trait Architecture {
    type State;
    fn initial_state() -> Self::State;
}

pub trait Core {
    type State;
    fn initial_state() -> <Self as Core>::State;
}

pub trait ExecutionEngine {
    fn step(&mut self, state: &mut <ExecutionEngine as Core>::State);
}

struct Aarch64Archicture {
    cores: [Aarch64Core; 4],
}

struct Aarch64Core {}
*/
