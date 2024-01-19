use {crate::serial::SerialPort, core::fmt, lazy_static::lazy_static, spin::Mutex};

lazy_static! {
    /// Global console writer
    static ref WRITER: Mutex<SerialPort> = Mutex::new(SerialPort::init());
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", core::file!(), core::line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
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
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(format_args!("{}\0", args)).unwrap();
}
