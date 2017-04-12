#![feature(lang_items)]
#![no_std]

extern crate wiring;
extern crate kbd;
extern crate framed;
extern crate futures;

pub mod lang_items;

use wiring::gpio_impl::GpioImpl;
use wiring::serial::Serial;
use framed::sink::FrameSink;
use kbd::keys::Keys;
use futures::{Async, AsyncSink};
use futures::sink::Sink;

//const LED: usize = 2;


#[no_mangle]
pub extern fn kbd_run_loop() {
    //wiring::pin_mode(LED, wiring::PinMode::Output);

    let mut matrix = kbd::matrix::Matrix::new(GpioImpl,
        &[ // rows
            // TODO
        ],
        &[ // cols
            // TODO
        ]
    );

    let serial = Serial;
    let mut buf = [0u8; 6];
    let mut sink = FrameSink::<_,Keys,_>::new(serial, |item, buf| {
        item.write(buf);
    }, &mut buf);

    loop {
        let keys = matrix.scan();
        while let AsyncSink::NotReady(_) = sink.start_send(keys).unwrap() {}
        while let Async::NotReady = sink.poll_complete().unwrap() {}

        //wiring::digital_write(LED, wiring::PinState::High);
        //wiring::delay(300);
        //wiring::digital_write(LED, wiring::PinState::Low);
        //wiring::delay(300);
        //wiring::serial_write(59); // ';'
    }
}
