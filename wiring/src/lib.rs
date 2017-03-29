#![no_std]

pub mod gpio;
pub mod gpio_impl;
pub mod serial;
pub mod serial_impl;
pub mod serial_mock;


pub fn delay(ms: u32) {
    unsafe { sys::delay(ms) }
}

pub fn debug_serial_write(v: u8) -> u8 {
    unsafe { sys::debug_serial_write(v) }
}

//pub fn send_key_report(keys: KeyReport) {
    //unsafe { sys::send_key_report(keys) }
//}

pub fn hid_send_report(report_id: u8, data: &[u8]) {
    unsafe { sys::hid_send_report(report_id, data.as_ptr(), data.len()) }
}

mod sys {
    extern {
        pub fn delay(ms: u32);
        pub fn debug_serial_write(b: u8) -> u8;
        //pub fn send_key_report(keys: KeyReport);
        pub fn hid_send_report(report_id: u8, data: *const u8, len: usize);
    }
}
