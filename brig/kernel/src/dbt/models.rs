use {
    crate::{
        dbt::{
            emitter::{Emitter, Type},
            init_register_file,
            translate::translate,
            x86::{
                emitter::{BinaryOperationKind, X86Emitter},
                X86TranslationContext,
            },
            Translation,
        },
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
        guest::register_device_factory,
    },
    alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::btree_map::BTreeMap,
        string::{String, ToString},
        sync::Arc,
        vec::Vec,
    },
    common::{rudder::Model, HashMap},
    core::fmt::{self, Debug},
    plugins_api::{
        guest::{Device, DeviceFactory},
        util::parse_hex_prefix,
    },
    spin::Mutex,
};

static MODEL_MANAGER: Mutex<BTreeMap<String, Arc<Model>>> = Mutex::new(BTreeMap::new());

pub fn register_model(name: &str, model: Model) {
    log::info!("registering {name:?} ISA model");
    let model = Arc::new(model);
    MODEL_MANAGER.lock().insert(name.to_owned(), model.clone());

    register_device_factory(
        name.to_string(),
        Box::new(ModelDeviceFactory::new(name.to_string(), model)),
    )
}

pub fn get(name: &str) -> Option<Arc<Model>> {
    MODEL_MANAGER.lock().get(name).cloned()
}

pub fn load_all(device: &SharedDevice) {
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

/// Factory for creating execution instances for a supplied model
struct ModelDeviceFactory {
    name: String,
    model: Arc<Model>,
}

impl ModelDeviceFactory {
    fn new(name: String, model: Arc<Model>) -> Self {
        Self { name, model }
    }
}

impl DeviceFactory for ModelDeviceFactory {
    fn create(
        &self,
        config: BTreeMap<String, String>,
        _environment: Box<dyn plugins_api::guest::Environment>,
    ) -> Arc<dyn plugins_api::guest::Device> {
        let initial_pc = config
            .get("initial_pc")
            .map(parse_hex_prefix)
            .unwrap()
            .unwrap();

        Arc::new(ModelDevice::new(
            self.name.clone(),
            self.model.clone(),
            initial_pc,
        ))
    }
}

struct ModelDevice {
    name: String,
    model: Arc<Model>,
    register_file: Mutex<Vec<u8>>,
}

impl Debug for ModelDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModelDevice({})", self.name)
    }
}

impl Device for ModelDevice {
    fn start(&self) {
        //self.block_exec();
        self.single_step_exec();
    }

    fn stop(&self) {
        todo!()
    }

    fn address_space_size(&self) -> u64 {
        u64::MAX
    }

    fn read(&self, _offset: u64, _value: &mut [u8]) {
        todo!()
    }

    fn write(&self, _offset: u64, _value: &[u8]) {
        todo!()
    }
}

impl ModelDevice {
    fn new(name: String, model: Arc<Model>, initial_pc: u64) -> Self {
        let mut register_file = init_register_file(&*model);

        unsafe {
            *(register_file
                .as_mut_ptr()
                .add(model.reg_offset("_PC") as usize) as *mut u64) = initial_pc
        }

        Self {
            name,
            model,
            register_file: Mutex::new(register_file),
        }
    }

