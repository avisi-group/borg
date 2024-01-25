//! Logger implementation

use {
    crate::devices::serial::SerialPort,
    core::fmt::{self, Write},
    log::{Level, LevelFilter, Log, Metadata, Record},
    spin::Once,
};

/// Global console writer
static mut WRITER: Once<SerialPort> = Once::INIT;

static LOGGER: Logger = Logger;

/// Initialise logger using the xen::console backend
pub fn init() {
    unsafe { WRITER.call_once(|| SerialPort::init()) };
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Failed to set logger");
    log::info!(
        r#"
        starting...
    _
   /\ \             __
   \ \ \____  _ __ /\_\     __
    \ \ '__`\/\`'__\/\ \  /'_ `\
     \ \ \L\ \ \ \/ \ \ \/\ \L\ \
      \ \_,__/\ \_\  \ \_\ \____ \
       \/___/  \/_/   \/_/\/___L\ \
                            /\____/
                            \_/__/
"#
    )
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        crate::println!("{} {}", format_level(record.level()), record.args());
    }

    fn flush(&self) {}
}

fn format_level(level: Level) -> &'static str {
    match level {
        Level::Trace => "\x1b[0;35mTRACE\x1b[0m",
        Level::Debug => "\x1b[0;34mDEBUG\x1b[0m",
        Level::Info => "\x1b[0;32mINFO \x1b[0m",
        Level::Warn => "\x1b[0;33mWARN \x1b[0m",
        Level::Error => "\x1b[0;31mERROR\x1b[0m",
    }
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging
#[macro_export]
macro_rules! dbg {
    () => {
        log::debug!("[{}:{}]", core::file!(), core::line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                log::debug!("[{}:{}] {} = {:#x?}",
                core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

/// Prints to the console with newline and carriage return
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n\r"));
    ($($arg:tt)*) => ($crate::print!("{}\n\r", format_args!($($arg)*)));
}

/// Prints to the console
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logger::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe { WRITER.get_mut() }
        .unwrap()
        .write_fmt(format_args!("{}\0", args))
        .unwrap();
}
