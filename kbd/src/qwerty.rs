use decoder::Decoder;
#[macro_use]
use layout::Layout;
use keycode::FnKeyCode::*;
use keycodes::*;

pub static DECODER: Decoder = Decoder {
    layer_fn: Layout(keyboard_layout![
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [FN2], [ ], [FN1], [ ], [ ], [ ], [ ], [FN0], [ ], [ ], [ ]],
    ]),
    layer_0: Layout(keyboard_layout![
        [[Q], [W], [E], [R], [T], [ ], [ ], [Y], [U], [I], [O], [P]],
        [[A], [S], [D], [F], [G], [ ], [ ], [H], [J], [K], [L], [SEMICOLON]],
        [[Z], [X], [C], [V], [B], [ ], [ ], [N], [M], [COMMA], [DOT], [SLASH]],
        [[ESC], [TAB], [SUPER], [LEFT_SHIFT], [BACKSPACE],
          [LEFT_CTRL], [LEFT_ALT],
          [SPACE], [ ], [MINUS], [QUOTE], [ENTER]],
    ]),
    layer_1: Layout(keyboard_layout![
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
    ]),
    layer_2: Layout(keyboard_layout![
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
        [[ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ], [ ]],
    ])
};
