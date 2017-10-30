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

pub type Layout = [[[Option<KeyCode>; 4]; 12]; 4];

//const FN_LAYOUT: Layout = keyboard_layout![
//[[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
//[[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
//[[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
//[[ ], [TAB], [ ], [SHIFT], [ ], [ ], [ ], [ ], [FN], [ ], [ ], [ ]],
//];

const QWERTY_LAYOUT_1: Layout =
    keyboard_layout![
  [[Q], [W], [E], [R], [T], [ ], [ ], [Y], [U], [I], [O], [P]],
  [[A], [S], [D], [F], [G], [ ], [ ], [H], [J], [K], [L], [SEMICOLON]],
  [[Z], [X], [C], [V], [B], [ ], [ ], [N], [M], [COMMA], [DOT], [SLASH]],
  [[ESC], [TAB], [SUPER], [LEFT_SHIFT], [BACKSPACE],
    [LEFT_CTRL], [LEFT_ALT],
    [SPACE], [ ], [MINUS], [QUOTE], [ENTER]],
];

const QWERTY_LAYOUT_2: Layout =
    keyboard_layout![
  [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
  [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
  [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
  [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
];


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let layout =
            keyboard_layout![
      [[Q], [ ], [E | LEFT_SHIFT]],
    ];
        let expected = [
            [
                [Some(Q), None, None, None],
                [None, None, None, None],
                [Some(E), Some(LEFT_SHIFT), None, None],
            ],
        ];
        assert_eq!(&layout[..], &expected[..]);
    }
}
