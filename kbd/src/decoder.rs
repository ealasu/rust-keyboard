use core::mem;
use core::slice;
use keys::Keys;
use layout::{Layout, KeyCombination, FnKeyCombination};

#[repr(C)]
pub struct KeyReport {
    pub modifiers: u8,
    pub reserved: u8,
    pub keys: [u8; 6],
}

pub struct Decoder {
    pub layer_fn: Layout<FnKeyCombination>,
    pub layer_0: Layout<KeyCombination>,
    pub layer_1: Layout<KeyCombination>,
    pub layer_2: Layout<KeyCombination>,
}

impl Decoder {
    pub fn update(&mut self, left: Keys, right: Keys) -> Option<KeyReport> {
        let report = KeyReport {
            modifiers: 0,
            reserved: 0,
            keys: [0, 0, 0, 0, 0, 0],
        };
        Some(report)
    }
}

//pub fn decode<T>(keys: Keys, layout: &Layout<T>) -> impl Iterator<Item=T>
//where T: Copy {
    //unimplemented!()
//}

