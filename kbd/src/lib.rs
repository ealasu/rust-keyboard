#![cfg_attr(not(feature = "std"), no_std)]
#![feature(conservative_impl_trait)]

#[cfg(feature = "std")]
extern crate core;
extern crate wiring;
extern crate bit_field;
extern crate byteorder;

pub mod keys;
pub mod matrix;
pub mod decoder;
pub mod keycode;
pub mod keycodes;
pub mod layout;
