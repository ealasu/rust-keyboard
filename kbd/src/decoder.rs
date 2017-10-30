use core::mem;
use core::slice;
use keys::Keys;

#[repr(C)]
pub struct KeyReport {
  pub modifiers: u8,
  pub reserved: u8,
  pub keys: [u8; 6],
}

pub struct Decoder {

}

impl Decoder {
  pub fn new() -> Self {
    Decoder {}
  }

  pub fn update(&mut self, left: Keys, right: Keys, cb: F) -> Option<KeyReport> {
    let report = KeyReport {
      modifiers: 0,
      reserved: 0,
      keys: [0,0,0,0,0,0],
    };
    Some(report)
  }
}
