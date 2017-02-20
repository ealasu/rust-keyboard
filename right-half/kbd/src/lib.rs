#![feature(lang_items)]
#![no_std]

mod lang_items;
mod wiring;

const LED: u8 = 2;

#[no_mangle]
pub extern fn kbd_run_loop() {
    wiring::pin_mode(LED, wiring::PinMode::Output);
    loop {
        wiring::digital_write(LED, wiring::PinState::High);
        wiring::delay(300);
        wiring::digital_write(LED, wiring::PinState::Low);
        wiring::delay(300);
    }
}
