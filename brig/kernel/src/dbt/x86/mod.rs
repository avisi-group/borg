use {
    crate::dbt::{
        emitter::Emitter,
        x86::{
            emitter::{X86Block, X86BlockMark, X86Emitter, X86SymbolRef},
            encoder::{Instruction, Opcode, OperandKind},
            register_allocator::solid_state::SolidStateRegisterAllocator,
        },
        Translation,
    },
    alloc::{collections::VecDeque, rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        HashMap, HashSet,
    },
    core::{cell::RefCell, fmt::Debug},
    iced_x86::code_asm::{qword_ptr, rax, AsmMemoryOperand, AsmRegister64, CodeAssembler},
};

pub mod emitter;
pub mod encoder;
pub mod register_allocator;

pub struct X86TranslationContext {
    blocks: Arena<X86Block>,
    initial_block: Ref<X86Block>,
    panic_block: Ref<X86Block>,
    writes_to_pc: bool,
    pc_offset: u64,
}

impl Debug for X86TranslationContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "X86TranslationContext:")?;
        writeln!(f, "\tinitial: {:?}", self.initial_block())?;
        writeln!(f, "\tpanic: {:?}", self.panic_block())?;
        writeln!(f)?;

        let mut visited = HashSet::default();
        let mut to_visit = alloc::vec![self.initial_block()];

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

impl X86TranslationContext {
    pub fn new(pc_offset: u64) -> Self {
        let mut arena = Arena::new();

        let initial_block = arena.insert(X86Block::new());
        let panic_block = arena.insert(X86Block::new());

        let mut celf = Self {
            blocks: arena,
            initial_block,
            panic_block,
            writes_to_pc: false,
            pc_offset,
        };

        // add panic to the panic block
        {
            let mut emitter = X86Emitter::new(&mut celf);
            emitter.set_current_block(panic_block);
            emitter.panic("panic block");
        }

        celf
    }

    pub fn arena(&self) -> &Arena<X86Block> {
        &self.blocks
    }

    pub fn arena_mut(&mut self) -> &mut Arena<X86Block> {
        &mut self.blocks
    }

    fn initial_block(&self) -> Ref<X86Block> {
        self.initial_block
    }

    pub fn panic_block(&self) -> Ref<X86Block> {
        self.panic_block
    }

    pub fn compile(mut self, num_virtual_registers: usize) -> Translation {
        let mut assembler = CodeAssembler::new(64).unwrap();
        let mut label_map = HashMap::default();

        log::info!("building work queue");

        let mut all_blocks = Vec::new();
        let mut work_queue = Vec::new();
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

        log::info!("allocating registers");

        let mut allocator = SolidStateRegisterAllocator::new(num_virtual_registers);
        all_blocks.iter().rev().for_each(|block| {
            block
                .get_mut(self.arena_mut())
                .allocate_registers(&mut allocator);
        });

        log::info!("encoding all blocks");

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

            let (last, rest) = instrs.split_last().unwrap();

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

        log::info!("assembling");
        let code = assembler.assemble(0).unwrap();

        log::info!("making executable");

        let res = Translation::new(code);

        log::info!("done");

        res
    }

    pub fn create_block(&mut self) -> Ref<X86Block> {
        self.arena_mut().insert(X86Block::new())
    }

    pub fn create_symbol(&mut self) -> X86SymbolRef {
        X86SymbolRef(Rc::new(RefCell::new(None)))
    }

    /// Sets the "PC was written to" flag
    pub fn set_pc_write_flag(&mut self) {
        self.writes_to_pc = true;
    }

    /// Gets the value of the "PC was written to" flag
    pub fn get_pc_write_flag(&self) -> bool {
        self.writes_to_pc
    }

    pub fn pc_offset(&self) -> u64 {
        self.pc_offset
    }
}

fn link_visit(
    block: Ref<X86Block>,
    arena: &mut Arena<X86Block>,
    sorted_blocks: &mut VecDeque<Ref<X86Block>>,
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

fn empty_block_jump_threading(arena: &mut Arena<X86Block>, current_block: Ref<X86Block>) {
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
