use msg::Msg;

pub struct MsgReader {
    buf: [u8; 5],
}

impl MsgReader {
    pub fn new() -> Self {
        MsgReader {
            buf: [0; 5]
        }
    }

    pub fn read(&mut self, next_byte: u8) -> Option<Msg> {
        self.buf[0] = self.buf[1];
        self.buf[1] = self.buf[2];
        self.buf[2] = self.buf[3];
        self.buf[3] = self.buf[4];
        self.buf[4] = next_byte;
        Msg::read(&self.buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut r = MsgReader::new();
        assert_eq!(r.read(0), None);

        assert_eq!(r.read(0b10101010), None);
        assert_eq!(r.read(0x56), None);
        assert_eq!(r.read(0x34), None);
        assert_eq!(r.read(0x12), None);
        assert_eq!(r.read(47), Some(Msg(0x123456)));

        assert_eq!(r.read(0b10101010), None);
        assert_eq!(r.read(0x56), None);
        assert_eq!(r.read(0x34), None);
        assert_eq!(r.read(0x12), None);
        assert_eq!(r.read(47), Some(Msg(0x123456)));

        assert_eq!(r.read(0), None);

        assert_eq!(r.read(0b10101010), None);
        assert_eq!(r.read(0x56), None);
        assert_eq!(r.read(0x34), None);
        assert_eq!(r.read(0x12), None);
        assert_eq!(r.read(47), Some(Msg(0x123456)));

        assert_eq!(r.read(0), None);
        assert_eq!(r.read(0), None);
        assert_eq!(r.read(0), None);
        assert_eq!(r.read(0), None);
        assert_eq!(r.read(0), None);
    }
}
