use {
    crate::{
        arch::x86::memory::ExecutableAllocator,
        dbt::{
            emitter::Emitter,
            x86::{
                emitter::{X86Block, X86Emitter, X86SymbolRef},
                register_allocator::{solid_state::SolidStateRegisterAllocator, RegisterAllocator},
            },
            Translation,
        },
    },
    alloc::{rc::Rc, vec::Vec},
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
    pub fn new() -> Self {
        let mut arena = Arena::new();

        let initial_block = arena.insert(X86Block::new());
        let panic_block = arena.insert(X86Block::new());

        let mut celf = Self {
            blocks: arena,
            initial_block,
            panic_block,
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

    fn allocate_registers<R: RegisterAllocator>(&mut self, mut allocator: R) {
        let mut visited = alloc::vec![];
        let mut to_visit = alloc::vec![self.initial_block()];

        while let Some(next) = to_visit.pop() {
            visited.push(next);

            to_visit.extend(
                next.get(self.arena())
                    .next_blocks()
                    .iter()
                    .filter(|next| !visited.contains(next))
                    .copied(),
            );
        }

        visited.into_iter().rev().for_each(|block| {
            block
                .get_mut(self.arena_mut())
                .allocate_registers(&mut allocator);
        });
    }

    pub fn compile(mut self, num_virtual_registers: usize) -> Translation {
        log::debug!("{self:?}");

        self.allocate_registers(SolidStateRegisterAllocator::new(num_virtual_registers));

        let mut assembler = CodeAssembler::new(64).unwrap();
        let mut label_map = HashMap::default();
        let mut visited = HashSet::default();
        let mut to_visit = alloc::vec![];

        {
            let panic_label = assembler.create_label();
            log::trace!("panic_block ({panic_label:?}) {:?}", self.panic_block());
            label_map.insert(self.panic_block(), panic_label);
            to_visit.push(self.panic_block()) // visit panic block last
        }

        {
            let initial_label = assembler.create_label();
            log::trace!(
                "initial_block ({initial_label:?}) {:?}",
                self.initial_block()
            );
            label_map.insert(self.initial_block(), initial_label);
            to_visit.push(self.initial_block())
        }

        while let Some(next) = to_visit.pop() {
            log::trace!("assembling {next:?}:");

            visited.insert(next.clone());

            next.get(self.arena())
                .next_blocks()
                .iter()
                .filter(|next| !visited.contains(*next))
                .copied()
                .for_each(|next| {
                    let label = assembler.create_label();
                    log::trace!("next: {label:?}");
                    label_map.insert(next, label);
                    to_visit.push(next);
                });

            // lower block
            assembler
                .set_label(label_map.get_mut(&next).unwrap())
                .unwrap_or_else(|e| {
                    panic!(
                        "{e}: label {:?} for block {next:?} re-used",
                        label_map.get_mut(&next).unwrap()
                    )
                });
            assembler
                .nop_1::<AsmMemoryOperand>(qword_ptr(AsmRegister64::from(rax) + next.index()))
                .unwrap();
            for instr in next.get(self.arena()).instructions() {
                log::debug!("\t{instr}");
                instr.encode(&mut assembler, &label_map);
            }
        }

        // todo fix unnecessary allocation and byte copy, might require passing
        // allocator T into assemble
        let code = {
            let output = assembler.assemble(0).unwrap();

            let mut code = Vec::with_capacity_in(output.len(), ExecutableAllocator::get());
            for byte in output {
                code.push(byte);
            }
            code
        };

        Translation { code }
    }

    pub fn create_block(&mut self) -> Ref<X86Block> {
        self.arena_mut().insert(X86Block::new())
    }

    pub fn create_symbol(&mut self) -> X86SymbolRef {
        X86SymbolRef(Rc::new(RefCell::new(None)))
    }
}
