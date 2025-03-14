use {
    crate::{
        arch::x86::memory::VirtualMemoryArea,
        dbt::{
            Translation,
            emitter::{Emitter, Type},
            init_register_file,
            translate::translate,
            x86::{
                X86TranslationContext,
                emitter::{BinaryOperationKind, X86Emitter},
            },
        },
        devices::SharedDevice,
        fs::{File, Filesystem, tar::TarFilesystem},
        guest::register_device_factory,
        logger::REG_TRACE_ONLY,
    },
    alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::btree_map::BTreeMap,
        string::{String, ToString},
        sync::Arc,
        vec::Vec,
    },
    common::{
        HashMap,
        intern::InternedString,
        rudder::{Model, RegisterCacheType, RegisterDescriptor},
    },
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
        .map(|(name, data)| (name, postcard::from_bytes::<Model>(&data).unwrap()))
        .for_each(|(name, mut model)| {
            model.registers_mut().iter_mut().for_each(
                |(name, RegisterDescriptor { cache, .. })| *cache = register_cache_type(*name),
            );
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

pub struct ModelDevice {
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
        unreachable!("execution should never terminate here")
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

        // interpret(
        //     &model,
        //     "__SetConfig",
        //     &[
        //         Value::String("cpu.cpu0.RVBAR".into()),
        //         Value::UnsignedInteger {
        //             value: 0x8000_0000,
        //             width: 64,
        //         },
        //     ],
        //     register_file.as_mut_ptr(),
        // );
        // interpret(
        //     &model,
        //     "__SetConfig",
        //     &[
        //         Value::String("cpu.has_tlb".into()),
        //         Value::UnsignedInteger {
        //             value: 0x0,
        //             width: 64,
        //         },
        //     ],
        //     register_file.as_mut_ptr(),
        // );
        // // from boot.sh command line args to `armv9` binary
        // u__SetConfig(&mut state, &NoopTracer, "cpu.cpu0.RVBAR", 0x8000_0000);
        // u__SetConfig(&mut state, &NoopTracer, "cpu.has_tlb", 0x0);

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

    pub fn get_register_mut<'a, T>(&self, register: &'a str) -> &mut T {
        let offset = self.model.reg_offset(register);
        unsafe { &mut *(self.register_file.lock().as_mut_ptr().add(offset as usize) as *mut T) }
    }

    fn print_regs(&self) {
        if REG_TRACE_ONLY {
            let register_file_ptr = self.register_file.lock().as_mut_ptr();
            unsafe {
                crate::print!(
                    "PC = {:016x}\n",
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64)
                );
                for reg in 0..=30 {
                    crate::print!(
                        "R{reg:02} = {:016x}\n",
                        *(register_file_ptr
                            .add(self.model.reg_offset(alloc::format!("R{reg}")) as usize)
                            as *mut u64)
                    );
                }
            }
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

                let mut ctx = X86TranslationContext::new(&self.model);
                let mut emitter = X86Emitter::new(&mut ctx);

                loop {
                    // reset SEE
                    *(register_file_ptr.add(self.model.reg_offset("SEE") as usize) as *mut i64) =
                        -1;

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

                let contains_mmu_write = ctx.get_mmu_write_flag();

                let translation = ctx.compile(num_regs);

                log::trace!("executing",);
                translation.execute(register_file_ptr);

                if contains_mmu_write {
                    let mmu_enabled = *(register_file_ptr
                        .add(self.model.reg_offset("SCTLR_EL1_bits") as usize)
                        as *mut u64)
                        & 1
                        == 1;

                    if mmu_enabled {
                        blocks.clear();
                        VirtualMemoryArea::current().invalidate_guest_mappings();
                        // clear guest page tables
                    }
                } else {
                    blocks.insert(start_pc, translation);
                }

                log::trace!(
                    "{:x} {}",
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("__BranchTaken") as usize)
                        as *mut u8)
                );
            }
        }
    }

    fn single_step_exec(&self) {
        let mut instructions_retired = 0;

        let register_file_ptr = {
            let mut lock = self.register_file.lock();

            let ptr = lock.as_mut_ptr();

            drop(lock);

            ptr
        };

        let mut instr_cache = HashMap::<u64, Translation>::default();

        loop {
            log::info!("instrs: {instructions_retired}");
            let current_pc = unsafe {
                *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64)
            };

            if let Some(translation) = instr_cache.get(&current_pc) {
                log::info!("executing cached translation @ {current_pc:x}");
                translation.execute(register_file_ptr);
                instructions_retired += 1;
                self.print_regs();
                continue;
            }

            log::info!("---- ---- ---- ---- starting instr translation: {current_pc:x}");

            unsafe {
                // reset SEE
                *(register_file_ptr.add(self.model.reg_offset("SEE") as usize) as *mut i64) = -1;

                let mut ctx = X86TranslationContext::new(&self.model);
                let mut emitter = X86Emitter::new(&mut ctx);

                // reset BranchTaken
                let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

                let current_pc =
                    *(register_file_ptr.add(self.model.reg_offset("_PC") as usize) as *mut u64);
                let opcode = *(current_pc as *const u32);

                log::debug!("translating {opcode:#08x} @ {current_pc:#08x}");
                log::debug!("{}", disarm64::decoder::decode(opcode).unwrap());

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

                let contains_mmu_write = ctx.get_mmu_write_flag();
                let needs_invalidate = ctx.get_mmu_needs_invalidate_flag();

                let translation = ctx.compile(num_regs);

                log::trace!("executing",);
                translation.execute(register_file_ptr);

                if contains_mmu_write | needs_invalidate {
                    let mmu_enabled = *(register_file_ptr
                        .add(self.model.reg_offset("SCTLR_EL1_bits") as usize)
                        as *mut u64)
                        & 1
                        == 1;

                    if mmu_enabled | needs_invalidate {
                        instr_cache.clear();
                        VirtualMemoryArea::current().invalidate_guest_mappings();
                    }
                } else {
                    instr_cache.insert(current_pc, translation);
                }

                instructions_retired += 1;

                log::trace!(
                    "sp: {:x}, x0: {:x}, x1: {:x}, x2: {:x}, x4: {:x}, x12: {:x}, x14: {:x}, x23: {:x}",
                    *(register_file_ptr.add(self.model.reg_offset("SP_EL3") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R0") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R1") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R2") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R4") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R12") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R14") as usize) as *mut u64),
                    *(register_file_ptr.add(self.model.reg_offset("R23") as usize) as *mut u64),
                );

                self.print_regs()
            }

            log::info!("finished\n\n")
        }
    }
}

fn register_cache_type(name: InternedString) -> RegisterCacheType {
    if name.as_ref() == "FeatureImpl"
        || name.as_ref().ends_with("IMPLEMENTED")
        || name.as_ref() == "EL0"
        || name.as_ref() == "EL1"
        || name.as_ref() == "EL2"
        || name.as_ref() == "EL3"
    {
        RegisterCacheType::Constant
    } else if name.as_ref() == "SEE" {
        RegisterCacheType::ReadWrite
    } else if name.as_ref() == "PSTATE_EL"
        || name.as_ref().starts_with("SPE")
        || name.as_ref() == "have_exception"
    {
        RegisterCacheType::Read
    } else {
        RegisterCacheType::None
    }
}
