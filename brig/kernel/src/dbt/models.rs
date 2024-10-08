use {
    crate::{
        dbt::{
            emitter::{self, BlockResult, Emitter},
            x86::{
                emitter::{X86BlockRef, X86NodeRef, X86SymbolRef},
                encoder::{Instruction, Operand, PhysicalRegister},
                X86TranslationContext, ENTRY_BLOCK_ID, EXIT_BLOCK_ID,
            },
            TranslationContext,
        },
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
    },
    alloc::{
        borrow::ToOwned, collections::btree_map::BTreeMap, string::String, sync::Arc, vec::Vec,
    },
    common::{
        arena::Ref,
        intern::InternedString,
        rudder::{
            self,
            block::Block,
            constant_value::ConstantValue,
            function::{Function, Symbol},
            statement::Statement,
            types::PrimitiveTypeClass,
            Model,
        },
        width_helpers::{signed_smallest_width_of_value, unsigned_smallest_width_of_value},
        HashMap, HashSet,
    },
    spin::Mutex,
};

const BLOCK_QUEUE_LIMIT: usize = 1000;

static MODEL_MANAGER: Mutex<BTreeMap<String, Arc<Model>>> = Mutex::new(BTreeMap::new());

pub fn register_model(name: &str, model: Model) {
    log::info!("registering {name:?} ISA model");
    MODEL_MANAGER
        .lock()
        .insert(name.to_owned(), Arc::new(model));
}

pub fn get(name: &str) -> Option<Arc<Model>> {
    MODEL_MANAGER.lock().get(name).cloned()
}

pub fn load_all(device: &SharedDevice) {
    common::intern::init(Default::default());

    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());

    log::info!("loading models");
    // todo: don't hardcode this, load all .postcards?
    ["aarch64.postcard"]
        .into_iter()
        .map(|path| {
            (
                path.strip_suffix(".postcard").unwrap(),
                fs.open(path).unwrap().read_to_vec().unwrap(),
            )
        })
        .map(|(name, data)| (name, postcard::from_bytes(&data).unwrap()))
        .for_each(|(name, model)| {
            register_model(name, model);
        });
}

pub fn execute(
    model: &Model,
    function: &str,
    arguments: &[X86NodeRef],
    ctx: &mut X86TranslationContext,
) -> X86NodeRef {
    FunctionExecutor::new(model, function, arguments, ctx).translate()
}

struct FunctionExecutor<'m, 'c> {
    model: &'m Model,
    function_name: InternedString,
    x86_blocks: HashMap<Ref<Block>, X86BlockRef>,
    rudder_blocks: HashMap<X86BlockRef, Ref<Block>>,
    //local_variables: HashMap<InternedString, X86SymbolRef>,
    stack_variables: HashMap<InternedString, (emitter::Type, usize)>, // type and stack offset
    stack_size: usize,
    exit_block_ref: X86BlockRef,
    ctx: &'c mut X86TranslationContext,
}

impl<'m, 'c> FunctionExecutor<'m, 'c> {
    fn new(
        model: &'m Model,
        function: &str,
        arguments: &[X86NodeRef],
        ctx: &'c mut X86TranslationContext,
    ) -> Self {
        let mut celf = Self {
            model,
            function_name: InternedString::from(function),
            x86_blocks: HashMap::default(),
            rudder_blocks: HashMap::default(),
            // local_variables: HashMap::default(),
            stack_variables: HashMap::default(),
            stack_size: 0,
            exit_block_ref: ctx.create_block(EXIT_BLOCK_ID),
            ctx,
        };

        let function = celf
            .model
            .functions()
            .get(&celf.function_name)
            .unwrap_or_else(|| panic!("function named {function:?} not found"));

        // set up symbols for local variables
        let locals = function.local_variables();

        locals
            .iter()
            .map(|sym| (sym.name(), sym.typ()))
            .for_each(|(name, typ)| {
                celf.stack_variables
                    .insert(name, (emit_rudder_type(&typ), celf.stack_size));
                celf.stack_size += typ.width_bytes();
            });

        // set up symbols for parameters, and write arguments into them
        function
            .parameters()
            .iter()
            .zip(arguments)
            .for_each(|(parameter, argument)| {
                celf.stack_variables.insert(
                    parameter.name(),
                    (emit_rudder_type(&parameter.typ()), celf.stack_size),
                );
                celf.ctx
                    .emitter()
                    .write_variable(celf.stack_size, argument.clone());
                celf.stack_size += parameter.typ().width_bytes();
            });

        // and the return value
        {
            celf.stack_variables.insert(
                "borealis_fn_return_value".into(),
                (emit_rudder_type(&function.return_type()), celf.stack_size),
            );
            celf.stack_size += function.return_type().width_bytes();
        }

        log::debug!("stack_variables: {:?}", celf.stack_variables);

        // set up block maps
        function.block_iter().for_each(|rudder_block| {
            let x86_block = celf
                .ctx
                .create_block(i32::try_from(rudder_block.index()).unwrap());
            celf.rudder_blocks.insert(x86_block.clone(), rudder_block);
            celf.x86_blocks.insert(rudder_block, x86_block);
        });

        celf
    }

