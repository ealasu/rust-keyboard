const MAGIC: u8 = 0b10101010;

pub struct Msg {
    bits: u32,
}

impl Msg {
    pub fn read(buf: &[u8; 5]) -> Option<Msg> {
        if buf[0] != MAGIC {
            return None;
        }
        if buf[4] != crc(buf[1..4]) {
            return None;
        }
        Msg {
            bits:
                buf[1] as u32 |
                ((buf[2] as u32) << 7) |
                ((buf[3] as u32) << 14)
        }
    }

    pub fn write(&self) -> [u8; 5] {
        let bytes = [
            (self.bits & 0x7F) as u8,
            (self.bits & (0x7F << 7)) as u8,
            (self.bits & (0x7F << 14)) as u8,
        ];
        [
            MAGIC,
            bytes[0],
            bytes[1],
            bytes[2],
            crc(&bytes),
        ]
    }
}

