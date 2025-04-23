use {
    crate::dbt::{
        Alloc,
        emitter::{self, Emitter, Type},
        register_file::RegisterFile,
        trampoline::{ExecutionResult, MAX_STACK_SIZE},
        x86::{
            emitter::{NodeKind, X86Block, X86Emitter, X86NodeRef},
            encoder::Instruction,
        },
    },
    alloc::{collections::BTreeMap, rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        hashmap::{HashMapA, hashmap_in},
        intern::InternedString,
        rudder::{
            self, Model, RegisterCacheType, block::Block, constant::Constant, function::Function,
            statement::Statement, types::PrimitiveType,
        },
        width_helpers::unsigned_smallest_width_of_value,
    },
    core::{
        hash::{Hash, Hasher},
        panic,
        sync::atomic::{AtomicUsize, Ordering},
    },
    derive_where::derive_where,
    itertools::Itertools,
};

const BLOCK_QUEUE_LIMIT: usize = 1000;

// if we attempt to translate any of these , something went wrong
const FN_DENYLIST: &[&str] = &["AArch64_TranslateAddress"];

/// DBT translation error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// SEE exception during instruction decode
    Decode,
}

/// Kind of jump to a target block
#[derive(Debug)]
enum JumpKind<A: Alloc> {
    // static jump (jump or branch with constant condition)
    Static {
        rudder: Ref<Block>,
        x86: Ref<X86Block<A>>,
        variables: BTreeMap<InternedString, LocalVariable<A>, A>,
    },
    // branch with non-constant condition
    Dynamic {
        rudder: Ref<Block>,
        x86: Ref<X86Block<A>>,
        variables: BTreeMap<InternedString, LocalVariable<A>, A>,
    },
}

#[derive(Debug, Clone)]
enum StatementResult<A: Alloc> {
    Data(Option<X86NodeRef<A>>),
    ControlFlow(ControlFlow<A>),
}

#[derive(Debug, Clone)]
enum ControlFlow<A: Alloc> {
    Jump(
        Ref<Block>,
        Ref<X86Block<A>>,
        BTreeMap<InternedString, LocalVariable<A>, A>,
    ),
    Branch(
        Ref<Block>,
        Ref<Block>,
        BTreeMap<InternedString, LocalVariable<A>, A>,
    ),
    Panic,
    Return,
}

const NUM_TRANSLATE_ATTEMPTS: usize = 3;

/// Top-level translation of a given guest instruction opcode
///
/// Includes logic for retrying decoding if a SEE exception is thrown.
pub fn translate_instruction<A: Alloc>(
    allocator: A,
    model: &Model,
    function: &str,
    emitter: &mut X86Emitter<A>,
    register_file: &RegisterFile,
    pc: u64,
    opcode: u32,
) -> Result<Option<X86NodeRef<A>>, Error> {
    register_file.write("SEE", -1i64);

    let initial_block = emitter.get_current_block();

    let mut attempts_remaining = NUM_TRANSLATE_ATTEMPTS;

    let (result, start_block) = loop {
        if attempts_remaining == 0 {
            panic!("Failed to translate in {NUM_TRANSLATE_ATTEMPTS} attempts")
        }

        let start_block = emitter.ctx_mut().create_block();
        emitter.set_current_block(start_block);

        let pc = emitter.constant(pc, Type::Unsigned(64));
        let opcode = emitter.constant(u64::from(opcode), Type::Unsigned(32));

        let res = translate(
            allocator,
            model,
            function,
            &[pc, opcode],
            emitter,
            register_file,
        );

        // from __InstructionExecute
        // if not_bool(AArch64_ExecutingERETInstr()) then {
        //     PSTATE.BTYPE = BTypeNext
        // };
        {
            let update_btype_block = emitter.ctx_mut().create_block();

            let end_block = emitter.ctx_mut().create_block();

            let executing_eret = translate(
                allocator,
                model,
                "AArch64_ExecutingERETInstr",
                &[],
                emitter,
                register_file,
            )
            .unwrap()
            .unwrap();

            emitter.branch(executing_eret, end_block, update_btype_block);

            emitter.set_current_block(update_btype_block);
            let btypenext = emitter.read_register(model.reg_offset("BTypeNext"), Type::Unsigned(2));
            emitter.write_register(model.reg_offset("PSTATE_BTYPE"), btypenext);
            emitter.jump(end_block);

            emitter.set_current_block(end_block);
        }

        match res {
            Ok(_) => break (res, start_block),
            Err(Error::Decode) => {
                // not resetting emitter on decode SEE retry, this is risky in
                // case we emitted stuff during translation, except we should
                // never have hit a write-mem or write-reg
                // inside decode, it should always be const

                attempts_remaining -= 1;
                // todo: timeout
            }
        }
    };

    let end_block = emitter.get_current_block();

    emitter.set_current_block(initial_block);
    emitter.jump(start_block);

    emitter.set_current_block(end_block);

    result
}

pub fn translate<A: Alloc>(
    allocator: A,
    model: &Model,
    function: &str,
    arguments: &[X86NodeRef<A>],
    emitter: &mut X86Emitter<A>,
    register_file: &RegisterFile,
) -> Result<Option<X86NodeRef<A>>, Error> {
    // x86_64 has full descending stack so current stack offset needs to start at 8
    // for first stack variable offset to point to the next empty slot
    let current_stack_offset = Rc::new_in(AtomicUsize::new(8), allocator.clone());
    FunctionTranslator::new(
        allocator,
        model,
        function,
        arguments,
        emitter,
        current_stack_offset,
        register_file,
    )
    .translate()
}

