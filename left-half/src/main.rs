#![no_std]
#![no_main]

#[macro_use] extern crate teensy3;

mod circular_buffer;

use circular_buffer::CircularBuffer;
use teensy3::bindings as t;
use teensy3::serial::Serial;

static mut RIGHT_KEYS: CircularBuffer<u8> = CircularBuffer {
    data: [0; 4],
    write_pos: 0,
};

#[no_mangle]
pub unsafe extern fn main() {
    t::Wire.begin2(44);
    t::Wire.onReceive(Some(on_receive));

    // Blink Loop

    t::pinMode(13, t::OUTPUT as u8);
    t::digitalWrite(13, t::LOW as u8);
    let ser = Serial{};
    loop {
        // Show we are alive
        alive();

        // If the serial write fails, we will halt (no more alive blinks)
        hello(&ser).unwrap();

        // Don't spam the console
        t::delay(1000);
    }
}

/// Blink the light twice to know we're alive
unsafe fn alive() {
    for _ in 0..2 {
        t::digitalWrite(13, t::LOW as u8);
        t::delay(200);
        t::digitalWrite(13, t::HIGH as u8);
        t::delay(200);
        t::digitalWrite(13, t::LOW as u8);
        t::delay(200);
    }
}

/// Send a message over the USB Serial port
fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}

pub unsafe extern fn on_receive(c: i32) {
    for _ in 0..c {
        RIGHT_KEYS.push(t::Wire.read() as u8);
    }
}
