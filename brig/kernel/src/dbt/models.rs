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

impl ModelDevice {
    fn new(name: String, model: Arc<Model>, initial_pc: u64) -> Self {
        let mut register_file = alloc::vec![0u8; model.register_file_size()];

        init_register_file(&*model, register_file.as_mut_ptr());

        unsafe {
            *(register_file.as_mut_ptr().add(model.reg_offset("_PC")) as *mut u64) = initial_pc
        }

        Self {
            name,
            model,
            register_file: Mutex::new(register_file),
        }
    }
}

impl Debug for ModelDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModelDevice({})", self.name)
    }
}

impl Device for ModelDevice {
    fn start(&self) {
        let register_file_ptr = self.register_file.lock().as_mut_ptr();

        let mut blocks = HashMap::<u64, Translation>::default();

        loop {
            unsafe {
                let pc_offset = self.model.reg_offset("_PC");
                let mut current_pc = *(register_file_ptr.add(pc_offset) as *mut u64);
                let start_pc = current_pc;
                if let Some(translation) = blocks.get(&start_pc) {
                    translation.execute(register_file_ptr);
                    continue;
                }

                if current_pc == 56 {
                    break;
                }

                let mut ctx = X86TranslationContext::new(pc_offset);
                let mut emitter = X86Emitter::new(&mut ctx);

                loop {
                    let see_offset =
                        emitter.constant(self.model.reg_offset("SEE") as u64, Type::Unsigned(64));
                    let neg1 = emitter.constant(-1i32 as u64, Type::Signed(32));
                    emitter.write_register(see_offset, neg1);

                    let branch_taken_offset = emitter.constant(
                        self.model.reg_offset("__BranchTaken") as u64,
                        Type::Unsigned(64),
                    );
                    let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                    emitter.write_register(branch_taken_offset, _false);

                    {
                        let opcode = *(current_pc as *const u32);

                        log::debug!("translating 0x{opcode:08x}");

                        let opcode =
                            emitter.constant(u64::try_from(opcode).unwrap(), Type::Unsigned(32));
                        let pc = emitter.constant(current_pc, Type::Unsigned(64));
                        let _return_value =
                            translate(&*self.model, "__DecodeA64", &[pc, opcode], &mut emitter);
                    }

                    if emitter.ctx().get_write_pc() {
                        break;
                    } else {
                        let pc_offset = emitter.constant(pc_offset as u64, Type::Unsigned(64));
                        let pc = emitter.read_register(pc_offset.clone(), Type::Unsigned(64));
                        let _4 = emitter.constant(4, Type::Unsigned(64));
                        let pc_inc = emitter.binary_operation(BinaryOperationKind::Add(pc, _4));
                        emitter.write_register(pc_offset, pc_inc);

                        current_pc += 4;
                    }
                }

                // inc PC if branch not taken
                {
                    let branch_taken_offset = emitter.constant(
                        self.model.reg_offset("__BranchTaken") as u64,
                        Type::Unsigned(64),
                    );
                    let branch_taken =
                        emitter.read_register(branch_taken_offset, Type::Unsigned(1));

                    let _0 = emitter.constant(0, Type::Unsigned(64));
                    let _4 = emitter.constant(4, Type::Unsigned(64));
                    let addend = emitter.select(branch_taken, _0, _4);

                    let pc_offset = emitter.constant(pc_offset as u64, Type::Unsigned(64));
                    let pc = emitter.read_register(pc_offset.clone(), Type::Unsigned(64));
                    let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
                    emitter.write_register(pc_offset, new_pc);
                }

                emitter.leave();
                let num_regs = emitter.next_vreg();
                let translation = ctx.compile(num_regs);

                // log::trace!("{translation:?}")

                translation.execute(register_file_ptr);
                blocks.insert(start_pc, translation);

                log::trace!(
                    "{:x} {}",
                    *(register_file_ptr.add(self.model.reg_offset("_PC")) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("__BranchTaken")) as *mut u8)
                );
            }
        }
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
