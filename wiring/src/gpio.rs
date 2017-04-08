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

pub type PinId = u8;

pub trait Gpio {
    fn pin_mode(&mut self, pin: PinId, mode: PinMode);
    fn digital_write(&mut self, pin: PinId, state: PinState);
    fn digital_read(&self, pin: PinId) -> PinState;
}
