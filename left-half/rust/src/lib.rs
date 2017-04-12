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
extern crate futures;
extern crate framed;

#[macro_use]
mod macros;
pub mod lang_items;

use core::slice;
use core::mem;
use cortex_m::asm::nop;
use wiring::gpio_impl::{GpioImpl};
use wiring::gpio::{Gpio, PinId};
use wiring::gpio::PinMode::*;
use wiring::gpio::PinState::*;
use wiring::serial::Serial;
use kbd::keys::Keys;
use kbd::decoder::KeyReport;
use framed::stream::FrameStream;
use futures::Async;
use futures::stream::Stream;

const LED: PinId = 26;

#[no_mangle]
pub extern fn kbd_run_loop() {
    // flash the LED once
    let mut gpio = GpioImpl;
    gpio.pin_mode(LED, Output);
    gpio.digital_write(LED, High);
    delay_with_nop();
    gpio.digital_write(LED, Low);
    delay_with_nop();

    let mut left_matrix = kbd::matrix::Matrix::new(
        GpioImpl,
        &[ // rows
            // TODO
        ],
        &[ // cols
            // TODO
        ]
    );
    let mut decoder = kbd::decoder::Decoder::new();
    let mut buf = [0u8; 4];
    let mut stream = FrameStream::new(Serial, &mut buf, |buf| Keys::read(buf));
    let mut right_keys = Keys::none();

    loop {
        while let Async::Ready(v) = stream.poll().unwrap() {
            right_keys = v.unwrap();

            gpio.digital_write(LED, High);
            delay_with_nop();
            gpio.digital_write(LED, Low);
            delay_with_nop();
        }
        let left_keys = left_matrix.scan();
        decoder.update(left_keys, right_keys, |report| {
            let report_ptr: *const KeyReport = &report;
            let data = unsafe {
                slice::from_raw_parts(report_ptr as *const u8, mem::size_of::<KeyReport>())
            };
            //wiring::hid_send_report(data);
            //wiring::debug_serial_write('u' as u8);
        });

        //gpio.digital_write(LED, High);
        //delay_with_nop();
        //gpio.digital_write(LED, Low);
        //delay_with_nop();
        //wiring::hid_send_report(4, &[0,0,0,0,0,0,0,0]);
        //wiring::send_key_report(wiring::KeyReport {
            //reserved: 0,
            //modifiers: 0,
            //keys: [0,0,0,0,0,0],
        //});

        //gpio.digital_write(LED, High);
        //delay_with_nop();
        //gpio.digital_write(LED, Low);
        //delay_with_nop();
        //wiring::hid_send_report(4, &[0,0,0xE9,0,0,0,0,0]);
        //wiring::send_key_report(wiring::KeyReport {
            //reserved: 0,
            //modifiers: 0,
            //keys: [16,0,0,0,0,0],
        //});
    }
}

fn delay_with_nop() {
    for _ in 0..1000000 {
        nop();
    }
}


