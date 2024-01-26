//! Logger implementation

use {
    crate::devices::{serial::UART16550Device, Device},
    core::fmt::{self, Write},
    log::{Level, LevelFilter, Log, Metadata, Record},
    spin::Once,
};

/*struct QemuWriter;

impl fmt::Write for QemuWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            unsafe { x86::io::outb(0xe9, *byte) };
        }
        Ok(())
    }
}*/

/// Global console writer
static mut WRITER: Once<UART16550Device> = Once::INIT;

static LOGGER: Logger = Logger;

// This is sort of a hack -- because we need the serial port REALLY early.
// Possible solution - use QEMU's debugcon support, until devices have been
// loaded Then switch the WRITER
const SERIAL_IO_PORT: u16 = 0x3F8;

/// Initialise logger using the xen::console backend
pub fn init() {
    unsafe {
        WRITER.call_once(|| {
            let mut sp = UART16550Device::new(SERIAL_IO_PORT);
            sp.configure();
            sp
        })
    };

    log::set_logger(&LOGGER).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Trace);

    log::info!(
        r#"
    starting...
 __
/\ \             __
\ \ \____  _ __ /\_\     __             __4___
 \ \  __ \/\  __\/\ \  / _  \        _  \ \ \ \
  \ \ \L\ \ \ \/ \ \ \/\ \L\ \      <'\ /_/_/_/
   \ \____/\ \_\  \ \_\ \____ \      ((____!___/)
    \/___/  \/_/   \/_/\/___L\ \      \0\0\0\0\/
                         /\____/   ~~~~~~~~~~~~~~~~
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

    //unsafe { WRITER.write_fmt(format_args!("{}\0", args)).unwrap() };
}