    fn block_exec(&self) {
        let register_file_ptr = self.register_file.lock().as_mut_ptr();

        let mut blocks = HashMap::<u64, Translation>::default();

        loop {
            unsafe {
                let pc_offset = self.model.reg_offset("_PC");
                let mut current_pc = *(register_file_ptr.add(pc_offset as usize) as *mut u64);
                let start_pc = current_pc;
                if let Some(translation) = blocks.get(&start_pc) {
                    translation.execute(register_file_ptr);
                    continue;
                }

                let mut ctx = X86TranslationContext::new(pc_offset);
                let mut emitter = X86Emitter::new(&mut ctx);

                loop {
                    let neg1 = emitter.constant(-1i32 as u64, Type::Signed(32));
                    emitter.write_register(self.model.reg_offset("SEE") as u64, neg1);

                    let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                    emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

                    {
                        let opcode = *(current_pc as *const u32);

                        log::debug!("translating 0x{opcode:08x}");

                        let opcode =
                            emitter.constant(u64::try_from(opcode).unwrap(), Type::Unsigned(32));
                        let pc = emitter.constant(current_pc, Type::Unsigned(64));
                        let _return_value = translate(
                            &*self.model,
                            "__DecodeA64",
                            &[pc, opcode],
                            &mut emitter,
                            register_file_ptr,
                        );
                    }

                    if emitter.ctx().get_pc_write_flag() {
                        break;
                    } else {
                        let pc = emitter.read_register(pc_offset as u64, Type::Unsigned(64));
                        let _4 = emitter.constant(4, Type::Unsigned(64));
                        let pc_inc = emitter.binary_operation(BinaryOperationKind::Add(pc, _4));
                        emitter.write_register(pc_offset as u64, pc_inc);

                        current_pc += 4;
                    }
                }

                // inc PC if branch not taken
                {
                    let branch_taken = emitter.read_register(
                        self.model.reg_offset("__BranchTaken") as u64,
                        Type::Unsigned(1),
                    );

                    let _0 = emitter.constant(0, Type::Unsigned(64));
                    let _4 = emitter.constant(4, Type::Unsigned(64));
                    let addend = emitter.select(branch_taken, _0, _4);

                    let pc = emitter.read_register(pc_offset as u64, Type::Unsigned(64));
                    let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
                    emitter.write_register(pc_offset as u64, new_pc);
                }

                emitter.leave();
                let num_regs = emitter.next_vreg();
                let translation = ctx.compile(num_regs);

                log::trace!("executing");

                translation.execute(register_file_ptr);
                blocks.insert(start_pc, translation);

                log::trace!(
                    "{:x} {}",
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("__BranchTaken") as usize)
                        as *mut u8)
                );
            }
        }

        unreachable!();
    }

    fn single_step_exec(&self) {
        let register_file_ptr = self.register_file.lock().as_mut_ptr();

        loop {
            unsafe {
                let mut ctx = X86TranslationContext::new(self.model.reg_offset("_PC"));
                let mut emitter = X86Emitter::new(&mut ctx);

                // reset SEE
                let neg1 = emitter.constant(-1i32 as u64, Type::Signed(32));
                emitter.write_register(self.model.reg_offset("SEE") as u64, neg1);

                // reset BranchTaken
                let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

                let current_pc =
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64);
                let opcode = *(current_pc as *const u32);

                log::debug!("translating {opcode:#08x} @ {current_pc:#08x}");

                let opcode = emitter.constant(u64::try_from(opcode).unwrap(), Type::Unsigned(32));
                let pc = emitter.constant(current_pc, Type::Unsigned(64));
                let _return_value = translate(
                    &*self.model,
                    "__DecodeA64",
                    &[pc, opcode],
                    &mut emitter,
                    register_file_ptr,
                );

                // if we didn't jump anywhere, increment PC by 4 bytes
                {
                    let branch_taken = emitter.read_register(
                        self.model.reg_offset("__BranchTaken") as u64,
                        Type::Unsigned(1),
                    );

                    let _0 = emitter.constant(0, Type::Unsigned(64));
                    let _4 = emitter.constant(4, Type::Unsigned(64));
                    let addend = emitter.select(branch_taken, _0, _4);

                    let pc =
                        emitter.read_register(self.model.reg_offset("_PC"), Type::Unsigned(64));
                    let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
                    emitter.write_register(self.model.reg_offset("_PC"), new_pc);
                }

                log::trace!("compiling");

                emitter.leave();
                let num_regs = emitter.next_vreg();
                let translation = ctx.compile(num_regs);

                log::trace!("executing");

                translation.execute(register_file_ptr);

                log::trace!(
                    "{:x} {}",
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("__BranchTaken") as usize)
                        as *mut u8)
                );
            }
        }

        unreachable!();
    }
}
