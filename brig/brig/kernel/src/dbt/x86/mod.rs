use {
    crate::{
        dbg,
        dbt::{
            emitter::{Emitter, Type},
            x86::{
                emitter::{
                    BinaryOperationKind, NodeKind, X86Block, X86BlockRef, X86Emitter, X86Node,
                    X86NodeRef,
                },
                encoder::Instruction,
                register_allocator::{solid_state::SolidStateRegisterAllocator, RegisterAllocator},
            },
            Translation, TranslationContext,
        },
    },
    alloc::{
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        vec::Vec,
    },
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

    // make this an iterator too
    fn linearize_instructions(&self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let mut visited = BTreeSet::new();
        let mut to_visit = alloc::vec![self.initial_block.clone()];

        while let Some(next) = to_visit.pop() {
            instructions.extend(next.get_instructions());

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

        instructions
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
        let mut instrs = self.linearize_instructions();

        log::debug!("pre-allocation: {instrs:#?}");

        SolidStateRegisterAllocator::allocate(&mut instrs, self.emitter.next_vreg());

        log::debug!("post-allocation: {instrs:#?}");

        Translation {
            code: self.initial_block.lower(),
        }
    }
}
