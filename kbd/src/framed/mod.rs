use crc8::{self, Crc8};

pub mod frame_stream;
pub mod frame_sink;
pub mod sync_sink;

const SOF: u8 = 0b01111110;
const ESC: u8 = 0b01111101;
const ESC_SOF: u8 = 0b01011110;
const ESC_ESC: u8 = 0b01011101;
static CRC: &Crc8 = &crc8::predefined::MAXIM;
