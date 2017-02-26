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
    wiring::pin_mode(LED, Output);
    wiring::digital_write(LED, High);
    delay_with_nop();
    wiring::digital_write(LED, Low);

    wiring::debug_serial_write('h' as u8);
    wiring::debug_serial_write('i' as u8);
    wiring::debug_serial_write('\n' as u8);

    loop {
        let v = wiring::serial_read();
        if let Some(v) = v {
            //wiring::debug_serial_write('y' as u8);
            wiring::debug_serial_write(v);
        }
        //} else {
            //wiring::debug_serial_write('n' as u8);
        //}

        //if v == Some(58) {
            //wiring::digital_write(LED, High);
            //delay_with_nop();
            //wiring::digital_write(LED, Low);
        //}
        //delay_with_nop();
        //if let Some(v) = v {
            //wiring::debug_serial_write('s' as u8);
            //wiring::debug_serial_write('\n' as u8);
            //wiring::debug_serial_write(v);
            //wiring::debug_serial_write('\n' as u8);
        //} else {
            //wiring::debug_serial_write('n' as u8);
        //}
    }
}

fn delay_with_nop() {
    for _ in 0..1000000 {
        nop();
    }
}
