#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")] extern crate core;
#[macro_use] extern crate futures;
extern crate crc8;
extern crate wiring;

pub mod msg;
pub mod msg_reader;
pub mod matrix;
pub mod decoder;
pub mod keycode;
pub mod keycodes;
pub mod layout;
pub mod frame_stream;
