
pub struct Decoder {
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {}
    }

    pub fn update<F>(&mut self, left: u32, right: u32, cb: F)
    where F: FnOnce(u32) {
        cb(0);
    }
}
