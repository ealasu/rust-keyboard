#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyCode {
    Modifier(u8),
    Keyboard(u8),
    Media(u16),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FnKeyCode {
    FN0,
    FN1,
    FN2,
}

//#[derive(Copy, Clone, Debug, PartialEq)]
//pub struct ModifierKeyCode(u8);
