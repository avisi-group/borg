use {crate::host::objects::Object, embedded_time::duration::Nanoseconds};

pub trait Tickable: Object {
    fn tick(&self, time_since_last_tick: Nanoseconds<u64>);
}
