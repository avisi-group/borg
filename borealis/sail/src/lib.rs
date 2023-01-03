#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{
        ast::Ast,
        error::Error,
        json::ModelConfig,
        runtime::RT,
        type_check::Env,
        wrapper::{
            descatter, parse_file, preprocess, process, set_no_lexp_bounds_check,
            set_non_lexical_flow, type_check_initial_env,
        },
    },
    common::error::ErrCtx,
    log::trace,
    ocaml::{FromValue, ToValue},
    std::{collections::LinkedList, fs::read_to_string, path::Path},
};

pub mod ast;
pub mod dot;
pub mod error;
pub mod json;
pub mod num;
mod runtime;
pub mod type_check;
pub mod types;
pub mod visitor;
mod wrapper;

const DEFAULT_SAIL_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/wrapper");

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
pub fn load_from_config<P: AsRef<Path>>(config_path: P) -> Result<(Ast, Env), Error> {
    let ModelConfig { options, files } = json::ModelConfig::load(config_path.as_ref())?;

    RT.lock().execute(move |rt| {
        let env = unsafe { type_check_initial_env(rt)?? };

        unsafe { set_non_lexical_flow(rt, options.non_lexical_flow) }??;
        unsafe { set_no_lexp_bounds_check(rt, options.no_lexp_bounds_check) }??;

        trace!("Parsing files");

        let mut parsed_files = vec![];
        let mut comments = LinkedList::new();

        for file_path in files {
            let contents = read_to_string(&file_path).map_err(ErrCtx::f(&file_path))?;

            // file path used for AST location annotation
            let path = file_path.as_os_str().to_string_lossy().to_string();

            let (file_comments, file_ast) = unsafe { parse_file(rt, contents, path.clone()) }??;

            parsed_files.push((path.clone(), file_ast));
            comments.push_back((path, file_comments));
        }

        let mut defs = LinkedList::new();
        for (path, file_defs) in parsed_files {
            trace!("Preprocessing {:?}", path);

            let file_defs = unsafe {
                preprocess(
                    rt,
                    DEFAULT_SAIL_DIR.to_owned(),
                    None,
                    LinkedList::new(),
                    file_defs.to_value(rt),
                )
            }??;

            defs.push_back((path, file_defs));
        }

        trace!("Calling process");

        let (ast, type_envs, side_effects) = unsafe { process(rt, defs, comments, env) }??;

        let b = Ast::from_value(ast.clone());
        let c = b.to_value(rt);
        let _d = Ast::from_value(c);

        //  assert_eq!(b, d);

        trace!("Calling descatter");

        let (ast, env) = unsafe { descatter(rt, side_effects, type_envs, ast) }??;

        Ok((ast, env))
    })?
}

#[cfg(test)]
mod tests {
    use {
        crate::{load_from_config, wrapper::util_dedup, RT},
        once_cell::sync::Lazy,
        proptest::{bits, collection::vec, prelude::*},
    };

    const FILTERS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
        vec![
            (r#""id": [0-9]+"#, r#""id": 0"#),
            (r#""[0-9a-zA-Z\.\-/+]+/(?P<n>.*\.sail)""#, r#""$n""#),
            (
                r#""kind_identifier": \[[\s,0-9]*\]"#,
                r#""kind_identifier": []"#,
            ),
        ]
    });

    fn dedup(list: Vec<i32>) -> Vec<i32> {
        RT.lock()
            .execute(|rt| {
                unsafe { util_dedup(rt, list.into_iter().collect()) }
                    .unwrap()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<_>>()
            })
            .unwrap()
    }

    proptest! {
        /// Checks equivalence between libsail dedup function and Rust stdlib dedup.
        ///
        /// Used as smoke test that OCaml interop is functioning correctly (intentionally doing a lot of allocating, many function calls, etc).
        #[test]
        fn smoke_test(v in vec(bits::i32::ANY, 0..1000)) {
            let mut v_d = v.clone();
            v_d.sort();

            v_d.dedup();

            let mut out = dedup(v);
            out.sort();

            assert_eq!(out, v_d);
        }
    }

    #[test]
    fn load_files_empty() {
        let (ast, env) = load_from_config(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../testdata/empty.json"
        ))
        .unwrap();

        crate::dot::render(&ast, &mut vec![]).unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast, env));
        });
    }

    #[test]
    fn load_from_config_arm() {
        let (ast, env) = load_from_config(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../testdata/sail-arm-small.json"
        ))
        .unwrap();

        crate::dot::render(&ast, &mut vec![]).unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast, env));
        });
    }
}
