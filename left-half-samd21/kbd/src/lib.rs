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

#[macro_use]
mod macros;
mod lang_items;
//mod wiring;

use cortex_m::asm::nop;
use wiring::PinMode::*;
use wiring::PinState::*;

const LED: usize = 26;

#[no_mangle]
pub extern fn kbd_run_loop() {
    wiring::pin_mode(LED, Output);
    //wiring::serial_read();
    loop {
        wiring::digital_write(LED, High);
        delay_with_nop();
        wiring::digital_write(LED, Low);
        delay_with_nop();
    }
}

fn delay_with_nop() {
    for _ in 0..1000000 {
        nop();
    }
}
