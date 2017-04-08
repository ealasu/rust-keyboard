#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")] extern crate core;
extern crate futures;
extern crate crc8;

pub mod stream;
pub mod sink;
pub mod sync_sink;

use crc8::Crc8;

const SOF: u8 = 0b01111110;
const ESC: u8 = 0b01111101;
const ESC_SOF: u8 = 0b01011110;
const ESC_ESC: u8 = 0b01011101;
static CRC: &Crc8 = &crc8::predefined::MAXIM;
