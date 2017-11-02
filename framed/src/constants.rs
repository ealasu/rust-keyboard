use crc8::{self, Crc8};

/// Start-Of-Frame
pub const SOF: u8 = 0b01111110;
/// Escape
pub const ESC: u8 = 0b01111101;
/// Escaped Start-Of-Frame
pub const ESC_SOF: u8 = 0b01011110;
/// Escaped escape
pub const ESC_ESC: u8 = 0b01011101;

pub static CRC: &Crc8 = &crc8::predefined::MAXIM;
