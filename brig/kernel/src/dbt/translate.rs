use {
    crate::dbt::{
        emitter::{self, BlockResult, Emitter, Type},
        x86::{
            emitter::{NodeKind, X86Block, X86Emitter, X86NodeRef, X86SymbolRef},
            encoder::Instruction,
        },
    },
    alloc::vec::Vec,
    common::{
        arena::{Arena, Ref},
        intern::InternedString,
        rudder::{
            self, block::Block, constant_value::ConstantValue, function::Function,
            statement::Statement, types::PrimitiveTypeClass, Model,
        },
        width_helpers::unsigned_smallest_width_of_value,
        HashMap, HashSet,
    },
};

const BLOCK_QUEUE_LIMIT: usize = 1000;

/// Kind of jump to a target block
#[derive(Debug)]
enum JumpKind {
    // static jump (jump or branch with constant condition)
    Static(Ref<Block>, Ref<X86Block>),
    // branch with non-constant condition
    Dynamic(Ref<Block>),
}

enum StatementResult {
    Data(Option<X86NodeRef>),
    ControlFlow(BlockResult),
}

pub fn translate(
    model: &Model,
    function: &str,
    arguments: &[X86NodeRef],
    emitter: &mut X86Emitter,
) -> Option<X86NodeRef> {
    // x86_64 has full descending stack so current stack offset needs to start at 8
    // for first stack variable offset to point to the next empty slot
    let current_stack_offset = 8;
    FunctionTranslator::new(model, function, arguments, emitter, current_stack_offset).translate()
}

#[derive(Debug, Clone)]
enum LocalVariable {
    Virtual {
        symbol: X86SymbolRef,
    },
    Stack {
        typ: emitter::Type,
        stack_offset: usize,
    },
}

struct FunctionTranslator<'m, 'e, 'c> {
    model: &'m Model,
    function_name: InternedString,
    x86_blocks: HashMap<Ref<Block>, Ref<X86Block>>,
    rudder_blocks: HashMap<Ref<X86Block>, Ref<Block>>,
    variables: HashMap<InternedString, LocalVariable>,
    current_stack_offset: usize,
    emitter: &'e mut X86Emitter<'c>,
}

impl<'m, 'e, 'c> FunctionTranslator<'m, 'e, 'c> {
    fn new(
        model: &'m Model,
        function: &str,
        arguments: &[X86NodeRef],
        emitter: &'e mut X86Emitter<'c>,
        current_stack_offset: usize,
    ) -> Self {
        log::debug!("translating {function:?}");

        let mut celf = Self {
            model,
            function_name: InternedString::from(function),
            x86_blocks: HashMap::default(),
            rudder_blocks: HashMap::default(),
            variables: HashMap::default(),
            current_stack_offset,
            emitter,
        };

        let function = celf
            .model
            .functions()
            .get(&celf.function_name)
            .unwrap_or_else(|| panic!("function named {function:?} not found"));

        // set up symbols for local variables
        let locals = function.local_variables();

        locals.iter().map(|sym| sym.name()).for_each(|name| {
            celf.variables.insert(
                name,
                LocalVariable::Virtual {
                    symbol: celf.emitter.ctx().create_symbol(),
                },
            );
        });

        // set up symbols for parameters, and write arguments into them
        function
            .parameters()
            .iter()
            .zip(arguments)
            .for_each(|(parameter, argument)| {
                let var = LocalVariable::Virtual {
                    symbol: celf.emitter.ctx().create_symbol(),
                };
                celf.variables.insert(parameter.name(), var.clone());
                celf.write_variable(var, argument.clone());
            });

        // and the return value
        celf.variables.insert(
            "borealis_fn_return_value".into(),
            LocalVariable::Virtual {
                symbol: celf.emitter.ctx().create_symbol(),
            },
        );

        // set up block maps
        function.block_iter().for_each(|rudder_block| {
            let x86_block = celf.emitter.ctx().create_block();
            celf.rudder_blocks.insert(x86_block.clone(), rudder_block);
            celf.x86_blocks.insert(rudder_block, x86_block);
        });

        log::trace!("blocks: {:#?}", celf.x86_blocks);

        celf
    }

