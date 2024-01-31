use crate::devices::manager::SharedDeviceManager;

pub mod config;

/// Start guest emulation
pub fn start() {
    log::trace!("starting guest");
    //check each connected block device for guest config
    let device_manager = SharedDeviceManager::get();
    let device = device_manager
        .get_device_by_alias("disk00:03.0")
        .expect("disk not found");

    let (config, kernel, _dtb) = config::load_from_device(&device).unwrap();

    // // todo device.as_block() -> Option
    // let DeviceKind::Block(ref mut block_device) = *device.inner else {
    //     panic!("disk was not block device");
    // };

    // let mut dyn_block_device = &mut **block_device;
    // let mut tar_fs = TarFilesystem::mount(&mut dyn_block_device);
    // let config = tar_fs.open("config.json").unwrap();

    // block_devices
    //     .into_iter()
    //     .map(|id| device_manager.get_device(id).unwrap());

    // search all drives for guest tar

    log::trace!("kernel len: {:#x}, got config: {:#?}", kernel.len(), config);
    panic!();
}
