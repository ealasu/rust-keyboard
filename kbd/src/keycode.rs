#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyCode {
    Modifier(u8),
    Keyboard(u8),
    Media(u16),
}
