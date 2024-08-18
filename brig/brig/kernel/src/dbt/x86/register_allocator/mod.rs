use crate::dbt::x86::encoder::Instruction;

//pub mod reverse_scan;
pub mod solid_state;

pub trait RegisterAllocator {
    fn allocate(instructions: &mut [Instruction], num_virtual_registers: usize);
}
