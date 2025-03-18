use {
    crate::dbt::{
        Alloc, Translation,
        emitter::Emitter,
        x86::{
            emitter::{X86Block, X86BlockMark, X86Emitter, X86SymbolRef},
            encoder::{Instruction, Opcode, OperandKind},
            register_allocator::naive::FreshAllocator,
        },
    },
    alloc::{alloc::Global, collections::VecDeque, rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        modname::{hashmap_in, hashset_in},
        rudder::Model,
    },
    core::{cell::RefCell, fmt::Debug},
    iced_x86::code_asm::{AsmMemoryOperand, AsmRegister64, CodeAssembler, qword_ptr, rax},
};

pub mod dot;
pub mod emitter;
pub mod encoder;
pub mod register_allocator;

pub struct X86TranslationContext<A: Alloc> {
    allocator: A,
    blocks: Arena<X86Block<A>, A>,
    initial_block: Ref<X86Block<A>>,
    panic_block: Ref<X86Block<A>>,
    writes_to_pc: bool,
    writes_to_sctlr: bool,
    mmu_invalidate: bool,
    pc_offset: u64,
    sctlr_el1_offset: u64,
    ttbr0_el1_offset: u64,
    ttbr1_el1_offset: u64,
    memory_mask: bool,
}

impl<A: Alloc> Debug for X86TranslationContext<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "X86TranslationContext:")?;
        writeln!(f, "\tinitial: {:?}", self.initial_block())?;
        writeln!(f, "\tpanic: {:?}", self.panic_block())?;
        writeln!(f)?;

        let mut visited = hashset_in(self.allocator());
        let mut to_visit = Vec::new_in(self.allocator());
        to_visit.push(self.initial_block());

        while let Some(next) = to_visit.pop() {
            writeln!(f, "{next:x?}:")?;
            for instr in next.get(self.arena()).instructions() {
                writeln!(f, "\t{instr}")?;
            }

            visited.insert(next);

            to_visit.extend(
                next.get(self.arena())
                    .next_blocks()
                    .iter()
                    .filter(|b| !visited.contains(*b)),
            );
        }

        Ok(())
    }
}

impl X86TranslationContext<Global> {
    pub fn new(model: &Model, memory_mask: bool) -> Self {
        Self::new_with_allocator(Global, model, memory_mask)
    }
}

