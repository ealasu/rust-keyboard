use crc8::{self, Crc8};

mod frame_stream;
mod frame_sink;

const SOF: u8 = 0b01111110;
const ESC: u8 = 0b01111101;
const ESC_SOF: u8 = 0b01011110;
const ESC_ESC: u8 = 0b01011101;
static CRC: &Crc8 = &crc8::predefined::MAXIM;
