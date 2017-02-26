#![feature(lang_items)]
#![no_std]

extern crate wiring;
extern crate kbd;

mod lang_items;

const LED: usize = 2;

#[no_mangle]
pub extern fn kbd_run_loop() {
    wiring::pin_mode(LED, wiring::PinMode::Output);
    loop {
        wiring::digital_write(LED, wiring::PinState::High);
        wiring::delay(300);
        wiring::digital_write(LED, wiring::PinState::Low);
        wiring::delay(300);
        wiring::serial_write(59); // ';'
    }
}
