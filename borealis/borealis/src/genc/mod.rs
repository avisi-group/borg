//! GenC model filesystem representation
//!
//! `Description` is a GenC ISA description which can be used to generate it's
//! constituent files, the

use {
    crate::{
        boom::{control_flow::ControlFlowBlock, Statement},
        genc::{
            self,
            codegen::{
                functions::{contains_write_pc, generate_enums, generate_fns},
                instruction::{generate_execute_entrypoint, get_instruction_entrypoint_fns},
            },
            files::{Execute, Files, Format, Function, Isa, Main},
            format::InstructionFormat,
        },
        passes::{
            self, builtin_fns::AddBuiltinFns, cycle_finder::CycleFinder,
            destruct_structs::DestructStructs, fold_unconditionals::FoldUnconditionals,
            lower_enums::LowerEnums, registers::RegisterHandler,
            remove_const_branch::RemoveConstBranch, remove_exception::RemoveExceptions,
            remove_redundant_assigns::RemoveRedundantAssigns, remove_unit::RemoveUnits,
            replace_bools::ReplaceBools, replace_strings::ReplaceStrings,
            resolve_bitvectors::ResolveBitvectors, resolve_return_assigns::ResolveReturns,
        },
        Error,
    },
    common::{intern::InternedString, HashMap, HashSet},
    errctx::PathCtx,
    log::info,
    sail::{jib_ast::Definition, sail_ast::Ast},
    std::{
        cell::RefCell,
        collections::LinkedList,
        fmt::Display,
        fs::{create_dir_all, File},
        io::Write,
        path::{Path, PathBuf},
        rc::Rc,
    },
};

pub mod codegen;
mod files;
pub mod format;

const MAIN_FILENAME: &str = "main.genc";
const ISA_FILENAME: &str = "isa.genc";
const EXECUTE_FILENAME: &str = "execute.genc";
const BEHAVIOURS_FILENAME: &str = "behaviours.genc";

