//! Based on SSRA by Matt Keeter https://www.mattkeeter.com/blog/2022-10-04-ssra/

use {
    crate::dbt::x86::{
        encoder::{Instruction, OperandDirection, PhysicalRegister, Register, UseDef},
        register_allocator::RegisterAllocator,
    },
    alloc::vec::Vec,
    bitset_core::BitSet,
};

pub struct SolidStateRegisterAllocator {
    register_allocations: Allocations,
    // 16 physical registers
    physical_used: u16,
}

impl SolidStateRegisterAllocator {
    /// Builds a new `RegisterAllocator`.
    ///
    /// Upon construction, SSA register 0 is bound to local register 0; you
    /// would be well advised to use it as the output of your function.
    pub fn new(num_virtual_registers: usize) -> Self {
        let mut physical_used = 0;

        // prevent use of the reserved registers
        physical_used
            .bit_set(PhysicalRegister::RSP.index())
            .bit_set(PhysicalRegister::RBP.index())
            .bit_set(PhysicalRegister::R15.index());

        Self {
            register_allocations: Allocations::new(num_virtual_registers),
            physical_used,
        }
    }

    fn lookup_or_allocate(
        &mut self,
        virtual_register: VirtualRegisterIndex,
    ) -> PhysicalRegisterIndex {
        match self.register_allocations.lookup(virtual_register) {
            Some(phys) => phys,
            None => {
                let phys_index = {
                    let first_empty = self.physical_used.trailing_ones();

                    if first_empty > 16 {
                        panic!("ran out of registers :(");
                    }

                    usize::try_from(first_empty).unwrap()
                };

                self.physical_used.bit_set(phys_index);

                let phys = PhysicalRegisterIndex(phys_index);

                self.register_allocations.insert(virtual_register, phys);

                phys
            }
        }
    }

    fn lookup_and_deallocate(
        &mut self,
        virtual_register: VirtualRegisterIndex,
    ) -> PhysicalRegisterIndex {
        let phys = self.register_allocations.remove(virtual_register);
        self.physical_used.bit_reset(phys.0);
        phys
    }
}

impl RegisterAllocator for SolidStateRegisterAllocator {
    fn process(&mut self, instruction: &mut Instruction) {
        instruction.get_use_defs().for_each(|usedef| match usedef {
            // treat read-writes the same as reads
            UseDef::UseDef(reg) | UseDef::Use(reg) => {
                match reg {
                    Register::PhysicalRegister(preg) => {
                        // use of preg
                        self.physical_used.bit_set(preg.index());
                    }
                    Register::VirtualRegister(vreg) => {
                        // use of vreg
                        // if this is the first read we see, it's live range starts here

                        // so allocate a register for it if its the first read, or lookup the
                        // existing allocation if not

                        // but we don't need to check if it's the first, just lookup or allocate
                        let phys = self.lookup_or_allocate(VirtualRegisterIndex(*vreg));
                        *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));
                    }
                }
            }
            UseDef::Def(reg) => match reg {
                Register::PhysicalRegister(preg) => {
                    // def of preg
                    self.physical_used.bit_reset(preg.index());
                }
                Register::VirtualRegister(vreg) => {
                    // def of vreg
                    // if the virtual register is written to, that means it's liveness is over,
                    // deallocate
                    let phys = self.lookup_and_deallocate(VirtualRegisterIndex(*vreg));
                    *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));
                }
            },
        });

        /*(|(dir, reg)| match reg {
            Register::PhysicalRegister(phys_reg) => match dir {
                OperandDirection::In => {
                    self.physical_used.bit_set(phys_reg.index());           // use of preg
                }
                OperandDirection::Out | OperandDirection::InOut => {
                    self.physical_used.bit_reset(phys_reg.index());         // def/defuse of preg
                }
            },
            Register::VirtualRegister(index) => match dir {
                OperandDirection::In => {
                    // if this is the first read we see, it's live range starts here

                    // so allocate a register for it if its the first read, or lookup the existing
                    // allocation if not

                    // but we don't need to check if it's the first, just lookup or allocate
                    let phys = self.lookup_or_allocate(VirtualRegisterIndex(*index));
                    *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));
                }
                OperandDirection::Out => {
                    // if the virtual register is written to, that means it's liveness is over,
                    // deallocate
                    let phys = self.lookup_and_deallocate(VirtualRegisterIndex(*index));
                    *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));
                }
                OperandDirection::InOut => {
                    // assuming always read then write

                    // but we're in reverse land so write then read

                    // deallocate
                    let _phys = self.lookup_and_deallocate(VirtualRegisterIndex(*index));
                    // don't bother actually mutating the instruction
                    // *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));

                    // start new
                    let phys = self.lookup_or_allocate(VirtualRegisterIndex(*index));
                    *reg = Register::PhysicalRegister(PhysicalRegister::from_index(phys.0));
                }
            },
        });*/
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VirtualRegisterIndex(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PhysicalRegisterIndex(usize);

struct Allocations(Vec<Option<PhysicalRegisterIndex>>);

impl Allocations {
    fn new(num_virtual_registers: usize) -> Self {
        Self(alloc::vec![None; num_virtual_registers])
    }

    fn lookup(&self, virtual_register: VirtualRegisterIndex) -> Option<PhysicalRegisterIndex> {
        self.0.get(virtual_register.0).copied().flatten()
    }

    fn insert(
        &mut self,
        virtual_register: VirtualRegisterIndex,
        physical_register: PhysicalRegisterIndex,
    ) {
        // should panic if access is out of bounds because we supplied
        // `num_virtual_registers`
        self.0[virtual_register.0] = Some(physical_register);
    }

    fn remove(&mut self, virtual_register: VirtualRegisterIndex) -> PhysicalRegisterIndex {
        self.0
            .get_mut(virtual_register.0).unwrap_or_else(|| panic!("tried to deallocate non-allocated register {virtual_register:?}")) // this can happen if a virtual register is written to but never read
            .take().unwrap_or_else(|| panic!("virtual register slot {virtual_register:?} was previously allocated but not currently"))
        // this should never happen
    }
}
