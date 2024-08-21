use {
    crate::{
        arch::x86::memory::ExecutableAllocator,
        dbt::{
            emitter::Emitter,
            x86::{
                emitter::{X86Block, X86BlockRef, X86Emitter, X86SymbolRef},
                register_allocator::{solid_state::SolidStateRegisterAllocator, RegisterAllocator},
            },
            Translation, TranslationContext,
        },
    },
    alloc::{
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        rc::Rc,
        vec::Vec,
    },
    core::{cell::RefCell, fmt::Debug},
    iced_x86::code_asm::CodeAssembler,
};

pub mod emitter;
pub mod encoder;
pub mod register_allocator;

pub struct X86TranslationContext {
    initial_block: X86BlockRef,
    emitter: X86Emitter,
    next_symbol_id: u64,
}

impl Debug for X86TranslationContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        {
            let mut visited = BTreeSet::new();
            let mut to_visit = alloc::vec![self.initial_block.clone()];

            while let Some(next) = to_visit.pop() {
                writeln!(f, "{next:x?}")?;
                visited.insert(next.clone());

                if let Some(next_0) = next.get_next_0() {
                    if !visited.contains(&next_0) {
                        to_visit.push(next_0)
                    }
                }

                if let Some(next_1) = next.get_next_1() {
                    if !visited.contains(&next_1) {
                        to_visit.push(next_1)
                    }
                }
            }

            Ok(())
        }
    }
}

impl X86TranslationContext {
    pub fn new() -> Self {
        let initial_block = X86BlockRef::from(X86Block::new());

        Self {
            initial_block: initial_block.clone(),
            emitter: X86Emitter::new(initial_block),
            next_symbol_id: 0,
        }
    }

    fn allocate_registers<R: RegisterAllocator>(&self, mut allocator: R) {
        let mut visited = alloc::vec![];
        let mut to_visit = alloc::vec![self.initial_block.clone()];

        while let Some(next) = to_visit.pop() {
            visited.push(next.clone());

            if let Some(next_0) = next.get_next_0() {
                if !visited.contains(&next_0) {
                    to_visit.push(next_0)
                }
            }

            if let Some(next_1) = next.get_next_1() {
                if !visited.contains(&next_1) {
                    to_visit.push(next_1)
                }
            }
        }

        visited.into_iter().rev().for_each(|block| {
            block.allocate_registers(&mut allocator);
        });
    }
}

impl TranslationContext for X86TranslationContext {
    type Emitter = X86Emitter;

    fn emitter(&mut self) -> &mut Self::Emitter {
        &mut self.emitter
    }

    fn create_block(&mut self) -> <<Self as TranslationContext>::Emitter as Emitter>::BlockRef {
        X86BlockRef::from(X86Block::new())
    }

    fn compile(mut self) -> Translation {
        let num_virtual_registers = self.emitter.next_vreg();
        self.allocate_registers(SolidStateRegisterAllocator::new(num_virtual_registers));

        let mut assembler = CodeAssembler::new(64).unwrap();

        let mut label_map = BTreeMap::new();
        let mut visited = alloc::vec![];
        let mut to_visit = alloc::vec![self.initial_block.clone()];
        label_map.insert(self.initial_block.clone(), assembler.create_label());

        while let Some(next) = to_visit.pop() {
            visited.push(next.clone());

            if let Some(next_0) = next.get_next_0() {
                if !visited.contains(&next_0) {
                    label_map.insert(next_0.clone(), assembler.create_label());
                    to_visit.push(next_0)
                }
            }

            if let Some(next_1) = next.get_next_1() {
                if !visited.contains(&next_1) {
                    label_map.insert(next_1.clone(), assembler.create_label());
                    to_visit.push(next_1)
                }
            }

            // lower block
            assembler
                .set_label(label_map.get_mut(&next).unwrap())
                .unwrap();
            for instr in next.instructions() {
                instr.encode(&mut assembler, &label_map);
            }
        }

        // todo fix unnecessary allocation and byte copy, might require passing
        // allocator T into assemble
        let output = assembler.assemble(0).unwrap();

        let mut code = Vec::with_capacity_in(output.len(), ExecutableAllocator::get());
        for byte in output {
            code.push(byte);
        }

        Translation { code }
    }

    fn create_symbol(&mut self) -> <<Self as TranslationContext>::Emitter as Emitter>::SymbolRef {
        X86SymbolRef(Rc::new(RefCell::new(None)))
    }
}
