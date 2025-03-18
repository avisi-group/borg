use {
    crate::dbt::{
        Alloc,
        emitter::{self, Emitter, Type},
        trampoline::MAX_STACK_SIZE,
        x86::{
            emitter::{NodeKind, X86Block, X86Emitter, X86NodeRef, X86SymbolRef},
            encoder::Instruction,
        },
    },
    alloc::{collections::BTreeMap, rc::Rc, vec::Vec},
    common::{
        arena::{Arena, Ref},
        intern::InternedString,
        hashmap::{HashMapA, hashmap_in},
        rudder::{
            self, Model, RegisterCacheType, block::Block, constant_value::ConstantValue,
            function::Function, statement::Statement, types::PrimitiveType,
        },
        width_helpers::unsigned_smallest_width_of_value,
    },
    core::{
        alloc::Allocator,
        cmp::max,
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

pub fn translate<A: Alloc>(
    allocator: A,
    model: &Model,
    function: &str,
    arguments: &[X86NodeRef<A>],
    emitter: &mut X86Emitter<A>,
    register_file_ptr: *mut u8,
) -> Option<X86NodeRef<A>> {
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
        register_file_ptr,
    )
    .translate()
}

#[derive(Clone)]
#[derive_where(Debug)]
enum LocalVariable<A: Alloc> {
    Virtual {
        symbol: X86SymbolRef<A>,
    },
    Stack {
        typ: emitter::Type,
        stack_offset: usize,
    },
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

        let mut variables = Vec::new_in(emitter.ctx().allocator());

        for _ in 0..num_variables {
            variables.push(LocalVariable::Virtual {
                symbol: emitter.ctx_mut().create_symbol(),
            });
        }

        Self {
            variables,
            previous_write: None,
        }
    }
}

struct FunctionTranslator<'model, 'emitter, 'context, A: Alloc> {
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

    // don't re-promote to a different location
    promoted_locations: HashMapA<InternedString, usize, A>,

    return_value: ReturnValue<A>,

    /// Stack offset used to allocate stack variables
    current_stack_offset: Rc<AtomicUsize, A>,

    /// X86 instruction emitter
    emitter: &'emitter mut X86Emitter<'context, A>,

    /// Pointer to the register file used for cached register reads
    register_file_ptr: *mut u8,
}

impl<'m, 'e, 'c, A: Alloc> FunctionTranslator<'m, 'e, 'c, A> {
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
                        let LocalVariable::Virtual { symbol } = local_variable else {
                            panic!()
                        };

                        let prev_value = (symbol.0.borrow()).clone().unwrap();

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