#[derive(Clone)]
#[derive_where(Debug)]
enum LocalVariable<A: Alloc> {
    Virtual {
        value: Option<X86NodeRef<A>>,
    },
    Stack {
        typ: emitter::Type,
        stack_offset: usize,
    },
}

impl<A: Alloc> Default for LocalVariable<A> {
    fn default() -> Self {
        LocalVariable::Virtual { value: None }
    }
}

// we only care if the variable is virtual or on the stack?
impl<A: Alloc> Hash for LocalVariable<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<A: Alloc> PartialEq for LocalVariable<A> {
    fn eq(&self, other: &LocalVariable<A>) -> bool {
        core::mem::discriminant(self).eq(&core::mem::discriminant(other))
    }
}

impl<A: Alloc> Eq for LocalVariable<A> {}

#[derive_where(Debug)]
struct ReturnValue<A: Alloc> {
    variables: Vec<LocalVariable<A>, A>,
    previous_write: Option<(Ref<X86Block<A>>, usize)>,
}

impl<A: Alloc> ReturnValue<A> {
    pub fn new<'e, 'c>(
        emitter: &'e mut X86Emitter<'c, A>,
        return_type: Option<rudder::types::Type>,
    ) -> Self {
        let num_variables = match return_type {
            Some(rudder::types::Type::Tuple(elements)) => elements.len(),
            Some(_) => 1,
            None => 0,
        };

        let mut variables = Vec::with_capacity_in(num_variables, emitter.ctx().allocator());

        for _ in 0..num_variables {
            variables.push(LocalVariable::default());
        }

        Self {
            variables,
            previous_write: None,
        }
    }
}

struct FunctionTranslator<'model, 'registers, 'emitter, 'context, A: Alloc> {
    allocator: A,

    /// The model we are translating guest code for
    model: &'model Model,

    /// Function being translated
    function: &'model Function,

    dynamic_blocks: HashMapA<
        (Ref<Block>, BTreeMap<InternedString, LocalVariable<A>, A>),
        (Ref<X86Block<A>>, bool),
        A,
    >,
    static_blocks: HashMapA<Ref<Block>, Vec<Ref<X86Block<A>>>, A>,

    entry_variables: BTreeMap<InternedString, LocalVariable<A>, A>,

    return_value: ReturnValue<A>,

    // don't re-promote stack variables to a different location
    promoted_locations: HashMapA<InternedString, usize, A>,

    /// Dynamic bitvector stack lengths
    bits_stack_widths: HashMapA<usize, u16, A>,

    /// Stack offset used to allocate stack variables
    current_stack_offset: Rc<AtomicUsize, A>,

    /// X86 instruction emitter
    emitter: &'emitter mut X86Emitter<'context, A>,

    /// Pointer to the register file used for cached register reads
    register_file: &'registers RegisterFile,
}

impl<'m, 'r, 'e, 'c, A: Alloc> FunctionTranslator<'m, 'r, 'e, 'c, A> {
    fn read_return_value(&mut self) -> Option<X86NodeRef<A>> {
        match self.function.return_type() {
            Some(rudder::types::Type::Tuple(_)) => {
                let mut values = Vec::new_in(self.allocator);

                self.return_value
                    .variables
                    .clone()
                    .into_iter()
                    .map(|var| self.read_variable(var))
                    .collect_into(&mut values);

                Some(self.emitter.create_tuple(values))
            }
            Some(_) => Some(self.read_variable(self.return_value.variables[0].clone())),
            None => None,
        }
    }

    fn write_return_value(&mut self, value: X86NodeRef<A>) {
        let values = match value.kind() {
            NodeKind::Tuple(elements) => (*elements).clone(),
            _ => {
                let mut values = Vec::new_in(self.allocator);
                values.push(value.clone());
                values
            }
        };

        // we should never have some promoted and some not

        let is_virtual = self
            .return_value
            .variables
            .iter()
            .map(|v| matches!(v, LocalVariable::Virtual { .. }))
            .all_equal_value()
            .expect("variables not all virtual or all stack");

        // if we haven't promoted already
        if is_virtual {
            let current_block = self.emitter.get_current_block();

            // if this is the second write, promote
            if let Some((previous_block, previous_index)) = self.return_value.previous_write {
                let types = match self.function.return_type() {
                    Some(rudder::types::Type::Tuple(types)) => types.clone(),
                    Some(t) => alloc::vec![t],
                    None => alloc::vec![],
                };

                self.emitter.set_current_block(previous_block);

                let post = previous_block
                    .get_mut(self.emitter.ctx_mut().arena_mut())
                    .instructions_mut()
                    .split_off(previous_index);

                let mut stack_variables = Vec::new_in(self.allocator);
                self.return_value
                    .variables
                    .clone()
                    .into_iter()
                    .zip(types.into_iter())
                    .map(|(local_variable, typ)| {
                        let LocalVariable::Virtual { value } = local_variable else {
                            panic!()
                        };

                        let prev_value =
                            value.expect("no previous value written to local variable");

                        let stack_offset = self.allocate_stack_offset(&typ);

                        // fix up the previous write
                        self.emitter.write_stack_variable(stack_offset, prev_value);

                        LocalVariable::Stack {
                            typ: emit_rudder_type(&typ),
                            stack_offset,
                        }
                    })
                    .collect_into(&mut stack_variables);

                previous_block
                    .get_mut(self.emitter.ctx_mut().arena_mut())
                    .instructions_mut()
                    .extend_from_slice(&post);

                self.emitter.set_current_block(current_block);

                // promote
                self.return_value.variables = stack_variables;
            } else {
                self.return_value.previous_write = Some((
                    current_block,
                    current_block
                        .get(self.emitter.ctx().arena())
                        .instructions()
                        .len(),
                ));
            }
        }

        let mut new_variables = self.return_value.variables.clone();

        values
            .into_iter()
            .zip(new_variables.iter_mut())
            .for_each(|(value, variable)| {
                self.write_variable(variable, value);
            });

        self.return_value.variables = new_variables;

        log::trace!(
            "wrote var borealis_fn_return_value: {:?}",
            self.return_value
        );
    }

