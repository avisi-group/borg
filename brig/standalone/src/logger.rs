use log::{Level, LevelFilter, Log};

struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // print trace messages completely plainly
        println!("{}", record.args());
    }

    fn flush(&self) {}
}

pub(crate) fn init() {
    log::set_logger(&Logger).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Trace);
}
