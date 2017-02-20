extern {
    fn digitalWrite(pin: u32, state: u32);
    fn digitalRead(pin: u32) -> u32;
    fn pinMode(pin: u32, mode: u32);
}

pub enum PinMode {
    Input = 0,
    Output = 1,
}

pub fn pin_mode(pin: u32, mode: PinMode) {
    unsafe { pinMode(pin, mode as u32) };
}

pub enum PinState {
    Low = 0,
    High = 1,
}

pub fn digital_write(pin: u32, state: PinState) {
    unsafe { digitalWrite(pin, state as u32) };
}

pub fn digital_read(pin: u32) -> PinState {
    let v = unsafe { digitalRead(pin) };
    match v {
        v if v == PinState::Low as u32 => PinState::Low,
        v if v == PinState::High as u32 => PinState::High,
        _ => panic!()
    }
}
