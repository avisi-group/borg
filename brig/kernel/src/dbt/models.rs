use {
    crate::{
        arch::x86::{memory::VirtualMemoryArea, safepoint::record_safepoint},
        dbt::{
            Translation,
            emitter::{Emitter, Type},
            register_file::RegisterFile,
            translate::{translate, translate_instruction},
            x86::{
                X86TranslationContext,
                emitter::{BinaryOperationKind, X86Emitter},
            },
        },
        devices::SharedDevice,
        fs::{File, Filesystem, tar::TarFilesystem},
        guest::register_device_factory,
        logger::PRINT_REGISTERS,
        memory::bump::{BumpAllocator, BumpAllocatorRef},
    },
    alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::btree_map::BTreeMap,
        string::{String, ToString},
        sync::Arc,
    },
    common::{
        hashmap::HashMap,
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
    pub register_file: RegisterFile,
}

impl Debug for ModelDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModelDevice({})", self.name)
    }
}

impl Device for ModelDevice {
    fn start(&self) {
        self.block_exec();
        //self.single_step_exec();
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
        let register_file = RegisterFile::init(&*model);

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

        register_file.write("_PC", initial_pc);

        Self {
            name,
            model,
            register_file,
        }
    }

    fn get_nzcv(&self) -> u8 {
        let n = self.register_file.read::<u8, _>("PSTATE_N");
        let z = self.register_file.read::<u8, _>("PSTATE_Z");
        let c = self.register_file.read::<u8, _>("PSTATE_C");
        let v = self.register_file.read::<u8, _>("PSTATE_V");

        assert!(n <= 1);
        assert!(z <= 1);
        assert!(c <= 1);
        assert!(v <= 1);

        n << 3 | z << 2 | c << 1 | v
    }

    fn print_regs(&self) {
        if PRINT_REGISTERS {
            crate::print!("PC = {:016x}\n", self.register_file.read::<u64, _>("_PC"));
            crate::print!("NZCV = {:04b}\n", self.get_nzcv());
            for reg in 0..=30 {
                crate::print!(
                    "R{reg:02} = {:016x}\n",
                    self.register_file.read::<u64, _>(alloc::format!("R{reg}"))
                );
            }
        }
    }

    fn block_exec(&self) {
        let mut block_cache = HashMap::<u64, (Translation, usize)>::default();

        let mut allocator = BumpAllocator::new(2 * 1024 * 1024 * 1024);

        let _status = record_safepoint();

        // block translation/execution loop
        loop {
            let block_start_pc = self.register_file.read::<u64, _>("_PC");

            if let Some((translation, num_instructions)) = block_cache.get(&block_start_pc) {
                log::debug!("executing cached @ {block_start_pc:#08x}");
                translation.execute(&self.register_file);
                self.print_regs();
                if PRINT_REGISTERS {
                    crate::println!("skip {}", num_instructions);
                }
                continue;
            }

            allocator.clear();
            let alloc_ref = BumpAllocatorRef::new(&allocator);

            let mut ctx = X86TranslationContext::new_with_allocator(alloc_ref, &self.model, true);
            let mut emitter = X86Emitter::new(&mut ctx);

            let mut current_pc = block_start_pc;

            let mut num_instructions = 0usize;

            // instruction translation loop
            let was_end_of_block = loop {
                // reset BranchTaken
                let _false = emitter.constant(0 as u64, Type::Unsigned(1));
                emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

                // read opcode
                let opcode = unsafe { *((current_pc & 0xFF_FFFF_FFFF) as *const u32) };

                log::debug!("translating {opcode:#08x} @ {current_pc:#08x}");

                let _return_value = translate_instruction(
                    alloc_ref,
                    &*self.model,
                    "__DecodeA64",
                    &mut emitter,
                    &self.register_file,
                    current_pc,
                    opcode,
                )
                .unwrap();

                num_instructions += 1;

                // hit a maybe-PC modifying instruction
                if emitter.ctx().get_pc_write_flag() {
                    // end of block
                    break true;
                } else {
                    // emit code to increment PC register by 4
                    let pc_offset = self.model.reg_offset("_PC");
                    let pc = emitter.read_register(pc_offset as u64, Type::Unsigned(64));
                    let _4 = emitter.constant(4, Type::Unsigned(64));
                    let pc_inc = emitter.binary_operation(BinaryOperationKind::Add(pc, _4));
                    emitter.write_register(pc_offset as u64, pc_inc);

                    // increase our local pc by 4
                    current_pc += 4;

                    // did we cross a page boundary?
                    if current_pc & !0xFFF != block_start_pc & !0xFFF {
                        break false;
                    }
                }
            };

            // if we didn't jump anywhere at the end of the block (IE. branch was not
            // taken), increment PC by 4 bytes
            if was_end_of_block {
                let branch_taken = emitter.read_register(
                    self.model.reg_offset("__BranchTaken") as u64,
                    Type::Unsigned(1),
                );

                let _0 = emitter.constant(0, Type::Unsigned(64));
                let _4 = emitter.constant(4, Type::Unsigned(64));
                let addend = emitter.select(branch_taken, _0, _4);

                let pc_offset = self.model.reg_offset("_PC");
                let pc = emitter.read_register(pc_offset, Type::Unsigned(64));
                let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
                emitter.write_register(pc_offset, new_pc);
            }

            log::trace!("compiling");
            emitter.leave();
            let num_regs = emitter.next_vreg();

            let contains_mmu_write = ctx.get_mmu_write_flag();
            let needs_invalidate = ctx.get_mmu_needs_invalidate_flag();

            let translation = ctx.compile(num_regs);

            log::trace!("executing");
            translation.execute(&self.register_file);

            log::trace!(
                "nzcv: {:04b}, sp: {:x}, x0: {:x}, x1: {:x}, x2: {:x}, x5: {:x}",
                self.get_nzcv(),
                self.register_file.read::<u64, _>("SP_EL3"),
                self.register_file.read::<u64, _>("R0"),
                self.register_file.read::<u64, _>("R1"),
                self.register_file.read::<u64, _>("R2"),
                self.register_file.read::<u64, _>("R5"),
            );

            self.print_regs();
            if PRINT_REGISTERS {
                crate::println!("skip {}", num_instructions);
            }

            if contains_mmu_write | needs_invalidate {
                let mmu_enabled = self.register_file.read::<u64, _>("SCTLR_EL1_bits") & 1 == 1;

                if mmu_enabled | needs_invalidate {
                    block_cache.clear();
                    VirtualMemoryArea::current().invalidate_guest_mappings();
                    // clear guest page tables
                }
            } else {
                block_cache.insert(block_start_pc, (translation, num_instructions));
            }

            log::info!("finished\n\n")
        }
    }

