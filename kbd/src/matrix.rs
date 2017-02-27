pub struct Matrix<'a> {
    pub row_pins: &'a [usize],
    pub col_pins: &'a [usize],
}

impl<'a> Matrix<'a> {
    pub fn init(&self) {
    }

    pub fn scan(&self) -> u32 {
        0
    }
}
