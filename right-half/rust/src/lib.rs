#![feature(lang_items)]
#![no_std]

extern crate wiring;
extern crate kbd;
extern crate framed;
extern crate futures;

mod lang_items;

use wiring::gpio_impl::GpioImpl;
use wiring::serial::Serial;
use framed::sink::FrameSink;
use kbd::msg::Msg;
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
    let mut buf = [0u8; 5];
    let mut sink = FrameSink::<_,Msg,_>::new(serial, |item, buf| {
        buf.copy_from_slice(&item.write());
    }, &mut buf);

    loop {
        let keys = matrix.scan();
        let msg = Msg(keys);
        while let AsyncSink::NotReady(_) = sink.start_send(msg).unwrap() {}
        while let Async::NotReady = sink.poll_complete().unwrap() {}

        //wiring::digital_write(LED, wiring::PinState::High);
        //wiring::delay(300);
        //wiring::digital_write(LED, wiring::PinState::Low);
        //wiring::delay(300);
        //wiring::serial_write(59); // ';'
    }
}
