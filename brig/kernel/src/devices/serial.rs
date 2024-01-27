use core::fmt;

pub trait SerialPort {
    fn write_byte(b: u8);
    fn write_bytes(b: &[u8]);
}

pub struct UART16550Device(uart_16550::SerialPort);

impl UART16550Device {
    pub fn new(io_port: u16) -> Self {
        let mut serial_port = unsafe { uart_16550::SerialPort::new(io_port) };
        serial_port.init();
        Self(serial_port)
    }
}

impl fmt::Write for UART16550Device {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}
