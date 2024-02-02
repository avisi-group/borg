use {
    ::x86::time::rdtscp,
    alloc::{boxed::Box, collections::BTreeMap},
    bootloader_api::BootInfo,
    core::any::{Any, TypeId},
    log::trace,
    spin::Once,
};

pub mod x86;

pub const PAGE_SIZE: usize = 4096;

/// Platform initialization, triggers device enumeration and
/// initialization
pub fn platform_init(boot_info: &BootInfo) {
    x86::init(boot_info);
}

#[derive(Default)]
pub struct Core {
    state: BTreeMap<TypeId, Box<dyn Any>>,
}

static mut CORES: [Once<Core>; 4] = [Once::INIT; 4];

fn get_local_pid() -> u32 {
    unsafe { rdtscp().1 }
}

impl Core {
    pub fn init_self() {
        trace!("initializing core {}", get_local_pid());
        unsafe {
            CORES
                .get(get_local_pid() as usize)
                .unwrap()
                .call_once(|| Core::default())
        };
    }

    pub fn this_mut() -> &'static mut Self {
        unsafe {
            CORES
                .get_mut(get_local_pid() as usize)
                .unwrap()
                .get_mut()
                .unwrap()
        }
    }

    pub fn set<O: 'static>(&mut self, o: O) {
        let key = TypeId::of::<O>();
        let value = Box::new(o);
        self.state.insert(key, value);
    }

    pub fn get<O: 'static>(&mut self) -> Option<&mut O> {
        let key = TypeId::of::<O>();
        self.state
            .get_mut(&key)
            .map(|any| any.downcast_mut::<O>())
            .flatten()
    }
}
