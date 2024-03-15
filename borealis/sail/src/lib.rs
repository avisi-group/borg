#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{
        error::Error,
        ffi::{generate_jib, run_sail},
        json::ModelConfig,
        runtime::RT,
        sail_ast::Ast,
    },
    log::trace,
    ocaml::FromValue,
    std::{collections::LinkedList, path::Path},
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

/// Loads Sail files from `sail.json` model configuration.
///
/// Parses supplied Sail files and returns the AST
pub fn load_from_config<P: AsRef<Path>>(
    config_path: P,
) -> Result<(Ast, LinkedList<jib_ast::Definition>), Error> {
    let ModelConfig { files } = ModelConfig::load(config_path.as_ref())?;

    RT.lock().execute(move |rt| {
        trace!("Compiling Sail");
        let (ast, env, effect_info) = unsafe {
            run_sail(
                rt,
                files
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect(),
            )
        }??;

        trace!("Generating JIB IR");
        let jib = unsafe { generate_jib(rt, ast.clone(), env, effect_info) }??;

        trace!("Parsing Sail and JIB ASTs");
        let ast = Ast::from_value(ast);
        let jib = LinkedList::<jib_ast::Definition>::from_value(jib);

        Ok((ast, jib))
    })?
}
