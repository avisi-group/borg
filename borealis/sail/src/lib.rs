#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{
        error::Error,
        ffi::{
            descatter, effects_infer_side_effects, generate_jib, move_loop_measures, parse_file,
            preprocess, process, register_isla_target, rewrites_rewrite, set_no_lexp_bounds_check,
            set_non_lexical_flow, target_asserts_termination, target_rewrites,
            target_run_pre_parse_hook, type_check_initial_env,
        },
        json::ModelConfig,
        parse_ast::Definition,
        runtime::RT,
        sail_ast::Ast,
    },
    common::intern::InternedStringKey,
    errctx::PathCtx,
    log::trace,
    ocaml::{FromValue, Runtime, Value},
    phf::{phf_map, Map},
    std::{collections::LinkedList, fs::read_to_string, path::Path},
};

pub mod error;
pub mod ffi;
pub mod jib_ast;
pub mod json;
pub mod num;
pub mod parse_ast;
pub mod runtime;
pub mod sail_ast;
pub mod type_check;
pub mod types;

/// Sail standard library files
const SAIL_LIB: Map<&'static str, &'static str> = phf_map! {
    "arith.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/arith.sail")),
    "exception_basic.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/exception_basic.sail")),
    "generic_equality.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/generic_equality.sail")),
    "mono_rewrites.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/mono_rewrites.sail")),
    "regfp.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/regfp.sail")),
    "string.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/string.sail")),
    "concurrency_interface.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/concurrency_interface.sail")),
    "exception_result.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/exception_result.sail")),
    "instr_kinds.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/instr_kinds.sail")),
    "option.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/option.sail")),
    "result.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/result.sail")),
    "trace.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/trace.sail")),
    "elf.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/elf.sail")),
    "float.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/float.sail")),
    "isla.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/isla.sail")),
    "prelude.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/prelude.sail")),
    "reverse_endianness.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/reverse_endianness.sail")),
    "vector_dec.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/vector_dec.sail")),
    "exception.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/exception.sail")),
    "flow.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/flow.sail")),
    "mapping.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/mapping.sail")),
    "real.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/real.sail")),
    "smt.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/smt.sail")),
    "vector_inc.sail" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/vector_inc.sail")),
};

/// Loads Sail files from `sail.json` model configuration.
///
/// Parses supplied Sail files and returns the AST
///
/// From Sail internal docs:
///
/// This function parses all the files passed to Sail, and then concatenates
/// their ASTs. The pre-processor is then run, which evaluates `$directive`
/// statements in Sail, such as
///
/// ```sail
/// $include <prelude.sail>
/// ```
///
/// Unlike the C pre-processor the Sail pre-processor operates over actual
/// Sail ASTs rather than strings. This can recursively include other
/// files into the AST, as well as add/remove parts of the AST with
/// `$ifdef` etc. Directives that are used are preserved in the AST, so
/// they also function as a useful way to pass auxiliary information to
/// the various Sail backends.
///
/// The initial check mentioned above is then run to desugar the AST, and
/// then the type-checker is run which produces a fully type-checked
/// AST. Type annotations are attached to every node (for which an
/// annotation makes sense) using the aux constructors.
///
/// After type-checking the Sail scattered definitions are de-scattered
/// into single functions.
pub fn load_from_config<P: AsRef<Path>>(
    config_path: P,
) -> Result<(Ast, LinkedList<jib_ast::Definition>), Error> {
    let ModelConfig { options, files } = ModelConfig::load(config_path.as_ref())?;

    RT.lock().execute(move |rt| {
        trace!("Registering isla target");
        let target = unsafe { register_isla_target(rt) }??;

        trace!("Running pre-parse hook");
        unsafe { target_run_pre_parse_hook(rt, target.clone()) }??;

        let env = unsafe { type_check_initial_env(rt)?? };

        unsafe { set_non_lexical_flow(rt, options.non_lexical_flow) }??;
        unsafe { set_no_lexp_bounds_check(rt, options.no_lexp_bounds_check) }??;

        let mut defs = LinkedList::new();
        let mut comments = LinkedList::new();

        for path in files {
            trace!("Preprocessing {:?}", path);
            let contents = read_to_string(&path).map_err(PathCtx::f(&path))?;

            // file path used for AST location annotation
            let path = path.as_os_str().to_string_lossy().to_string();

            let (file_comments, file_defs) = preprocess_file(rt, contents, path.clone())?;

            defs.push_back((path.clone(), file_defs));
            comments.push_back((path.clone(), file_comments));
        }

        trace!("Processing");
        let (ast, type_envs, side_effects) = unsafe { process(rt, defs, comments, env) }??;

        trace!("Moving loop measures");
        let ast = unsafe { move_loop_measures(rt, ast) }??;

        trace!("Descattering");
        let (descattered_ast, env) = unsafe { descatter(rt, side_effects, type_envs, ast) }??;

        trace!("Inferring side effects");
        let asserts_termination = unsafe { target_asserts_termination(rt, target.clone()) }??;
        let effect_info = unsafe {
            effects_infer_side_effects(rt, asserts_termination, descattered_ast.clone())
        }??;

        trace!("Rewriting");
        let rewrite_sequence = unsafe { target_rewrites(rt, target) }??;
        let (ast, effect_info, env) = unsafe {
            rewrites_rewrite(
                rt,
                effect_info,
                env,
                rewrite_sequence,
                descattered_ast.clone(),
            )
        }??;

        trace!("Generating JIB IR");
        let jib = unsafe { generate_jib(rt, ast, effect_info, env) }??;

        Ok((
            Ast::from_value(descattered_ast),
            LinkedList::<jib_ast::Definition>::from_value(jib),
        ))
    })?
}

