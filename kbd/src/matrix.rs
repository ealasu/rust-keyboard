use wiring::{pin_mode, digital_write, digital_read, PinMode, PinState};

pub struct Matrix<'a> {
    pub row_pins: &'a [usize],
    pub col_pins: &'a [usize],
}

impl<'a> Matrix<'a> {
    pub fn init(&self) {
        for &pin in self.col_pins.iter() {
            pin_mode(pin, PinMode::Input);
            digital_write(pin, PinState::High); // TODO: make sure this enables pull-up
        }
        for &pin in self.row_pins.iter() {
            pin_mode(pin, PinMode::Output);
            digital_write(pin, PinState::High);
        }
    }

    pub fn scan(&self) -> u32 {
        let mut res = 0;
        let mut key_idx = 0;
        for &row_pin in self.row_pins.iter() {
            digital_write(row_pin, PinState::Low);
            for &col_pin in self.col_pins.iter() {
                let state = digital_read(col_pin);
                if state == PinState::High {
                    res |= 1 << key_idx;
                }
                key_idx += 1;
            }
        }
        res
    }
}
