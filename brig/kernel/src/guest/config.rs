use {
    crate::{
        devices::SharedDevice,
        fs::{File, Filesystem, tar::TarFilesystem},
    },
    alloc::{collections::BTreeMap, format, string::String, vec::Vec},
    plugins_api::util::parse_hex_prefix,
    serde::{Deserialize, Deserializer, de::Error as _},
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

/// Load guest configuration from the config tar
/// image
pub fn load_from_device(device: &SharedDevice) -> Result<Config, ConfigLoadError> {
    let mut device = device.lock();
    let mut fs = TarFilesystem::mount(device.as_block());

    Ok(serde_json::from_slice(
        &fs.open("/config.json")?.read_to_vec()?,
    )?)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub memory: BTreeMap<String, AddressSpace>,
    pub load: Vec<Load>,
    pub devices: BTreeMap<String, Device>,
}

pub type AddressSpace = BTreeMap<String, Memory>;

#[derive(Debug, Deserialize)]
pub struct Memory {
    #[serde(deserialize_with = "hex_address")]
    pub start: u64,
    #[serde(deserialize_with = "hex_address")]
    pub end: u64,
}

#[derive(Debug, Deserialize)]
pub struct Load {
    pub path: String,
    #[serde(deserialize_with = "hex_address")]
    pub address: u64,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub kind: String,
    pub attach: Option<DeviceAttachment>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceAttachment {
    Memory {
        address_space: String,
        #[serde(deserialize_with = "hex_address")]
        base: u64,
    },

    SysReg(BTreeMap<String, [u64; 5]>),
}

/// Function to be passed in `deserialize_with` serde attribute for parsing JSON
/// strings containing hex memory addresses into u64s.
fn hex_address<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let s = String::deserialize(deserializer)?;

    Ok(parse_hex_prefix(&s).map_err(|e| {
        D::Error::custom(format!("Failed to parse u64 from hex string {s:?}: {e:?}"))
    })?)
}
