#![cfg_attr(not(feature = "std"), no_std)]

extern crate crc8;
extern crate wiring;

pub mod msg;
pub mod msg_reader;
pub mod matrix;
pub mod decoder;
pub mod keycode;
pub mod keycodes;
pub mod layout;
