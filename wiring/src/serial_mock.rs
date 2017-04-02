use serial::{ReadU8, WriteU8};

pub struct SerialMock;

impl ReadU8 for SerialMock {
    fn read(&mut self) -> Option<u8> {
        None
    }
}

impl WriteU8 for SerialMock {
    fn write(&mut self, v: u8) {
    }
}
