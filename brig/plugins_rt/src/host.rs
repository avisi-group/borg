use plugins_api::PluginHost;

static mut HOST: Option<&'static dyn PluginHost> = None;

pub(crate) fn init(host: &'static dyn PluginHost) {
    unsafe { HOST = Some(host) }
}

pub fn get() -> &'static dyn PluginHost {
    unsafe { HOST.unwrap() }
}
