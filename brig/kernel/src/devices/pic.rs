//! Taken from `https://github.com/maestro-os/maestro/blob/master/kernel/src/arch/x86/pic.rs`

use x86::io::{inb, outb};

/// The master PIC's command port.
const MASTER_COMMAND: u16 = 0x20;
/// The master PIC's data port.
const MASTER_DATA: u16 = 0x21;
/// The slave PIC's command port.
const SLAVE_COMMAND: u16 = 0xa0;
/// The slave PIC's data port.
const SLAVE_DATA: u16 = 0xa1;

/// Indicates that ICW4 will be present
const ICW1_ICW4: u8 = 0x01;
/// Single (cascade) mode
const ICW1_SINGLE: u8 = 0x02;
/// Call address interval 4 (8)
const ICW1_INTERVAL4: u8 = 0x04;
/// Level triggered (edge) mode
const ICW1_LEVEL: u8 = 0x08;
/// Initialization
const ICW1_INIT: u8 = 0x10;
/// TODO doc
const ICW3_SLAVE_PIC: u8 = 0x04;
/// TODO doc
const ICW3_CASCADE: u8 = 0x02;
/// 8086/88 (MCS-80/85) mode
const ICW4_8086: u8 = 0x01;
/// Auto (normal) EOI
const ICW4_AUTO: u8 = 0x02;
/// Buffered mode/slave
const ICW4_BUF_SLAVE: u8 = 0x08;
/// Buffered mode/master
const ICW4_BUF_MASTER: u8 = 0x0c;
/// Special fully nested
const ICW4_SFNM: u8 = 0x10;

/// The end-of-interrupt command.
const COMMAND_EOI: u8 = 0x20;

/// Initializes the PIC.
pub fn init(offset1: u8, offset2: u8) {
    unsafe {
        let mask1 = inb(MASTER_DATA);
        let mask2 = inb(SLAVE_DATA);

        outb(MASTER_COMMAND, ICW1_INIT | ICW1_ICW4);
        outb(SLAVE_COMMAND, ICW1_INIT | ICW1_ICW4);

        outb(MASTER_DATA, offset1);
        outb(SLAVE_DATA, offset2);

        outb(MASTER_DATA, ICW3_SLAVE_PIC);
        outb(SLAVE_DATA, ICW3_CASCADE);

        outb(MASTER_DATA, ICW4_8086);
        outb(SLAVE_DATA, ICW4_8086);

        outb(MASTER_DATA, mask1);
        outb(SLAVE_DATA, mask2);
    }
}

/// Enable interruptions on the given IRQ.
pub fn enable_irq(mut n: u8) {
    let port = if n < 8 {
        MASTER_DATA
    } else {
        n -= 8;
        SLAVE_DATA
    };

    unsafe {
        let value = inb(port) & !(1 << n);
        outb(port, value);
    }
}

/// Disable interruptions on the given IRQ.
pub fn disable_irq(mut n: u8) {
    let port = if n < 8 {
        MASTER_DATA
    } else {
        n -= 8;
        SLAVE_DATA
    };

    unsafe {
        let value = inb(port) | (1 << n);
        outb(port, value);
    }
}

/// Sends an End-Of-Interrupt message to the PIC for the given interrupt `irq`.
#[no_mangle]
pub extern "C" fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq >= 0x8 {
            outb(SLAVE_COMMAND, COMMAND_EOI);
        }
        outb(MASTER_COMMAND, COMMAND_EOI);
    }
}
