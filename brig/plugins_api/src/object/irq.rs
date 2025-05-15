use crate::object::Object;

#[derive(Debug)]
pub struct IrqLine;

pub trait IrqController: Object {
    fn request_irq(&self, index: usize) -> IrqLine;
}

impl IrqLine {
    pub fn raise(&self) {
        todo!()
    }
}
