//! `sail.json` parsing support
//!
//! ## Developer's Notes
//!
//! Some of the path-modifying code is potentially fragile. I was unable to find detailed specifications for `sail.json` files; unfortunately this means there is a *very* wide range of possible inputs for the array of paths to files (absolute, relative, non-UTF8, etc).

use {
    errctx::PathCtx,
    serde::Deserialize,
    std::{
        fs, io,
        path::{Path, PathBuf},
    },
};

/// Configuration for a Sail model
#[derive(Debug)]
pub struct ModelConfig {
    /// Sail compiler options as list of space-separated flags
    pub options: Options,
    /// Ordered list of paths to Sail files
    pub files: Vec<PathBuf>,
}

impl ModelConfig {
    /// Load config from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let config_path = path.as_ref();

        #[derive(Deserialize)]
        struct Intermediate {
            options: String,
            files: Vec<PathBuf>,
        }

        // read from JSON (using intermediate private struct to parse command line options)
        let Intermediate { options, files } =
            serde_json::from_reader(fs::File::open(config_path).map_err(PathCtx::f(config_path))?)
                .map_err(PathCtx::f(config_path))?;

        let mut config = ModelConfig {
            options: options.as_str().try_into()?,
            files,
        };

        // directory that config file is in
        let config_dir = (config_path)
            .parent()
            .ok_or(Error::NoParent(config_path.to_owned()))?;

        // if a path is not absolute, prepend the directory the config file is in to each
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

/// Sail compiler options
#[derive(Debug, Default)]
pub struct Options {
    /// When set, enables non-lexical flow
    pub non_lexical_flow: bool,
    /// When set, disables lexp bounds check
    pub no_lexp_bounds_check: bool,
}

impl TryFrom<&str> for Options {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut options = Options {
            non_lexical_flow: false,
            no_lexp_bounds_check: false,
        };

        for flag in s.split_ascii_whitespace() {
            match flag {
                "-non_lexical_flow" => options.non_lexical_flow = true,
                "-no_lexp_bounds_check" => options.no_lexp_bounds_check = true,
                s => return Err(Error::UnknownFlag(s.to_owned())),
            }
        }

        Ok(options)
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

    /// Encountered unknown or invalid flag in options: {0:?}
    UnknownFlag(String),
}

#[cfg(test)]
mod tests {
    use crate::json::ModelConfig;

    #[test]
    fn snapshot() {
        insta::assert_debug_snapshot!(ModelConfig::load(
            "../data/sail-arm/arm-v8.5-a/model/sail.json"
        )
        .unwrap());
    }
}
