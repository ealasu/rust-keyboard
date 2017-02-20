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

#[macro_use]
mod macros;
mod lang_items;
mod wiring;


use cortex_m::asm::nop;


#[no_mangle]
pub extern fn kbd_run_loop() {
    wiring::pin_mode(26, wiring::PinMode::Output);
    loop {
        wiring::digital_write(26, wiring::PinState::High);
        delay_with_nop();
        wiring::digital_write(26, wiring::PinState::Low);
        delay_with_nop();
    }
}

fn delay_with_nop() {
    for _ in 0..1000000{
        nop();
    }
}
