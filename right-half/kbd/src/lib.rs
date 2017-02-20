#![feature(lang_items)]
//#![feature(no_core)]
#![no_std]
//#![no_core]

mod lang_items;
mod wiring;

const LED: u8 = 2;

#[no_mangle]
pub extern fn kbd_run_loop() {
    loop {
        wiring::digital_write(LED, wiring::PinState::High);
        wiring::delay(1000);
        wiring::digital_write(LED, wiring::PinState::Low);
        wiring::delay(1000);
    }
}

//#[lang = "copy"]
//trait Copy {}

//#[lang = "sized"]
//trait Sized {}
