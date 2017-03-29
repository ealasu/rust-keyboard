use serial::Serial;

pub struct SerialMock;

impl Serial for SerialMock {
    fn read() -> Option<u8> {
        None
    }

    fn write(v: u8) {
    }
}
