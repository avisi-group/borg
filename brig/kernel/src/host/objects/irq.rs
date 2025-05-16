use crate::host::objects::Object;

pub trait IrqController: Object {
    fn raise(&self, line: usize);
    fn rescind(&self, line: usize);
    fn acknowledge(&self, line: usize);
}
