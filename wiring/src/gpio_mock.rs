use gpio::{Gpio, PinId, PinMode, PinState};

pub struct GpioMock;

impl Gpio for GpioMock {
    fn pin_mode(&mut self, pin: PinId, mode: PinMode) {
    }

    fn digital_write(&mut self, pin: PinId, state: PinState) {
    }

    fn digital_read(&self, pin: PinId) -> PinState {
        PinState::Low
    }
}