    fn translate(&mut self) -> X86NodeRef {
        let function = self
            .model
            .functions()
            .get(&self.function_name)
            .unwrap_or_else(|| panic!("failed to find function {:?} in model", self.function_name));

        // todo: write arguments

        #[derive(Debug)]
        enum BlockKind {
            Static(Ref<Block>),
            Dynamic(Ref<Block>),
        }

        {
            // insert stack setup prologue to entry block and stack teardown epilogue to
            // exit block
            //
            // push rbp
            // mov  rbp, rsp
            // sub  rsp, TOTAL_SIZE
            let x86_entry_block = self.ctx.initial_block();
            x86_entry_block.append(Instruction::push(Operand::preg(64, PhysicalRegister::RBP)));
            x86_entry_block.append(Instruction::mov(
                Operand::preg(64, PhysicalRegister::RSP),
                Operand::preg(64, PhysicalRegister::RBP),
            ));
            x86_entry_block.append(Instruction::sub(
                Operand::imm(32, self.stack_size as u64),
                Operand::preg(64, PhysicalRegister::RSP),
            ));
        }

        let mut block_queue = alloc::vec![BlockKind::Static(function.entry_block())];
        let mut visited_dynamic_blocks = HashSet::default();

        while let Some(block) = block_queue.pop() {
            if block_queue.len() > BLOCK_QUEUE_LIMIT {
                panic!(
                    "block queue exceeded limit, head: {:?}",
                    &block_queue[BLOCK_QUEUE_LIMIT - 10..]
                )
            }

            let result = match block {
                BlockKind::Static(b) => {
                    log::trace!("static block {}", b.index());
                    self.translate_block(function, b)
                }
                BlockKind::Dynamic(b) => {
                    if visited_dynamic_blocks.contains(&b) {
                        continue;
                    }

                    log::trace!("dynamic block {}", b.index());
                    self.ctx
                        .emitter()
                        .set_current_block(self.x86_blocks.get(&b).unwrap().clone());

                    let res = self.translate_block(function, b);
                    visited_dynamic_blocks.insert(b);
                    res
                }
            };

            match result {
                BlockResult::Static(block) => {
                    let block = *self.rudder_blocks.get(&block).unwrap();
                    log::trace!("block result: static({})", block.index());
                    block_queue.push(BlockKind::Static(block));
                }
                BlockResult::Dynamic(b0, b1) => {
                    let block0 = *self.rudder_blocks.get(&b0).unwrap();
                    let block1 = *self.rudder_blocks.get(&b1).unwrap();
                    log::trace!(
                        "block result: dynamic({}, {})",
                        block0.index(),
                        block1.index()
                    );
                    // will be popped in order 0,1
                    block_queue.push(BlockKind::Dynamic(block1));
                    block_queue.push(BlockKind::Dynamic(block0));
                }
                BlockResult::Return => {
                    log::trace!("block result: return");
                    self.ctx.emitter().jump(self.exit_block_ref.clone());
                }
                BlockResult::Panic => {
                    log::trace!("block result: panic");
                    // unreachable but inserted just to make sure *every* block has a path to the
                    // exit block
                    self.ctx.emitter().jump(self.exit_block_ref.clone());
                }
            }
        }

        self.ctx
            .emitter()
            .set_current_block(self.exit_block_ref.clone());

        {
            // insert stack teardown epilogue to exit block
            //
            // mov     rsp, rbp
            // pop     rbp
            self.exit_block_ref.append(Instruction::mov(
                Operand::preg(64, PhysicalRegister::RBP),
                Operand::preg(64, PhysicalRegister::RSP),
            ));
            self.exit_block_ref
                .append(Instruction::pop(Operand::preg(64, PhysicalRegister::RBP)));
        }

        log::trace!("queue empty, reading return value and exiting");

        // well damn I can't return a value on the stack that I just invalidated
        let (typ, offset) = self
            .stack_variables
            .get(&InternedString::from_static("borealis_fn_return_value"))
            .unwrap()
            .clone();
        return self.ctx.emitter().read_variable(offset, typ);
    }

