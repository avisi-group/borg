use {
    alloc::{collections::BTreeMap, string::String, vec::Vec},
    serde::Deserialize,
    tar_no_std::TarArchiveRef,
};

/// Load guest configuration, kernel image, and platform dtb from the config tar
/// image
pub fn load_guest_config(config_tar: &[u8]) -> (Config, Vec<u8>, Vec<u8>) {
    let tar = TarArchiveRef::new(config_tar);

    let config_entry = tar
        .entries()
        .find(|e| *e.filename() == *"config.json")
        .unwrap();

    let config: Config = serde_json::from_slice(config_entry.data()).unwrap();

    let (kernel_path, dt_path) = match config.boot {
        BootProtocol::Arm64Linux(Arm64LinuxBootProtocol { ref kernel, ref dt }) => (kernel, dt),
    };

    let kernel_entry = tar
        .entries()
        .find(|e| *e.filename() == *(kernel_path.trim_start_matches("./")))
        .unwrap();

    let dtb_entry = tar
        .entries()
        .find(|e| *e.filename() == *(dt_path.trim_start_matches("./")))
        .unwrap();

    (
        config,
        kernel_entry.data().to_vec(),
        dtb_entry.data().to_vec(),
    )
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
    //extra: HashMap<String, String>,
}
