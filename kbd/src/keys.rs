use bit_field::BitField;
use byteorder::{ByteOrder, LittleEndian as LE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keys(pub u32);

impl Keys {
    pub fn none() -> Self {
        Keys(0)
    }

    pub fn read(buf: &[u8]) -> Self {
        Keys(LE::read_u32(buf))
    }

    pub fn write(&self, buf: &mut [u8]) {
        LE::write_u32(buf, self.0);
    }

    pub fn key(&self, index: u8) -> bool {
        self.0.get_bit(index)
    }

    pub fn set_key(&mut self, index: u8, state: bool) {
        self.0.set_bit(index, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(bits: u32, expected_bytes: [u8; 4]) {
        let msg = Keys(bits);
        let mut buf = [0u8; 4];
        msg.write(&mut buf);
        assert_eq!(&buf[..], &expected_bytes[..]);
        assert_eq!(Keys::read(&buf), msg);
    }

    #[test]
    fn test_good() {
        run_test(0x12345678, [0x78, 0x56, 0x34, 0x12]);
        run_test(0, [0, 0, 0, 0]);
    }
}
