//! `sail.json` parsing support
//!
//! ## Developer's Notes
//!
//! Some of the path-modifying code is potentially fragile. I was unable to find
//! detailed specifications for `sail.json` files; unfortunately this means
//! there is a *very* wide range of possible inputs for the array of paths to
//! files (absolute, relative, non-UTF8, etc).
//!
//! 2024-03-14 update: I'm hardcoding everything except the file list

use {
    errctx::PathCtx,
    serde::Deserialize,
    std::{
        fs, io,
        path::{Path, PathBuf},
    },
};

/// Configuration for a Sail model
#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    /// Ordered list of paths to Sail files
    pub files: Vec<PathBuf>,
}

impl ModelConfig {
    /// Load config from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let config_path = path.as_ref();

        let file = fs::File::open(config_path).map_err(PathCtx::f(config_path))?;

        let mut config = serde_json::from_reader::<_, ModelConfig>(file).map_err(PathCtx::f(config_path))?;

        // directory that config file is in
        let config_dir = (config_path).parent().ok_or(Error::NoParent(config_path.to_owned()))?;

        // if a path is not absolute, prepend the directory the config file is in to
        // each
        for file_path in &mut config.files {
            // if path is absolute, no changes necessary
            if file_path.has_root() {
                break;
            }

            *file_path = config_dir.join(&file_path);

            // verify that each path is valid and points to a file
            let attr = fs::metadata(&file_path).map_err(PathCtx::f(&file_path))?;
            if !attr.is_file() {
                return Err(Error::NotFile(file_path.clone()));
            }
        }

        Ok(config)
    }
}

/// JSON model config error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// IO error
    Io(#[from] PathCtx<io::Error>),

    /// JSON serialisation/deserialisation error
    Json(#[from] PathCtx<serde_json::Error>),

    /// Failed to find parent for {0:?}
    NoParent(PathBuf),

    /// Path in `files` is not a file: {0:?}
    NotFile(PathBuf),
}
