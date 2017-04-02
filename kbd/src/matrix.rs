use core::marker::PhantomData;
use wiring::gpio::{Gpio, PinId, PinMode, PinState};
use wiring::gpio_impl::GpioImpl;

pub struct Matrix<'a, G: Gpio> {
    row_pins: &'a [PinId],
    col_pins: &'a [PinId],
    gpio: PhantomData<G>,
}

impl<'a, G: Gpio> Matrix<'a, G> {
    pub fn new(row_pins: &'a [PinId], col_pins: &'a [PinId]) -> Self {
        for &pin in col_pins.iter() {
            G::pin_mode(pin, PinMode::Input);
            G::digital_write(pin, PinState::High); // TODO: make sure this enables pull-up
        }
        for &pin in row_pins.iter() {
            G::pin_mode(pin, PinMode::Output);
            G::digital_write(pin, PinState::High);
        }
        Matrix {
            row_pins: row_pins,
            col_pins: col_pins,
            gpio: PhantomData,
        }
    }

    pub fn scan(&self) -> u32 {
        let mut res = 0;
        let mut key_idx = 0;
        for &row_pin in self.row_pins.iter() {
            G::digital_write(row_pin, PinState::Low);
            for &col_pin in self.col_pins.iter() {
                let state = G::digital_read(col_pin);
                if state == PinState::High {
                    res |= 1 << key_idx;
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
    fn test() {
        let row_pins = [1,2];
        let col_pins = [3,4];
        let unit = Matrix::<GpioMock>::new(&row_pins, &col_pins);
        let res = unit.scan();
        assert_eq!(res, 0);
    }
}
