//! `sail.json` parsing support
//!
//! ## Developer's Notes
//!
//! Some of the path-modifying code is potentially fragile. I was unable to find detailed specifications for `sail.json` files; unfortunately this means there is a *very* wide range of possible inputs for the array of paths to files (absolute, relative, non-UTF8, etc).

use {
    serde::Deserialize,
    std::{
        fs, io,
        path::{Path, PathBuf},
    },
};

/// JSON model config error
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// IO error
    Io(#[from] io::Error),

    /// JSON serialisation/deserialisation error
    Json(#[from] serde_json::Error),

    /// Failed to find parent for {0:?}
    NoParent(PathBuf),

    /// Path in `files` is not a file: {0:?}
    NotFile(PathBuf),
}

/// Configuration for a Sail model
#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    /// Sail compiler options as list of space-separated flags
    pub options: String,
    /// Ordered list of paths to Sail files
    pub files: Vec<PathBuf>,
}

impl ModelConfig {
    /// Load config from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        // read from JSON
        let mut config: ModelConfig = serde_json::from_reader(fs::File::open(path)?)?;

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
            let attr = fs::metadata(path)?;
            if !attr.is_file() {
                return Err(Error::NotFile(path.to_owned()));
            }
        }

        Ok(config)
    }
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
