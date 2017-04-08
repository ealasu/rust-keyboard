#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")] extern crate core;
extern crate wiring;

pub mod msg;
pub mod matrix;
pub mod decoder;
pub mod keycode;
pub mod keycodes;
pub mod layout;
