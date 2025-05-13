//! Sail frontend for GenSim

use {
    crate::{
        boom::{
            Ast,
            passes::{
                builtin_fns::HandleBuiltinFunctions, constant_propogation::ConstantPropogation,
                cycle_finder::CycleFinder, destruct_composites::DestructComposites,
                fold_unconditionals::FoldUnconditionals, lower_reals::LowerReals,
                remove_const_branch::RemoveConstBranch, remove_constant_type::RemoveConstantType,
                remove_units::RemoveUnits,
            },
        },
        example_fns::{example_functions, variable_corrupted_example},
        rudder::{
            opt::{self, OptLevel},
            validator,
        },
    },
    color_eyre::eyre::Context,
    common::{bytes::bytes, hashmap::HashSet, intern::InternedString},
    errctx::PathCtx,
    isla_lib::{
        bitvector::b64::B64,
        ir::{Def, Instr, Name, Symtab},
        ir_lexer::new_ir_lexer,
        ir_parser::IrParser,
        smt::Sym,
    },
    log::{debug, info},
    once_cell::sync::Lazy,
    std::{
        fs::{File, create_dir_all},
        io::{BufRead, BufReader, BufWriter, Write},
        path::{Path, PathBuf},
    },
};

pub mod boom;
mod example_fns;
pub mod rudder;
pub mod shared;

/// Deserializes an AST from an archive.
///
/// Internally, deserialization is performed on a new thread with a sufficient
/// stack size to perform the deserialization.
pub fn parse_ir<'ir>(ir: &'ir str) -> Vec<Def<InternedString, B64>> {
    let mut symtab = Symtab::new();

    let defs = IrParser::new()
        .parse(&mut symtab, new_ir_lexer(ir))
        .unwrap();

    convert_names(defs, &symtab)
}

#[derive(Debug, Clone)]
pub enum GenerationMode {
    CodeGen,
    CodeGenWithIr(PathBuf),
    IrOnly(PathBuf),
}

/// Compiles a Sail model to a Brig module
pub fn sail_to_brig(jib_ast: Vec<Def<InternedString, B64>>, path: PathBuf, mode: GenerationMode) {
    let dump_ir = match &mode {
        GenerationMode::CodeGen => None,
        GenerationMode::CodeGenWithIr(p) | GenerationMode::IrOnly(p) => Some(p),
    };

    if let Some(path) = &dump_ir {
        create_dir_all(path).unwrap()
    }

    info!("Converting JIB to BOOM");
    let ast = Ast::from_jib(jib_wip_filter(jib_ast));

    // // useful for debugging
    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file_buffered(path.join("ast.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Running passes on BOOM");
    [
        LowerReals::new_boxed(),
        HandleBuiltinFunctions::new_boxed(),
        RemoveConstantType::new_boxed(),
        DestructComposites::new_boxed(),
        RemoveUnits::new_boxed(),
    ]
    .into_iter()
    .for_each(|mut pass| {
        pass.run(ast.clone());
    });
    boom::passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            ConstantPropogation::new_boxed(),
            // MonomorphizeVectors::new_boxed(),
            CycleFinder::new_boxed(),
        ],
    );

    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file_buffered(path.join("ast.processed.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Building rudder");
    let mut rudder = rudder::build::from_boom(&ast.get());

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder");
    let msgs = validator::validate(&rudder);
    for msg in msgs {
        debug!("{msg}");
    }

    info!("Optimising rudder");
    opt::optimise(&mut rudder, OptLevel::Level3);

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file_buffered(path.join("ast.opt.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder again");
    let msgs = validator::validate(&rudder);
    for msg in msgs {
        debug!("{msg}");
    }

    rudder
        .functions_mut()
        .extend(example_functions().into_iter());
    let r0_offset = rudder.reg_offset("R0");
    let r1_offset = rudder.reg_offset("R1");
    let r2_offset = rudder.reg_offset("R2");
    rudder
        .functions_mut()
        .extend(variable_corrupted_example(r0_offset, r1_offset, r2_offset).into_iter());

    let to_remove = rudder
        .functions()
        .keys()
        .copied()
        .filter(|name| !fn_is_allowlisted(*name))
        .collect::<Vec<_>>();
    for name in to_remove {
        let function = rudder.functions_mut().get_mut(&name).unwrap();
        let block = function.new_block();
        function.set_entry_block(block);
    }

    {
        let func = rudder
            .functions()
            .get(&InternedString::from_static(
                "decode_hint_aarch64_instrs_system_hints",
            ))
            .unwrap();
        rudder::dot::render(
            &mut create_file_buffered(
                dump_ir
                    .unwrap()
                    .join("decode_hint_aarch64_instrs_system_hints.rudder.opt.dot"),
            )
            .unwrap(),
            func.arena(),
            func.entry_block(),
        )
        .unwrap();
    }

    if matches!(
        &mode,
        GenerationMode::CodeGen | GenerationMode::CodeGenWithIr(_)
    ) {
        info!("Serializing Rudder");

        let buf = postcard::to_allocvec(&rudder).unwrap();

        info!("Writing {:.2} to {:?}", bytes(buf.len()), &path);
        File::create(path).unwrap().write_all(&buf).unwrap();
    }
}

/// Calls to these functions will be replaced with units
pub const DELETED_CALLS: &[&str] = &[
    "RestoreTransactionCheckpointParameterised",
    "Z_set",
    "MaybeZeroSVEUppers",
    "ResetSVEState",
];

fn fn_is_allowlisted(name: InternedString) -> bool {
    static FN_DENYLIST: Lazy<HashSet<InternedString>> = Lazy::new(|| {
        BufReader::new(File::open("denylist.txt").unwrap())
            .lines()
            .map(|s| InternedString::from(s.unwrap()))
            .collect()
    });

    !FN_DENYLIST.contains(&name)
}

fn jib_wip_filter(
    jib_ast: Vec<Def<InternedString, B64>>,
) -> impl Iterator<Item = Def<InternedString, B64>> {
    jib_ast.into_iter().map(|d| {
        if let Def::Fn(name, args, body) = d {
            let new_body = if fn_is_allowlisted(name) {
                body
            } else {
                vec![Instr::Arbitrary].into()
            };

            Def::Fn(name, args, new_body)
        } else {
            d
        }
    })
}

/// Creates the file supplied in `path`.
///
/// If the file at the supplied path already exists it will
/// be overwritten.
pub fn create_file_buffered<P: AsRef<Path>>(path: P) -> color_eyre::Result<BufWriter<File>> {
    File::options()
        .write(true) // we want to write to the file...
        .create(true) // ...creating if it does not exist..
        .truncate(true) // ...and truncate before writing
        .open(path.as_ref())
        .map(BufWriter::new)
        .map_err(PathCtx::f(path))
        .wrap_err("Failed to write to file")
}

fn convert_names(defs: Vec<Def<Name, B64>>, symtab: &Symtab) -> Vec<Def<InternedString, B64>> {
    todo!()
}
