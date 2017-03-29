use serial::Serial;

pub struct SerialImpl;

impl Serial for SerialImpl {
    fn read() -> Option<u8> {
        let v = unsafe { sys::serial_read() };
        if v < 0 {
            None
        } else {
            Some(v as u8)
        }
    }

    fn write(v: u8) {
        unsafe { sys::serial_write(v); }
    }
}

mod sys {
    extern {
        pub fn serial_write(b: u8) -> u8;
    }

    #[cfg(target_arch = "msp430")]
    pub mod platform {
        extern {
            pub fn serial_read() -> i16;
        }
    }

    #[cfg(target_arch = "arm")]
    pub mod platform {
        extern {
            pub fn serial_read() -> i32;
        }
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    pub mod platform {
        pub fn serial_read() -> i32 { unimplemented!() }
    }

    pub use self::platform::*;
}
