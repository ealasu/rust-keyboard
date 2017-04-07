//#[cfg(feature = "std")] use std::mem;
//#[cfg(feature = "std")] use std::slice;
//#[cfg(not(feature = "std"))] use core::mem;
//#[cfg(not(feature = "std"))] use core::slice;
use core::mem;
use core::slice;

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

    pub fn update<F>(&mut self, left: u32, right: u32, cb: F)
    where F: FnOnce(KeyReport) {
        let report = KeyReport {
          reserved: 0,
          modifiers: 0,
          keys: [0,0,0,0,0,0],
        };
        cb(report);
        //let report_ptr: *const KeyReport = &report;
        //let data = unsafe { slice::from_raw_parts(report_ptr as *const u8, mem::size_of::<KeyReport>()) };
        //cb(data);
    }
}
