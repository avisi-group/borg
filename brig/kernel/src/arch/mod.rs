use bootloader_api::BootInfo;

pub mod x86;

/// Platform initialization, triggers device enumeration and
/// initialization
pub fn platform_init(boot_info: &BootInfo) {
    x86::init(boot_info);
}
