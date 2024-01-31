use {
    crate::{devices::SharedDevice, fs::tar::TarFilesystem},
    alloc::{collections::BTreeMap, string::String, vec::Vec},
    serde::Deserialize,
    thiserror_core as thiserror,
};

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum ConfigLoadError {
    /// File {0:?} was not found in TAR archive
    FileNotFoundInTar(String),
    /// Failed to parse JSON config: {0:#?}
    JsonParse(serde_json::Error),
    /// Supplied device was not a block device
    NotBlockDevice,
}

impl From<serde_json::Error> for ConfigLoadError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParse(value)
    }
}

/// Load guest configuration, kernel image, and platform dtb from the config tar
/// image
pub fn load_from_device(
    device: &SharedDevice,
) -> Result<(Config, Vec<u8>, Vec<u8>), ConfigLoadError> {
    let crate::devices::Device::Block(ref mut dyn_block) = *device.inner.lock() else {
        return Err(ConfigLoadError::NotBlockDevice);
    };
    let mut fs = TarFilesystem::mount(dyn_block);

    let config: Config = { serde_json::from_slice(&fs.open("/config.json").read_to_vec())? };

    let (kernel_path, dtb_path) = match config.boot {
        BootProtocol::Arm64Linux(Arm64LinuxBootProtocol { ref kernel, ref dt }) => (kernel, dt),
    };

    let kernel_entry = fs.open(kernel_path).read_to_vec();
    let dtb_entry = fs.open(dtb_path).read_to_vec();

    Ok((config, kernel_entry, dtb_entry))
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub boot: BootProtocol,
    pub memory: BTreeMap<String, Memory>,
    pub devices: BTreeMap<String, Device>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "protocol")]
pub enum BootProtocol {
    #[serde(rename = "arm64-linux")]
    Arm64Linux(Arm64LinuxBootProtocol),
}

#[derive(Debug, Deserialize)]
pub struct Arm64LinuxBootProtocol {
    pub kernel: String,
    pub dt: String,
}

#[derive(Debug, Deserialize)]
pub struct Memory {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub kind: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, String>,
}
