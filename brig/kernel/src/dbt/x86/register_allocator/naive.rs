//! Based on SSRA by Matt Keeter https://www.mattkeeter.com/blog/2022-10-04-ssra/

use {
    crate::dbt::x86::{
        encoder::{Instruction, Opcode, PhysicalRegister, Register, UseDef},
        register_allocator::RegisterAllocator,
    },
    alloc::vec::Vec,
    bitset_core::BitSet,
    common::{HashMap, HashSet},
    core::panic,
    itertools::Itertools,
};

pub struct FreshAllocator {
    live_ranges: HashMap<Register, Vec<(usize, Option<usize>)>>,

    allocation_plan: HashMap<usize, usize>,
}

impl RegisterAllocator for FreshAllocator {
    fn allocate(&mut self, instructions: &mut [Instruction]) {
        log::debug!("----------------------");

        self.build_live_ranges(instructions);
        self.live_ranges.iter().for_each(|(reg, ranges)| {
            log::debug!(
                "{reg:?} = {}",
                ranges
                    .iter()
                    .map(|(start, end)| alloc::format!("{start} -> {end:?}"))
                    .join(", ")
            )
        });

        self.build_allocation_plan(instructions);
        self.allocation_plan
            .iter()
            .for_each(|(vreg, preg)| log::debug!("{vreg} = {preg}",));

        // apply allocation plan
        instructions.iter_mut().for_each(|instruction| {
            instruction.get_use_defs().for_each(|ud| {
                let (UseDef::Def(reg) | UseDef::Use(reg) | UseDef::UseDef(reg)) = ud;
                if let Register::VirtualRegister(vreg) = &*reg {
                    *reg = Register::PhysicalRegister(PhysicalRegister::from_index(
                        *self.allocation_plan.get(vreg).unwrap(),
                    ));
                }
            });
        });

        instructions.iter_mut().for_each(|instruction| {
            if let Opcode::MOV(src, dst) = instruction.0 {
                if src == dst {
                    instruction.0 = Opcode::DEAD;
                }
            }
        });

        log::debug!("post alloc----------------------------");
        for i in instructions {
            log::debug!("{i}");
        }
    }
}

impl FreshAllocator {
    pub fn new(_num_virt_regs: usize) -> Self {
        Self {
            live_ranges: HashMap::default(),
            allocation_plan: HashMap::default(),
        }
    }

    fn build_live_ranges(&mut self, instructions: &mut [Instruction]) {
        // stores stack pointer from brig, can't clobber
        self.live_ranges.insert(
            Register::PhysicalRegister(PhysicalRegister::RSP),
            alloc::vec![(0, Some(usize::MAX))],
        );
        // register file pointer
        self.live_ranges.insert(
            Register::PhysicalRegister(PhysicalRegister::RBP),
            alloc::vec![(0, Some(usize::MAX))],
        );
        // ???
        self.live_ranges.insert(
            Register::PhysicalRegister(PhysicalRegister::R15),
            alloc::vec![(0, Some(usize::MAX))],
        );
        // frame pointer
        self.live_ranges.insert(
            Register::PhysicalRegister(PhysicalRegister::R14),
            alloc::vec![(0, Some(usize::MAX))],
        );

        let instrs_clone = instructions.to_vec();
        log::debug!("before alloc ----------------------------");
        for (idx, i) in instrs_clone.iter().enumerate() {
            log::debug!("{idx}: {i}");
        }

        instructions
            .iter_mut()
            .enumerate()
            .for_each(|(instruction_index, instruction)| {
                let instr_clone = instruction.clone();
                instruction.get_use_defs().for_each(|ud| {
                    let is_usedef = ud.is_usedef();
                    if let UseDef::Def(reg) | UseDef::UseDef(reg) = ud {
                        if is_usedef {
                            if let Opcode::XOR(l, r) = instr_clone.0 {
                                if l == r {
                                    //
                                } else {
                                    return;
                                }
                            } else {
                                return;
                            }
                        }

                        self.live_ranges
                            .entry(*reg)
                            .and_modify(|live_ranges| {
                                // assert last live range had some end
                                let last_range = live_ranges.as_mut_slice().last_mut().unwrap();

                                if last_range.1.is_none() {
                                    // silenced due to CMOVNE, will give it an end in a second
                                    // log::warn!(
                                    //     "last live range had no end, but re-def'd: {reg} in {}",
                                    //     instr_clone
                                    // );
                                    last_range.1 = Some(instruction_index);
                                }

                                // start new live range if past the current end
                                if instruction_index >= last_range.1.unwrap_or_default() {
                                    if let Register::VirtualRegister(_) = *reg {
                                        if let Opcode::CMOVNE(_, _) = instr_clone.0 {
                                            // do nothing for CMOVNE
                                        } else {
                                            panic!(
                                            "cannot re-start virtual register {reg} in instr {}",
                                            instr_clone
                                        )
                                        }
                                    } else {
                                        live_ranges.push((instruction_index, None));
                                    }
                                }
                            })
                            .or_insert(alloc::vec![(instruction_index, None)]);
                    }
                });
                instruction.get_use_defs().for_each(|ud| {
                    if let UseDef::Use(reg) | UseDef::UseDef(reg) = ud {
                        // assert exists
                        let live_ranges = self.live_ranges.get_mut(reg).unwrap_or_else(|| {
                            panic!("use of undef'd register {reg} @ {instruction_index}")
                        });

                        // update end
                        let last_use = &mut live_ranges
                            .as_mut_slice()
                            .last_mut()
                            .expect("should have at least one live range")
                            .1;

                        if last_use.unwrap_or_default() < instruction_index {
                            *last_use = Some(instruction_index);
                        }
                    }
                });
            });
    }

