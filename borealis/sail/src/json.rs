//! `sail.json` parsing support
//!
//! ## Developer's Notes
//!
//! Some of the path-modifying code is potentially fragile. I was unable to find detailed specifications for `sail.json` files; unfortunately this means there is a *very* wide range of possible inputs for the array of paths to files (absolute, relative, non-UTF8, etc).

use {
    common::error::ErrCtx,
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
        let path = path.as_ref();

        #[derive(Deserialize)]
        struct Intermediate {
            options: String,
            files: Vec<PathBuf>,
        }

        // read from JSON (using intermediate private struct to parse command line options)
        let Intermediate { options, files } =
            serde_json::from_reader(fs::File::open(path).map_err(ErrCtx::f(&path))?)
                .map_err(ErrCtx::f(&path))?;

        let mut config = ModelConfig {
            options: options.as_str().try_into()?,
            files,
        };

        // if all the paths are just filenames then prepend the directory the config file is in to each
        if config
            .files
            .iter()
            .all(|p| p.parent() == Some(&Path::new("")))
        {
            let model_dir = path.parent().ok_or(Error::NoParent(path.to_owned()))?;

            config.files = config
                .files
                .into_iter()
                .map(|path| model_dir.join(path))
                .collect();
        }

        // verify that each path is valid and points to a file
        for path in &config.files {
            let attr = fs::metadata(path).map_err(ErrCtx::f(&path))?;
            if !attr.is_file() {
                return Err(Error::NotFile(path.to_owned()));
            }
        }

        Ok(config)
    }
}

/// Sail compiler options
#[derive(Debug, Default)]
pub struct Options {
    pub non_lexical_flow: bool,
    pub no_lexp_bounds_check: bool,
}

impl TryFrom<&str> for Options {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut options = Options {
            non_lexical_flow: false,
            no_lexp_bounds_check: false,
        };

        for flag in s.split_ascii_whitespace().into_iter() {
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
    Io(#[from] ErrCtx<io::Error>),

    /// JSON serialisation/deserialisation error
    Json(#[from] ErrCtx<serde_json::Error>),

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
            "../testdata/sail-arm/arm-v8.5-a/model/sail.json"
        )
        .unwrap());
    }
}