fn preprocess_file(
    rt: &Runtime,
    contents: String,
    path: String,
) -> Result<(LinkedList<Value>, LinkedList<Definition>), Error> {
    trace!("Parsing {:?}", path);

    let (comments, file_ast) = unsafe { parse_file(rt, contents, path.clone()) }??;

    trace!("Importing includes for {:?}", path);

    let file_ast = resolve_includes(rt, file_ast)?;

    trace!("Preprocessing {:?}", path);

    let defs = unsafe { preprocess(rt, "".to_owned(), None, LinkedList::new(), file_ast) }??;

    Ok((comments, defs))
}

/// Finds `$include <filename.sail>` pragmas and replaces with the parsed AST of the referenced Sail standard library file
fn resolve_includes(
    rt: &Runtime,
    file_ast: LinkedList<Definition>,
) -> Result<LinkedList<Definition>, Error> {
    let mut out_defs = LinkedList::new();

    for def in file_ast {
        let Definition::Pragma(key, value, l) = &def else {
            out_defs.push_back(def);
            continue;
        };

        if *key != InternedStringKey::from_static("include") {
            out_defs.push_back(def);
            continue;
        }

        let include = value.to_string();

        if !(include.starts_with('<') && include.ends_with(".sail>")) {
            out_defs.push_back(def);
            continue;
        }

        let filename = include
            .strip_prefix('<')
            .unwrap()
            .strip_suffix('>')
            .unwrap();

        let Some(included_contents) = SAIL_LIB.get(filename) else {
            return Err(Error::MissingIncludeFile(filename.to_owned(), l.clone()));
        };

        trace!("Found {:?} in SAIL_LIB, recursing", filename);
        let (_, mut defs) =
            preprocess_file(rt, (*included_contents).to_owned(), filename.to_owned())?;

        if !defs.is_empty() {
            out_defs.push_back(Definition::Pragma(
                "include_start".into(),
                filename.into(),
                l.clone(),
            ));
            out_defs.append(&mut defs);
            out_defs.push_back(Definition::Pragma(
                "include_end".into(),
                filename.into(),
                l.clone(),
            ));
        }
    }

    Ok(out_defs)
}

#[cfg(test)]
mod tests {
    use {crate::load_from_config, once_cell::sync::Lazy};

    const FILTERS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
        vec![
            (r#""[0-9a-zA-Z\.\-/+]+/(?P<n>.*\.sail)""#, r#""$n""#),
            (
                r#""kind_identifier": \[[\s,0-9]*\]"#,
                r#""kind_identifier": []"#,
            ),
        ]
    });

    #[test]
    fn load_files_empty() {
        let (ast, _) = load_from_config(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../testdata/empty.json"
        ))
        .unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast));
        });
    }
}
