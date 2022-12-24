#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{
        ast::Ast,
        error::Error,
        json::ModelConfig,
        runtime::Runtime,
        type_check::Env,
        wrapper::{
            internal_descatter, internal_load_files, internal_set_no_lexp_bounds_check,
            internal_set_non_lexical_flow, internal_type_check_initial_env,
        },
    },
    log::trace,
    ocaml::{
        interop::{BoxRoot, ToOCaml},
        FromValue, List,
    },
    once_cell::sync::Lazy,
    parking_lot::Mutex,
    std::{os::unix::ffi::OsStringExt, path::Path},
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

/// Global runtime shared by all public functions
static RT: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(Runtime::new()));

const DEFAULT_SAIL_DIR: &str = "../sail/wrapper";

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
        let env = unsafe { internal_type_check_initial_env(rt)?? };

        let mut file_list = List::empty();

        for path in files.into_iter().rev() {
            let path = unsafe { String::from_utf8_unchecked(path.into_os_string().into_vec()) };
            let file_rooted: BoxRoot<String> = path.to_boxroot(rt);
            file_list = unsafe { file_list.add(rt, &file_rooted) };
        }

        unsafe { internal_set_non_lexical_flow(rt, options.non_lexical_flow) }??;
        unsafe { internal_set_no_lexp_bounds_check(rt, options.no_lexp_bounds_check) }??;

        let default_sail_dir: BoxRoot<String> = DEFAULT_SAIL_DIR.to_owned().to_boxroot(rt);

        trace!("Calling internal_load_files");

        // opaque `Value`s here
        let (ast, env, effect_info) =
            unsafe { internal_load_files(rt, default_sail_dir, List::empty(), env, file_list) }??;

        trace!("Calling internal_descatter");

        let (ast, env) = unsafe { internal_descatter(rt, effect_info, env, ast) }??;

        trace!("Converting AST from ocaml::Value");
        let ast = Ast::from_value(ast);
        trace!("Finished converting AST from ocaml::Value");

        trace!("Converting Env from ocaml::Value");
        let env = Env::from_value(env);
        trace!("Finished converting Env from ocaml::Value");

        Ok((ast, env))
    })?
}

#[cfg(test)]
mod tests {
    use {
        crate::{load_from_config, wrapper::internal_util_dedup, RT},
        ocaml::{List, ToValue},
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
                let mut l = List::empty();

                for element in list {
                    l = unsafe { l.add(rt, &element.to_value(rt)) };
                }

                unsafe { internal_util_dedup(rt, l) }
                    .unwrap()
                    .unwrap()
                    .into_vec()
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
        let (ast, env) = load_from_config("../testdata/empty.json").unwrap();

        crate::dot::render(&ast, &mut vec![]).unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast, env));
        });
    }

    #[test]
    fn load_from_config_arm() {
        let (ast, env) = load_from_config("../testdata/sail-arm-small.json").unwrap();

        crate::dot::render(&ast, &mut vec![]).unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast, env));
        });
    }
}