/// Compiles a Sail ISA specification to a GenC description
pub fn sail_to_genc(sail_ast: &Ast, jib_ast: &LinkedList<Definition>) -> Description {
    // uncomment me to dump the JIB AST to stdout
    // sail::jib_ast::pretty_print::print_ast(jib_ast);
    // panic!();

    info!("Generating BOOM from JIB");
    let ast = crate::boom::Ast::from_jib(jib_ast);

    // filter out non addsub functions
    let funcs = ast
        .borrow()
        .functions
        .clone()
        .into_iter()
        .map(|(k, mut def)| {
            // if it's not an allowlisted function, delete the body
            if ![
                "integer_arithmetic_addsub_immediate_decode",
                "integer_arithmetic_addsub_immediate",
                "u__id",
                "AddWithCarry",
                "IsZero",
                "u__GetSlice_int",
                "integer_logical_shiftedreg_decode",
                "DecodeShift",
                "integer_logical_shiftedreg",
                "ShiftReg",
                "branch_conditional_cond_decode",
                "branch_conditional_cond",
                "integer_insext_insert_movewide_decode",
                "integer_insext_insert_movewide",
                "integer_arithmetic_addsub_shiftedreg_decode",
                "DecodeShift",
                "integer_arithmetic_addsub_shiftedreg",
                "IsZeroBit",
                "ConditionHolds",
                "BranchTo",
                "branch_unconditional_immediate_decode",
                "branch_unconditional_immediate",
                "memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode",
                "memory_single_general_immediate_signed_postidx",
                "ConstrainUnpredictable",
                "system_hints_decode",
                "integer_arithmetic_address_pcrel_decode",
                "integer_arithmetic_address_pcrel",
                "memory_pair_general_preidx_memory_pair_general_postidx__decode",
                "memory_pair_general_postidx",
                "memory_pair_general_offset_memory_pair_general_postidx__decode",
                "integer_arithmetic_addsub_extendedreg_decode",
                "DecodeRegExtend",
                "integer_arithmetic_addsub_extendedreg",
                "ExtendReg",
                "memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode",
                "branch_conditional_compare_decode",
                "branch_conditional_compare",
                "memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode",
                "integer_conditional_select_decode",
                "integer_conditional_select",
                "integer_logical_immediate_decode",
                "DecodeBitMasks",
                "HighestSetBit",
                "integer_logical_immediate",
                "memory_pair_general_postidx_memory_pair_general_postidx__decode",
                "branch_unconditional_register_decode",
                "branch_unconditional_register",
                "system_exceptions_runtime_svc_decode",
                "system_exceptions_runtime_svc",
                "integer_conditional_compare_immediate_decode",
                "integer_conditional_compare_immediate",
                "memory_single_general_register_memory_single_general_register__decode",
                "memory_single_general_register",
                "integer_conditional_compare_register_decode",
                "integer_conditional_compare_register",
                "memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode",
                "memory_single_general_immediate_signed_offset_normal",
                "integer_bitfield_decode",
                "integer_bitfield",
                "branch_conditional_test_decode",
                "branch_conditional_test",
                "system_register_system_decode",
                "AArch64_CheckSystemAccess",
                "system_register_system",
                "u__IMPDEF_boolean",
                "u__IMPDEF_boolean_map",
                "vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode",
                "memory_literal_general_decode",
                "memory_literal_general"

                // // CHECKPOINT

                //"AArch64_SysRegRead",
                // "AArch64_SysRegWrite",
                // "HaveBTIExt",
                // "HasArchVersion",
                // "BranchTargetCheck",
                // "AArch64_ExecutingBROrBLROrRetInstr",
                // "AArch64_ExecutingBTIInstr",
                // "ThisInstr",
                // "HaveNVExt"
            ]
            .contains(&k.as_ref())
            {
                def.entry_block = ControlFlowBlock::new();
                def.entry_block.set_statements(vec![Rc::new(RefCell::new(
                    Statement::FunctionCall {
                        expression: None,
                        name: "trap".into(),
                        arguments: vec![],
                    },
                ))])
            }

            (k, def)
        })
        .collect();
    ast.borrow_mut().functions = funcs;

    info!("Running passes on BOOM");
    passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            CycleFinder::new_boxed(),
            ResolveReturns::new_boxed(),
            DestructStructs::new_boxed(),
            ReplaceBools::new_boxed(),
            ReplaceStrings::new_boxed(),
            LowerEnums::new_boxed(),
            RemoveUnits::new_boxed(ast.clone()),
            RemoveExceptions::new_boxed(),
            ResolveBitvectors::new_boxed(),
            AddBuiltinFns::new_boxed(),
            RegisterHandler::new_boxed(),
            RemoveRedundantAssigns::new_boxed(),
        ],
    );

    // let mut buf = String::new();
    // crate::boom::pretty_print::print_ast(&mut buf, ast.clone());
    // write!(&mut File::create("target/ast.boom").unwrap(), "{buf}").unwrap();

    // set up entrypoints in GenC execute behaviours
    let (instruction_names, instructions) = get_instruction_entrypoint_fns(sail_ast)
        .into_iter()
        .map(|clause| generate_execute_entrypoint(ast.clone(), &clause))
        .map(
            |(instruction_name, mangled_name, format, execute, disasm)| {
                (
                    instruction_name,
                    (
                        mangled_name.to_string(),
                        Instruction {
                            format,
                            execute,
                            disasm,
                            is_branch: contains_write_pc(ast.clone(), instruction_name),
                        },
                    ),
                )
            },
        )
        .unzip::<_, _, _, _>();

    // generate all functions, using the names of the instructions as the entrypoint
    let functions = generate_fns(ast.clone(), instruction_names);

    let constants = {
        let mut constants = HashMap::default();
        let enum_constants = generate_enums(ast.clone());

        constants.extend(enum_constants);

        constants.insert("reg_u__v85_implemented".into(), (Typ::Uint8, 1));
        constants.insert("reg_u__v84_implemented".into(), (Typ::Uint8, 1));
        constants.insert("reg_u__v83_implemented".into(), (Typ::Uint8, 1));
        constants.insert("reg_u__v82_implemented".into(), (Typ::Uint8, 1));
        constants.insert("reg_u__v81_implemented".into(), (Typ::Uint8, 1));

        constants.insert("u__unpred_tsize_aborts".into(), (Typ::Uint8, 0));

        constants.insert("u__crypto_aes_implemented".into(), (Typ::Uint8, 0));

        constants.insert("reg_u__mte_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__mpam_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sm4_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sm3_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sha3_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sha256_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sha512_implemented".into(), (Typ::Uint8, 0));
        constants.insert("reg_u__crypto_sha1_implemented".into(), (Typ::Uint8, 0));

        constants.insert("EL0".into(), (Typ::Uint8, 0));
        constants.insert("EL1".into(), (Typ::Uint8, 1));
        constants.insert("EL2".into(), (Typ::Uint8, 2));
        constants.insert("EL3".into(), (Typ::Uint8, 3));

        constants
    };

    Description {
        name: "arm64".to_owned(),
        endianness: Endianness::LittleEndian,
        wordsize: 64,
        fetchsize: 32,
        predicated: false,
        registers: vec![
            RegisterSpace {
                size: 256,
                views: vec![View::Bank(Bank {
                    name: "reg_RB".into(),
                    typ: genc::Typ::Uint64,
                    offset: 0,
                    count: 31,
                    stride: 8,
                    element_count: 1,
                    element_size: 8,
                    element_stride: 8,
                })],
            },
            RegisterSpace {
                size: 24,
                views: vec![
                    View::Slot(Slot {
                        name: "reg_PC".into(),
                        typ: genc::Typ::Uint64,
                        width: 8,
                        offset: 0,
                        tag: Some("PC".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_SP".into(),
                        typ: genc::Typ::Uint64,
                        width: 8,
                        offset: 8,
                        tag: Some("SP".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_PC_target".into(),
                        typ: genc::Typ::Uint64,
                        width: 8,
                        offset: 16,
                        tag: None,
                    }),
                ],
            },
            RegisterSpace {
                size: 4,
                views: vec![
                    View::Slot(Slot {
                        name: "reg_PSTATE_N".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 0,
                        tag: Some("N".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_PSTATE_Z".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 1,
                        tag: Some("Z".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_PSTATE_C".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 2,
                        tag: Some("C".into()),
                    }),
                    View::Slot(Slot {
                        name: "reg_PSTATE_V".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 3,
                        tag: Some("V".into()),
                    }),
                ],
            },
            RegisterSpace {
                size: 8,
                views: vec![View::Slot(Slot {
                    name: "reg_HCR_EL2".into(),
                    typ: genc::Typ::Uint64,
                    width: 8,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 8,
                views: vec![View::Slot(Slot {
                    name: "reg_VMPIDR_EL2".into(),
                    typ: genc::Typ::Uint64,
                    width: 8,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 4,
                views: vec![View::Slot(Slot {
                    name: "reg_VPIDR_EL2".into(),
                    typ: genc::Typ::Uint32,
                    width: 4,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 4,
                views: vec![View::Slot(Slot {
                    name: "reg_ESR_EL1".into(),
                    typ: genc::Typ::Uint32,
                    width: 4,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 8,
                views: vec![View::Slot(Slot {
                    name: "reg_FAR_EL1".into(),
                    typ: genc::Typ::Uint64,
                    width: 8,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 8,
                views: vec![View::Slot(Slot {
                    name: "reg_SCTLR_EL1".into(),
                    typ: genc::Typ::Uint64,
                    width: 8,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 4,
                views: vec![View::Slot(Slot {
                    name: "reg_DCZID_EL0".into(),
                    typ: genc::Typ::Uint32,
                    width: 4,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 4,
                views: vec![View::Slot(Slot {
                    name: "reg_u__currentInstr".into(),
                    typ: genc::Typ::Uint32,
                    width: 4,
                    offset: 0,
                    tag: None,
                })],
            },
            RegisterSpace {
                size: 5,
                views: vec![
                    View::Slot(Slot {
                        name: "BTypeCompatible".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 0,
                        tag: None,
                    }),
                    View::Slot(Slot {
                        name: "reg_BTypeNext".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 1,
                        tag: None,
                    }),
                    View::Slot(Slot {
                        name: "InGuardedPage".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 2,
                        tag: None,
                    }),
                    View::Slot(Slot {
                        name: "reg_PSTATE_EL".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 3,
                        tag: None,
                    }),
                    View::Slot(Slot {
                        name: "reg_PSTATE_BTYPE".into(),
                        typ: genc::Typ::Uint8,
                        width: 1,
                        offset: 4,
                        tag: None,
                    }),
                ],
            },
        ],
        instructions,
        behaviours: Behaviours {
            handle_exception: "".to_owned(),
            reset: "".to_owned(),
            irq: "".to_owned(),
            mmu_fault: "".to_owned(),
            page_fault: "".to_owned(),
            undefined_instruction: "".to_owned(),
            single_step: "".to_owned(),
            undef: "".to_owned(),
            custom: vec![],
        },
        helpers: functions,
        features: ["EMULATE_LINUX_ARCHSIM".into()].into_iter().collect(),
        constants,
    }
}

/// GenC description of an instruction set architecture
#[derive(Debug)]
pub struct Description {
    /// Name of the architecture (e.g. "arm64" or "riscv")
    pub name: String,

    /// Endianness of the architecture
    pub endianness: Endianness,

    /// Word size in bytes
    pub wordsize: u32,

    /// Instruction size in bytes
    pub fetchsize: u32,

    /// ?
    pub predicated: bool,

    /// Register definitions
    pub registers: Vec<RegisterSpace>,

    /// Instruction definitions
    pub instructions: HashMap<String, Instruction>,

    /// GenC behaviours
    pub behaviours: Behaviours,

    /// Helper functions
    pub helpers: Vec<HelperFunction>,

    /// AC features
    pub features: HashSet<String>,

    /// Constant values
    pub constants: HashMap<InternedString, (Typ, u64)>,
}

/// Export a GenC description to the supplied empty directory
pub fn export<P: AsRef<Path>>(description: &Description, path: P) -> Result<(), Error> {
    let path = path.as_ref();

    create_dir_all(path).map_err(PathCtx::f(path))?;

    let Files {
        main,
        isa,
        execute,
        behaviours,
    } = description.files();

    write_file(main, path.join(MAIN_FILENAME))?;
    write_file(isa, path.join(ISA_FILENAME))?;
    write_file(execute, path.join(EXECUTE_FILENAME))?;
    write_file(behaviours, path.join(BEHAVIOURS_FILENAME))?;

    Ok(())
}

/// Creates and writes an value implementing `Display` to a file at the supplied
/// path.
fn write_file<D: Display>(contents: D, path: PathBuf) -> Result<(), Error> {
    let mut file = File::create(&path).map_err(PathCtx::f(&path))?;
    writeln!(file, "{contents}").map_err(PathCtx::f(&path))?;
    Ok(())
}

impl Description {
    /// Creates the internal representation of the GenC files
    fn files(&self) -> Files {
        // `main.genc` file
        let main = Main {
            name: self.name.clone(),
            endianness: self.endianness,
            wordsize: self.wordsize,
            registers: self.registers.clone(),
            features: self.features.clone(),
            constants: self.constants.clone(),
        };

        // construct Vec of Formats
        let formats = self
            .instructions
            .iter()
            .map(
                |(
                    instruction_ident,
                    Instruction {
                        format,
                        disasm,
                        is_branch,
                        ..
                    },
                )| Format {
                    format_ident: format!("fmt_{instruction_ident}"),
                    instruction_ident: instruction_ident.clone(),
                    contents: format.to_string(),
                    disasm: disasm.clone(),
                    is_branch: *is_branch,
                },
            )
            .collect();

        // `isa.genc` file containg arch info and instruction decoding
        let isa = Isa {
            name: self.name.clone(),
            predicated: self.predicated,
            fetchsize: self.fetchsize,
            formats,
        };

        // semantics of each instruction
        let execute = Execute(
            self.instructions
                .iter()
                .map(
                    |(
                        instruction_ident,
                        Instruction {
                            execute, is_branch, ..
                        },
                    )| Function::Execute {
                        name: instruction_ident.clone(),
                        body: if *is_branch {
                            "\twrite_register(reg_PC_target, read_pc() + 4);\n".to_owned()
                                + execute
                                + "\n\twrite_pc(read_register(reg_PC_target));\n"
                        } else {
                            execute.clone()
                        },
                    },
                )
                .chain(self.helpers.iter().cloned().map(
                    |HelperFunction {
                         return_type,
                         parameters,
                         name,
                         body,
                     }| Function::Helper {
                        return_type,
                        parameters,
                        name,
                        body,
                    },
                ))
                .collect(),
        );

        let required_behaviours = [
            Function::Behaviour {
                global: true,
                name: "handle_exception".into(),
                body: self.behaviours.handle_exception.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "reset".into(),
                body: self.behaviours.reset.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "irq".into(),
                body: self.behaviours.irq.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "mmu_fault".into(),
                body: self.behaviours.mmu_fault.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "page_fault".into(),
                body: self.behaviours.page_fault.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "undefined_instruction".into(),
                body: self.behaviours.undefined_instruction.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "single_step".into(),
                body: self.behaviours.single_step.clone(),
            },
            Function::Behaviour {
                global: true,
                name: "undef".into(),
                body: self.behaviours.undef.clone(),
            },
        ];

        let behaviours = files::Behaviours(
            self.behaviours
                .custom
                .iter()
                .map(|CustomBehaviour { name, body }| Function::Behaviour {
                    name: name.clone(),
                    body: body.clone(),
                    global: false,
                })
                .chain(required_behaviours)
                .collect::<Vec<_>>(),
        );

        Files {
            main,
            isa,
            execute,
            behaviours,
        }
    }
}

/// Endianness of an instruction set architecture
#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    /// Most significant byte of a word at the smallest memory address
    BigEndian,
    /// Most significant byte of a word at the largest memory address
    LittleEndian,
}

/// Definition of a register space
#[derive(Debug, Clone)]
pub struct RegisterSpace {
    /// Size in bytes of the register space
    pub size: u32,
    /// Views in register space
    pub views: Vec<View>,
}

/// Register view
#[derive(Debug, Clone)]
pub enum View {
    /// Register bank
    Bank(Bank),
    /// Register slot
    Slot(Slot),
}

/// Register bank
#[derive(Debug, Clone)]
pub struct Bank {
    /// Identifier for bank
    pub name: String,
    /// Register type
    pub typ: Typ,
    /// Offset
    pub offset: u32,
    /// Number of registers
    pub count: u32,
    /// Register stride
    pub stride: u32,
    /// Number of elements
    pub element_count: u32,
    /// Size of elements
    pub element_size: u32,
    /// Element stride
    pub element_stride: u32,
}

/// Register slot
#[derive(Debug, Clone)]
pub struct Slot {
    /// Identifier for slot
    pub name: String,

    /// Register type
    pub typ: Typ,

    /// Register slot width
    pub width: u32,

    /// Register slot offset in register space
    pub offset: u32,

    /// Optional tag
    pub tag: Option<String>,
}

/// Data type
#[derive(Debug, Clone)]
pub enum Typ {
    /// Unsigned 8-bit integer
    Uint8,
    /// Unsigned 16-bit integer
    Uint16,
    /// Unsigned 32-bit integer
    Uint32,
    /// Unsigned 64-bit integer
    Uint64,
    /// Unsigned 128-bit integer
    Uint128,
    /// 32-bit floating point
    Float,
    /// 64-bit floating point
    Double,
}

/// Definition of the syntax and semantics of a single instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Instruction format string used for decoding
    pub format: InstructionFormat,
    /// GenC execution behaviour
    pub execute: String,
    /// GenC disassembly behaviour
    pub disasm: String,
    /// Is branch instruction (modifies PC)
    pub is_branch: bool,
}

/// Required and custom behaviours for architecture
#[derive(Debug, Clone)]
pub struct Behaviours {
    /// Exception handler
    pub handle_exception: String,
    /// Reset
    pub reset: String,
    /// Interrupt request
    pub irq: String,
    /// Memory management unit fault
    pub mmu_fault: String,
    /// Page fault
    pub page_fault: String,
    /// Undefined instruction
    pub undefined_instruction: String,
    /// Single step
    pub single_step: String,
    /// Undefined
    pub undef: String,
    /// Additional, non-required behaviours
    pub custom: Vec<CustomBehaviour>,
}

/// Custom behaviour
#[derive(Debug, Clone)]
pub struct CustomBehaviour {
    /// Name of behaviour
    pub name: String,
    /// Function body
    pub body: String,
}

/// GenC internal helper function
#[derive(Debug, Clone)]
pub struct HelperFunction {
    /// Function return type
    pub return_type: String,
    /// Function parameters
    pub parameters: String,
    /// Function name
    pub name: String,
    /// Function body
    pub body: String,
}
