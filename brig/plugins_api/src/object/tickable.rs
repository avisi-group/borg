use crate::object::Object;

pub trait Tickable: Object {
    fn tick(&self);
}