    fn translate(&mut self) -> Option<X86NodeRef> {
        let function = self
            .model
            .functions()
            .get(&self.function_name)
            .unwrap_or_else(|| panic!("failed to find function {:?} in model", self.function_name));

        // create an empty block all control flow will end at
        let exit_block = self.emitter.ctx().arena_mut().insert(X86Block::new());

        // create an empty entry block for this function
        let entry_x86 = self.emitter.ctx().arena_mut().insert(X86Block::new());

        // jump from the *current* emitter block to this function's entry block
        self.emitter.append(Instruction::jmp(entry_x86));
        self.emitter.add_target(entry_x86);

        let mut block_queue = alloc::collections::VecDeque::new();

        // start translation of the function's rudder entry block to the new (empty) X86
        // entry block
        block_queue.push_front(JumpKind::Static(function.entry_block(), entry_x86));

        let mut visited_dynamic_blocks = HashSet::default();

        while let Some(block) = block_queue.pop_front() {
            if block_queue.len() > BLOCK_QUEUE_LIMIT {
                panic!("block queue exceeded limit")
            }

            let result = match block {
                JumpKind::Static(rudder_block, x86_block) => {
                    self.emitter.set_current_block(x86_block);
                    log::trace!(
                        "translating static block rudder={rudder_block:?}, x86={x86_block:?}",
                    );
                    let res = self.translate_block(function, rudder_block, false);
                    log::trace!("emitted: {:?}", x86_block.get(self.emitter.ctx().arena()));
                    res
                }
                JumpKind::Dynamic(b) => {
                    log::trace!("dynamic block {}", b.index());

                    if visited_dynamic_blocks.contains(&b) {
                        log::trace!("already visited");
                        continue;
                    }

                    let x86_block = *self.x86_blocks.get(&b).unwrap();
                    self.emitter.set_current_block(x86_block);

                    log::trace!("translating dynamic block rudder={b:?}, x86={x86_block:?}",);

                    let res = self.translate_block(function, b, true);
                    log::trace!("emitted: {:?}", x86_block.get(self.emitter.ctx().arena()));
                    visited_dynamic_blocks.insert(b);
                    res
                }
            };

            match result {
                BlockResult::Static(x86) => {
                    let rudder = self.lookup_rudder_block(x86);
                    log::trace!("block result: static(rudder={rudder:?},x86={x86:?})",);
                    block_queue.push_front(JumpKind::Static(rudder, x86));
                }
                BlockResult::Dynamic(b0, b1) => {
                    let block0 = self.lookup_rudder_block(b0);
                    let block1 = self.lookup_rudder_block(b1);
                    log::trace!(
                        "block result: dynamic({}, {})",
                        block0.index(),
                        block1.index()
                    );
                    block_queue.push_back(JumpKind::Dynamic(block0));
                    block_queue.push_back(JumpKind::Dynamic(block1));
                }
                BlockResult::Return => {
                    log::trace!("block result: return ({exit_block:?})");
                    self.emitter.jump(exit_block);
                }
                BlockResult::Panic => {
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

        log::trace!("queue empty, reading return value and exiting");

        if function.return_type().is_some() {
            Some(
                self.read_variable(
                    self.variables
                        .get(&InternedString::from_static("borealis_fn_return_value"))
                        .unwrap()
                        .clone(),
                ),
            )
        } else {
            None
        }
    }

    fn translate_block(
        &mut self,
        function: &Function,
        block_ref: Ref<Block>,
        is_dynamic: bool,
    ) -> BlockResult {
        let block = block_ref.get(function.arena());

        let mut statement_values = HashMap::<Ref<Statement>, X86NodeRef>::default();

        for s in block.statements() {
            match self.translate_statement(
                &statement_values,
                is_dynamic,
                s.get(block.arena()),
                block_ref,
                function,
                block.arena(),
            ) {
                StatementResult::Data(Some(value)) => {
                    statement_values.insert(*s, value);
                }
                StatementResult::Data(None) => (),
                StatementResult::ControlFlow(block_result) => return block_result,
            }
        }

        unreachable!(
            "last statement in block should have returned a control flow statement result in {:?}",
            function.name()
        )
    }

    // todo: fix these parameters this is silly
    fn translate_statement(
        &mut self,
        statement_values: &HashMap<Ref<Statement>, X86NodeRef>,
        is_dynamic: bool,
        statement: &Statement,
        block: Ref<Block>,
        function: &Function,
        arena: &Arena<Statement>,
    ) -> StatementResult {
        match statement {
            Statement::Constant { typ, value } => {
                let typ = emit_rudder_constant_type(value, typ);
                StatementResult::Data(Some(match value {
                    ConstantValue::UnsignedInteger(v) => self.emitter.constant(*v, typ),
                    ConstantValue::SignedInteger(v) => self.emitter.constant(*v as u64, typ),
                    ConstantValue::FloatingPoint(v) => self.emitter.constant(*v as u64, typ),

                    ConstantValue::String(s) => self
                        .emitter
                        .constant(s.key().into(), emitter::Type::Unsigned(32)),

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
                }))
            }
            Statement::ReadVariable { symbol } => {
                log::trace!("reading var {}", symbol.name());
                let var = self.variables.get(&symbol.name()).unwrap().clone();
                StatementResult::Data(Some(self.read_variable(var)))
            }
            Statement::WriteVariable { symbol, value } => {
                log::trace!("writing var {}", symbol.name());
                if is_dynamic {
                    // if we're in a dynamic block and the local variable is not on the
                    // stack, put it there
                    if let LocalVariable::Virtual { .. } =
                        self.variables.get(&symbol.name()).unwrap()
                    {
                        log::trace!("upgrading {:?} from virtual to stack", symbol.name());
                        self.variables.insert(
                            symbol.name(),
                            LocalVariable::Stack {
                                typ: emit_rudder_type(&symbol.typ()),
                                stack_offset: self.current_stack_offset,
                            },
                        );
                        self.current_stack_offset += symbol.typ().width_bytes();
                    }
                }

                let var = self.variables.get(&symbol.name()).unwrap().clone();

                let value = statement_values
                    .get(value)
                    .unwrap_or_else(|| {
                        panic!(
                            "no value for {value} when writing to {symbol:?} in {} {block:?}",
                            function.name()
                        )
                    })
                    .clone();

                self.write_variable(var, value);

                StatementResult::Data(None)
            }
            Statement::ReadRegister { typ, offset } => {
                let offset = statement_values.get(offset).unwrap().clone();
                let typ = emit_rudder_type(typ);
                StatementResult::Data(Some(self.emitter.read_register(offset, typ)))
            }
            Statement::WriteRegister { offset, value } => {
                let offset = statement_values.get(offset).unwrap().clone();
                let value = statement_values.get(value).unwrap().clone();
                self.emitter.write_register(offset, value);
                StatementResult::Data(None)
            }
            Statement::ReadMemory { .. } => {
                // {
                //     let mut buf = alloc::vec![0; #size as usize / 8];
                //     state.read_memory(#offset, &mut buf);

                //     let mut bytes = [0u8; 16];
                //     bytes[..buf.len()].copy_from_slice(&buf);

                //     Bits::new(u128::from_ne_bytes(bytes), #size as u16)
                // }
                todo!()
            }
            Statement::WriteMemory { .. } => {
                // match &value.get(s_arena).typ(s_arena) {
                //     Type::Primitive(PrimitiveType { .. }) => {
                //         quote! {
                //             state.write_memory(#offset, &#value.to_ne_bytes())
                //         }
                //     }
                //     Type::Bits => {
                //         quote! {
                //             state.write_memory(#offset,
                // &#value.value().to_ne_bytes()[..#value.length() as usize / 8])
                //         }
                //     }
                //     _ => todo!(),
                // }
                todo!()
            }
            Statement::ReadPc => todo!(),
            Statement::WritePc { value } => {
                self.emitter.ctx().set_write_pc();

                let pc_offset = self.emitter.ctx().pc_offset() as u64;

                let offset = self.emitter.constant(pc_offset, Type::Unsigned(64));
                let value = statement_values.get(value).unwrap().clone();
                self.emitter.write_register(offset, value);
                StatementResult::Data(None)
            }
            Statement::GetFlags { operation } => {
                let operation = statement_values.get(operation).unwrap().clone();
                StatementResult::Data(Some(self.emitter.get_flags(operation)))
            }
            Statement::UnaryOperation { kind, value } => {
                use {
                    crate::dbt::x86::emitter::UnaryOperationKind as EmitterOp,
                    rudder::statement::UnaryOperationKind as RudderOp,
                };

                let value = statement_values.get(value).unwrap().clone();

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

                let lhs = statement_values.get(lhs).unwrap().clone();
                let rhs = statement_values.get(rhs).unwrap().clone();

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

                StatementResult::Data(Some(self.emitter.binary_operation(op)))
            }
            Statement::TernaryOperation { kind, a, b, c } => {
                use {
                    crate::dbt::x86::emitter::TernaryOperationKind as EmitterOp,
                    rudder::statement::TernaryOperationKind as RudderOp,
                };

                let a = statement_values.get(a).unwrap().clone();
                let b = statement_values.get(b).unwrap().clone();
                let c = statement_values.get(c).unwrap().clone();

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

                let value = statement_values.get(value).unwrap().clone();
                let amount = statement_values.get(amount).unwrap().clone();

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
                    .map(|a| statement_values.get(a).unwrap())
                    .cloned()
                    .collect::<Vec<_>>();

                StatementResult::Data(
                    FunctionTranslator::new(
                        self.model,
                        target.as_ref(),
                        &args,
                        self.emitter,
                        self.current_stack_offset, /* pass in the current stack offset so
                                                    * called functions' stack variables
                                                    * don't corrupt this function's */
                    )
                    .translate(),
                )
            }
            Statement::Jump { target } => {
                // make new empty x86 block
                let x86 = self.emitter.ctx().arena_mut().insert(X86Block::new());
                self.rudder_blocks.insert(x86, *target);
                StatementResult::ControlFlow(self.emitter.jump(x86))
            }
            Statement::Branch {
                condition,
                true_target,
                false_target,
            } => {
                let condition = statement_values.get(condition).unwrap().clone();

                // todo: obviously refactor this to re-use jump logic

                return match condition.kind() {
                    NodeKind::Constant { value, .. } => {
                        if *value == 0 {
                            let x86 = self.emitter.ctx().arena_mut().insert(X86Block::new());
                            self.rudder_blocks.insert(x86, *false_target);
                            StatementResult::ControlFlow(self.emitter.jump(x86))
                        } else {
                            let x86 = self.emitter.ctx().arena_mut().insert(X86Block::new());
                            self.rudder_blocks.insert(x86, *true_target);
                            StatementResult::ControlFlow(self.emitter.jump(x86))
                        }
                    }
                    _ => {
                        let true_target = self.lookup_x86_block(*true_target);
                        let false_target = self.lookup_x86_block(*false_target);
                        StatementResult::ControlFlow(self.emitter.branch(
                            condition,
                            true_target,
                            false_target,
                        ))
                    }
                };
            }
            Statement::Return { value } => {
                if let Some(value) = value {
                    log::trace!("writing var borealis_fn_return_value");

                    let var = self
                        .variables
                        .get(&InternedString::from_static("borealis_fn_return_value"))
                        .unwrap()
                        .clone();

                    let value = statement_values.get(value).cloned().unwrap();
                    self.write_variable(var, value);
                }

                StatementResult::ControlFlow(BlockResult::Return)
            }

            Statement::Cast { kind, typ, value } => {
                use {
                    crate::dbt::x86::emitter::CastOperationKind as EmitterOp,
                    rudder::statement::CastOperationKind as RudderOp,
                };

                let value = statement_values.get(value).unwrap().clone();
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
                length,
            } => {
                use {
                    crate::dbt::x86::emitter::CastOperationKind as EmitterOp,
                    rudder::statement::CastOperationKind as RudderOp,
                };

                let value = statement_values.get(value).unwrap().clone();
                let length = statement_values.get(length).unwrap().clone();
                let typ = emit_rudder_type(typ);

                let kind = match kind {
                    RudderOp::ZeroExtend => EmitterOp::ZeroExtend,
                    RudderOp::SignExtend => EmitterOp::SignExtend,
                    RudderOp::Truncate => EmitterOp::Truncate,
                    RudderOp::Reinterpret => EmitterOp::Reinterpret,
                    RudderOp::Convert => EmitterOp::Convert,
                    RudderOp::Broadcast => EmitterOp::Broadcast,
                };

                StatementResult::Data(Some(self.emitter.bits_cast(value, length, typ, kind)))
            }

            Statement::PhiNode { .. } => todo!(),

            Statement::Select {
                condition,
                true_value,
                false_value,
            } => {
                let condition = statement_values.get(condition).unwrap().clone();
                let true_value = statement_values.get(true_value).unwrap().clone();
                let false_value = statement_values.get(false_value).unwrap().clone();
                StatementResult::Data(Some(self.emitter.select(
                    condition,
                    true_value,
                    false_value,
                )))
            }
            Statement::BitExtract {
                value,
                start,
                length,
            } => {
                let value = statement_values.get(value).unwrap().clone();
                let start = statement_values.get(start).unwrap().clone();
                let length = statement_values.get(length).unwrap().clone();
                StatementResult::Data(Some(self.emitter.bit_extract(value, start, length)))
            }
            Statement::BitInsert {
                target,
                source,
                start,
                length,
            } => {
                let target = statement_values.get(target).unwrap().clone();
                let source = statement_values.get(source).unwrap().clone();
                let start = statement_values.get(start).unwrap().clone();
                let length = statement_values.get(length).unwrap().clone();
                StatementResult::Data(Some(self.emitter.bit_insert(target, source, start, length)))
            }
            Statement::ReadElement { .. } => {
                todo!()
            }
            Statement::AssignElement {
                vector,
                value,
                index,
            } => {
                let vector = statement_values.get(vector).unwrap().clone();
                let value = statement_values.get(value).unwrap().clone();
                let index = statement_values.get(index).unwrap().clone();
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

                StatementResult::ControlFlow(BlockResult::Panic)
            }

            Statement::Assert { condition } => {
                let condition = statement_values.get(condition).unwrap().clone();
                self.emitter.assert(condition);
                StatementResult::Data(None)
            }
            Statement::CreateBits { value, length } => {
                let value = statement_values.get(value).unwrap().clone();
                let length = statement_values.get(length).unwrap().clone();
                StatementResult::Data(Some(self.emitter.create_bits(value, length)))
            }
            Statement::SizeOf { value } => {
                let value = statement_values.get(value).unwrap().clone();
                StatementResult::Data(Some(self.emitter.size_of(value)))
            }
            Statement::MatchesUnion { .. } => todo!(),
            Statement::UnwrapUnion { .. } => todo!(),
            Statement::CreateTuple(values) => {
                let values = values
                    .iter()
                    .map(|v| statement_values.get(v).unwrap())
                    .cloned()
                    .collect();
                StatementResult::Data(Some(self.emitter.create_tuple(values)))
            }
            Statement::TupleAccess { index, source } => {
                let source = statement_values.get(source).unwrap().clone();
                StatementResult::Data(Some(self.emitter.access_tuple(source, *index)))
            }
        }
    }

    fn lookup_x86_block(&self, rudder: Ref<Block>) -> Ref<X86Block> {
        self.x86_blocks.get(&rudder).unwrap().clone()
    }

    fn lookup_rudder_block(&self, x86: Ref<X86Block>) -> Ref<Block> {
        *self.rudder_blocks.get(&x86).unwrap()
    }

    fn read_variable(&mut self, variable: LocalVariable) -> X86NodeRef {
        match variable {
            LocalVariable::Virtual { symbol } => self.emitter.read_virt_variable(symbol),
            LocalVariable::Stack { stack_offset, typ } => {
                self.emitter.read_stack_variable(stack_offset, typ)
            }
        }
    }

    fn write_variable(&mut self, variable: LocalVariable, value: X86NodeRef) {
        match variable {
            LocalVariable::Virtual { symbol } => self.emitter.write_virt_variable(symbol, value),
            LocalVariable::Stack {
                typ: _,
                stack_offset,
            } => self.emitter.write_stack_variable(stack_offset, value),
        }
    }
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
        rudder::types::Type::Primitive(primitive) => {
            let width = u16::try_from(primitive.width()).unwrap();
            match primitive.tc {
                PrimitiveTypeClass::UnsignedInteger => emitter::Type::Unsigned(width),
                PrimitiveTypeClass::SignedInteger => emitter::Type::Signed(width),
                PrimitiveTypeClass::FloatingPoint => emitter::Type::Floating(width),
            }
        }
        rudder::types::Type::Bits => emitter::Type::Bits,
        rudder::types::Type::Tuple(_) => emitter::Type::Tuple,
        t => panic!("todo codegen type instance: {t:?}"),
    }
}
