use {crate::devices::Device, core::fmt};

pub trait SerialPort {
    fn write_byte(b: u8);
    fn write_bytes(b: &[u8]);
}

pub struct UART16550Device(uart_16550::SerialPort);

impl UART16550Device {
    pub fn new(io_port: u16) -> Self {
        Self(unsafe { uart_16550::SerialPort::new(io_port) })
    }
}

impl Device for UART16550Device {
    fn configure(&mut self) {
        self.0.init();
    }
}

impl fmt::Write for UART16550Device {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}
