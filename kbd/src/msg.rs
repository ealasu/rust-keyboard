use crc8::Crc8;

lazy_static! {
    static ref CRC: Crc8 = Crc8::new_maxim();
}

const MAGIC: u8 = 0b10101010;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Msg(pub u32);

impl Msg {
    pub fn read(buf: &[u8; 5]) -> Option<Msg> {
        if buf[0] != MAGIC {
            return None;
        }
        if buf[4] != CRC.calc_buf(&buf[0..4]) {
            return None;
        }
        Some(Msg(
                  buf[1] as u32 |
                ((buf[2] as u32) << 8) |
                ((buf[3] as u32) << 16)
        ))
    }

    pub fn write(&self) -> [u8; 5] {
        let bytes = [
            MAGIC,
            (self.0 & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
        ];
        [
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            CRC.calc_buf(&bytes),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(bits: u32, expected_bytes: [u8; 5]) {
        let msg = Msg(bits);
        let buf = msg.write();
        assert_eq!(&buf[..], &expected_bytes[..]);
        assert_eq!(Msg::read(&buf), Some(msg));
    }

    #[test]
    fn test_good() {
        run_test(0x123456, [0b10101010, 0x56, 0x34, 0x12, 47]);
        run_test(0, [0b10101010, 0, 0, 0, 178]);
    }

    #[test]
    fn test_bad() {
        assert_eq!(Msg::read(&[0,0,0,0,0]), None);
        assert_eq!(Msg::read(&[0b10101010,0,0,0,0]), None);
    }
}
