use {
    crate::{
        host::{
            arch::x86::{
                aarch64_mmu::{self, take_arm_exception},
                memory::VirtualMemoryArea,
                safepoint::record_safepoint,
            },
            dbt::{
                Alloc, Translation,
                emitter::{Emitter, Type},
                register_file::{RegisterFile, WellKnownRegister},
                translate::translate_instruction,
                x86::{
                    X86TranslationContext,
                    emitter::{BinaryOperationKind, X86Emitter},
                },
            },
            devices::manager::SharedDeviceManager,
            fs::Filesystem,
            memory::bump::{BumpAllocator, BumpAllocatorRef},
            objects::{
                Object, ObjectId, ObjectStore, ToIrqController, ToMemoryMappedDevice,
                ToRegisterMappedDevice, ToTickable, device::Device,
            },
        },
        util::parse_hex_prefix,
    },
    alloc::{
        alloc::alloc_zeroed,
        borrow::ToOwned,
        collections::btree_map::BTreeMap,
        string::{String, ToString},
        sync::Arc,
        vec::Vec,
    },
    common::{
        hashmap::HashMap,
        intern::InternedString,
        rudder::{Model, RegisterCacheType, RegisterDescriptor},
    },
    core::{
        alloc::Layout,
        fmt::{self, Debug, Write},
        ptr::NonNull,
    },
    proc_macro_lib::guest_device_factory,
    spin::Mutex,
    x86_64::structures::paging::{PageSize, Size4KiB},
};

/// Size in bytes for the per-translation bump allocator
const TRANSLATION_ALLOCATOR_SIZE: usize = 4 * 1024 * 1024 * 1024;

/// Limit blocks to contain only 1 instruction
const SINGLE_STEP: bool = false;

/// Write register trace to file
const PRINT_REGISTERS: bool = false;

/// Enable the jump table chain cache
const CHAIN_CACHE_ENABLED: bool = true;
pub const CHAIN_CACHE_ENTRY_COUNT: usize = 65536;
const _: () = assert!(CHAIN_CACHE_ENTRY_COUNT.is_power_of_two());

static MODEL_MANAGER: Mutex<BTreeMap<InternedString, Arc<Model>>> = Mutex::new(BTreeMap::new());

pub fn register_model(name: InternedString, model: Model) {
    log::info!("registering {name:?} ISA model");
    let model = Arc::new(model);
    MODEL_MANAGER.lock().insert(name.to_owned(), model.clone());
}

pub fn get(name: &str) -> Option<Arc<Model>> {
    MODEL_MANAGER
        .lock()
        .get(&InternedString::from(name))
        .cloned()
}

