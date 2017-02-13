#![no_std]
#![no_main]

extern crate board;
extern crate cortex_m;

use board::periph;
use cortex_m::asm::nop;

const LED_PIN: u32 = 1 << 16; // pin 16

#[no_mangle]
pub unsafe fn main() -> ! {
    // Enable system clock on GPIO port B
    periph::sim().scgc5.modify(|_,w| w.portb(true));

    // Configure the led pin
    periph::portb().pcr16.modify(|_, w| w.dse(true).pe(true).ps(true).mux(1));

    // Set the led pin to output
    periph::ptb().pddr.modify(|_, w| w.pdd(LED_PIN));

    blink_led();
}

fn led_on() {
    periph::ptb().pdor.modify(|_, w| w.pdo(LED_PIN));
}

fn led_off() {
    periph::ptb().pdor.modify(|_, w| w.pdo(0));
}

fn delay(ms: i32) {
    for _ in 0..ms * 1250 {
        nop();
    }
}

fn blink_led() -> ! {
    loop {
        led_on();
        delay(1000);
        led_off();
        delay(1000);
    }
}