impl<'a, A: Alloc> X86TranslationContext<A> {
    pub fn new_with_allocator(allocator: A, model: &Model, memory_mask: bool) -> Self {
        let mut arena = Arena::new_in(allocator.clone());

        let initial_block = arena.insert(X86Block::new_in(allocator.clone()));
        let panic_block = arena.insert(X86Block::new_in(allocator.clone()));

        let mut celf = Self {
            allocator,
            blocks: arena,
            initial_block,
            panic_block,
            writes_to_pc: false,
            writes_to_sctlr: false,
            mmu_invalidate: false,
            pc_offset: model.reg_offset("_PC"),
            sctlr_el1_offset: model.reg_offset("SCTLR_EL1_bits"),
            ttbr0_el1_offset: model.reg_offset("_TTBR0_EL1_bits"),
            ttbr1_el1_offset: model.reg_offset("_TTBR1_EL1_bits"),
            memory_mask,
        };

        // add panic to the panic block
        {
            let mut emitter = X86Emitter::new(&mut celf);
            emitter.set_current_block(panic_block);
            emitter.panic("panic block");
        }

        celf
    }

    pub fn allocator(&self) -> A {
        self.allocator.clone()
    }

    pub fn arena(&self) -> &Arena<X86Block<A>, A> {
        &self.blocks
    }

    pub fn arena_mut(&mut self) -> &mut Arena<X86Block<A>, A> {
        &mut self.blocks
    }

    fn initial_block(&self) -> Ref<X86Block<A>> {
        self.initial_block
    }

    pub fn panic_block(&self) -> Ref<X86Block<A>> {
        self.panic_block
    }

    pub fn compile(mut self, num_virtual_registers: usize) -> Translation {
        let mut assembler = CodeAssembler::new(64).unwrap();
        let mut label_map = hashmap_in(self.allocator());

        log::trace!("{}", dot::render(self.arena(), self.initial_block()));

        log::trace!("building work queue");

        let mut all_blocks = Vec::new_in(self.allocator());
        let mut work_queue = Vec::new_in(self.allocator());
        work_queue.push(self.panic_block());
        work_queue.push(self.initial_block());

        while let Some(block) = work_queue.pop() {
            if !block.get(self.arena()).is_linked() {
                block.get_mut(self.arena_mut()).set_linked();

                if let Some(label) = label_map.insert(block, assembler.create_label()) {
                    panic!("created label for {block:?} but label {label:?} already existed")
                }
                all_blocks.push(block);

                empty_block_jump_threading(self.arena_mut(), block);
                for block in block.get(self.arena()).next_blocks() {
                    work_queue.push(*block);
                }
            }
        }

        log::trace!("allocating registers");

        all_blocks.iter().for_each(|block| {
            block
                .get_mut(self.arena_mut())
                .allocate_registers(&mut FreshAllocator::new(num_virtual_registers));
        });

        log::trace!("encoding all blocks");

        log::trace!("{}", dot::render(self.arena(), self.initial_block()));

        for (i, block) in all_blocks.iter().enumerate() {
            assembler
                .set_label(label_map.get_mut(block).unwrap())
                .unwrap_or_else(|e| {
                    panic!(
                        "{e}: label already set OR label {:?} for block {block:?} re-used",
                        label_map.get_mut(block).unwrap()
                    )
                });
            assembler
                .nop_1::<AsmMemoryOperand>(qword_ptr(AsmRegister64::from(rax) + block.index()))
                .unwrap();

            let instrs = block.get(self.arena()).instructions();

            let (last, rest) = instrs.split_last().unwrap_or_else(|| {
                panic!(
                    "block {:?} {block:?} was empty",
                    label_map.get_mut(block).unwrap()
                )
            });

            // all but last
            for instr in rest {
                instr.encode(&mut assembler, &label_map);
            }

            assert!(matches!(
                last,
                Instruction(Opcode::JMP(_) | Opcode::INT(_) | Opcode::RET)
            ));

            // fallthrough jump optimization
            if let Instruction(Opcode::JMP(op)) = last {
                if let OperandKind::Target(target) = op.kind() {
                    if all_blocks.get(i + 1).copied() == Some(*target) {
                        // do not emit jump
                        continue;
                    }
                }
            }

            last.encode(&mut assembler, &label_map);
        }

        log::trace!("assembling");
        let code = assembler.assemble(0).unwrap();

        log::trace!("making executable");

        let res = Translation::new(code);

        log::trace!("done");

        res
    }

    pub fn create_block(&mut self) -> Ref<X86Block<A>> {
        let b = X86Block::new_in(self.allocator());
        self.arena_mut().insert(b)
    }

    pub fn create_symbol(&mut self) -> X86SymbolRef<A> {
        X86SymbolRef(Rc::new_in(RefCell::new(None), self.allocator()))
    }

    /// Sets the "PC was written to" flag
    pub fn set_pc_write_flag(&mut self) {
        self.writes_to_pc = true;
    }

    /// Gets the value of the "PC was written to" flag
    pub fn get_pc_write_flag(&self) -> bool {
        self.writes_to_pc
    }

    /// Sets the "SCTLR register was written to" flag
    pub fn set_mmu_config_flag(&mut self) {
        self.writes_to_sctlr = true;
    }

    /// Gets the value of the "SCTLR register was written to" flag
    pub fn get_mmu_write_flag(&self) -> bool {
        self.writes_to_sctlr
    }

    /// Sets the "SCTLR register was written to" flag
    pub fn set_mmu_needs_invalidate_flag(&mut self) {
        self.mmu_invalidate = true;
    }

    /// Gets the "SCTLR register was written to" flag
    pub fn get_mmu_needs_invalidate_flag(&mut self) -> bool {
        self.mmu_invalidate
    }

    pub fn pc_offset(&self) -> u64 {
        self.pc_offset
    }
}

fn link_visit<A: Alloc>(
    block: Ref<X86Block<A>>,
    arena: &mut Arena<X86Block<A>>,
    sorted_blocks: &mut VecDeque<Ref<X86Block<A>>>,
) -> bool {
    match block.get(arena).get_mark() {
        X86BlockMark::Permanent => true,
        X86BlockMark::Temporary => false,
        X86BlockMark::None => {
            block.get_mut(arena).set_mark(X86BlockMark::Temporary);

            for next_block in block
                .get(arena)
                .next_blocks()
                .iter()
                .copied()
                .collect::<Vec<_>>()
            {
                if !link_visit(next_block, arena, sorted_blocks) {
                    return false;
                }
            }

            block.get_mut(arena).set_mark(X86BlockMark::Permanent);

            sorted_blocks.push_front(block);

            true
        }
    }
}

fn empty_block_jump_threading<A: Alloc>(
    arena: &mut Arena<X86Block<A>, A>,
    current_block: Ref<X86Block<A>>,
) {
    // if the current block only has one target
    if let [child] = current_block.get(arena).next_blocks() {
        // and that target only has a single instruction (a jump)
        if let [Instruction(Opcode::JMP(op))] = child.get(arena).instructions() {
            let op = *op;

            // replace the jump in the current block with the jump of the child
            *current_block
                .get_mut(arena)
                .instructions_mut()
                .last_mut()
                .unwrap() = Instruction(Opcode::JMP(op));

            let OperandKind::Target(grandchild) = op.kind() else {
                unreachable!();
            };

            // replace the child block in the current block's "next blocks" with the
            // grandchild block
            current_block.get_mut(arena).clear_next_blocks();
            current_block.get_mut(arena).push_next(*grandchild);

            // recurse
            empty_block_jump_threading(arena, current_block);
        }
    }
}