    fn build_allocation_plan(&mut self, instructions: &mut [Instruction]) {
        let mut physical_used = 0u16;

        instructions.iter().enumerate().for_each(|(instruction_index, _instruction)| {
            {
                let ended_registers = self.live_ranges.iter().map(|(reg, ranges)| ranges.iter().map(move |range| (*reg, *range))).flatten().filter(|(_, (_, end))| *end == Some(instruction_index)).map(|(reg, _)| reg).collect::<Vec<_>>();

                ended_registers.iter().for_each(|reg| match reg {
                    Register::PhysicalRegister(idx) => {
                        assert!(physical_used.bit_test(idx.index()));
                        physical_used.bit_reset(idx.index());
                    }
                    Register::VirtualRegister(idx) => {
                        let phys_reg = *self.allocation_plan.get(&*idx).unwrap();
                        assert!(physical_used.bit_test(phys_reg));
                        physical_used.bit_reset(phys_reg);
                    }
                });
            }

            let started_registers = self.live_ranges.iter().map(|(reg, ranges)| ranges.iter().map(move |range| (*reg, *range))).flatten().filter(|(_, (start, _))| *start == instruction_index).map(|(reg, _)| reg).collect::<Vec<_>>();

            started_registers.iter().filter_map(|reg| if let Register::PhysicalRegister(idx) = reg { Some(idx.index()) } else { None }).for_each(|idx| {
                if physical_used.bit_test(idx) {
                    let currently_live_registers = self.live_ranges.iter().filter(|(_, ranges)| ranges.iter().any(|(start, end)| (*start <= instruction_index) && (instruction_index < end.unwrap()))).filter_map(|(reg, _)| if let Register::VirtualRegister(idx) = reg { Some(*idx) } else { None }).collect::<Vec<usize>>();

                    // vregs that use our just-started physical register
                    let mut vregs = self.allocation_plan.iter().filter(|(vreg, preg)| **preg == idx && currently_live_registers.contains(vreg)).map(|(vreg, _)| *vreg).collect::<Vec<_>>();

                    assert!(vregs.len() == 1);

                    let conflicting_vreg = vregs.pop().unwrap();
                    log::trace!("detected conflict with preg {idx} and vreg {}", conflicting_vreg);

                    // todo: maybe only need to check intersections with start of current range

                    // virt so should onyl have one range
                    let vreg_range = self.live_ranges.get(&Register::VirtualRegister(conflicting_vreg)).unwrap()[0];


                    // now we need to choose a new phys reg

                    // find all registers that intersect with the conflicting register
                let intersecting_registers=    query_intersections(vreg_range, &self.live_ranges);

               let intersecting_physicals = intersecting_registers.iter().filter_map(|reg| match reg {
                    Register::VirtualRegister(idx) => self.allocation_plan.get(&*idx).copied(), // intersects in the future but not yet allocated
                    Register::PhysicalRegister(idx) => Some(idx.index()),
                }).collect::<Vec<_>>();

                // todo: maybe start at 0 and set bits, rather than copying currently used
                    let mut temp_physical_used = physical_used;
                    for idx in intersecting_physicals {
                        temp_physical_used.bit_set(idx);
                    }
                    let reallocated_phys_index = {
                        let first_empty = temp_physical_used.trailing_ones();

                        if first_empty > 16 {
                            panic!("ran out of registers :(");
                        }

                        usize::try_from(first_empty).unwrap()
                    };
                    physical_used.bit_set(reallocated_phys_index);

                    self.allocation_plan.insert(conflicting_vreg, reallocated_phys_index);
                } else {
                    physical_used.bit_set(idx);
                }
            });

            started_registers.iter().filter_map(|reg| if let Register::VirtualRegister(idx) = reg { Some(idx) } else { None }).for_each(|vreg_idx| {
                let phys_index = {
                    let first_empty = physical_used.trailing_ones();

                    if first_empty > 16 {
                        panic!("ran out of registers :(");
                    }

                    usize::try_from(first_empty).unwrap()
                };

                physical_used.bit_set(phys_index);

                // assert that virtual register never re-starts
                if let Some(old_preg) = self.allocation_plan.insert(*vreg_idx, phys_index) {
                    panic!("cannot re-start virtual register! vreg = {vreg_idx}, old_preg = {old_preg}, new allocation = {phys_index}");
                }
            })
        });
    }
}

fn query_intersections(
    (x_start, x_end): (usize, Option<usize>),
    live_ranges: &HashMap<Register, Vec<(usize, Option<usize>)>>,
) -> HashSet<Register> {
    live_ranges
        .iter()
        .map(|(reg, ranges)| ranges.iter().map(move |range| (*reg, *range)))
        .flatten()
        .filter(|(_, (y_start, y_end))| x_start <= y_end.unwrap() && *y_start <= x_end.unwrap())
        .map(|(reg, _)| reg)
        .collect()
}
