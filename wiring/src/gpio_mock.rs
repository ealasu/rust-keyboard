use gpio::{Gpio, PinId, PinMode, PinState};

pub struct GpioMock;

impl Gpio for GpioMock {
    fn pin_mode(pin: PinId, mode: PinMode) {
    }

    fn digital_write(pin: PinId, state: PinState) {
    }

    fn digital_read(pin: PinId) -> PinState {
        PinState::Low
    }
}
