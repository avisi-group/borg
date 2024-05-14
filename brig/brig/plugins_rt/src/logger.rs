use {
    crate::host,
    alloc::format,
    log::{Level, LevelFilter, Log},
};

struct HostLogger;

impl Log for HostLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // print trace messages completely plainly
        if record.level() == Level::Trace {
            host::get().print_message(&format!("{}\n", record.args()), true);
        } else {
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            host::get().print_message(
                &format!("\x1b[0;30m[{}]\x1b[0m {}", target, record.args()),
                false,
            );
        }
    }

    fn flush(&self) {}
}

pub(crate) fn init() {
    log::set_logger(&HostLogger).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Trace);
}
