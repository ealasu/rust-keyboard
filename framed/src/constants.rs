use crc8::{self, Crc8};

pub const SOF: u8 = 0b01111110;
pub const ESC: u8 = 0b01111101;
pub const ESC_SOF: u8 = 0b01011110;
pub const ESC_ESC: u8 = 0b01011101;

pub static CRC: &Crc8 = &crc8::predefined::MAXIM;