pub fn load_all<FS: Filesystem>(fs: &mut FS) {
    log::info!("loading models");
    // todo: don't hardcode this, load all .postcards?

    // todo: don't hardcode this, load all .postcards?
    ["aarch64.postcard"]
        .into_iter()
        .map(|path| {
            (
                InternedString::from(path.strip_suffix(".postcard").unwrap()),
                fs.read_to_vec(path).unwrap(),
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

#[guest_device_factory(core)]
fn create_core(config: &BTreeMap<InternedString, InternedString>) -> Arc<dyn Device> {
    let model_name = config.get(&InternedString::from_static("model")).unwrap();
    let model = get(model_name.as_ref()).unwrap();
    let initial_pc = config
        .get(&InternedString::from_static("initial_pc"))
        .map(parse_hex_prefix)
        .unwrap()
        .unwrap();

    Arc::new(ModelDevice::new(model_name.to_string(), model, initial_pc))
}

pub struct WellKnownRegisters {
    pc: WellKnownRegister<u64>,
    i: WellKnownRegister<bool>,
}

impl WellKnownRegisters {
    pub fn pc(&self) -> WellKnownRegister<u64> {
        self.pc
    }

    pub fn i(&self) -> WellKnownRegister<bool> {
        self.i
    }
}

pub struct ModelDevice {
    id: ObjectId,
    name: String,
    model: Arc<Model>,
    pub register_file: RegisterFile,
    pub well_known_registers: WellKnownRegisters,
}

impl Debug for ModelDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModelDevice({})", self.name)
    }
}

impl Object for ModelDevice {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl ToTickable for ModelDevice {}
impl ToRegisterMappedDevice for ModelDevice {}
impl ToMemoryMappedDevice for ModelDevice {}
impl ToIrqController for ModelDevice {}

impl Device for ModelDevice {
    fn start(&self) {
        self.block_exec(SINGLE_STEP);
        unreachable!("execution should never terminate here")
    }

    fn stop(&self) {
        todo!()
    }
}

impl ModelDevice {
    fn new(name: String, model: Arc<Model>, initial_pc: u64) -> Self {
        let register_file = RegisterFile::init(&*model);
        let well_known_registers = WellKnownRegisters {
            pc: register_file.as_wellknown::<u64>("_PC"),
            i: register_file.as_wellknown::<bool>("PSTATE_I"),
        };

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
            id: ObjectId::new(),
            name,
            model,
            register_file,
            well_known_registers,
        }
    }

    fn get_nzcv(&self) -> u8 {
        let n = self.register_file.read::<u8>("PSTATE_N");
        let z = self.register_file.read::<u8>("PSTATE_Z");
        let c = self.register_file.read::<u8>("PSTATE_C");
        let v = self.register_file.read::<u8>("PSTATE_V");

        assert!(n <= 1);
        assert!(z <= 1);
        assert!(c <= 1);
        assert!(v <= 1);

        n << 3 | z << 2 | c << 1 | v
    }

    fn block_exec(&self, single_step_mode: bool) {
        let shared = SharedDeviceManager::get()
            .get_device_by_alias("transport00:04.0")
            .unwrap();
        let crate::host::devices::Device::Transport(transport) = &mut *shared.lock() else {
            panic!();
        };

        let mut instructions_executed = 0usize;

        // guest PC to translated block cache
        // todo: should be guest physical address not virtual so we dont need to
        // invalidate
        let mut block_cache = HashMap::<u64, TranslatedBlock>::default();
        // guest virtual address
        let mut chain_cache = DirectMappedCache::<CHAIN_CACHE_ENTRY_COUNT, *const u8>::new(1);

        // virtual to physical PCs
        let mut translation_cache = DirectMappedCache::<1024, u64>::new(1);

        let mut allocator = BumpAllocator::new(TRANSLATION_ALLOCATOR_SIZE);

        //  log::set_max_level(log::LevelFilter::Error);

        let _status = record_safepoint();

        // block translation/execution loop
        loop {
            // if instructions_executed == 389280 {
            //     log::set_max_level(log::LevelFilter::Trace);
            // }

            // if instructions_executed == 52590 {
            //     panic!();
            // }

            let block_start_virtual_pc = self.well_known_registers.pc().read(); // self.register_file.read::<u64>("_PC");

            let block_start_physical_pc =
                if let Some(pc) = translation_cache.get(block_start_virtual_pc as usize) {
                    pc
                } else {
                    let pc = aarch64_mmu::guest_translate(self, block_start_virtual_pc).unwrap();
                    translation_cache.insert(block_start_virtual_pc as usize, pc);
                    pc
                };

            let translated_block =
                block_cache
                    .entry(block_start_physical_pc)
                    .or_insert_with(|| {
                        allocator.clear();
                        self.translate_block(
                            BumpAllocatorRef::new(&allocator),
                            chain_cache.table as u64,
                            block_start_virtual_pc,
                            single_step_mode,
                        )
                    });

            if CHAIN_CACHE_ENABLED {
                chain_cache.insert(
                    block_start_virtual_pc as usize,
                    translated_block.translation.as_ptr(),
                );
            }

            instructions_executed += translated_block.opcodes.len();

            log::debug!(
                "executing {block_start_virtual_pc:#08x} ({block_start_physical_pc:#08x}): {:08x?} (instr {instructions_executed})",
                translated_block.opcodes,
            );

            let exec_result = translated_block.translation.execute(&self.register_file);

            // log::trace!(
            //     "nzcv: {:04b}, sp: {:x}, x0: {:x}, x1: {:x}, x2: {:x}, x3: {:x}, x18:
            // {:x}",     self.get_nzcv(),
            //     self.register_file.read::<u64>("SP_EL3"),
            //     self.register_file.read::<u64>("R0"),
            //     self.register_file.read::<u64>("R1"),
            //     self.register_file.read::<u64>("R2"),
            //     self.register_file.read::<u64>("R3"),
            //     self.register_file.read::<u64>("R18"),
            // );

            if PRINT_REGISTERS {
                write!(transport, "instr = {:08x}\n", translated_block.opcodes[0]).unwrap();
                write!(
                    transport,
                    "PC = {:016x}\n",
                    self.register_file.read::<u64>("_PC")
                )
                .unwrap();
                write!(transport, "PSTATE:\n").unwrap();
                for field in [
                    "A", "ALLINT", //"BTYPE",
                    "C", "D", "DIT", "E", "EL", "EXLOCK", "F", "GE", "I", "IL", "IT", "J", "M",
                    "N", "PAN", "PM", "PPEND", "Q", "SM", "SP", "SS", "SSBS", "T", "TCO", "UAO",
                    "V", "Z", "ZA", "nRW",
                ] {
                    write!(
                        transport,
                        "\t{field} = {}\n",
                        self.register_file
                            .read::<u8>(alloc::format!("PSTATE_{field}"))
                    )
                    .unwrap();
                }
                // write!(
                //     transport,
                //     "BTypeNext = {}\n",
                //     self.register_file.read::<u8>("BTypeNext")
                // )
                // .unwrap();
                for el in 0..=3 {
                    write!(
                        transport,
                        "SP_EL{el} = {:016x}\n",
                        self.register_file.read::<u64>(alloc::format!("SP_EL{el}"))
                    )
                    .unwrap();
                }
                for el in 1..=3 {
                    write!(
                        transport,
                        "SPSR_EL{el} = {:016x}\n",
                        self.register_file
                            .read::<u64>(alloc::format!("SPSR_EL{el}_bits"))
                    )
                    .unwrap();
                }
                for el in 1..=3 {
                    write!(
                        transport,
                        "ELR_EL{el} = {:016x}\n",
                        self.register_file.read::<u64>(alloc::format!("ELR_EL{el}"))
                    )
                    .unwrap();
                }
                for reg in 0..=30 {
                    write!(
                        transport,
                        "R{reg:02} = {:016x}\n",
                        self.register_file.read::<u64>(alloc::format!("R{reg}"))
                    )
                    .unwrap();
                }
                write!(transport, "\n\n").unwrap();
                if !single_step_mode {
                    write!(transport, "skip {}\n", translated_block.opcodes.len()).unwrap();
                }
            }

            if exec_result.need_tlb_invalidate() {
                chain_cache.fill_keys(1);
                translation_cache.fill_keys(1);
                VirtualMemoryArea::current().invalidate_guest_mappings();
            }

            if exec_result.interrupt_pending() {
                let masked = self.well_known_registers.i().read(); //self.register_file.read::<bool>("PSTATE_I");

                if !masked {
                    let pc = self.well_known_registers.pc().read();
                    // log::error!("interrupt pending @ {pc:x}, masked: {masked}");
                    take_arm_exception(self, 1, 255, 0, 0, pc, 0x80);
                    //let pc = self.well_known_registers.pc().read();
                    //log::error!("took arm exception to {pc:x}");
                } else {
                    //   log::error!("masked interrupt pending");
                }
            }
        }
    }

    fn translate_block<A: Alloc>(
        &self,
        allocator: A,
        chain_cache: u64,
        block_start_pc: u64,
        single_step_mode: bool,
    ) -> TranslatedBlock {
        let mut ctx = X86TranslationContext::new_with_allocator(allocator, &self.model, true);
        let mut emitter = X86Emitter::new(&mut ctx);

        let mut current_pc = block_start_pc;

        let mut opcodes = Vec::new();

        // block prologue
        emitter.prologue();

        // instruction translation loop
        let was_end_of_block = loop {
            // reset BranchTaken
            let _false = emitter.constant(0 as u64, Type::Unsigned(1));
            emitter.write_register(self.model.reg_offset("__BranchTaken") as u64, _false);

            // read opcode
            let opcode = unsafe { *((current_pc & 0xFF_FFFF_FFFF) as *const u32) };

            log::debug!("translating {opcode:#08x} @ {current_pc:#08x}");
            log::debug!("{}", disarm64::decoder::decode(opcode).unwrap());

            #[cfg(feature = "debug_translation")]
            opcodes.push(opcode);

            let _return_value = translate_instruction(
                allocator,
                &*self.model,
                "__DecodeA64",
                &mut emitter,
                &self.register_file,
                opcode,
            )
            .unwrap();

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

            // if we have a TLB invalidation or other non-zero status in that instruction,
            // do not translate the rest of the block
            if emitter.execution_result.need_tlb_invalidate() {
                break false;
            }

            // only translate single instruction in single_step_mode
            if single_step_mode {
                break false;
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
        emitter.leave_with_cache(chain_cache);
        let num_regs = emitter.next_vreg();

        let translation = ctx.compile(num_regs);

        // if translation.code.len() > 1024 {
        //     crate::println!("WARNING! Large block @ {block_start_pc:x}");

        //     crate::println!("INPUT ASM:");
        //     for opcode in opcodes {
        //         crate::println!("  {}", disarm64::decoder::decode(opcode).unwrap());
        //     }

        //     crate::println!("\nOUTPUT ASM:");
        //     crate::println!("{translation:?}");

        //     panic!();
        // }

        log::trace!("finished");

        TranslatedBlock {
            translation,
            opcodes,
        }
    }
}

pub struct TranslatedBlock {
    translation: Translation,
    opcodes: Vec<u32>,
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

#[repr(C)]
struct ChainCacheEntry<V> {
    key: usize,
    value: V,
}

#[repr(C)]
struct DirectMappedCache<const N: usize, V> {
    table: *mut ChainCacheEntry<V>,
}

impl<const N: usize, V: Copy> DirectMappedCache<N, V> {
    pub fn new(initial_keys: usize) -> Self {
        let ptr = unsafe {
            alloc_zeroed(
                Layout::from_size_align(
                    N * size_of::<ChainCacheEntry<V>>(),
                    Size4KiB::SIZE.try_into().unwrap(),
                )
                .unwrap(),
            )
        };

        let mut celf = Self {
            table: ptr as *mut ChainCacheEntry<V>,
        };

        celf.fill_keys(initial_keys);

        celf
    }

    pub fn insert(&mut self, key: usize, value: V) {
        self.table()[Self::index(key)] = ChainCacheEntry { key, value };
    }

    pub fn get(&mut self, key: usize) -> Option<V> {
        let entry = &self.table()[Self::index(key)];

        if entry.key == key {
            Some(entry.value)
        } else {
            None
        }
    }

    fn index(key: usize) -> usize {
        (key >> 2) & (N - 1)
    }

    fn table(&mut self) -> &mut [ChainCacheEntry<V>] {
        unsafe { core::slice::from_raw_parts_mut(self.table, N) }
    }

    pub fn fill_keys(&mut self, key: usize) {
        self.table().iter_mut().for_each(|e| e.key = key);
    }
}
