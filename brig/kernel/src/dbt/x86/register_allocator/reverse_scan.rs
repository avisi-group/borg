use {
    crate::{
        dbg,
        dbt::x86::{
            encoder::{Instruction, OperandDirection, PhysicalRegister, Register},
            register_allocator::RegisterAllocator,
        },
    },
    alloc::vec::Vec,
    bitset_core::BitSet,
};

pub struct ReverseScanRegisterAllocator;

impl RegisterAllocator for ReverseScanRegisterAllocator {
    fn allocate(instructions: &mut [Instruction], num_virtual_registers: usize) {
        let mut state = State::new(instructions, num_virtual_registers);
        state.resolve_live_ranges();
        state.allocate();
        state.commit();
    }
}

struct State<'instrs> {
    register_descriptors: Vec<VirtualRegisterDescriptor>,
    instructions: &'instrs mut [Instruction],
}

impl<'instrs> State<'instrs> {
    fn new(instructions: &'instrs mut [Instruction], num_virtual_registers: usize) -> Self {
        Self {
            register_descriptors: alloc::vec![
                VirtualRegisterDescriptor::default();
                num_virtual_registers
            ],
            instructions,
        }
    }

    fn resolve_live_ranges(&mut self) {
        // resolve first defs
        self.instructions.iter().enumerate().for_each(|(instruction_index, instruction)| {
            let use_defs = instruction.get_use_defs();
            use_defs
                .iter()
                .filter(|(direction, _)| matches!(direction, OperandDirection::Out | OperandDirection::InOut))
                .filter_map(|(dir, register)| match register {
                    Register::PhysicalRegister(_) => None,
                    Register::VirtualRegister(idx) => Some((dir, idx)),
                })
                .for_each(|(_, virtual_register_index)| {
                    if self.register_descriptors[*virtual_register_index].first_def.is_none() {
                        self.register_descriptors[*virtual_register_index].first_def = Some(instruction_index)
                    }
                });
        });

        // resolve last uses
        self.instructions.iter().enumerate().for_each(|(instruction_index, instruction)| {
            let use_defs = instruction.get_use_defs();
            use_defs
                .iter()
                .filter(|(direction, _)| matches!(direction, OperandDirection::In | OperandDirection::InOut))
                .filter_map(|(dir, register)| match register {
                    Register::PhysicalRegister(_) => None,
                    Register::VirtualRegister(idx) => Some((dir, idx)),
                })
                .for_each(|(_, virtual_register_index)| {
                    self.register_descriptors[*virtual_register_index].last_use = Some(instruction_index);
                });
        });
    }