    fn new(
        allocator: A,
        model: &'m Model,
        function: &str,
        arguments: &[X86NodeRef<A>],
        emitter: &'e mut X86Emitter<'c, A>,
        current_stack_offset: Rc<AtomicUsize, A>,
        register_file: &'r RegisterFile,
    ) -> Self {
        log::debug!("translating {function:?}: {:?}", arguments);
        assert!(!FN_DENYLIST.contains(&function));

        let function_name = InternedString::from(function);

        let function = model
            .functions()
            .get(&function_name)
            .unwrap_or_else(|| panic!("function named {function:?} not found"));

        let mut celf = Self {
            allocator,
            model,
            function,
            dynamic_blocks: hashmap_in(emitter.ctx().allocator()),
            static_blocks: hashmap_in(emitter.ctx().allocator()),
            entry_variables: BTreeMap::new_in(emitter.ctx().allocator()),
            promoted_locations: hashmap_in(emitter.ctx().allocator()),
            bits_stack_widths: hashmap_in(emitter.ctx().allocator()),
            return_value: ReturnValue::new(emitter, function.return_type()),
            current_stack_offset,
            emitter,
            register_file,
        };

        // set up symbols for parameters, and write arguments into them

        function
            .parameters()
            .iter()
            .zip(arguments)
            .map(|(parameter, argument)| {
                (
                    parameter.name(),
                    LocalVariable::Virtual {
                        value: Some(argument.clone()),
                    },
                )
            })
            .collect_into(&mut celf.entry_variables);

        celf
    }

    fn translate(&mut self) -> Result<Option<X86NodeRef<A>>, Error> {
        // create an empty block all control flow will end at
        let exit_block = self
            .emitter
            .ctx_mut()
            .arena_mut()
            .insert(X86Block::new_in(self.allocator.clone()));

        // create an empty entry block for this function
        let entry_x86 = self
            .emitter
            .ctx_mut()
            .arena_mut()
            .insert(X86Block::new_in(self.allocator.clone()));

        // jump from the *current* emitter block to this function's entry block
        self.emitter.push_instruction(Instruction::jmp(entry_x86));
        self.emitter.push_target(entry_x86);

        let mut block_queue = alloc::collections::VecDeque::new_in(self.allocator.clone());

        // start translation of the function's rudder entry block to the new (empty) X86
        // entry block
        block_queue.push_front(JumpKind::Static {
            rudder: self.function.entry_block(),
            x86: entry_x86,
            variables: self.entry_variables.clone(),
        });

        while let Some(block) = block_queue.pop_front() {
            if block_queue.len() > BLOCK_QUEUE_LIMIT {
                panic!("block queue exceeded limit")
            }

            let result = match block {
                JumpKind::Static {
                    rudder: rudder_block,
                    x86: x86_block,
                    variables,
                } => {
                    self.emitter.set_current_block(x86_block);
                    log::trace!(
                        "translating static block rudder={rudder_block:?}, x86={x86_block:?}, variables: {variables:?}",
                    );
                    let res = self.translate_block(rudder_block, false, variables)?;
                    log::trace!("emitted: {:?}", x86_block.get(self.emitter.ctx().arena()));
                    res
                }
                JumpKind::Dynamic {
                    rudder: rudder_block,
                    x86: x86_block,
                    variables,
                } => {
                    //  log::trace!("dynamic block {}", b.index());
                    if !x86_block
                        .get(self.emitter.ctx_mut().arena_mut())
                        .instructions()
                        .is_empty()
                    {
                        log::trace!("already visited");
                        continue;
                    }

                    self.emitter.set_current_block(x86_block);

                    log::trace!(
                        "translating dynamic block rudder={rudder_block:?}, x86={x86_block:?}, variables: {variables:?}",
                    );

                    let res = self.translate_block(rudder_block, true, variables)?;
                    log::trace!("emitted: {:?}", x86_block.get(self.emitter.ctx().arena()));
                    res
                }
            };

            match result {
                ControlFlow::Jump(rudder, x86, lives) => {
                    log::trace!("block result: static(rudder={rudder:?},x86={x86:?})",);

                    block_queue.push_front(JumpKind::Static {
                        rudder,
                        x86,
                        variables: lives,
                    });
                }
                ControlFlow::Branch(block0, block1, lives) => {
                    let block0_x86 = self.dynamic_blocks.get(&(block0, lives.clone())).unwrap().0;
                    let block1_x86 = self.dynamic_blocks.get(&(block1, lives.clone())).unwrap().0;
                    log::trace!(
                        "block result: dynamic({}, {})",
                        block0.index(),
                        block1.index()
                    );

                    block_queue.push_back(JumpKind::Dynamic {
                        rudder: block0,
                        x86: block0_x86,
                        variables: lives.clone(),
                    });
                    block_queue.push_back(JumpKind::Dynamic {
                        rudder: block1,
                        x86: block1_x86,
                        variables: lives,
                    });
                }
                ControlFlow::Return => {
                    log::trace!("block result: return ({exit_block:?})");
                    self.emitter.jump(exit_block);
                }
                ControlFlow::Panic => {
                    let panic = self.emitter.ctx().panic_block();
                    log::trace!("block result: panic ({panic:?})");
                    // unreachable but inserted just to make sure *every* block has a path to the
                    // exit block
                    self.emitter.jump(panic);
                }
            }
        }

        // finish translation with current block set to the exit block
        self.emitter.set_current_block(exit_block);

        log::debug!("finished translating {:?}", self.function.name());

        Ok(self.read_return_value())
    }

