pub trait Serial {
    fn read() -> Option<u8>;
    fn write(v: u8);
}
