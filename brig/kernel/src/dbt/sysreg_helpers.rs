use {common::hashmap::HashMap, spin::Lazy};

pub type SysRegId = (u64, u64, u64, u64, u64);

pub const REG_CNTVCT_EL0: SysRegId = (3, 3, 14, 0, 2);

type ReadHandler = fn(reg: SysRegId) -> u64;
type WriteHandler = fn(reg: SysRegId, value: u64) -> ();

pub static HELPER_MAP: Lazy<HashMap<SysRegId, (ReadHandler, WriteHandler)>> = Lazy::new(|| {
    let mut map = HashMap::<SysRegId, (ReadHandler, WriteHandler)>::default();
    map.insert(REG_CNTVCT_EL0, (generic_timer_read, generic_timer_write));
    map
});

fn generic_timer_read(_reg: SysRegId) -> u64 {
    0x6789
}

fn generic_timer_write(_reg: SysRegId, _value: u64) {
    //
}