    fn translate_block(
        &mut self,

        block_ref: Ref<Block>,
        is_dynamic: bool,
        mut variables: BTreeMap<InternedString, LocalVariable<A>, A>,
    ) -> Result<ControlFlow<A>, Error> {
        let block = block_ref.get(self.function.arena());

        let mut statement_value_store = StatementValueStore::new(self.allocator.clone());

        for s in block.statements() {
            match self.translate_statement(
                &statement_value_store,
                is_dynamic,
                s.get(block.arena()),
                block_ref,
                block.arena(),
                &mut variables,
            )? {
                StatementResult::Data(Some(value)) => {
                    log::trace!(
                        "{} {} = {:?}",
                        s,
                        s.get(block.arena()).to_string(block.arena()),
                        value.kind(),
                    );
                    statement_value_store.insert(*s, value);
                }
                StatementResult::Data(None) => {
                    log::trace!(
                        "{} {} = ()",
                        s,
                        s.get(block.arena()).to_string(block.arena()),
                    );
                }
                StatementResult::ControlFlow(block_result) => {
                    log::trace!(
                        "{} {} = {:?}",
                        s,
                        s.get(block.arena()).to_string(block.arena()),
                        block_result
                    );
                    return Ok(block_result);
                }
            }
        }

        unreachable!(
            "last statement in block should have returned a control flow statement result in {:?}",
            self.function.name()
        )
    }

