use {
    crate::error::Error,
    alloc::{borrow::ToOwned, collections::BTreeMap, string::String, vec::Vec},
    serde::Deserialize,
    tar_no_std::{ArchiveEntry, TarArchiveRef},
};

/// Load guest configuration, kernel image, and platform dtb from the config tar
/// image
pub fn load_guest_config(config_tar: &[u8]) -> Result<(Config, Vec<u8>, Vec<u8>), Error> {
    let tar = TarArchiveRef::new(config_tar);

    let config: Config = serde_json::from_slice(find_file(&tar, "config.json")?.data())?;

    let (kernel_path, dtb_path) = match config.boot {
        BootProtocol::Arm64Linux(Arm64LinuxBootProtocol { ref kernel, ref dt }) => (kernel, dt),
    };

    let kernel_entry = find_file(&tar, kernel_path)?;
    let dtb_entry = find_file(&tar, dtb_path)?;

    Ok((
        config,
        kernel_entry.data().to_vec(),
        dtb_entry.data().to_vec(),
    ))
}

fn find_file<'a>(tar: &'a TarArchiveRef<'a>, path: &str) -> Result<ArchiveEntry<'a>, Error> {
    tar.entries()
        .find(|e| *e.filename() == *(path.trim_start_matches("./")))
        .ok_or(Error::FileNotFoundInTar(path.to_owned()))
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
