use keycode::KeyCode;
use keycodes::*;

macro_rules! keyboard_layout_key {
  ([]) => {
    [None, None, None, None]
  };
  ([$k1:path]) => {
    [Some($k1), None, None, None]
  };
  ([$k1:path | $k2:path]) => {
    [Some($k1), Some($k2), None, None]
  };
  ([$k1:path | $k2:path | $k3:path]) => {
    [Some($k1), Some($k2), Some($k3), None]
  };
  ([$k1:path | $k2:path | $k3:path | $k4:path]) => {
    [Some($k1), Some($k2), Some($k3), Some($k4)]
  };
}

macro_rules! keyboard_layout {
  (
    $(
      [
        $(
           $key:tt 
        ),+
      ],
    )+
  ) => {
    [
      $(
        [
          $(
            keyboard_layout_key!($key)
          ),+
        ],
      )+
    ]
  };
}

pub type Layout = [[[Option<KeyCode>; 4]; 10]; 2];

const QWERTY_LAYOUT: Layout = keyboard_layout![
  [[Q], [W], [E], [R], [T], [Y], [U], [I], [O], [P]], 
  [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]], 
  [[Q | LEFT_SHIFT]],
];

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {

    println!("{:?}", QWERTY_LAYOUT);
  }
}
