use keycode::KeyCode;
use keycodes::*;

macro_rules! keyboard_layout_key {
  ([]) => {
    [None, None, None, None]
  };
  ([$k1:path]) => {
    [Some($k1), None, None, None]
  };
  ([$k1:path, $k2:path]) => {
    [Some($k1), Some($k2), None, None]
  };
  ([$k1:path, $k2:path, $k3:path]) => {
    [Some($k1), Some($k2), Some($k3), None]
  };
  ([$k1:path, $k2:path, $k3:path, $k4:path]) => {
    [Some($k1), Some($k2), Some($k3), Some($k4)]
  };
}

macro_rules! keyboard_layout {
  ( $( $key:tt , )+ ) => {
    [$( keyboard_layout_key!($key), )+]
  };
}

const LAYOUT: [[Option<KeyCode>; 4]; 3] = keyboard_layout![
  [], [A], [LEFT_SHIFT, B],
];

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    println!("{:?}", LAYOUT);
  }
}