    // todo: fix these parameters this is silly
    fn translate_statement(
        &mut self,
        value_store: &StatementValueStore<A>,
        is_dynamic: bool,
        statement: &Statement,
        block: Ref<Block>,

        arena: &Arena<Statement>,
        variables: &mut BTreeMap<InternedString, LocalVariable<A>, A>,
    ) -> Result<StatementResult<A>, Error> {
        log::trace!("translate stmt: {statement:?}");

        Ok(match statement {
            Statement::Constant(value) => {
                let typ = emit_rudder_constant_type(value, &value.typ());
                StatementResult::Data(Some(match value {
                    Constant::UnsignedInteger { value, .. } => self.emitter.constant(*value, typ),
                    Constant::SignedInteger { value, .. } => {
                        self.emitter.constant(*value as u64, typ)
                    }
                    Constant::FloatingPoint { value, .. } => {
                        self.emitter.constant(*value as u64, typ)
                    }

                    Constant::String(_) => self
                        .emitter
                        .constant(0xDEAD5555, emitter::Type::Unsigned(32)),

                    Constant::Tuple(_) => {
                        // let Type::Tuple(types) = &typ else { panic!() };
                        // let values = values
                        //     .iter()
                        //     .cloned()
                        //     .zip(types.iter().cloned())
                        //     .map(|(value, typ)| codegen_constant_value(value, typ));
                        // ((#(#values),*))
                        todo!("tuple")
                    }
                    Constant::Vector(_) => {
                        // let Type::Tuple(types) = &typ else { panic!() };
                        // let values = values
                        //     .iter()
                        //     .cloned()
                        //     .zip(types.iter().cloned())
                        //     .map(|(value, typ)| codegen_constant_value(value, typ));
                        // ((#(#values),*))
                        todo!("tuple")
                    }
                }))
            }
            Statement::ReadVariable { symbol } => {
                let Some(var) = variables.get(&symbol.name()).cloned() else {
                    panic!(
                        "attempted to read {} in block {:#x} in {:?} before it was written to",
                        symbol.name(),
                        block.index(),
                        self.function.name()
                    )
                };

                log::trace!(
                    "reading var {} in block {:#x} in {:?} = {:?}",
                    symbol.name(),
                    block.index(),
                    self.function.name(),
                    var
                );

                StatementResult::Data(Some(self.read_variable(var)))
            }
            Statement::WriteVariable { symbol, value } => {
                log::trace!(
                    "writing var {} in block (dynamic={is_dynamic}) {:#x} in {:?}, value: {value:#?}",
                    symbol.name(),
                    block.index(),
                    self.function.name()
                );

                let variable = variables.entry(symbol.name()).or_insert_with(|| {
                    log::trace!("writing var {} for the first time", symbol.name());
                    LocalVariable::default()
                });

                if is_dynamic
                // terrible hack to workaround ldp     x0, x21, [x0] bug, where x0 gets written to halfway through, thus corrupting the second read of x0 to write *(x0 + 8) to x21
                    || (symbol.name().as_ref() == "address"
                        && self.function.name().as_ref()
                            == "execute_aarch64_instrs_memory_pair_general_post_idx")
                {
                    // if we're in a dynamic block and the local variable is not on the
                    // stack, put it there
                    match variable {
                        LocalVariable::Virtual { .. } => {
                            log::trace!(
                                "promoting {:?} from virtual to stack in block {:#x} in {:?}",
                                symbol.name(),
                                block.index(),
                                self.function.name(),
                            );

                            let stack_offset =
                                if let Some(offset) = self.promoted_locations.get(&symbol.name()) {
                                    log::trace!(
                                        "variable {:?} already promoted to stack @ {offset:#x}",
                                        symbol.name()
                                    );
                                    *offset
                                } else {
                                    let offset = self.allocate_stack_offset(&symbol.typ());
                                    self.promoted_locations.insert(symbol.name(), offset);

                                    log::trace!(
                                        "variable {:?} promoted to stack @ {offset:#x}",
                                        symbol.name()
                                    );

                                    offset
                                };

                            *variable = LocalVariable::Stack {
                                typ: emit_rudder_type(&symbol.typ()),
                                stack_offset,
                            };

                            // clears operands??? todo: understand this
                            let current_block = self.emitter.get_current_block();
                            self.emitter.set_current_block(current_block);
                        }
                        LocalVariable::Stack { stack_offset, .. } => {
                            log::debug!(
                                "local var {:?} already on stack @ {:#x} in block {:#x} in {:?}",
                                symbol.name(),
                                stack_offset,
                                block.index(),
                                self.function.name()
                            );
                        }
                    }
                }

                let value = value_store.get(*value);

                self.write_variable(variable, value);

                StatementResult::Data(None)
            }
            Statement::ReadRegister { typ, offset } => {
                let offset = match value_store.get(*offset).kind() {
                    NodeKind::Constant { value, .. } => *value,
                    k => panic!("can't read non constant offset: {k:#?}"),
                };

                let name = self
                    .model
                    .get_register_by_offset(offset)
                    .unwrap_or_else(|| panic!("no register found for offset {offset}"));

                let typ = emit_rudder_type(typ);

                match self.model.registers().get(&name).unwrap().cache {
                    RegisterCacheType::Constant
                    | RegisterCacheType::Read
                    | RegisterCacheType::ReadWrite => {
                        let offset = usize::try_from(offset).unwrap();
                        let value = match typ.width() {
                            1..=8 => u64::from(self.register_file.read_raw::<u8>(offset)),
                            9..=16 => u64::from(self.register_file.read_raw::<u16>(offset)),
                            17..=32 => u64::from(self.register_file.read_raw::<u32>(offset)),
                            33..=64 => u64::from(self.register_file.read_raw::<u64>(offset)),
                            w => todo!("width {w}"),
                        };
                        log::trace!("read from cacheable {name:?}: {value:x}");
                        StatementResult::Data(Some(self.emitter.constant(value, typ)))
                    }
                    RegisterCacheType::None => {
                        StatementResult::Data(Some(self.emitter.read_register(offset, typ)))
                    }
                }
            }
            Statement::WriteRegister { offset, value } => {
                let offset = match value_store.get(*offset).kind() {
                    NodeKind::Constant { value, .. } => *value,
                    k => panic!("can't write non constant offset: {k:#?}"),
                };

                let name = self
                    .model
                    .get_register_by_offset(offset)
                    .unwrap_or_else(|| panic!("no register found for offset {offset}"));

                assert_eq!(offset, self.model.registers().get(&name).unwrap().offset);

                let value = value_store.get(*value);

                if name.as_ref() == "current_exception_tag" {
                    if let NodeKind::Constant { value, .. } = value.kind() {
                        // SEE exception
                        if *value == 5 {
                            return Err(Error::Decode);
                        }
                    }
                }

                // if cacheable and writing a constant, update the register file during
                // translation
                match self.model.registers().get(&name).unwrap().cache {
                    RegisterCacheType::Constant => {
                        panic!("cannot write to constant register {name:?}")
                    }
                    RegisterCacheType::ReadWrite => {
                        log::trace!("attempting write to cacheable {name:?}: {value:?}");
                        if let NodeKind::Constant { value, width } = value.kind() {
                            match width {
                                1..=8 => self.register_file.write::<u8>(name, (*value) as u8),
                                9..=16 => self.register_file.write::<u16>(name, (*value) as u16),
                                17..=32 => self.register_file.write::<u32>(name, (*value) as u32),
                                33..=64 => self.register_file.write::<u64>(name, *value),
                                w => todo!("width {w}"),
                            }

                            log::trace!("wrote to cacheable {name:?}: {value:x}");

                            StatementResult::Data(None)
                        } else {
                            panic!(
                                "attempting to write non-constant value to cacheable register {name:?}"
                            );
                        }
                    }
                    RegisterCacheType::None | RegisterCacheType::Read => {
                        // otherwise emit a write register that will mutate the register file during
                        // execution
                        self.emitter.write_register(offset, value);
                        StatementResult::Data(None)
                    }
                }
            }
            Statement::ReadMemory { address, size } => {
                let address = value_store.get(*address);
                let size = value_store.get(*size);

                let NodeKind::Constant { value, .. } = size.kind() else {
                    panic!("expected constant got {:#?}", size.kind());
                };

                let typ = match value {
                    1 => Type::Unsigned(8),
                    2 => Type::Unsigned(16),
                    4 => Type::Unsigned(32),
                    8 => Type::Unsigned(64),
                    16 => Type::Unsigned(128),
                    _ => todo!("{value}"),
                };

                StatementResult::Data(Some(self.emitter.read_memory(address, typ)))
            }
            Statement::WriteMemory { address, value } => {
                let address = value_store.get(*address);
                let value = value_store.get(*value);
                self.emitter.write_memory(address, value);
                StatementResult::Data(None)
            }
            Statement::ReadPc => todo!(),
            Statement::WritePc { value } => {
                self.emitter.ctx_mut().set_pc_write_flag();

                let offset = self.emitter.ctx().pc_offset() as u64;
                let value = value_store.get(*value);

                self.emitter.write_register(offset, value);
                StatementResult::Data(None)
            }
            Statement::GetFlags { operation } => {
                let operation = value_store.get(*operation);
                StatementResult::Data(Some(self.emitter.get_flags(operation)))
            }
            Statement::UnaryOperation { kind, value } => {
                use {
                    crate::dbt::x86::emitter::UnaryOperationKind as EmitterOp,
                    rudder::statement::UnaryOperationKind as RudderOp,
                };

                let value = value_store.get(*value);

                let op = match kind {
                    RudderOp::Not => EmitterOp::Not(value),
                    RudderOp::Negate => EmitterOp::Negate(value),
                    RudderOp::Complement => EmitterOp::Complement(value),
                    RudderOp::Power2 => EmitterOp::Power2(value),
                    RudderOp::Absolute => EmitterOp::Absolute(value),
                    RudderOp::Ceil => EmitterOp::Ceil(value),
                    RudderOp::Floor => EmitterOp::Floor(value),
                    RudderOp::SquareRoot => EmitterOp::SquareRoot(value),
                };

                StatementResult::Data(Some(self.emitter.unary_operation(op)))
            }
            Statement::BinaryOperation { kind, lhs, rhs } => {
                use {
                    crate::dbt::x86::emitter::BinaryOperationKind as EmitterOp,
                    rudder::statement::BinaryOperationKind as RudderOp,
                };

                let lhs = value_store.get(*lhs);
                let rhs = value_store.get(*rhs);

                let op = match kind {
                    RudderOp::Add => EmitterOp::Add(lhs, rhs),
                    RudderOp::Sub => EmitterOp::Sub(lhs, rhs),
                    RudderOp::Multiply => EmitterOp::Multiply(lhs, rhs),
                    RudderOp::Divide => EmitterOp::Divide(lhs, rhs),
                    RudderOp::Modulo => EmitterOp::Modulo(lhs, rhs),
                    RudderOp::And => EmitterOp::And(lhs, rhs),
                    RudderOp::Or => EmitterOp::Or(lhs, rhs),
                    RudderOp::Xor => EmitterOp::Xor(lhs, rhs),
                    RudderOp::PowI => EmitterOp::PowI(lhs, rhs),
                    RudderOp::CompareEqual => EmitterOp::CompareEqual(lhs, rhs),
                    RudderOp::CompareNotEqual => EmitterOp::CompareNotEqual(lhs, rhs),
                    RudderOp::CompareLessThan => EmitterOp::CompareLessThan(lhs, rhs),
                    RudderOp::CompareLessThanOrEqual => EmitterOp::CompareLessThanOrEqual(lhs, rhs),
                    RudderOp::CompareGreaterThan => EmitterOp::CompareGreaterThan(lhs, rhs),
                    RudderOp::CompareGreaterThanOrEqual => {
                        EmitterOp::CompareGreaterThanOrEqual(lhs, rhs)
                    }
                };

                StatementResult::Data(Some(
                    self.emitter.binary_operation(op), /* .unwrap_or_else(|e| {
                                                        *     panic!("{}::{}: {e}",
                                                        * self.function.name(), block.index())
                                                        * }), */
                ))
            }
            Statement::TernaryOperation { kind, a, b, c } => {
                use {
                    crate::dbt::x86::emitter::TernaryOperationKind as EmitterOp,
                    rudder::statement::TernaryOperationKind as RudderOp,
                };

                let a = value_store.get(*a);
                let b = value_store.get(*b);
                let c = value_store.get(*c);

                let op = match kind {
                    RudderOp::AddWithCarry => EmitterOp::AddWithCarry(a, b, c),
                };

                StatementResult::Data(Some(self.emitter.ternary_operation(op)))
            }
            Statement::ShiftOperation {
                kind,
                value,
                amount,
            } => {
                use {
                    crate::dbt::x86::emitter::ShiftOperationKind as EmitterOp,
                    rudder::statement::ShiftOperationKind as RudderOp,
                };

                let value = value_store.get(*value);
                let amount = value_store.get(*amount);

                let op = match kind {
                    RudderOp::LogicalShiftLeft => EmitterOp::LogicalShiftLeft,
                    RudderOp::LogicalShiftRight => EmitterOp::LogicalShiftRight,
                    RudderOp::ArithmeticShiftRight => EmitterOp::ArithmeticShiftRight,
                    RudderOp::RotateRight => EmitterOp::RotateRight,
                    RudderOp::RotateLeft => EmitterOp::RotateLeft,
                };

                StatementResult::Data(Some(self.emitter.shift(value, amount, op)))
            }

            Statement::Call { target, args, .. } => {
                let args = args.iter().map(|a| value_store.get(*a)).collect::<Vec<_>>();

                if target.as_ref() == "sail_tlbi" {
                    self.emitter.execution_result = ExecutionResult::NeedTLBInvalidate;
                }

                StatementResult::Data(
                    FunctionTranslator::new(
                        self.allocator.clone(),
                        self.model,
                        target.as_ref(),
                        &args,
                        self.emitter,
                        self.current_stack_offset.clone(), /* pass in the current stack offset
                                                            * so
                                                            * called functions' stack variables
                                                            * don't corrupt this function's */
                        self.register_file,
                    )
                    .translate()?,
                )
            }
            Statement::Jump { target } => {
                // make new empty x86 block
                let x86 = self.emitter.ctx_mut().create_block();

                self.static_blocks
                    .entry(*target)
                    .and_modify(|blocks| blocks.push(x86))
                    .or_insert(alloc::vec![x86]);

                self.emitter.jump(x86);
                StatementResult::ControlFlow(ControlFlow::Jump(*target, x86, variables.clone()))
            }
            Statement::Branch {
                condition,
                true_target,
                false_target,
            } => {
                let condition = value_store.get(*condition);

                // todo: obviously refactor this to re-use jump logic

                return match condition.kind() {
                    NodeKind::Constant { value, .. } => {
                        if *value == 0 {
                            let x86 = self.emitter.ctx_mut().create_block();

                            self.static_blocks
                                .entry(*false_target)
                                .and_modify(|blocks| blocks.push(x86))
                                .or_insert(alloc::vec![x86]);

                            self.emitter.jump(x86);
                            Ok(StatementResult::ControlFlow(ControlFlow::Jump(
                                *false_target,
                                x86,
                                variables.clone(),
                            )))
                        } else {
                            let x86 = self.emitter.ctx_mut().create_block();

                            self.static_blocks
                                .entry(*true_target)
                                .and_modify(|blocks| blocks.push(x86))
                                .or_insert(alloc::vec![x86]);

                            self.emitter.jump(x86);
                            Ok(StatementResult::ControlFlow(ControlFlow::Jump(
                                *true_target,
                                x86,
                                variables.clone(),
                            )))
                        }
                    }
                    _ => {
                        let true_x86 = (*self
                            .dynamic_blocks
                            .entry((*true_target, variables.clone()))
                            .or_insert_with(|| (self.emitter.ctx_mut().create_block(), false)))
                        .0;
                        let false_x86 = (*self
                            .dynamic_blocks
                            .entry((*false_target, variables.clone()))
                            .or_insert_with(|| (self.emitter.ctx_mut().create_block(), false)))
                        .0;
                        self.emitter.branch(condition, true_x86, false_x86);
                        Ok(StatementResult::ControlFlow(ControlFlow::Branch(
                            *true_target,
                            *false_target,
                            variables.clone(),
                        )))
                    }
                };
            }
            Statement::Return { value } => {
                if let Some(value) = value {
                    let value = value_store.get(*value);
                    self.write_return_value(value);
                }

                StatementResult::ControlFlow(ControlFlow::Return)
            }

            Statement::Cast { kind, typ, value } => {
                use {
                    crate::dbt::x86::emitter::CastOperationKind as EmitterOp,
                    rudder::statement::CastOperationKind as RudderOp,
                };

                let value = value_store.get(*value);
                let typ = emit_rudder_type(typ);

                let kind = match kind {
                    RudderOp::ZeroExtend => EmitterOp::ZeroExtend,
                    RudderOp::SignExtend => EmitterOp::SignExtend,
                    RudderOp::Truncate => EmitterOp::Truncate,
                    RudderOp::Reinterpret => EmitterOp::Reinterpret,
                    RudderOp::Convert => EmitterOp::Convert,
                    RudderOp::Broadcast => EmitterOp::Broadcast,
                };

                StatementResult::Data(Some(self.emitter.cast(value, typ, kind)))
            }
            Statement::BitsCast {
                kind,
                typ,
                value,
                width,
            } => {
                use {
                    crate::dbt::x86::emitter::CastOperationKind as EmitterOp,
                    rudder::statement::CastOperationKind as RudderOp,
                };

                let value = value_store.get(*value);
                let width = value_store.get(*width);
                let typ = emit_rudder_type(typ);

                let kind = match kind {
                    RudderOp::ZeroExtend => EmitterOp::ZeroExtend,
                    RudderOp::SignExtend => EmitterOp::SignExtend,
                    RudderOp::Truncate => EmitterOp::Truncate,
                    RudderOp::Reinterpret => EmitterOp::Reinterpret,
                    RudderOp::Convert => EmitterOp::Convert,
                    RudderOp::Broadcast => EmitterOp::Broadcast,
                };

                StatementResult::Data(Some(self.emitter.bits_cast(value, width, typ, kind)))
            }

            Statement::PhiNode { .. } => todo!(),

            Statement::Select {
                condition,
                true_value,
                false_value,
            } => {
                let condition = value_store.get(*condition);
                let true_value = value_store.get(*true_value);
                let false_value = value_store.get(*false_value);
                StatementResult::Data(Some(self.emitter.select(
                    condition,
                    true_value,
                    false_value,
                )))
            }
            Statement::BitExtract {
                value,
                start,
                width,
            } => {
                let value = value_store.get(*value);
                let start = value_store.get(*start);
                let width = value_store.get(*width);
                StatementResult::Data(Some(self.emitter.bit_extract(value, start, width)))
            }
            Statement::BitInsert {
                target,
                source,
                start,
                width,
            } => {
                let target = value_store.get(*target);
                let source = value_store.get(*source);
                let start = value_store.get(*start);
                let width = value_store.get(*width);
                StatementResult::Data(Some(self.emitter.bit_insert(target, source, start, width)))
            }
            Statement::ReadElement { .. } => {
                todo!()
            }
            Statement::AssignElement {
                vector,
                value,
                index,
            } => {
                let vector = value_store.get(*vector);
                let value = value_store.get(*value);
                let index = value_store.get(*index);
                StatementResult::Data(Some(self.emitter.mutate_element(vector, index, value)))
            }
            Statement::Panic(value) => {
                let Statement::Constant(Constant::String(msg)) = value.get(arena) else {
                    todo!();
                };

                self.emitter.panic(msg.as_ref());

                StatementResult::ControlFlow(ControlFlow::Panic)
            }

            Statement::Assert { condition } => {
                let mut meta = (self.function.name().key() as u64) << 32;
                meta |= (block.index() as u64 & 0xFFFF) << 16;
                meta |= (condition.index() as u64) & 0xFFFF;

                let condition = value_store.get(*condition);
                self.emitter.assert(condition, meta);
                StatementResult::Data(None)
            }
            Statement::CreateBits { value, width } => {
                let value = value_store.get(*value);
                let width = value_store.get(*width);
                StatementResult::Data(Some(self.emitter.create_bits(value, width)))
            }
            Statement::SizeOf { value } => {
                let value = value_store.get(*value);
                StatementResult::Data(Some(self.emitter.size_of(value)))
            }
            Statement::MatchesUnion { .. } => todo!(),
            Statement::UnwrapUnion { .. } => todo!(),
            Statement::CreateTuple(values) => {
                let mut tuple = Vec::new_in(self.allocator);
                values
                    .iter()
                    .map(|v| value_store.get(*v))
                    .collect_into(&mut tuple);
                StatementResult::Data(Some(self.emitter.create_tuple(tuple)))
            }
            Statement::TupleAccess { index, source } => {
                let source = value_store.get(*source);
                StatementResult::Data(Some(self.emitter.access_tuple(source, *index)))
            }
        })
    }

