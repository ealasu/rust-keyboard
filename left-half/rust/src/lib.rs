#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]

#[cfg(feature = "semihosting")]
#[macro_use]
extern crate cortex_m_semihosting;
extern crate compiler_builtins;
#[macro_use]
extern crate cortex_m;
extern crate wiring;
extern crate kbd;

#[macro_use]
mod macros;
mod lang_items;

use cortex_m::asm::nop;
use wiring::PinMode::*;
use wiring::PinState::*;

const LED: usize = 26;

#[no_mangle]
pub extern fn kbd_run_loop() {
    // flash the LED once
    wiring::pin_mode(LED, Output);
    wiring::digital_write(LED, High);
    delay_with_nop();
    wiring::digital_write(LED, Low);

    let mut msg_reader = kbd::msg_reader::MsgReader::new();
    let left_matrix = kbd::matrix::Matrix {
        row_pins: &[
            // TODO
        ],
        col_pins: &[
            // TODO
        ],
    };
    left_matrix.init();
    let mut decoder = kbd::decoder::Decoder::new();

    let mut right_keys = 0;

    loop {
        while let Some(v) = wiring::serial_read() {
            if let Some(scan) = msg_reader.read(v) {
                right_keys = scan.0;
            }
        }
        let left_keys = left_matrix.scan();
        decoder.update(left_keys, right_keys, |state| {
            // TODO send to usb
            wiring::debug_serial_write('u' as u8);
        });
    }
}

fn delay_with_nop() {
    for _ in 0..1000000 {
        nop();
    }
}


#[repr(C)]
struct KeyReport {
  modifiers: u8,
  reserved: u8,
  keys: [u8; 6],
}
