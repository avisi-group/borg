#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{error::Error, ffi::run_sail, json::ModelConfig, runtime::RT, types::ListVec},
    color_eyre::{eyre::WrapErr, Result},
    errctx::PathCtx,
    log::trace,
    ocaml::FromValue,
    std::{fs::File, io::BufWriter, path::Path},
};

pub mod builder;
pub mod convert;
pub mod error;
pub mod ffi;
pub mod jib_ast;
pub mod json;
pub mod num;
pub mod parse_ast;
pub mod runtime;
pub mod sail_ast;
pub mod shared;
pub mod type_check;
pub mod types;

/// Loads Sail files from `sail.json` model configuration.
///
/// Parses supplied Sail files and returns the AST
pub fn load_from_config<P: AsRef<Path>>(
    config_path: P,
) -> Result<ListVec<jib_ast::Definition>, Error> {
    let ModelConfig { files } = ModelConfig::load(config_path.as_ref())?;

    RT.lock().execute(move |rt| {
        trace!("Compiling Sail");
        let jib = unsafe {
            run_sail(
                rt,
                files
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect(),
            )
        }??;

        trace!("Parsing JIB AST");
        let jib = ListVec::<jib_ast::Definition>::from_value(jib);

        Ok(jib)
    })?
}