    fn read_variable(&mut self, variable: LocalVariable<A>) -> X86NodeRef<A> {
        match variable {
            LocalVariable::Virtual { value } => {
                value.expect("local virtual variable never written to")
            }
            LocalVariable::Stack { stack_offset, typ } => {
                let read = self.emitter.read_stack_variable(stack_offset, typ);

                if matches!(typ, Type::Bits) {
                    let width = *self.bits_stack_widths.get(&stack_offset).unwrap();
                    self.emitter.cast(
                        read,
                        Type::Unsigned(width),
                        super::x86::emitter::CastOperationKind::Truncate,
                    )
                } else {
                    read
                }
            }
        }
    }

    fn write_variable(&mut self, variable: &mut LocalVariable<A>, new_value: X86NodeRef<A>) {
        match variable {
            LocalVariable::Virtual { value } => *value = Some(new_value),
            LocalVariable::Stack { typ, stack_offset } => {
                if matches!(typ, Type::Bits) {
                    // no panic even if we tried to write two different sizes to the stack :(
                    // this relies on the depth first block translation order
                    // if we see any issues, we need to actually support writing different sizes to
                    // stack, using an extra stack variable containing the size
                    self.bits_stack_widths
                        .insert(*stack_offset, new_value.typ().width());
                }
                self.emitter.write_stack_variable(*stack_offset, new_value)
            }
        }
    }

