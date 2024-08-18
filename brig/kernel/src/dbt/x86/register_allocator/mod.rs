use crate::dbt::x86::encoder::Instruction;

//pub mod reverse_scan;
pub mod solid_state;

pub trait RegisterAllocator {
    fn process(&mut self, instruction: &mut Instruction);
}
