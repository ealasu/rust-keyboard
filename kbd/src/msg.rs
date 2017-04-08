#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Msg(pub u32);

impl Msg {
    pub fn read(buf: &[u8; 3]) -> Msg {
        Msg(
            buf[0] as u32 |
            ((buf[1] as u32) << 8) |
            ((buf[2] as u32) << 16)
        )
    }

    pub fn write(&self) -> [u8; 3] {
        [
            (self.0 & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(bits: u32, expected_bytes: [u8; 3]) {
        let msg = Msg(bits);
        let buf = msg.write();
        assert_eq!(&buf[..], &expected_bytes[..]);
        assert_eq!(Msg::read(&buf), msg);
    }

    #[test]
    fn test_good() {
        run_test(0x123456, [0x56, 0x34, 0x12]);
        run_test(0, [0, 0, 0]);
    }
}
