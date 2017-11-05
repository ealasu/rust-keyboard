use keycode::{KeyCode, FnKeyCode};
use keycode::FnKeyCode::*;
use keycodes::*;
use keys::Keys;

pub type KeyCombination = [Option<KeyCode>; 4];
pub type FnKeyCombination = [Option<FnKeyCode>; 4];

#[macro_export]
macro_rules! keyboard_layout_key {
    ([]) => {
        //KeyComibination::None
        [None, None, None, None]
    };
    ([$k1:path]) => {
        //KeyComibination::Single($k1)
        [Some($k1), None, None, None]
    };
    ([$k1:path | $k2:path]) => {
        //KeyComibination::Modified($k1, $k2)
        [Some($k1), Some($k2), None, None]
    };
    //([$k1:path + $k2:path + $k3:path]) => {
        //KeyComibination::ModifiedKeyboard($k1, $k2 | $k3)
    //};
    //([$k1:path | $k2:path | $k3:path | $k4:path]) => {
        //[Some($k1), Some($k2), Some($k3), Some($k4)]
    //};
}

#[macro_export]
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
                $(
                    keyboard_layout_key!($key)
                ),+
                ,
            )+
        ]
    };
}


pub struct Layout<T>(pub [T; COLS * ROWS]);

impl <T: Copy> Layout<T> {
    pub fn decode<'a>(&'a self, keys: &'a Keys) -> impl Iterator<Item=T> + 'a {
        keys.on().map(move |key_idx| {
            self.0[key_idx as usize] 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let layout = keyboard_layout![
            [[Q], [ ], [E | LEFT_SHIFT]],
        ];
        let expected = [
            [Some(Q), None, None, None],
            [None, None, None, None],
            [Some(E), Some(LEFT_SHIFT), None, None],
        ];
        assert_eq!(&layout[..], &expected[..]);
    }
}
