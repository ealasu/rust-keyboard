#![no_std]

pub enum PinMode {
    Input = 0,
    Output = 1,
}

pub enum PinState {
    Low = 0,
    High = 1,
}

#[cfg(target_arch = "msp430")]
pub fn pin_mode(pin: usize, mode: PinMode) {
    unsafe { sys::pinMode(pin as u8, mode as u8) };
}

#[cfg(target_arch = "arm")]
pub fn pin_mode(pin: usize, mode: PinMode) {
    unsafe { sys::pinMode(pin as u32, mode as u32) };
}


#[cfg(target_arch = "msp430")]
pub fn digital_write(pin: usize, state: PinState) {
    unsafe { sys::digitalWrite(pin as u8, state as u8) };
}

#[cfg(target_arch = "arm")]
pub fn digital_write(pin: usize, state: PinState) {
    unsafe { sys::digitalWrite(pin as u32, state as u32) };
}

#[cfg(target_arch = "msp430")]
pub fn digital_read(pin: usize) -> PinState {
    let v = unsafe { sys::digitalRead(pin as u8) };
    match v {
        v if v == PinState::Low as u16 => PinState::Low,
        v if v == PinState::High as u16 => PinState::High,
        _ => panic!()
    }
}

#[cfg(target_arch = "arm")]
pub fn digital_read(pin: usize) -> PinState {
    let v = unsafe { sys::digitalRead(pin as u32) };
    match v {
        v if v == PinState::Low as u32 => PinState::Low,
        v if v == PinState::High as u32 => PinState::High,
        _ => panic!()
    }
}

pub fn delay(ms: u32) {
    unsafe { sys::delay(ms) }
}

pub fn serial_write(v: u8) -> u8 {
    unsafe { sys::serial_write(v) }
}

pub fn serial_read() -> Option<u8> {
    let v = unsafe { sys::serial_read() };
    if v < 0 {
        None
    } else {
        Some(v as u8)
    }
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
    use super::*;

    extern {
        pub fn delay(ms: u32);
        pub fn serial_write(b: u8) -> u8;
        pub fn debug_serial_write(b: u8) -> u8;
        //pub fn send_key_report(keys: KeyReport);
        pub fn hid_send_report(report_id: u8, data: *const u8, len: usize);
    }

    #[cfg(target_arch = "msp430")]
    extern {
        pub fn digitalWrite(pin: u8, state: u8);
        pub fn digitalRead(pin: u8) -> u16;
        pub fn pinMode(pin: u8, mode: u8);
        pub fn serial_read() -> i16;
    }

    #[cfg(target_arch = "arm")]
    extern {
        pub fn digitalWrite(pin: u32, state: u32);
        pub fn digitalRead(pin: u32) -> u32;
        pub fn pinMode(pin: u32, mode: u32);
        pub fn serial_read() -> i32;
    }
}
