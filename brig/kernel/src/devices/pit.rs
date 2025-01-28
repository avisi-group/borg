//! Only used to calibrate the LAPIC
//!
//! Taken from `https://github.com/maestro-os/maestro/blob/master/kernel/src/time/hw/pit.rs`

use x86::io::outb;

/// PIT channel number 0.
const CHANNEL_0: u16 = 0x40;
/// PIT channel number 1.
const CHANNEL_1: u16 = 0x41;
/// PIT channel number 2.
const CHANNEL_2: u16 = 0x42;
/// The port to send a command to the PIT.
const PIT_COMMAND: u16 = 0x43;

/// The command to enable the PC speaker.
const BEEPER_ENABLE_COMMAND: u8 = 0x61;

/// Select PIT channel 0.
const SELECT_CHANNEL_0: u8 = 0b00 << 6;
/// Select PIT channel 1.
const SELECT_CHANNEL_1: u8 = 0b01 << 6;
/// Select PIT channel 2.
const SELECT_CHANNEL_2: u8 = 0b10 << 6;
/// The read back command, used to read the current state of the PIT (doesn't
/// work on 8253 and older).
const READ_BACK_COMMAND: u8 = 0b11 << 6;

/// Tells the PIT to copy the current count to the latch register to be read by
/// the CPU.
const ACCESS_LATCH_COUNT_VALUE: u8 = 0b00 << 4;
/// Tells the PIT to read only the lowest 8 bits of the counter value.
const ACCESS_LOBYTE: u8 = 0b01 << 4;
/// Tells the PIT to read only the highest 8 bits of the counter value.
const ACCESS_HIBYTE: u8 = 0b10 << 4;
/// Tells the PIT to read the whole counter value.
const ACCESS_LOBYTE_HIBYTE: u8 = 0b11 << 4;

/// Interrupt on terminal count.
const MODE_0: u8 = 0b000 << 1;
/// Hardware re-triggerable one-shot.
const MODE_1: u8 = 0b001 << 1;
/// Rate generator.
const MODE_2: u8 = 0b010 << 1;
/// Square wave generator.
const MODE_3: u8 = 0b011 << 1;
/// Software triggered strobe.
const MODE_4: u8 = 0b100 << 1;
/// Hardware triggered strobe.
const MODE_5: u8 = 0b101 << 1;

/// Tells whether the BCD mode is enabled.
const BCD_MODE: u8 = 0b1;

/// By default, the PIT hardware produces frequencies at 1.19 Mhz
const PIT_OSCILLATION_FREQUENCY: u32 = 1193182;

pub fn init() {
    unsafe {
        outb(
            PIT_COMMAND,
            SELECT_CHANNEL_0 | ACCESS_LOBYTE_HIBYTE | MODE_3,
        )
    };
    // set_frequency_hz(PIT_OSCILLATION_FREQUENCY);
}

pub fn set_frequency_hz(frequency: u32) {
    let mut count = if frequency != 0 {
        u16::try_from(PIT_OSCILLATION_FREQUENCY / frequency).unwrap()
    } else {
        0
    };
    if count == 0xffff {
        count = 0;
    }

    unsafe {
        outb(CHANNEL_0, (count & 0xff) as u8);
        outb(CHANNEL_0, ((count >> 8) & 0xff) as u8);
    }
}