    fn translate_block(&mut self, function: &Function, block_ref: Ref<Block>) -> BlockResult {
        let block = block_ref.get(function.arena());

        let mut statement_values = HashMap::<Ref<Statement>, X86NodeRef>::default();

        for s in block.statements() {
            let value = match s.get(block.arena()) {
                Statement::Constant { typ, value } => {
                    let typ = emit_rudder_constant_type(value, typ);
                    Some(match value {
                        ConstantValue::UnsignedInteger(v) => self.ctx.emitter().constant(*v, typ),
                        ConstantValue::SignedInteger(v) => {
                            self.ctx.emitter().constant(*v as u64, typ)
                        }
                        ConstantValue::FloatingPoint(v) => {
                            self.ctx.emitter().constant(*v as u64, typ)
                        }
                        ConstantValue::Unit => self.ctx.emitter().constant(0, typ),
                        ConstantValue::String(s) => self
                            .ctx
                            .emitter()
                            .constant(s.key().into(), emitter::Type::Unsigned(32)),
                        ConstantValue::Rational(_) => todo!(),

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
                    })
                }
                Statement::ReadVariable { symbol } => {
                    let (typ, offset) = self.lookup_symbol(symbol);
                    Some(self.ctx.emitter().read_variable(offset, typ))
                }
                Statement::WriteVariable { symbol, value } => {
                    let (_, offset) = self.lookup_symbol(symbol);
                    let value = statement_values
                        .get(value)
                        .unwrap_or_else(|| {
                            panic!(
                                "no value for {value} when writing to {symbol:?} in {} {block_ref:?}",
                                function.name()
                            )
                        })
                        .clone();
                    self.ctx.emitter().write_variable(offset, value);
                    None
                }
                Statement::ReadRegister { typ, offset } => {
                    let offset = statement_values.get(offset).unwrap().clone();
                    let typ = emit_rudder_type(typ);
                    Some(self.ctx.emitter().read_register(offset, typ))
                }
                Statement::WriteRegister { offset, value } => {
                    let offset = statement_values.get(offset).unwrap().clone();
                    let value = statement_values.get(value).unwrap().clone();
                    self.ctx.emitter().write_register(offset, value);
                    None
                }
                Statement::ReadMemory { offset, size } => {
                    // {
                    //     let mut buf = alloc::vec![0; #size as usize / 8];
                    //     state.read_memory(#offset, &mut buf);

                    //     let mut bytes = [0u8; 16];
                    //     bytes[..buf.len()].copy_from_slice(&buf);

                    //     Bits::new(u128::from_ne_bytes(bytes), #size as u16)
                    // }
                    todo!()
                }
                Statement::WriteMemory { offset, value } => {
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
                Statement::WritePc { .. } => todo!(),
                Statement::GetFlags => Some(self.ctx.emitter().get_flags()),
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

                    Some(self.ctx.emitter().unary_operation(op))
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
                        RudderOp::CompareLessThanOrEqual => {
                            EmitterOp::CompareLessThanOrEqual(lhs, rhs)
                        }
                        RudderOp::CompareGreaterThan => EmitterOp::CompareGreaterThan(lhs, rhs),
                        RudderOp::CompareGreaterThanOrEqual => {
                            EmitterOp::CompareGreaterThanOrEqual(lhs, rhs)
                        }
                    };

                    Some(self.ctx.emitter().binary_operation(op))
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

                    Some(self.ctx.emitter().ternary_operation(op))
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

                    Some(self.ctx.emitter().shift(value, amount, op))
                }

                Statement::Call { target, args, .. } => {
                    let args = args
                        .iter()
                        .map(|a| statement_values.get(a).unwrap())
                        .cloned()
                        .collect::<Vec<_>>();

                    Some(execute(self.model, target.as_ref(), &args, self.ctx))
                }
                Statement::Jump { target } => {
                    let target = self.lookup_x86_block(target);
                    return self.ctx.emitter().jump(target);
                }
                Statement::Branch {
                    condition,
                    true_target,
                    false_target,
                } => {
                    let condition = statement_values.get(condition).unwrap().clone();
                    let true_target = self.lookup_x86_block(true_target);
                    let false_target = self.lookup_x86_block(false_target);

                    return self
                        .ctx
                        .emitter()
                        .branch(condition, true_target, false_target);
                }
                Statement::Return { value } => {
                    let value = statement_values.get(value).unwrap().clone();
                    let (_, offset) = self
                        .stack_variables
                        .get(&InternedString::from_static("borealis_fn_return_value"))
                        .unwrap()
                        .clone();
                    self.ctx.emitter().write_variable(offset, value);
                    return BlockResult::Return;
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

                    Some(self.ctx.emitter().cast(value, typ, kind))
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

                    Some(self.ctx.emitter().bits_cast(value, length, typ, kind))
                }

                Statement::PhiNode { members } => todo!(),

                Statement::Select {
                    condition,
                    true_value,
                    false_value,
                } => {
                    let condition = statement_values.get(condition).unwrap().clone();
                    let true_value = statement_values.get(true_value).unwrap().clone();
                    let false_value = statement_values.get(false_value).unwrap().clone();
                    Some(
                        self.ctx
                            .emitter()
                            .select(condition, true_value, false_value),
                    )
                }
                Statement::BitExtract {
                    value,
                    start,
                    length,
                } => {
                    let value = statement_values.get(value).unwrap().clone();
                    let start = statement_values.get(start).unwrap().clone();
                    let length = statement_values.get(length).unwrap().clone();
                    Some(self.ctx.emitter().bit_extract(value, start, length))
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
                    Some(self.ctx.emitter().bit_insert(target, source, start, length))
                }
                Statement::ReadElement { vector, index } => {
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
                    Some(self.ctx.emitter().mutate_element(vector, index, value))
                }
                Statement::Panic(value) => {
                    let Statement::Constant {
                        value: ConstantValue::String(msg),
                        ..
                    } = value.get(block.arena())
                    else {
                        todo!();
                    };

                    self.ctx.emitter().panic(msg.as_ref());
                    return BlockResult::Panic;
                }
                Statement::Undefined => todo!(),
                Statement::Assert { condition } => {
                    let condition = statement_values.get(condition).unwrap().clone();
                    self.ctx.emitter().assert(condition);
                    None
                }
                Statement::CreateBits { value, length } => {
                    let value = statement_values.get(value).unwrap().clone();
                    let length = statement_values.get(length).unwrap().clone();
                    Some(self.ctx.emitter().create_bits(value, length))
                }
                Statement::SizeOf { value } => {
                    let value = statement_values.get(value).unwrap().clone();
                    Some(self.ctx.emitter().size_of(value))
                }
                Statement::MatchesUnion { value, variant } => todo!(),
                Statement::UnwrapUnion { value, variant } => todo!(),
                Statement::CreateTuple(values) => {
                    let values = values
                        .iter()
                        .map(|v| statement_values.get(v).unwrap())
                        .cloned()
                        .collect();
                    Some(self.ctx.emitter().create_tuple(values))
                }
                Statement::TupleAccess { index, source } => {
                    let source = statement_values.get(source).unwrap().clone();
                    Some(self.ctx.emitter().access_tuple(source, *index))
                }
            };

            statement_values.insert(
                *s,
                value.unwrap_or(self.ctx.emitter().constant(0, emitter::Type::Unsigned(0))), // insert unit for statements that return no values
            );
        }

