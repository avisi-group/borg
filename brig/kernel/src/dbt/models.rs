use {
    crate::{
        dbt::{
            emitter::{self, BlockResult, Emitter},
            x86::{
                emitter::{X86BlockRef, X86NodeRef, X86SymbolRef},
                X86TranslationContext,
            },
            TranslationContext,
        },
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
    },
    alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String, sync::Arc},
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
        HashMap,
    },
    spin::Mutex,
};

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
    FunctionExecutor::new(model, function, arguments, ctx).execute()
}

struct FunctionExecutor<'m, 'c> {
    model: &'m Model,
    function_name: InternedString,
    x86_blocks: HashMap<Ref<Block>, X86BlockRef>,
    rudder_blocks: HashMap<X86BlockRef, Ref<Block>>,
    local_variables: HashMap<InternedString, X86SymbolRef>,
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
        // todo: write arguments to local variables

        let mut celf = Self {
            model,
            function_name: InternedString::from(function),
            x86_blocks: HashMap::default(),
            rudder_blocks: HashMap::default(),
            local_variables: HashMap::default(),
            exit_block_ref: ctx.create_block(),
            ctx,
        };

        let function = celf.model.get_functions().get(&celf.function_name).unwrap();

        // set up symbols for local variables
        function.local_variables().iter().for_each(|symbol| {
            celf.local_variables
                .insert(symbol.name(), celf.ctx.create_symbol());
        });

        // set up symbols for parameters, and write arguments into them
        function
            .parameters()
            .iter()
            .zip(arguments)
            .for_each(|(parameter, argument)| {
                let sym = celf.ctx.create_symbol();
                celf.local_variables.insert(parameter.name(), sym.clone());
                celf.ctx.emitter().write_variable(sym, argument.clone());
            });

        // set up block maps
        function.block_iter().for_each(|rudder_block| {
            let x86_block = celf.ctx.create_block();
            celf.rudder_blocks.insert(x86_block.clone(), rudder_block);
            celf.x86_blocks.insert(rudder_block, x86_block);
        });

        celf
    }

    fn execute(&mut self) -> X86NodeRef {
        let function = self
            .model
            .get_functions()
            .get(&self.function_name)
            .unwrap_or_else(|| panic!("failed to find function {:?} in model", self.function_name));

        // todo: write arguments

        enum BlockKind {
            Static(Ref<Block>),
            Dynamic(Ref<Block>),
        }

        let mut block_queue = alloc::vec![BlockKind::Static(function.entry_block())];

        while let Some(block) = block_queue.pop() {
            let result = match block {
                BlockKind::Static(b) => {
                    // log::debug!("static block {i}");
                    self.execute_block(function, b)
                }
                BlockKind::Dynamic(b) => {
                    // log::debug!("dynamic block {i}");
                    self.ctx
                        .emitter()
                        .set_current_block(self.x86_blocks.get(&b).unwrap().clone());
                    self.execute_block(function, b)
                }
            };

            match result {
                BlockResult::Static(block) => {
                    let block = *self.rudder_blocks.get(&block).unwrap();
                    // log::debug!("block result: static({idx})");
                    block_queue.push(BlockKind::Static(block));
                }
                BlockResult::Dynamic(b0, b1) => {
                    let block0 = *self.rudder_blocks.get(&b0).unwrap();
                    let block1 = *self.rudder_blocks.get(&b1).unwrap();
                    //  log::debug!("block result: dynamic({i0}, {i1})");
                    block_queue.push(BlockKind::Dynamic(block0));
                    block_queue.push(BlockKind::Dynamic(block1));
                }
                BlockResult::Return => {
                    // log::debug!("block result: return");
                    self.ctx.emitter().jump(self.exit_block_ref.clone());
                }
                BlockResult::Panic => {
                    //  log::debug!("block result: panic");
                    // unreachable but inserted just to make sure *every* block has a path to the
                    // exit block
                    self.ctx.emitter().jump(self.exit_block_ref.clone());
                }
            }
        }

        self.ctx
            .emitter()
            .set_current_block(self.exit_block_ref.clone());

        return self.ctx.emitter().read_variable(
            self.local_variables
                .get(&InternedString::from_static("borealis_fn_return_value"))
                .unwrap()
                .clone(),
        );
    }

    fn execute_block(&mut self, function: &Function, block_ref: Ref<Block>) -> BlockResult {
        let block = block_ref.get(function.arena());

        let mut statement_values = HashMap::default();

        for s in block.statements() {
            let value = match s.get(block.arena()) {
                Statement::Constant { typ, value } => {
                    let typ = emit_rudder_constant_type(value, typ);
                    match value {
                        ConstantValue::UnsignedInteger(v) => (self.ctx.emitter().constant(*v, typ)),
                        ConstantValue::SignedInteger(v) => {
                            (self.ctx.emitter().constant(*v as u64, typ))
                        }
                        ConstantValue::FloatingPoint(v) => {
                            (self.ctx.emitter().constant(*v as u64, typ))
                        }
                        ConstantValue::Unit => self.ctx.emitter().constant(0, typ),
                        ConstantValue::String(s) => {
                            todo!("string {s:?}")
                        }
                        ConstantValue::Rational(_) => todo!(),

                        ConstantValue::Tuple(values) => {
                            // let Type::Tuple(types) = &typ else { panic!() };
                            // let values = values
                            //     .iter()
                            //     .cloned()
                            //     .zip(types.iter().cloned())
                            //     .map(|(value, typ)| codegen_constant_value(value, typ));
                            // ((#(#values),*))
                            todo!("tuple")
                        }
                    }
                }
                Statement::Jump { target } => {
                    let target = self.lookup_x86_block(target);
                    return self.ctx.emitter().jump(target);
                }
                _ => todo!(),
            };

            statement_values.insert(*s, value);
        }

        unreachable!()
    }

    fn lookup_x86_block(&self, rudder: &Ref<Block>) -> X86BlockRef {
        self.x86_blocks.get(rudder).unwrap().clone()
    }

    fn lookup_rudder_block(&self, x86: &X86BlockRef) -> Ref<Block> {
        *self.rudder_blocks.get(x86).unwrap()
    }
}

/// Converts a rudder type to a `Type` value
fn emit_rudder_constant_type(value: &ConstantValue, typ: &rudder::types::Type) -> emitter::Type {
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
        rudder::types::Type::String => {
            todo!()
        }
        rudder::types::Type::Union { width } => {
            let width = u16::try_from(*width).unwrap();
            emitter::Type::Unsigned(width)
        }
        t => panic!("todo codegen type instance: {t:?}"),
    }
}
