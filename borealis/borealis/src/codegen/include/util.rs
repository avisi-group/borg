pub trait Tracer {
    fn begin(&self, instruction: u32, pc: u64);
    fn end(&self);
    fn read_register(&self, offset: usize, value: &dyn core::fmt::Debug);
    fn write_register(&self, offset: usize, value: &dyn core::fmt::Debug);
    fn read_memory(&self, address: usize, value: &dyn core::fmt::Debug);
    fn write_memory(&self, address: usize, value: &dyn core::fmt::Debug);
}

pub trait RatioExt {
    fn powi(&self, i: i32) -> Self;
    fn sqrt(&self) -> Self;
    fn abs(&self) -> Self;
}

impl RatioExt for num_rational::Ratio<i128> {
    fn powi(&self, i: i32) -> Self {
        self.pow(i)
    }

    fn sqrt(&self) -> Self {
        todo!();
    }

    fn abs(&self) -> Self {
        let n = *self.numer();
        let d = *self.denom();
        Self::new(n.abs(), d)
    }
}

#[derive(Debug)]
pub enum ExecuteResult {
    Ok,
    EndOfBlock,
    UndefinedInstruction,
}
