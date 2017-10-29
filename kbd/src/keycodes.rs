use keycode::KeyCode;

macro_rules! keycodes_modifier {
  ( $( $name:ident = $id:expr; )+ ) => {
    $( pub const $name: KeyCode = KeyCode::Modifier($id);)+
  }
}

macro_rules! keycodes_keyboard {
  ( $( $name:ident = $id:expr; )+ ) => {
    $( pub const $name: KeyCode = KeyCode::Keyboard($id);)+
  }
}

macro_rules! keycodes_media {
  ( $( $name:ident = $id:expr; )+ ) => {
    $( pub const $name: KeyCode = KeyCode::Media($id);)+
  }
}

keycodes_modifier! {
  LEFT_CTRL               = 0xE0;
  LEFT_SHIFT              = 0xE1;
  LEFT_ALT                = 0xE2;
  LEFT_GUI                = 0xE3;
  RIGHT_CTRL              = 0xE4;
  RIGHT_SHIFT             = 0xE5;
  RIGHT_ALT               = 0xE6;
  RIGHT_GUI               = 0xE7;
}

keycodes_keyboard! {
  A               =  4;
  B               =  5;
  C               =  6;
  D               =  7;
  E               =  8;
  F               =  9;
  G               = 10;
  H               = 11;
  I               = 12;
  J               = 13;
  K               = 14;
  L               = 15;
  M               = 16;
  N               = 17;
  O               = 18;
  P               = 19;
  Q               = 20;
  R               = 21;
  S               = 22;
  T               = 23;
  U               = 24;
  V               = 25;
  W               = 26;
  X               = 27;
  Y               = 28;
  Z               = 29;
  N1               = 30;
  N2               = 31;
  N3               = 32;
  N4               = 33;
  N5               = 34;
  N6               = 35;
  N7               = 36;
  N8               = 37;
  N9               = 38;
  N0               = 39;
  ENTER           = 40;
  ESC             = 41;
  BACKSPACE       = 42;
  TAB             = 43;
  SPACE           = 44;
  MINUS           = 45;
  EQUAL           = 46;
  LEFT_BRACE      = 47;
  RIGHT_BRACE     = 48;
  BACKSLASH       = 49;
  NON_US_NUM      = 50;
  SEMICOLON       = 51;
  QUOTE           = 52;
  TILDE           = 53;
  COMMA           = 54;
  DOT             = 55;
  SLASH           = 56;
  CAPSLOCK        = 57;
  F1              = 58;
  F2              = 59;
  F3              = 60;
  F4              = 61;
  F5              = 62;
  F6              = 63;
  F7              = 64;
  F8              = 65;
  F9              = 66;
  F10             = 67;
  F11             = 68;
  F12             = 69;

  SUPER = 101;
}

keycodes_media! {
  // Some keys might only work with linux
  POWER	= 0x30;
  SLEEP = 0x32;
  RECORD = 0xB2;
  FAST_FORWARD	= 0xB3;
  REWIND	= 0xB4;
  NEXT	= 0xB5;
  PREVIOUS	= 0xB6;
  STOP	= 0xB7;
  PLAY_PAUSE	= 0xCD;
  PAUSE	= 0xB0;
  VOLUME_MUTE	= 0xE2;
  VOLUME_UP	= 0xE9;
  VOLUME_DOWN	= 0xEA;
}
