use keycode::KeyCode;
use keycodes;

pub enum Keys {
  None,
  One(KeyCode),
  Two(KeyCode, KeyCode),
}

macro_rules! keyboard_layout_key {
  ([nil]) => {
    [None, None, None]
  };
  ([$k1:ident]) => {
    [Some($k1), None, None]
  };
  ([$k1:tt, $k2:tt]) => {
    [Some($k1), Some($k2), None]
  };
}

macro_rules! keyboard_layout {
  ( $( $key:tt , )+ ) => {
    [$( keyboard_layout_key!($key), )+]
  };
}

const LAYOUT: [[Option<KeyCode>; 3]; 3] = keyboard_layout![
  [ nil ],
  [keycodes::A],
  [keycodes::LEFT_SHIFT, keycodes::B],
];
