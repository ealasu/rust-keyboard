#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;
extern crate futures;
extern crate crc8;

#[macro_use]
mod macros;
mod constants;
#[cfg(test)]
mod sync_sink;
pub mod stream;
pub mod sink;

pub use stream::FrameStream;
pub use sink::FrameSink;
