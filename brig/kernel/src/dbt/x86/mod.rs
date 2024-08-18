use {
    crate::dbt::{
        emitter::Emitter,
        x86::{
            emitter::{X86Block, X86BlockRef, X86Emitter},
            register_allocator::{solid_state::SolidStateRegisterAllocator, RegisterAllocator},
        },
        Translation, TranslationContext,
    },
    alloc::collections::btree_set::BTreeSet,
    core::fmt::Debug,
};

pub mod emitter;
pub mod encoder;
pub mod register_allocator;

pub struct X86TranslationContext {
    initial_block: X86BlockRef,
    emitter: X86Emitter,
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

        Translation {
            code: self.initial_block.lower(),
        }
    }
}
