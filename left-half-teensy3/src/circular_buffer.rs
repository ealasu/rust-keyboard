pub struct CircularBuffer<T> {
    pub data: [T; 4],
    pub write_pos: usize,
}

impl<T> CircularBuffer<T> {
    pub fn push(&mut self, item: T) {
        self.data[self.write_pos] = item;
        self.write_pos += 1;
        if self.write_pos == self.data.len() {
            self.write_pos = 0;
        }
    }
}