    fn single_step_exec(&self) {
        let mut instructions_retired = 0u64;

        let mut instr_cache = HashMap::<u64, Translation>::default();

        // 4GB
        let mut allocator = BumpAllocator::new(2 * 1024 * 1024 * 1024);

        let _status = record_safepoint();

        loop {
            let current_pc = self.register_file.read::<u64, _>("_PC") & 0x0000_00FF_FFFF_FFFF;

            if let Some(translation) = instr_cache.get(&current_pc) {
                log::info!("executing cached translation @ {current_pc:x}");
                translation.execute(&self.register_file);
                instructions_retired += 1;
                self.print_regs();
                continue;
            }

            log::info!(
                "---- ---- ---- ---- starting instr translation: {current_pc:x}, retired: {instructions_retired}"
            );

            allocator.clear();
            let alloc_ref = BumpAllocatorRef::new(&allocator);

            let mut ctx = X86TranslationContext::new_with_allocator(alloc_ref, &self.model, true);
            let mut emitter = X86Emitter::new(&mut ctx);

            // reset BranchTaken
            let _false = emitter.constant(0 as u64, Type::Unsigned(1));
            emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

            let opcode = unsafe { *(current_pc as *const u32) };

            log::info!("translating {opcode:#08x} @ {current_pc:#08x}");
            log::info!("{}", disarm64::decoder::decode(opcode).unwrap());

            let _return_value = translate_instruction(
                alloc_ref,
                &*self.model,
                "__DecodeA64",
                &mut emitter,
                &self.register_file,
                current_pc,
                opcode,
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

                let pc = emitter.read_register(self.model.reg_offset("_PC"), Type::Unsigned(64));
                let new_pc = emitter.binary_operation(BinaryOperationKind::Add(pc, addend));
                emitter.write_register(self.model.reg_offset("_PC"), new_pc);
            }
            log::trace!("compiling");

            emitter.leave();
            let num_regs = emitter.next_vreg();

            let contains_mmu_write = ctx.get_mmu_write_flag();
            let needs_invalidate = ctx.get_mmu_needs_invalidate_flag();

            let translation = ctx.compile(num_regs);

            log::trace!("executing");
            translation.execute(&self.register_file);

            if contains_mmu_write | needs_invalidate {
                let mmu_enabled = self.register_file.read::<u64, _>("SCTLR_EL1_bits") & 1 == 1;
                log::trace!("mmu_enabled: {mmu_enabled}");
                if mmu_enabled | needs_invalidate {
                    log::trace!("clearing cache");
                    instr_cache.clear();
                    VirtualMemoryArea::current().invalidate_guest_mappings();
                }
                // no insertion here?
            } else {
                log::trace!("inserting into cache");
                instr_cache.insert(current_pc, translation);
            }

            instructions_retired += 1;

            log::trace!(
                "nzcv: {:04b}, sp: {:x}, x0: {:x}, x1: {:x}, x2: {:x}, x4: {:x}, x5: {:x}",
                self.get_nzcv(),
                self.register_file.read::<u64, _>("SP_EL3"),
                self.register_file.read::<u64, _>("R0"),
                self.register_file.read::<u64, _>("R1"),
                self.register_file.read::<u64, _>("R2"),
                self.register_file.read::<u64, _>("R4"),
                self.register_file.read::<u64, _>("R5"),
            );

            self.print_regs();

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
    } else if name.as_ref() == "SEE"
        || name.as_ref() == "have_exception"
        || name.as_ref().starts_with("current_exception")
    {
        RegisterCacheType::ReadWrite
    } else if name.as_ref() == "PSTATE_EL" || name.as_ref().starts_with("SPE") {
        RegisterCacheType::Read
    } else {
        RegisterCacheType::None
    }
}