    fn allocate_stack_offset(&self, typ: &rudder::types::Type) -> usize {
        assert!(typ.width_bytes() <= 8);
        let width = 8;
        let offset = self
            .current_stack_offset
            .fetch_add(width, Ordering::Relaxed);

        let next_offset = self.current_stack_offset.load(Ordering::Relaxed);

        if next_offset >= MAX_STACK_SIZE {
            panic!("stack offset {next_offset:#x} exceeded MAX_STACK_SIZE ({MAX_STACK_SIZE:#x})")
        }

        offset
    }
}

/// Converts a rudder type to a `Type` value
fn emit_rudder_constant_type(value: &Constant, typ: &rudder::types::Type) -> emitter::Type {
    match typ {
        rudder::types::Type::Bits => {
            let Constant::UnsignedInteger { value: cv, .. } = value else {
                panic!();
            };

            let width = unsigned_smallest_width_of_value(*cv);

            emitter::Type::Unsigned(width)
        }
        rudder::types::Type::String => emitter::Type::Unsigned(32),
        _ => emit_rudder_type(typ),
    }
}

/// Converts a rudder type to a `Type` value
fn emit_rudder_type(typ: &rudder::types::Type) -> emitter::Type {
    match typ {
        rudder::types::Type::Primitive(primitive) => match *primitive {
            PrimitiveType::UnsignedInteger(width) => emitter::Type::Unsigned(width),
            PrimitiveType::SignedInteger(width) => emitter::Type::Signed(width),
            PrimitiveType::FloatingPoint(width) => emitter::Type::Floating(width),
        },
        rudder::types::Type::Bits => emitter::Type::Bits,
        rudder::types::Type::Tuple(_) => emitter::Type::Tuple,
        t => panic!("todo codegen type instance: {t:?}"),
    }
}

/// Stores the intermediate value of translated statements for use by future
/// statements
///
/// Tried linear search vec but same perf
struct StatementValueStore<A: Alloc> {
    map: HashMapA<Ref<Statement>, X86NodeRef<A>, A>,
}

impl<A: Alloc> StatementValueStore<A> {
    pub fn new(allocator: A) -> Self {
        Self {
            map: hashmap_in(allocator),
        }
    }

    pub fn insert(&mut self, s: Ref<Statement>, v: X86NodeRef<A>) {
        self.map.insert(s, v);
    }

    pub fn get(&self, s: Ref<Statement>) -> X86NodeRef<A> {
        self.map.get(&s).unwrap().clone()
    }
}
