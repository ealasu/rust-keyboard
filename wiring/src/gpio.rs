#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PinMode {
    Input = 0,
    Output = 1,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PinState {
    Low = 0,
    High = 1,
}

pub trait Gpio {
    fn pin_mode(pin: u8, mode: PinMode);
    fn digital_write(pin: u8, state: PinState);
    fn digital_read(pin: u8) -> PinState;
}
