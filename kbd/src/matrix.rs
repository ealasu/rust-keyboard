use core::marker::PhantomData;
use wiring::gpio::{Gpio, PinId, PinMode, PinState};
use wiring::gpio_impl::GpioImpl;
use keys::Keys;

pub struct Matrix<'a, G> {
    row_pins: &'a [PinId],
    col_pins: &'a [PinId],
    gpio: G,
}

impl<'a, G: Gpio> Matrix<'a, G> {
    pub fn new(mut gpio: G, row_pins: &'a [PinId], col_pins: &'a [PinId]) -> Self {
        for &pin in col_pins.iter() {
            gpio.pin_mode(pin, PinMode::Input);
            gpio.digital_write(pin, PinState::High); // TODO: make sure this enables pull-up
        }
        for &pin in row_pins.iter() {
            gpio.pin_mode(pin, PinMode::Output);
            gpio.digital_write(pin, PinState::High);
        }
        Matrix {
            row_pins: row_pins,
            col_pins: col_pins,
            gpio: gpio,
        }
    }

    pub fn scan(&mut self) -> Keys {
        let mut res = Keys::none();
        let mut key_idx = 0;
        for &row_pin in self.row_pins.iter() {
            self.gpio.digital_write(row_pin, PinState::Low);
            for &col_pin in self.col_pins.iter() {
                let state = self.gpio.digital_read(col_pin);
                if state == PinState::High {
                    res.set_key(key_idx, true);
                }
                key_idx += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiring::gpio_mock::GpioMock;

    #[test]
    fn none() {
        let gpio = GpioMock;
        let row_pins = [1,2];
        let col_pins = [3,4];
        let mut unit = Matrix::new(gpio, &row_pins, &col_pins);
        let res = unit.scan();
        assert_eq!(res, Keys::none());
    }
}