    fn allocate(&mut self) {
        let mut live_phys_registers = 0u16;
        let mut phys_reg_tracking = [0usize; 16];

        live_phys_registers.bit_set(PhysicalRegister::R15.index());
        live_phys_registers.bit_set(PhysicalRegister::RBP.index());
        live_phys_registers.bit_set(PhysicalRegister::RSP.index());

        phys_reg_tracking[PhysicalRegister::R15.index()] = PhysicalRegister::R15.index();
        phys_reg_tracking[PhysicalRegister::RBP.index()] = PhysicalRegister::RBP.index();
        phys_reg_tracking[PhysicalRegister::RSP.index()] = PhysicalRegister::RSP.index();

        self.instructions.iter().enumerate().rev().for_each(|(instruction_index, instruction)| {
            log::debug!("@ {instruction_index} = {:?}, {:b}, {:?}", instruction, live_phys_registers, phys_reg_tracking);

            let use_defs = instruction.get_use_defs();

            let mut skip = false;

            for (dir, register) in use_defs.iter().filter(|(direction, _)| matches!(direction, OperandDirection::Out | OperandDirection::InOut)) {
                match register {
                    Register::VirtualRegister(vreg_idx) => {
                        let vreg = self.register_descriptors[*vreg_idx];
                        log::debug!("vreg idx {vreg_idx} = {vreg:?}");
                        if vreg.first_def == Some(instruction_index) {
                            match vreg.last_use {
                                None => {
                                    log::debug!("def of unused vreg {}", *vreg_idx);
                                    skip = true;
                                    break;
                                }
                                Some(_) => {
                                    if let Some(allocated_reg) = vreg.allocated_register {
                                        live_phys_registers.bit_reset(allocated_reg);
                                    }

                                    log::debug!("ending live-range of vreg {} in preg {:?}", vreg_idx, vreg.allocated_register);
                                }
                            }
                        }
                    }

                    Register::PhysicalRegister(phys_reg) => {
                        let register_index = phys_reg.index();
                        log::debug!("phys reg idx {register_index}");

                        // Definition of PREG
                        if (live_phys_registers.bit_test(register_index)) {
                            live_phys_registers.bit_reset(register_index);

                            if (phys_reg_tracking[register_index] > 16) {
                                let conflicting_vreg_index = phys_reg_tracking[register_index];
                                let conflicting_vreg = &mut self.register_descriptors[conflicting_vreg_index];

                                log::debug!("def of preg {register_index}, but it's tracking vreg {conflicting_vreg_index}!");

                                // Find a new preg for the vreg
                                let new_preg = u16::try_from(conflicting_vreg.interference.trailing_ones()).unwrap();

                                if (new_preg == u16::MAX) {
                                    // check this!! may not be correct
                                    panic!("out of registers in re-assignment: vreg {} interference={:08x}", conflicting_vreg_index, conflicting_vreg.interference);
                                }

                                log::debug!("re-assigning vreg {} to preg {:?} ({:08x})", conflicting_vreg_index, new_preg, conflicting_vreg.interference);

                                conflicting_vreg.allocated_register = Some(usize::try_from(new_preg).unwrap());
                                phys_reg_tracking[conflicting_vreg.allocated_register.unwrap()] = conflicting_vreg_index;
                                live_phys_registers.bit_set(conflicting_vreg.allocated_register.unwrap());
                                self.register_descriptors[conflicting_vreg_index].interference = live_phys_registers;

                                // Update ALL vreg interferences
                                for i in 0..16 {
                                    if live_phys_registers.bit_test(i) {
                                        log::debug!("updating preg={}, vreg={}, prev={:08x}, cur={:08x}", i, phys_reg_tracking[i], self.register_descriptors[phys_reg_tracking[i]].interference, live_phys_registers);

                                        self.register_descriptors[phys_reg_tracking[i]].interference |= live_phys_registers;
                                    }
                                }
                            }

                            log::debug!("ending live-range of preg {register_index}, tracking {}", phys_reg_tracking[register_index]);
                        }
                    }
                }
            }

            if (!skip) {
                dbg!();
                for use_def in use_defs.iter().filter(|(direction, _)| matches!(direction, OperandDirection::Out | OperandDirection::InOut)) {
                    let register_index = match &use_def.1 {
                        Register::PhysicalRegister(pr) => {
                            log::debug!("use def of phys reg {}", pr.index());
                            pr.index()
                        }
                        Register::VirtualRegister(idx) => {
                            log::debug!("use def of virt reg {}", idx);
                            *idx
                        }
                    };

                    match use_def.1 {
                        Register::VirtualRegister(_) => {
                            // Use of VREG

                            if (self.register_descriptors[register_index].last_use == Some(instruction_index) || self.register_descriptors[register_index].allocated_register == Some(32)) {
                                // If this is the last use, then allocate a register to start tracking this vreg

                                let xxx = live_phys_registers;

                                self.register_descriptors[register_index].allocated_register = Some(usize::try_from(live_phys_registers.trailing_ones()).unwrap());

                                if (self.register_descriptors[register_index].allocated_register.unwrap() < 0) {
                                    panic!("out of registers in allocation");
                                }

                                phys_reg_tracking[self.register_descriptors[register_index].allocated_register.unwrap()] = register_index;
                                live_phys_registers.bit_set(self.register_descriptors[register_index].allocated_register.unwrap());
                                self.register_descriptors[register_index].interference = live_phys_registers;

                                // TODO: Update ALL vreg interferences
                                for i in 0..16 {
                                    if (live_phys_registers.bit_test(i)) {
                                        log::debug!(" updating preg={}, vreg={}, prev={:08x}, cur={:08x}", i, phys_reg_tracking[i], self.register_descriptors[phys_reg_tracking[i]].interference, live_phys_registers);

                                        self.register_descriptors[phys_reg_tracking[i]].interference |= live_phys_registers;
                                    }
                                }

                                log::debug!("starting live-range of vreg {}, allocated to preg {:?} ({:08x}, {:08x})", register_index, self.register_descriptors[register_index].allocated_register, xxx, live_phys_registers);
                            }
                        }
                        Register::PhysicalRegister(_) => {
                            if (register_index > 32) {
                                panic!("use of invalid preg");
                            }

                            // Use of PREG
                            if (live_phys_registers.bit_test(register_index) && phys_reg_tracking[register_index] != register_index) {
                                let conflicting_vreg_index = phys_reg_tracking[register_index];
                                let conflicting_vreg = &mut self.register_descriptors[conflicting_vreg_index];

                                log::debug!("conflicting use of preg {}, currently tracking {}", register_index, conflicting_vreg_index);

                                // Find a new preg for the vreg

                                let new_preg = u16::try_from(conflicting_vreg.interference.trailing_ones()).unwrap();

                                if (new_preg == u16::MAX) {
                                    log::debug!("vreg {} interference={:08x}", conflicting_vreg_index, conflicting_vreg.interference);

                                    panic!("out of registers in re-assignment");
                                }

                                log::debug!("re-assigning vreg {} to preg {} ({:08x})", conflicting_vreg_index, new_preg, conflicting_vreg.interference);

                                conflicting_vreg.allocated_register = Some(usize::try_from(new_preg).unwrap());
                                phys_reg_tracking[conflicting_vreg.allocated_register.unwrap()] = conflicting_vreg_index;
                                live_phys_registers.bit_set(conflicting_vreg.allocated_register.unwrap());
                                self.register_descriptors[conflicting_vreg_index].interference = live_phys_registers;

                                // Update ALL vreg interferences
                                for i in 0..16 {
                                    if (live_phys_registers.bit_test(i)) {
                                        log::debug!(" updating preg={}, vreg={}, prev={:08x}, cur={:08x}", i, phys_reg_tracking[i], self.register_descriptors[phys_reg_tracking[i]].interference, live_phys_registers);

                                        self.register_descriptors[phys_reg_tracking[i]].interference |= live_phys_registers;
                                    }
                                }

                                phys_reg_tracking[register_index] = register_index;
                            } else {
                                phys_reg_tracking[register_index] = register_index;
                                live_phys_registers.bit_set(register_index);
                                self.register_descriptors[register_index].interference = live_phys_registers;

                                // Update ALL vreg interferences
                                for i in 0..16 {
                                    if (live_phys_registers.bit_test(i)) {
                                        log::debug!(" updating preg={}, vreg={}, prev={:08x}, cur={:08x}", i, phys_reg_tracking[i], self.register_descriptors[phys_reg_tracking[i]].interference, live_phys_registers);

                                        self.register_descriptors[phys_reg_tracking[i]].interference |= live_phys_registers;
                                    }
                                }

                                log::debug!("starting live-range of preg {} ({:08x})", register_index, live_phys_registers);
                            }
                        }
                    }
                }
            }
        });
    }

    fn commit(&mut self) {
        dbg!(&self.register_descriptors);

        let mapping = self.register_descriptors.iter().filter_map(|VirtualRegisterDescriptor { allocated_register, .. }| *allocated_register).collect::<Vec<_>>();

        dbg!(&mapping);

        self.instructions.iter_mut().enumerate().for_each(|(instruction_index, instruction)| instruction.replace_virt(&mapping));
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct VirtualRegisterDescriptor {
    first_def: Option<usize>,
    last_use: Option<usize>,
    allocated_register: Option<usize>,
    // bitset
    interference: u16,
}
