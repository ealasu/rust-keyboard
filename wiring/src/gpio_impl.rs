use gpio::{Gpio, PinMode, PinState};

pub struct GpioImpl;

impl Gpio for GpioImpl {
    #[cfg(target_arch = "arm")]
    fn pin_mode(pin: u8, mode: PinMode) {
        unsafe { sys::pinMode(pin as u32, mode as u32) };
    }

    #[cfg(target_arch = "arm")]
    fn digital_write(pin: u8, state: PinState) {
        unsafe { sys::digitalWrite(pin as u32, state as u32) };
    }

    #[cfg(target_arch = "arm")]
    fn digital_read(pin: u8) -> PinState {
        let v = unsafe { sys::digitalRead(pin as u32) };
        match v {
            v if v == PinState::Low as u32 => PinState::Low,
            v if v == PinState::High as u32 => PinState::High,
            _ => panic!()
        }
    }


    #[cfg(target_arch = "msp430")]
    fn pin_mode(pin: u8, mode: PinMode) {
        unsafe { sys::pinMode(pin as u8, mode as u8) };
    }

    #[cfg(target_arch = "msp430")]
    fn digital_write(pin: u8, state: PinState) {
        unsafe { sys::digitalWrite(pin as u8, state as u8) };
    }

    #[cfg(target_arch = "msp430")]
    fn digital_read(pin: u8) -> PinState {
        let v = unsafe { sys::digitalRead(pin as u8) };
        match v {
            v if v == PinState::Low as u16 => PinState::Low,
            v if v == PinState::High as u16 => PinState::High,
            _ => panic!()
        }
    }


    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    fn pin_mode(pin: u8, mode: PinMode) {
        unimplemented!()
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    fn digital_write(pin: u8, state: PinState) {
        unimplemented!()
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    fn digital_read(pin: u8) -> PinState {
        unimplemented!()
    }
}


mod sys {
    #[cfg(target_arch = "msp430")]
    pub mod platform {
        extern {
            pub fn digitalWrite(pin: u8, state: u8);
            pub fn digitalRead(pin: u8) -> u16;
            pub fn pinMode(pin: u8, mode: u8);
        }
    }

    #[cfg(target_arch = "arm")]
    pub mod platform {
        extern {
            pub fn digitalWrite(pin: u32, state: u32);
            pub fn digitalRead(pin: u32) -> u32;
            pub fn pinMode(pin: u32, mode: u32);
        }
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    pub mod platform {
        pub fn digitalWrite(pin: u32, state: u32) { unimplemented!() }
        pub fn digitalRead(pin: u32) -> u32 { unimplemented!() }
        pub fn pinMode(pin: u32, mode: u32) { unimplemented!() }
    }

    pub use self::platform::*;
}
