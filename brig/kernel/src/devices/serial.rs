use core::fmt;

const SERIAL_IO_PORT: u16 = 0x3F8;

pub struct SerialPort(uart_16550::SerialPort);

impl SerialPort {
    pub fn init() -> Self {
        let mut serial_port = unsafe { uart_16550::SerialPort::new(SERIAL_IO_PORT) };
        serial_port.init();
        Self(serial_port)
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            self.0.send(*byte)
        }
        Ok(())
    }
}