        values
            .into_iter()
            .zip(self.return_value.variables.clone().into_iter())
            .for_each(|(value, variable)| {
                self.write_variable(variable, value);
            });

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
        register_file_ptr: *mut u8,
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
            return_value: ReturnValue::new(emitter, function.return_type()),
            current_stack_offset,
            emitter,
            register_file_ptr,
        };

        // set up symbols for parameters, and write arguments into them
        function
            .parameters()
            .iter()
            .zip(arguments)
            .for_each(|(parameter, argument)| {
                let var = LocalVariable::Virtual {
                    symbol: celf.emitter.ctx_mut().create_symbol(),
                };
                celf.entry_variables.insert(parameter.name(), var.clone());
                celf.write_variable(var, argument.clone());
            });

        celf
    }

    fn translate(&mut self) -> Option<X86NodeRef<A>> {
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
                    let res = self.translate_block(rudder_block, false, variables);
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

                    let res = self.translate_block(rudder_block, true, variables);
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

        self.read_return_value()
    }

    fn translate_block(
        &mut self,

        block_ref: Ref<Block>,
        is_dynamic: bool,
        mut variables: BTreeMap<InternedString, LocalVariable<A>, A>,
    ) -> ControlFlow<A> {
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
            ) {
                StatementResult::Data(Some(value)) => {
                    // log::trace!(
                    //     "{} {} = {:?}",
                    //     s,
                    //     s.get(block.arena()).to_string(block.arena()),
                    //     value.kind(),
                    // );
                    statement_value_store.insert(*s, value);
                }
                StatementResult::Data(None) => {
                    // log::trace!(
                    //     "{} {} = ()",
                    //     s,
                    //     s.get(block.arena()).to_string(block.arena()),
                    // );
                }
                StatementResult::ControlFlow(block_result) => {
                    // log::trace!(
                    //     "{} {} = {:?}",
                    //     s,
                    //     s.get(block.arena()).to_string(block.arena()),
                    //     block_result
                    // );
                    return block_result;
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
        statement_values: &StatementValueStore<A>,
        is_dynamic: bool,
        statement: &Statement,
        block: Ref<Block>,

        arena: &Arena<Statement>,
        variables: &mut BTreeMap<InternedString, LocalVariable<A>, A>,
    ) -> StatementResult<A> {
        //        log::debug!("translate stmt: {statement:?}");

        match statement {
            Statement::Constant { typ, value } => {
                let typ = emit_rudder_constant_type(value, typ);
                StatementResult::Data(Some(match value {
                    ConstantValue::UnsignedInteger(v) => self.emitter.constant(*v, typ),
                    ConstantValue::SignedInteger(v) => self.emitter.constant(*v as u64, typ),
                    ConstantValue::FloatingPoint(v) => self.emitter.constant(*v as u64, typ),

                    ConstantValue::String(_) => self
                        .emitter
                        .constant(0xDEAD5555, emitter::Type::Unsigned(32)),

                    ConstantValue::Tuple(_) => {
                        // let Type::Tuple(types) = &typ else { panic!() };
                        // let values = values
                        //     .iter()
                        //     .cloned()
                        //     .zip(types.iter().cloned())
                        //     .map(|(value, typ)| codegen_constant_value(value, typ));
                        // ((#(#values),*))
                        todo!("tuple")
                    }
                    ConstantValue::Vector(_) => {
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
                    "writing var {} in block (dynamic={is_dynamic}) {:#x} in {:?}",
                    symbol.name(),
                    block.index(),
                    self.function.name()
                );

                if variables.get(&symbol.name()).is_none() {
                    log::trace!("writing var {} for the first time", symbol.name());
                    variables.insert(
                        symbol.name(),
                        LocalVariable::Virtual {
                            symbol: self.emitter.ctx_mut().create_symbol(),
                        },
                    );
                }

                if is_dynamic {
                    // if we're in a dynamic block and the local variable is not on the
                    // stack, put it there
                    match variables.get(&symbol.name()).unwrap() {
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

                            variables.insert(
                                symbol.name(),
                                LocalVariable::Stack {
                                    typ: emit_rudder_type(&symbol.typ()),
                                    stack_offset,
                                },
                            );

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
                let var = variables.get(&symbol.name()).unwrap().clone();

                let value = statement_values
                    .get(*value)
                    .unwrap_or_else(|| {
                        panic!(
                            "no value for {value} when writing to {symbol:?} in {} {block:?}",
                            self.function.name()
                        )
                    })
                    .clone();

                self.write_variable(var, value);

                StatementResult::Data(None)
            }
            Statement::ReadRegister { typ, offset } => {
                let offset = match statement_values.get(*offset).unwrap().kind() {
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
                        let value = unsafe {
                            let ptr = self.register_file_ptr.add(usize::try_from(offset).unwrap());

                            match typ.width() {
                                1..=8 => u64::from((ptr as *const u8).read()),
                                9..=16 => u64::from((ptr as *const u16).read()),
                                17..=32 => u64::from((ptr as *const u32).read()),
                                33..=64 => u64::from((ptr as *const u64).read()),
                                w => todo!("width {w}"),
                            }
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
                let offset = match statement_values.get(*offset).unwrap().kind() {
                    NodeKind::Constant { value, .. } => *value,
                    k => panic!("can't write non constant offset: {k:#?}"),
                };

                let name = self
                    .model
                    .get_register_by_offset(offset)
                    .unwrap_or_else(|| panic!("no register found for offset {offset}"));

                let value = statement_values.get(*value).unwrap().clone();

                // if cacheable and writing a constant, update the register file during
                // translation
                match self.model.registers().get(&name).unwrap().cache {
                    RegisterCacheType::Constant => {
                        panic!("cannot write to constant register {name:?}")
                    }
                    RegisterCacheType::ReadWrite => {
                        log::trace!("attempting write to cacheable {name:?}: {value:?}");
                        if let NodeKind::Constant { value, width } = value.kind() {
                            unsafe {
                                let ptr =
                                    self.register_file_ptr.add(usize::try_from(offset).unwrap());

                                match width {
                                    1..=8 => (ptr as *mut u8).write(*value as u8),
                                    9..=16 => (ptr as *mut u16).write(*value as u16),
                                    17..=32 => (ptr as *mut u32).write(*value as u32),
                                    33..=64 => (ptr as *mut u64).write(*value),
                                    w => todo!("width {w}"),
                                }
                            };

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
                let address = statement_values.get(*address).unwrap().clone();
                let size = statement_values.get(*size).unwrap().clone();

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
                let address = statement_values.get(*address).unwrap().clone();
                let value = statement_values.get(*value).unwrap().clone();
                self.emitter.write_memory(address, value);
                StatementResult::Data(None)
            }
            Statement::ReadPc => todo!(),
            Statement::WritePc { value } => {
                self.emitter.ctx_mut().set_pc_write_flag();

                let offset = self.emitter.ctx().pc_offset() as u64;
                let value = statement_values.get(*value).unwrap().clone();

                self.emitter.write_register(offset, value);
                StatementResult::Data(None)
            }
            Statement::GetFlags { operation } => {
                let operation = statement_values.get(*operation).unwrap().clone();
                StatementResult::Data(Some(self.emitter.get_flags(operation)))
            }
            Statement::UnaryOperation { kind, value } => {
                use {
                    crate::dbt::x86::emitter::UnaryOperationKind as EmitterOp,
                    rudder::statement::UnaryOperationKind as RudderOp,
                };

                let value = statement_values.get(*value).unwrap().clone();

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

                let lhs = statement_values.get(*lhs).unwrap().clone();
                let rhs = statement_values.get(*rhs).unwrap().clone();

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

                let a = statement_values.get(*a).unwrap().clone();
                let b = statement_values.get(*b).unwrap().clone();
                let c = statement_values.get(*c).unwrap().clone();

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

                let value = statement_values.get(*value).unwrap().clone();
                let amount = statement_values.get(*amount).unwrap().clone();

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
                let args = args
                    .iter()
                    .map(|a| statement_values.get(*a).unwrap())
                    .collect::<Vec<_>>();

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
                        self.register_file_ptr,
                    )
                    .translate(),
                )
            }
            Statement::Jump { target } => {
                // make new empty x86 block
                let x86 = self
                    .emitter
                    .ctx_mut()
                    .arena_mut()
                    .insert(X86Block::new_in(self.allocator.clone()));

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
                let condition = statement_values.get(*condition).unwrap().clone();

                // todo: obviously refactor this to re-use jump logic

                return match condition.kind() {
                    NodeKind::Constant { value, .. } => {
                        if *value == 0 {
                            let x86 = self
                                .emitter
                                .ctx_mut()
                                .arena_mut()
                                .insert(X86Block::new_in(self.allocator.clone()));

                            self.static_blocks
                                .entry(*false_target)
                                .and_modify(|blocks| blocks.push(x86))
                                .or_insert(alloc::vec![x86]);

                            self.emitter.jump(x86);
                            StatementResult::ControlFlow(ControlFlow::Jump(
                                *false_target,
                                x86,
                                variables.clone(),
                            ))
                        } else {
                            let x86 = self
                                .emitter
                                .ctx_mut()
                                .arena_mut()
                                .insert(X86Block::new_in(self.allocator.clone()));

                            self.static_blocks
                                .entry(*true_target)
                                .and_modify(|blocks| blocks.push(x86))
                                .or_insert(alloc::vec![x86]);

                            self.emitter.jump(x86);
                            StatementResult::ControlFlow(ControlFlow::Jump(
                                *true_target,
                                x86,
                                variables.clone(),
                            ))
                        }
                    }
                    _ => {
                        let true_x86 = (*self
                            .dynamic_blocks
                            .entry((*true_target, variables.clone()))
                            .or_insert_with(|| {
                                (
                                    self.emitter
                                        .ctx_mut()
                                        .arena_mut()
                                        .insert(X86Block::new_in(self.allocator.clone())),
                                    false,
                                )
                            }))
                        .0;
                        let false_x86 = (*self
                            .dynamic_blocks
                            .entry((*false_target, variables.clone()))
                            .or_insert_with(|| {
                                (
                                    self.emitter
                                        .ctx_mut()
                                        .arena_mut()
                                        .insert(X86Block::new_in(self.allocator.clone())),
                                    false,
                                )
                            }))
                        .0;
                        self.emitter.branch(condition, true_x86, false_x86);
                        StatementResult::ControlFlow(ControlFlow::Branch(
                            *true_target,
                            *false_target,
                            variables.clone(),
                        ))
                    }
                };
            }
            Statement::Return { value } => {
                if let Some(value) = value {
                    let value = statement_values.get(*value).unwrap();
                    self.write_return_value(value);
                }

                StatementResult::ControlFlow(ControlFlow::Return)
            }

            Statement::Cast { kind, typ, value } => {
                use {
                    crate::dbt::x86::emitter::CastOperationKind as EmitterOp,
                    rudder::statement::CastOperationKind as RudderOp,
                };

                let value = statement_values.get(*value).unwrap();
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

                let value = statement_values.get(*value).unwrap().clone();
                let width = statement_values.get(*width).unwrap().clone();
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
                let condition = statement_values.get(*condition).unwrap().clone();
                let true_value = statement_values.get(*true_value).unwrap().clone();
                let false_value = statement_values.get(*false_value).unwrap().clone();
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
                let value = statement_values.get(*value).unwrap().clone();
                let start = statement_values.get(*start).unwrap().clone();
                let width = statement_values.get(*width).unwrap().clone();
                StatementResult::Data(Some(self.emitter.bit_extract(value, start, width)))
            }
            Statement::BitInsert {
                target,
                source,
                start,
                width,
            } => {
                let target = statement_values.get(*target).unwrap().clone();
                let source = statement_values.get(*source).unwrap().clone();
                let start = statement_values.get(*start).unwrap().clone();
                let width = statement_values.get(*width).unwrap().clone();
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
                let vector = statement_values.get(*vector).unwrap().clone();
                let value = statement_values.get(*value).unwrap().clone();
                let index = statement_values.get(*index).unwrap().clone();
                StatementResult::Data(Some(self.emitter.mutate_element(vector, index, value)))
            }
            Statement::Panic(value) => {
                let Statement::Constant {
                    value: ConstantValue::String(msg),
                    ..
                } = value.get(arena)
                else {
                    todo!();
                };

                self.emitter.panic(msg.as_ref());

                StatementResult::ControlFlow(ControlFlow::Panic)
            }

            Statement::Assert { condition } => {
                let mut meta = (self.function.name().key() as u64) << 32;
                meta |= (block.index() as u64 & 0xFFFF) << 16;
                meta |= (condition.index() as u64) & 0xFFFF;

                let condition = statement_values.get(*condition).unwrap().clone();
                self.emitter.assert(condition, meta);
                StatementResult::Data(None)
            }
            Statement::CreateBits { value, width } => {
                let value = statement_values.get(*value).unwrap().clone();
                let width = statement_values.get(*width).unwrap().clone();
                StatementResult::Data(Some(self.emitter.create_bits(value, width)))
            }
            Statement::SizeOf { value } => {
                let value = statement_values.get(*value).unwrap().clone();
                StatementResult::Data(Some(self.emitter.size_of(value)))
            }
            Statement::MatchesUnion { .. } => todo!(),
            Statement::UnwrapUnion { .. } => todo!(),
            Statement::CreateTuple(values) => {
                let mut tuple = Vec::new_in(self.allocator);
                values
                    .iter()
                    .map(|v| statement_values.get(*v).unwrap())
                    .collect_into(&mut tuple);
                StatementResult::Data(Some(self.emitter.create_tuple(tuple)))
            }
            Statement::TupleAccess { index, source } => {
                let source = statement_values.get(*source).unwrap().clone();
                StatementResult::Data(Some(self.emitter.access_tuple(source, *index)))
            }
        }
    }

    fn read_variable(&mut self, variable: LocalVariable<A>) -> X86NodeRef<A> {
        match variable {
            LocalVariable::Virtual { symbol } => self.emitter.read_virt_variable(symbol),
            LocalVariable::Stack { stack_offset, typ } => {
                self.emitter.read_stack_variable(stack_offset, typ)
            }
        }
    }

    fn write_variable(&mut self, variable: LocalVariable<A>, value: X86NodeRef<A>) {
        match variable {
            LocalVariable::Virtual { symbol } => self.emitter.write_virt_variable(symbol, value),
            LocalVariable::Stack {
                typ: _,
                stack_offset,
            } => self.emitter.write_stack_variable(stack_offset, value),
        }
    }

    fn allocate_stack_offset(&self, typ: &rudder::types::Type) -> usize {
        let width = max(usize::from(typ.width_bytes()), 8);
        let offset = self
            .current_stack_offset
            .fetch_add(width, Ordering::Relaxed);

        let next_offset = self.current_stack_offset.load(Ordering::Relaxed);

        if next_offset >= MAX_STACK_SIZE {
            panic!("stack offset {next_offset:#x} exceeded MAX_STACK_SIZE ({MAX_STACK_SIZE:#x})")
        }

        offset
    }

    // fn get_x86(
    //     &mut self,
    //     rudder: Ref<Block>,
    //     live_ins: BTreeMap<InternedString, bool>,
    // ) -> Ref<X86Block> {
    //     (*self.translated_blocks.get(&(rudder, live_ins)).unwrap()).0
    // }

    // fn get_or_insert_x86(
    //     &mut self,
    //     rudder: Ref<Block>,
    //     live_ins: BTreeMap<InternedString, bool>,
    // ) -> Ref<X86Block> {
    //     (*self
    //         .translated_blocks
    //         .entry((rudder, live_ins))
    //         .or_insert_with(|| {
    //             (
    //                 self.emitter.ctx_mut().arena_mut().insert(X86Block::new()),
    //                 false,
    //             )
    //         }))
    //     .0
    // }
}

/// Converts a rudder type to a `Type` value
fn emit_rudder_constant_type(value: &ConstantValue, typ: &rudder::types::Type) -> emitter::Type {
    match typ {
        rudder::types::Type::Bits => {
            let ConstantValue::UnsignedInteger(cv) = value else {
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

    pub fn get(&self, s: Ref<Statement>) -> Option<X86NodeRef<A>> {
        self.map.get(&s).cloned()
    }
}
