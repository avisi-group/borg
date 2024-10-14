//! Logger implementation

use {
    crate::devices::serial::UART16550Device,
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
pub static mut WRITER: Once<UART16550Device> = Once::INIT;

static LOGGER: Logger<5> = Logger {
    default_level: LevelFilter::Trace,
    module_levels: [
        ("virtio_drivers", LevelFilter::Warn),
        ("tar_no_std", LevelFilter::Warn),
        ("elfloader", LevelFilter::Info),
        ("kernel::dbt::x86", LevelFilter::Info),
        ("kernel::dbt::models", LevelFilter::Info),
    ],
};

// This is sort of a hack -- because we need the serial port REALLY early.
// Possible solution - use QEMU's debugcon support, until devices have been
// loaded Then switch the WRITER
const SERIAL_IO_PORT: u16 = 0x3F8;

/// Initialise logger
pub fn init() {
    unsafe { WRITER.call_once(|| UART16550Device::new(SERIAL_IO_PORT)) };

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

struct Logger<const N: usize> {
    default_level: LevelFilter,
    module_levels: [(&'static str, LevelFilter); N],
}

impl<const N: usize> Log for Logger<N> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter()
            <= self
                .module_levels
                .iter()
                /* At this point the Vec is already sorted so that we can simply take
                 * the first match
                 */
                .find(|(name, _level)| metadata.target().starts_with(name))
                .map(|(_name, level)| level)
                .unwrap_or(&self.default_level)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let target = if !record.target().is_empty() {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };

        crate::println!(
            "{} \x1b[0;30m[{}]\x1b[0m {}",
            format_level(record.level()),
            target,
            record.args()
        );
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
        .expect("WRITER not initialized")
        .write_fmt(args)
        .expect("failed to write format args to WRITER");
}
