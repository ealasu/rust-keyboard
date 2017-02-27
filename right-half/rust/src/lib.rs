#![feature(lang_items)]
#![no_std]

extern crate wiring;
extern crate kbd;

mod lang_items;

//const LED: usize = 2;

#[no_mangle]
pub extern fn kbd_run_loop() {
    //wiring::pin_mode(LED, wiring::PinMode::Output);

    let matrix = kbd::matrix::Matrix {
        row_pins: &[
            // TODO
        ],
        col_pins: &[
            // TODO
        ],
    };
    matrix.init();

    loop {
        let keys = matrix.scan();
        let msg = kbd::msg::Msg(keys);
        let buf = msg.write();
        for &v in buf.iter() {
            wiring::serial_write(v);
        }

        //wiring::digital_write(LED, wiring::PinState::High);
        //wiring::delay(300);
        //wiring::digital_write(LED, wiring::PinState::Low);
        //wiring::delay(300);
        //wiring::serial_write(59); // ';'
    }
}