        unreachable!()
    }

    fn lookup_x86_block(&self, rudder: &Ref<Block>) -> X86BlockRef {
        self.x86_blocks.get(rudder).unwrap().clone()
    }

    fn lookup_rudder_block(&self, x86: &X86BlockRef) -> Ref<Block> {
        *self.rudder_blocks.get(x86).unwrap()
    }

    fn lookup_symbol(&self, symbol: &Symbol) -> (emitter::Type, usize) {
        self.stack_variables.get(&symbol.name()).unwrap().clone()
    }
}

/// Converts a rudder type to a `Type` value
fn emit_rudder_constant_type(value: &ConstantValue, typ: &rudder::types::Type) -> emitter::Type {
    match typ {
        rudder::types::Type::ArbitraryLengthInteger => {
            let ConstantValue::SignedInteger(cv) = value else {
                panic!();
            };

            let width = signed_smallest_width_of_value(*cv);

            emitter::Type::Signed(width)
        }
        rudder::types::Type::Bits => {
            let ConstantValue::UnsignedInteger(cv) = value else {
                panic!();
            };

            let width = unsigned_smallest_width_of_value(*cv);

            emitter::Type::Unsigned(width)
        }
        rudder::types::Type::Union { width } => {
            let width = u16::try_from(*width).unwrap();
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
                PrimitiveTypeClass::Void => todo!(),
                PrimitiveTypeClass::Unit => emitter::Type::Unsigned(0),
                PrimitiveTypeClass::SignedInteger => emitter::Type::Signed(width),
                PrimitiveTypeClass::FloatingPoint => emitter::Type::Floating(width),
            }
        }
        rudder::types::Type::ArbitraryLengthInteger => emitter::Type::Signed(64),
        rudder::types::Type::Bits => emitter::Type::Bits,
        t => panic!("todo codegen type instance: {t:?}"),
    }
}
