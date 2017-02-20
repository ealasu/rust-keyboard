mod sys {
    extern {
        pub fn digitalWrite(pin: u8, state: u8);
        pub fn digitalRead(pin: u8) -> u16;
        pub fn pinMode(pin: u8, mode: u8);
        pub fn delay(ms: u32);
    }
}

pub enum PinMode {
    Input = 0,
    Output = 1,
}

pub fn pin_mode(pin: u8, mode: PinMode) {
    unsafe { sys::pinMode(pin, mode as u8) };
}

pub enum PinState {
    Low = 0,
    High = 1,
}

pub fn digital_write(pin: u8, state: PinState) {
    unsafe { sys::digitalWrite(pin, state as u8) };
}

pub fn digital_read(pin: u8) -> PinState {
    let v = unsafe { sys::digitalRead(pin) };
    match v {
        v if v == PinState::Low as u16 => PinState::Low,
        v if v == PinState::High as u16 => PinState::High,
        _ => panic!()
    }
}

pub fn delay(ms: u32) {
    unsafe { sys::delay(ms) }
}
