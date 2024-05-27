use {
    crate::{
        devices::SharedDevice,
        fs::{tar::TarFilesystem, File, Filesystem},
    },
    alloc::{collections::BTreeMap, string::String, vec::Vec},
    serde::Deserialize,
    thiserror_core as thiserror,
};

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum ConfigLoadError {
    /// Filesystem error: {0:?}
    Filesystem(crate::fs::Error),
    /// Failed to parse JSON config: {0:#?}
    JsonParse(serde_json::Error),
}

impl From<crate::fs::Error> for ConfigLoadError {
    fn from(value: crate::fs::Error) -> Self {
        Self::Filesystem(value)
    }
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
    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());

    let config: Config = { serde_json::from_slice(&fs.open("/config.json")?.read_to_vec()?)? };

    let (kernel_path, dtb_path) = match config.boot {
        BootProtocol::Arm64Linux(Arm64LinuxBootProtocol { ref kernel, ref dt }) => (kernel, dt),
    };

    let kernel_entry = fs.open(kernel_path)?.read_to_vec()?;
    let dtb_entry = fs.open(dtb_path)?.read_to_vec()?;

    Ok((config, kernel_entry, dtb_entry))
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub boot: BootProtocol,
    pub memory: BTreeMap<String, AddressSpace>,
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

//#[derive(Debug, Deserialize)]
pub type AddressSpace = BTreeMap<String, Memory>;

#[derive(Debug, Deserialize)]
pub struct Memory {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub kind: String,
    pub attach: Option<DeviceAttachment>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceAttachment {
    pub address_space: String,
    pub base: String,
}
