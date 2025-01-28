use {
    ::x86::msr::{rdmsr, wrmsr, IA32_TSC_AUX},
    alloc::boxed::Box,
    bootloader_api::BootInfo,
    common::HashMap,
    core::{
        any::{Any, TypeId},
        sync::atomic::{AtomicU64, Ordering},
    },
    log::trace,
    spin::Once,
};

pub mod x86;

pub const PAGE_SIZE: usize = 4096;

/// Platform initialization, triggers device enumeration and
/// initialization
pub fn platform_init(boot_info: &BootInfo) {
    trace!("initializing platform");
    CoreStorage::init_self();
    x86::init(boot_info);
}

#[derive(Default)]
pub struct CoreStorage {
    state: HashMap<TypeId, Box<dyn Any>>,
}

static mut NEXT_CORE_ID: AtomicU64 = AtomicU64::new(0);
static mut CORES: [Once<CoreStorage>; 4] = [Once::INIT; 4];

fn get_local_pid() -> u64 {
    unsafe { rdmsr(IA32_TSC_AUX) }
}

impl CoreStorage {
    pub fn init_self() {
        unsafe { wrmsr(IA32_TSC_AUX, NEXT_CORE_ID.fetch_add(1, Ordering::SeqCst)) };

        trace!("initializing core {}", get_local_pid());
        unsafe { CORES[get_local_pid() as usize].call_once(Self::default) };
    }

    pub fn this_mut() -> &'static mut Self {
        unsafe { CORES[get_local_pid() as usize].get_mut().unwrap() }
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
            .and_then(|any| any.downcast_mut::<O>())
    }
}
